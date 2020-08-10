use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub uuid: String,
    pub main_net: String,
    pub main_net_mask: String,
    pub server_port: u16,
    pub server_public_ip: String,
}


pub fn config() -> Config {
    use std::fs::read_to_string;
    match toml::from_str(&read_to_string("config/config.toml").unwrap_or_default()) {
        Ok(config) => config,
        Err(error) => panic!("Failed to read config: {}", error),
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
    if config.server_port == 0 {
        panic!("Config: server_port can't be 0!")
    }
}
