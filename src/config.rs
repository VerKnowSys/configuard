use serde_derive::Deserialize;
use std::path::Path;


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub uuid: String,
    pub main_net: String,
    pub main_net_mask: String,
    pub server_port: u16,
    pub server_public_ip: String,
    pub wireguard_conf: String,
    pub wireguard_bin: String,
    pub error_log: String,
}


pub fn config() -> Config {
    use std::fs::read_to_string;
    match toml::from_str(&read_to_string("config/config.toml").unwrap_or_default()) {
        Ok(config) => config,
        Err(error) => panic!("Failed to read config: {error}"),
    }
}


pub fn validate_config(config: &Config) {
    if config.server_public_ip.is_empty() {
        panic!("Config: server_public_ip endpoint can't be empty!")
    }
    if config.main_net.is_empty() {
        panic!("Config: main_net can't be empty!")
    }
    if config.main_net_mask.is_empty() {
        panic!("Config: main_net_mask can't be empty!")
    }
    if config.uuid.is_empty() {
        panic!("Config: uuid can't be empty!")
    }
    if config.wireguard_conf.is_empty() || !Path::new(&config.wireguard_conf).exists() {
        panic!("Config: wireguard_conf has to be path to wg0.conf and has to exist!")
    }
    if config.wireguard_bin.is_empty() || !Path::new(&config.wireguard_bin).exists() {
        panic!("Config: wireguard_bin has to be path to wg binary!")
    }
    if config.server_port == 0 {
        panic!("Config: server_port can't be 0!")
    }
}
