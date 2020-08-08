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
    // println!("SEC ENC: {}", private_key);
    // println!("PUB ENC: {}", public_key);

    let server_template = WireguardServerConfigurationTemplate {
        server_port: SERVER_PORT,
        server_private_key: SERVER_PRIVATE_KEY,
    };

    // TODO: iterate over user entries and generate full list
    let server_entry_template = WireguardServerConfigurationEntryTemplate {
        user_name: &name,
        user_public_key: &public_key,
        user_ips: "",
    };
    // let server_config_entries_rendered =
    // let server_config_rendered = format!("{}\n\n{}\n", server_template.render().unwrap_or_default(), server_config_entries_rendered);

    // if IP entry with given name already exists - we wish to re-use it:
    let existing_entry = Path::new(&format!("entries/workstations/{}", name)).to_owned();
    let user_ipv4 = if existing_entry.exists() {
        read_to_string(existing_entry)
            .unwrap_or_default()
            .replace('\n', "")
    } else {
        let all_used_ipv4s = WalkDir::new("entries/workstations/")
            .into_iter()
            .filter_map(|v| v.ok())
            .filter(|file| file.path().is_file())
            .filter_map(|file| read_to_string(file.path()).ok())
            .map(|line| line.replace('\n', ""))
            .collect::<Vec<_>>();

        let last_ipv4 = match find_last_ipv4(all_used_ipv4s) {
            Some(ipv4) => ipv4,
            None => SERVER_ROUTER_IP.to_string(), /* if list of entries is empty, assign next address after router */
        };

        let ipv4 = match next_workstation_ipv4(&last_ipv4) {
            Some(ipv4) => ipv4,
            None => panic!("Address pool exhausted!"),
        };
        // store entry for user:
        write_atomic(&format!("entries/workstations/{}", name), &ipv4);
        ipv4
    };
    let user_nets = format!("{}{}", user_ipv4, MAIN_MASK);

    // TODO: setup_routes(); // route add 100.64.64.1/32 -interface wg0
    // route -6 add fde4:82c4:04eb:dd8d::1:5 -interface wg0

    // TODO: sync_wg_confifguration(); // wg syncconf wg0 wg0.conf

    let user_template = WireguardWorkstationTemplate {
        user_name: &name,
        user_private_key: &private_key,
        user_nets: &user_nets,
        server_public_key: SERVER_PUBLIC_KEY,
        default_server_endpoint: &format!("{}:{}", SERVER_IP, SERVER_PORT),
    };

    format!("{}\n", user_template.render().unwrap_or_default())
}
