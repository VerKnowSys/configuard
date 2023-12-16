use askama::Template;


/// Wireguard Configuration Templates

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
    pub modified_at: &'a str,
}


/// Command Templates

#[derive(Template, Debug)]
#[template(path = "bridge-setup.sh", escape = "none")]
pub struct BridgeRouterAliasTemplate<'a> {
    pub router_ip_address: &'a str,
    pub net_mask: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "route-add.sh", escape = "none")]
pub struct RouteAddTemplate<'a> {
    pub ipv4_address: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "route-del.sh", escape = "none")]
pub struct RouteDelTemplate<'a> {
    pub ipv4_address: &'a str,
}


#[derive(Template, Debug)]
#[template(path = "wg-syncconf.sh", escape = "none")]
pub struct WireguardSyncConfigTemplate<'a> {
    pub wireguard_bin: &'a str,
    pub wireguard_conf: &'a str,
}
