use askama::Template;
use std::{
    fs::{File, OpenOptions},
    io::Error,
    process::{Command, ExitStatus, Stdio},
};


pub fn next_workstation_ipv4(ipv4: &str) -> Option<String> {
    match ipv4
        .split('.')
        .map(|elem| elem.parse().unwrap_or(1))
        .collect::<Vec<u8>>()
        .as_slice()
    {
        [a, b, c, d] => {
            if d == &0 {
                // disallow assigning .1 address reserved for routers:
                Some(format!("{}.{}.{}.{}", a, b, c, d + 2))
            } else if d < &(u8::MAX - 1) {
                Some(format!("{}.{}.{}.{}", a, b, c, d + 1))
            } else {
                None
            }
        }
        _ => None,
    }
}


pub fn find_last_ipv4(list: Vec<String>) -> Option<String> {
    if list.is_empty() {
        None
    } else {
        let mut out = list;
        out.sort_by(|a, b| {
            use std::net::Ipv4Addr;
            let first: Ipv4Addr = a.parse().unwrap();
            let second: Ipv4Addr = b.parse().unwrap();
            // sort IPv4 by octets
            first.octets().partial_cmp(&second.octets()).unwrap()
        });

        out.last().map(|last| last.to_string())
    }
}


/// Invoke command defined by a defined command template
pub fn run(log_file: &str, template: impl Template) -> Result<ExitStatus, Error> {
    let rendered = template
        .render()
        .unwrap_or_default()
        .replace(['\t', '\n', '\r', '\\'], " ");

    let command = rendered.split_whitespace().collect::<Vec<&str>>();

    let command_name = &command[0];
    let command_args = &command[1..];

    let mut options = OpenOptions::new();
    let out_file = options.create(true).append(true).open(log_file)?;
    let err_file = options.create(true).append(true).open(log_file)?;

    Command::new(command_name)
        .args(command_args)
        .stdin(Stdio::null())
        .stdout(out_file)
        .stderr(err_file)
        .spawn()?
        .wait()
}


/// Write-once-and-atomic to a file
pub fn write_atomic(file_path: &str, contents: &str) {
    use std::io::prelude::*;

    // NOTE: since file is written in "write only, all at once" mode, we have to be sure not to write empty buffer
    if !contents.is_empty() {
        match File::create(file_path) {
            Ok(mut file) => {
                file.write_all(contents.as_bytes()).unwrap_or_else(|_| {
                    panic!("Access denied? File can't be written: {file_path}")
                });
            }

            Err(err) => {
                panic!("Atomic write to: {file_path} has failed! Cause: {err}")
            }
        }
    }
}


pub fn first_of_pair(line: String) -> Option<String> {
    let vector = line.split(',').collect::<Vec<_>>();
    vector
        .first()
        .map(|first_element| first_element.replace('\n', ""))
}


pub fn both_elements(line: String) -> Option<(String, String)> {
    let vector = line.split(',').collect::<Vec<_>>();
    if let (Some(ip), Some(pubkey)) = (vector.first(), vector.last()) {
        Some((ip.to_string(), pubkey.to_string()))
    } else {
        None
    }
}
