use crate::templates::WireguardServerConfigurationEntryTemplate;
use crate::templates::WireguardServerConfigurationTemplate;
use crate::templates::WireguardWorkstationTemplate;
use crate::utils::find_last_ipv4;
use crate::utils::generate_wireguard_keys;
use crate::utils::next_workstation_ipv4;
use crate::utils::write_atomic;
use crate::MAIN_MASK;
use crate::SERVER_IP;
use crate::SERVER_PORT;
use crate::SERVER_PRIVATE_KEY;
use crate::SERVER_PUBLIC_KEY;
use crate::SERVER_ROUTER_IP;
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
        let vector = line.split(',').collect::<Vec<_>>();
        if let Some(first_element) = vector.first() {
            first_element.replace('\n', "")
        } else {
            String::from("0.0.0.0")
        }
    } else {
        let all_used_ipv4s = WalkDir::new("entries/workstations/")
            .into_iter()
            .filter_map(|v| v.ok())
            .filter(|file| file.path().is_file())
            .filter_map(|file| read_to_string(file.path()).ok())
            .filter_map(|line| {
                let vector = line.split(',').collect::<Vec<_>>();
                if let Some(first_element) = vector.first() {
                    Some(first_element.replace('\n', ""))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        let last_ipv4 = match find_last_ipv4(all_used_ipv4s) {
            Some(ipv4) => ipv4,
            None => SERVER_ROUTER_IP.to_string(), /* if list of entries is empty, assign next address after router */
        };

        let ipv4 = match next_workstation_ipv4(&last_ipv4) {
            Some(ipv4) => ipv4,
            None => panic!("Address pool exhausted!"),
        };
        // store entry for user:
        write_atomic(
            &format!("entries/workstations/{}", name),
            &format!("{},{}", ipv4, public_key),
        );
        ipv4
    };
    let user_nets = format!("{}{}", user_ipv4, MAIN_MASK);

    // server main template
    let server_template = (WireguardServerConfigurationTemplate {
        server_port: SERVER_PORT,
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
    let all_entries_ipv4s = all_entries_files
        .iter()
        .filter_map(|file| read_to_string(file.path()).ok())
        .filter_map(|line| {
            let vector = line.split(',').collect::<Vec<_>>();
            if let (Some(ip), Some(pubkey)) = (vector.first(), vector.last()) {
                Some((ip.to_string(), pubkey.to_string()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let zipped = all_entries_files.iter().zip(&all_entries_ipv4s);

    // render server configuration header and append entries based on available entries
    let server_config_entries_rendered = zipped
        .map(|(config_name, (ip, pubkey))| {
            // entries
            format!(
                "{}\n",
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
        "{}\n\n{}\n",
        server_template, server_config_entries_rendered
    );

    println!("SERVER_CONFIG:\n{}", server_config_rendered);

    // TODO: setup_routes(); // route add 100.64.64.1/32 -interface wg0
    // route -6 add fde4:82c4:04eb:dd8d::1:5 -interface wg0

    // TODO: sync_wg_confifguration(); // wg syncconf wg0 wg0.conf

    let user_template = WireguardWorkstationTemplate {
        user_name: &name,
        user_private_key: &private_key,
        user_nets: &user_nets,
        server_public_key: &read_to_string(SERVER_PUBLIC_KEY)
            .unwrap_or_default()
            .replace('\n', ""),
        default_server_endpoint: &format!("{}:{}", SERVER_IP, SERVER_PORT),
    };

    format!("{}\n", user_template.render().unwrap_or_default())
}
