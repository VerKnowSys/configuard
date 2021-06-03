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
use std::{fs::read_to_string, path::Path};


#[post("/<name>")]
pub fn new(name: String) -> String {
    Lockfile::create(format!("/tmp/workstation-{}.lock", name))
        .and_then(|lockfile| {
            // throw a decoy if name doesn't match requirements
            if !FILE_NAME_REGEX.is_match(&name) {
                return Ok(new_decoy());
            }

            let (private_key, public_key) = generate_wireguard_keys();

            // if IP entry with given name already exists - we wish to re-use it:
            let existing_entry =
                Path::new(&format!("{}{}{}", ENTRIES_DIR, WORKSTATIONS_DIR, name)).to_owned();
            let user_ipv4 = if existing_entry.exists() {
                let line = read_to_string(existing_entry).unwrap_or_default();
                first_of_pair(line).unwrap_or_default()
            } else {
                let all_used_ipv4s = read_all_used_ipv4(WORKSTATIONS_DIR);
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
                &format!("{}{}{}", ENTRIES_DIR, WORKSTATIONS_DIR, name),
                &format!("{},{}", user_ipv4, public_key),
            );

            let user_nets = format!("{}{}", user_ipv4, config().main_net_mask);

            // server main template
            let server_template = render_server_config_head();

            // iterate over all entries, build public side of server-side wireguard server configuration
            let server_config_entries_rendered = render_all_entries();

            let server_config_rendered = format!(
                "{}\n\n\n{}\n",
                server_template, server_config_entries_rendered
            );

            // write altered server configuration:
            write_atomic(&config().wireguard_conf, &server_config_rendered);

            // commit changes to system
            commit_wireguard_configuration(&user_ipv4);

            lockfile.release()?; // or just let the lockfile be dropped

            let user_template = WireguardWorkstationTemplate {
                user_name: &name,
                user_private_key: &private_key,
                user_nets: &user_nets,
                server_public_key: &read_server_key(SERVER_PUBLIC_KEY),
                default_server_endpoint: &format!(
                    "{}:{}",
                    config().server_public_ip,
                    config().server_port
                ),
            };

            Ok(format!("{}\n", user_template.render().unwrap_or_default()))
        })
        .unwrap_or_else(|_| new_decoy())
}
