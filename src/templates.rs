use askama::Template;


#[derive(Template, Debug)]
#[template(path = "workstation.conf", escape = "none")]
pub struct WireguardWorkstationTemplate<'a> {
    pub user_name: &'a str,
    pub user_private_key: &'a str,
    pub user_nets: &'a str,
    pub server_public_key: &'a str,
    pub default_server_endpoint: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "instance.conf", escape = "none")]
pub struct WireguardInstanceTemplate<'a> {
    pub user_name: &'a str,
    pub user_private_key: &'a str,
    pub user_nets: &'a str,
    pub server_router_ip: &'a str,
    pub server_public_key: &'a str,
    pub default_server_endpoint: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "server.conf", escape = "none")]
pub struct WireguardServerConfigurationTemplate<'a> {
    pub server_port: &'a str,
    pub server_private_key: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "server-entry.conf", escape = "none")]
pub struct WireguardServerConfigurationEntryTemplate<'a> {
    pub user_name: &'a str,
    pub user_public_key: &'a str,
    pub user_ips: &'a str,
}
