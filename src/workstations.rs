use crate::config::*;
use crate::templates::WireguardServerConfigurationEntryTemplate;
use crate::templates::WireguardServerConfigurationTemplate;
use crate::templates::WireguardWorkstationTemplate;
use crate::utils::both_elements;
use crate::utils::find_last_ipv4;
use crate::utils::first_of_pair;
use crate::utils::generate_wireguard_keys;
use crate::utils::next_workstation_ipv4;
use crate::utils::write_atomic;
use crate::SERVER_PRIVATE_KEY;
use crate::SERVER_PUBLIC_KEY;
use askama::Template;
use rocket::{ignite, Rocket};
use std::{fs::read_to_string, path::Path};
use walkdir::{DirEntry, WalkDir};


#[post("/<name>")]
pub fn new(name: String) -> String {
    let (private_key, public_key) = generate_wireguard_keys();

    // if IP entry with given name already exists - we wish to re-use it:
    let existing_entry = Path::new(&format!("entries/workstations/{}", name)).to_owned();
    let user_ipv4 = if existing_entry.exists() {
        let line = read_to_string(existing_entry).unwrap_or_default();
        first_of_pair(&line).unwrap_or_default()
    } else {
        let all_used_ipv4s = WalkDir::new("entries/workstations/")
            .into_iter()
            .filter_map(|v| v.ok())
            .filter(|file| file.path().is_file())
            .filter_map(|file| read_to_string(file.path()).ok())
            .filter_map(|line| first_of_pair(&line))
            .collect::<Vec<String>>();

        let last_ipv4 = match find_last_ipv4(all_used_ipv4s) {
            Some(ipv4) => ipv4,
            None => format!("{}.1.1", config().main_net), /* if list of entries is empty, assign next address after router */
        };
        match next_workstation_ipv4(&last_ipv4) {
            Some(ipv4) => ipv4,
            None => panic!("Address pool exhausted!"),
        }
    };

    // store entry for user with new generated pubkey:
    write_atomic(
        &format!("entries/workstations/{}", name),
        &format!("{},{}", user_ipv4, public_key),
    );

    let user_nets = format!("{}{}", user_ipv4, config().main_net_mask);

    // server main template
    let server_template = (WireguardServerConfigurationTemplate {
        server_port: &format!("{}", config().server_port),
        server_private_key: &read_to_string(SERVER_PRIVATE_KEY)
            .unwrap_or_default()
            .replace('\n', ""),
    })
    .render()
    .unwrap_or_default();

    // iterate over all entries, build public side of server-side wireguard server configuration
    let all_entries_files = WalkDir::new("entries/")
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|file| file.path().is_file())
        .collect::<Vec<_>>();
    let all_entries_ipv4s_and_pubkeys = all_entries_files
        .iter()
        .filter_map(|file| read_to_string(file.path()).ok())
        .filter_map(|line| both_elements(&line))
        .collect::<Vec<_>>();

    // render server configuration header and append entries based on available entries
    let server_config_entries_rendered = all_entries_files
        .iter()
        .zip(&all_entries_ipv4s_and_pubkeys)
        .map(|(config_name, (ip, pubkey))| {
            // entries
            format!(
                "{}\n\n",
                (WireguardServerConfigurationEntryTemplate {
                    user_name: &config_name
                        .file_name()
                        .to_os_string()
                        .into_string()
                        .unwrap_or_default(),
                    user_ips: ip,
                    user_public_key: pubkey,
                })
                .render()
                .unwrap_or_default()
            )
        })
        .collect::<String>();

    let server_config_rendered = format!(
        "{}\n\n\n{}\n",
        server_template, server_config_entries_rendered
    );

    println!("SERVER_CONFIG:\n{}", server_config_rendered);

    // NOTE: Create bridge0 with router ip assigned to it. Don't assign .1.1 to server-side wg
    // TODO: add bridge0 preconfiguration as below:
    // ifconfig bridge0 inet 100.64.1.1/10 up

    // TODO: add_routes(); // route add 100.64.64.1/32 -interface wg0
    // route -6 add fde4:82c4:04eb:dd8d::1:5 -interface wg0

    // TODO: sync_wg_configuration(); // wg syncconf wg0 wg0.conf

    let user_template = WireguardWorkstationTemplate {
        user_name: &name,
        user_private_key: &private_key,
        user_nets: &user_nets,
        server_public_key: &read_to_string(SERVER_PUBLIC_KEY)
            .unwrap_or_default()
            .replace('\n', ""),
        default_server_endpoint: &format!(
            "{}:{}",
            config().server_public_ip,
            config().server_port
        ),
    };

    format!("{}\n", user_template.render().unwrap_or_default())
}
