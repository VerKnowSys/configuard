#![forbid(unsafe_code)]
#![deny(
        // missing_docs,
        unstable_features,
        unsafe_code,
        missing_debug_implementations,
        // missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]
#![allow(unused_imports, dead_code)]


#[macro_use]
extern crate rocket;

use rocket::{ignite, Rocket};


mod instances;
mod templates;
mod utils;
mod workstations;

#[cfg(test)]
mod tests;


const SERVER_IP: &str = "[[ your-external-ip-address ]]";
const SERVER_PORT: &str = "61194";
const SERVER_PUBLIC_KEY: &str = "/Services/Wireguard-tools/pub.key";
const SERVER_PRIVATE_KEY: &str = "/Services/Wireguard-tools/private.key";

const MAIN_NET: &str = "100.64";
// const MAIN_NET6: &str = "fde4:82c4:4eb:dd8d::";
const WORKSTATIONS_SUBNET: &str = "1";

const SERVER_ROUTER_IP: &str = "100.64.1.1";
// const SERVER_ROUTER_IP6: &str = "fde4:82c4:4eb:dd8d::1:1";
const MAIN_MASK: &str = "/10";
// const MAIN_MASK6: &str = "/64";


#[launch]
fn start() -> Rocket {
    ignite()
        .mount(
            "/796A425F-DD50-4A61-B535-92920EA54818/wireguard/new/instance/",
            routes![instances::new],
        )
        .mount(
            "/796A425F-DD50-4A61-B535-92920EA54818/wireguard/new/workstation/",
            routes![workstations::new],
        )
}
