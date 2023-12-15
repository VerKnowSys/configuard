#![forbid(unsafe_code)]
#![deny(
        // missing_docs,
        unstable_features,
        missing_debug_implementations,
        // missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        // unused_qualifications,
        )]
// #![allow(unused_imports, dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

pub use crate::{
    common::new_decoy,
    config::{config, validate_config},
};
pub use regex::Regex;

pub mod common;
mod config;
mod templates;
pub mod utils;
pub mod workstations;


pub const ENTRIES_DIR: &str = "entries/";
pub const WORKSTATIONS_DIR: &str = "workstations/";
pub const SERVER_PUBLIC_KEY: &str = "/Services/Wireguard-tools/pub.key";
pub const SERVER_PRIVATE_KEY: &str = "/Services/Wireguard-tools/private.key";
pub const NULL_LOG: &str = "/dev/null";


lazy_static! {
    // this will be reused after first regex compilation:
    static ref FILE_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9 -\.]{3,}$").unwrap();
}

#[cfg(test)]
mod tests;
