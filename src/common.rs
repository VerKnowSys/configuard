use crate::{
    config::config,
    templates::{
        BridgeRouterAliasTemplate, RouteAddTemplate, RouteDelTemplate,
        WireguardServerConfigurationEntryTemplate, WireguardServerConfigurationTemplate,
        WireguardSyncConfigTemplate, WireguardWorkstationTemplate,
    },
    utils::{both_elements, first_of_pair, run},
    SERVER_PRIVATE_KEY, SERVER_PUBLIC_KEY,
};
use askama::Template;
use chrono::prelude::*;
use rand_core::OsRng;
use std::{ffi::OsStr, fs::read_to_string};
use walkdir::{DirEntry, WalkDir};
use x25519_dalek::{PublicKey, StaticSecret};


pub fn generate_wireguard_keys() -> (String, String) {
    let private = StaticSecret::new(&mut OsRng);
    let public = PublicKey::from(&private);
    let public_b64 = base64::encode(public.as_bytes());
    let private_b64 = base64::encode(private.to_bytes());
    (private_b64, public_b64)
}


pub fn commit_wireguard_configuration(
    user_ipv4: &str,
    main_net: &str,
    main_net_mask: &str,
    wireguard_bin: &str,
    wireguard_conf: &str,
    error_log: &str,
) {
    // NOTE: Create bridge0 with router ip assigned to it. Don't assign .1.1 to server-side wg
    // println!("Setting up bridge0");
    run(
        error_log,
        BridgeRouterAliasTemplate {
            router_ip_address: &format!("{main_net}.1.1"),
            net_mask: main_net_mask,
        },
    )
    .ok();

    // for ipv6: route -6 add fde4:82c4:04eb:dd8d::1:5 -interface wg0
    // println!("Setting up Wireguard routes for: {}", &user_ipv4);
    run(
        error_log,
        RouteDelTemplate {
            ipv4_address: user_ipv4,
        },
    )
    .ok();

    run(
        error_log,
        RouteAddTemplate {
            ipv4_address: user_ipv4,
        },
    )
    .ok();

    // println!("Synchronizing server configuration");
    run(
        error_log,
        WireguardSyncConfigTemplate {
            wireguard_bin,
            wireguard_conf,
        },
    )
    .ok();
}


pub fn render_entry(config_name: &DirEntry, user_ips: &str, user_public_key: &str) -> String {
    let local_time: DateTime<Local> = Local::now();
    let modified_at = &local_time.to_rfc3339();
    let user_name = &file_name_to_string(config_name.file_name());
    (WireguardServerConfigurationEntryTemplate {
        user_name,
        user_ips,
        user_public_key,
        modified_at,
    })
    .render()
    .unwrap_or_default()
}


pub fn read_all_entries(entries_dir: &str) -> (Vec<DirEntry>, Vec<(String, String)>) {
    let dir_entries = read_files_list(entries_dir);
    let ipv4s_and_pubkeys = dir_entries
        .iter()
        .filter_map(|file| read_to_string(file.path()).ok())
        .filter_map(both_elements)
        .collect::<Vec<_>>();
    (dir_entries, ipv4s_and_pubkeys)
}


pub fn render_all_entries(entries_dir: &str) -> String {
    let (dir_entries, ipv4s_and_pubkeys) = read_all_entries(entries_dir);
    dir_entries.iter().zip(&ipv4s_and_pubkeys).fold(
        String::new(),
        |mut rendered_configuration, (config_name, (ip, pubkey))| {
            rendered_configuration.push_str(&format!(
                "{entry}\n\n",
                entry = render_entry(config_name, ip, pubkey)
            ));
            rendered_configuration
        },
    )
}


pub fn render_server_config_head(server_port: u16) -> String {
    (WireguardServerConfigurationTemplate {
        server_port: &format!("{server_port}"),
        server_private_key: &read_server_key(SERVER_PRIVATE_KEY),
    })
    .render()
    .unwrap_or_default()
}


pub fn is_not_hidden_file(file: &DirEntry) -> bool {
    file.path().is_file() && !file_name_to_string(file.file_name()).starts_with('.')
}


pub fn read_files_list(from_dir: &str) -> Vec<DirEntry> {
    WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(is_not_hidden_file)
        .collect()
}


pub fn file_name_to_string(name: &OsStr) -> String {
    name.to_os_string().into_string().unwrap_or_default()
}


pub fn read_all_used_ipv4(from_dir: &str) -> Vec<String> {
    WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(is_not_hidden_file)
        .filter_map(|file| read_to_string(file.path()).ok())
        .filter_map(first_of_pair)
        .collect()
}


pub fn read_server_key(file: &str) -> String {
    read_to_string(file).unwrap_or_default().replace('\n', "")
}


pub fn random_name(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}


pub fn random_byte() -> u8 {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    rng.gen_range(2, 254)
}


pub fn random_word() -> u16 {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    rng.gen_range(966, 65535)
}


pub fn new_decoy() -> String {
    let (private_key, _) = generate_wireguard_keys();
    let user_ipv4 = format!(
        "{}.{}.{}.{}",
        random_byte(),
        random_byte(),
        random_byte(),
        random_byte()
    );
    let user_nets = format!("{}/{}", user_ipv4, random_byte() % 32 + 9);
    let user_template = WireguardWorkstationTemplate {
        user_name: &random_name(10),
        user_private_key: &private_key,
        user_nets: &user_nets,
        server_public_key: &read_server_key(SERVER_PUBLIC_KEY),
        default_server_endpoint: &format!("{}:{}", config().server_public_ip, &random_word()),
    };

    format!("{}\n", user_template.render().unwrap_or_default())
}
