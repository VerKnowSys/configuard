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

use crate::config::config;
use crate::config::validate_config;
use rocket::{ignite, Rocket};


mod common;
mod config;
mod instances;
mod templates;
mod utils;
mod workstations;

#[cfg(test)]
mod tests;


const ENTRIES_DIR: &str = "entries/";
const INSTANCES_DIR: &str = "instances/";
const WORKSTATIONS_DIR: &str = "workstations/";
const SERVER_PUBLIC_KEY: &str = "/Services/Wireguard-tools/pub.key";
const SERVER_PRIVATE_KEY: &str = "/Services/Wireguard-tools/private.key";


#[launch]
fn start() -> Rocket {
    // TODO: add validation if os is local vs server
    // TODO: add validation if user is root
    validate_config(&config());
    ignite()
        .mount(
            &format!("/{}/wireguard/new/instance/", config().uuid),
            routes![instances::new],
        )
        .mount(
            &format!("/{}/wireguard/new/workstation/", config().uuid),
            routes![workstations::new],
        )
}
