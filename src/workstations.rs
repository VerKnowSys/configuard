use crate::{
    common::{
        commit_wireguard_configuration, generate_wireguard_keys, new_decoy,
        read_all_used_ipv4, read_server_key, render_all_entries, render_server_config_head,
    },
    config::*,
    templates::WireguardWorkstationTemplate,
    utils::{find_last_ipv4, first_of_pair, next_workstation_ipv4, write_atomic},
    ENTRIES_DIR, FILE_NAME_REGEX, SERVER_PUBLIC_KEY, WORKSTATIONS_DIR,
};
use askama::Template;
use lockfile::Lockfile;
use std::{fs::read_to_string, io::Error, path::Path};


pub fn new_configuration(user_name: &str) -> Result<String, Error> {
    let (user_private_key, user_public_key) = generate_wireguard_keys();
    let config = config();
    let main_net = config.main_net;
    let main_net_mask = config.main_net_mask;
    let server_public_ip = config.server_public_ip;
    let server_port = config.server_port;
    let wireguard_conf = config.wireguard_conf;
    let wireguard_bin = config.wireguard_bin;

    // if IP entry with given name already exists - we wish to re-use it:
    let existing_entry =
        Path::new(&format!("{ENTRIES_DIR}{WORKSTATIONS_DIR}{user_name}")).to_owned();
    let user_ipv4 = if existing_entry.exists() {
        let line = read_to_string(existing_entry).unwrap_or_default();
        first_of_pair(line).unwrap_or_default()
    } else {
        let all_used_ipv4s = read_all_used_ipv4(&format!("{ENTRIES_DIR}{WORKSTATIONS_DIR}"));
        let last_ipv4 = match find_last_ipv4(all_used_ipv4s) {
            Some(ipv4) => ipv4,
            None => format!("{main_net}.1.1"), /* if list of entries is empty, assign next address after router */
        };
        match next_workstation_ipv4(&last_ipv4) {
            Some(ipv4) => ipv4,
            None => panic!("Address pool exhausted!"),
        }
    };
    let user_nets = &format!("{user_ipv4}{main_net_mask}");

    // store entry for user with new generated pubkey:
    write_atomic(
        &format!("{ENTRIES_DIR}{WORKSTATIONS_DIR}{user_name}"),
        &format!("{user_ipv4},{user_public_key}"),
    );

    // server main template
    let server_template = render_server_config_head(server_port);

    // iterate over all entries, build public side of server-side wireguard server configuration
    let server_config_entries_rendered = render_all_entries(ENTRIES_DIR);

    let server_config_rendered =
        format!("{server_template}\n\n\n{server_config_entries_rendered}\n");

    // write altered server configuration:
    write_atomic(&wireguard_conf, &server_config_rendered);

    // commit changes to system
    commit_wireguard_configuration(
        &user_ipv4,
        &main_net,
        &main_net_mask,
        &wireguard_bin,
        &wireguard_conf,
    );

    let server_public_key = &read_server_key(SERVER_PUBLIC_KEY);
    let user_private_key = &user_private_key;
    let default_server_endpoint = &format!("{server_public_ip}:{server_port}");
    let user_template = WireguardWorkstationTemplate {
        user_name,
        user_nets,
        server_public_key,
        user_private_key,
        default_server_endpoint,
    };
    let rendered_template = user_template.render().unwrap_or_default();
    Ok(format!("{rendered_template}\n"))
}


#[post("/<user_name>")]
pub fn new(user_name: &str) -> String {
    Lockfile::create(format!("/tmp/.configuard.workstation-{user_name}.lock"))
        .and_then(|lockfile| {
            // throw a decoy if name doesn't match requirements
            if !FILE_NAME_REGEX.is_match(user_name) {
                return Ok(new_decoy());
            }
            new_configuration(user_name).and_then(|the_config| {
                lockfile.release()?;
                Ok(the_config)
            })
        })
        .unwrap_or_else(|_| new_decoy())
}
