use askama::Template;
use rand_core::OsRng;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Output, Stdio};
use x25519_dalek::{PublicKey, StaticSecret};


pub fn generate_wireguard_keys() -> (String, String) {
    let private = StaticSecret::new(&mut OsRng);
    let public = PublicKey::from(&private);
    let public_b64 = base64::encode(public.as_bytes());
    let private_b64 = base64::encode(&private.to_bytes());
    (private_b64, public_b64)
}


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


pub fn next_instance_ipv4(ipv4: &str) -> Option<String> {
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
            } else if d == &(u8::MAX - 1) && c < &(u8::MAX - 2) {
                Some(format!("{}.{}.{}.{}", a, b, c + 2, 2))
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

        match out.last() {
            Some(last) => Some(last.to_string()),
            None => None,
        }
    }
}


#[test]
fn check_find_last_ipv4() {
    let list = vec![
        "127.0.0.150".to_string(),
        "127.0.0.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.0.120".to_string(),
        "127.0.0.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list), Some("127.0.0.150".to_string()));

    let list2 = vec![
        "127.0.0.150".to_string(),
        "127.0.0.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.0.120".to_string(),
        "127.0.2.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list2), Some("127.0.2.100".to_string()));

    let list2 = vec![
        "127.0.8.150".to_string(),
        "127.0.224.3".to_string(),
        "127.0.0.10".to_string(),
        "127.0.118.120".to_string(),
        "127.0.2.100".to_string(),
    ];
    assert_eq!(find_last_ipv4(list2), Some("127.0.224.3".to_string()));
}


/// Invoke command defined by a defined command template
pub fn run(log_file: &str, template: impl Template) -> Result<ExitStatus, Error> {
    let rendered = template
        .render()
        .unwrap_or_default()
        .replace("\t", " ")
        .replace("\n", " ")
        .replace("\r", " ")
        .replace("\\", " ");

    let command = rendered.split_whitespace().collect::<Vec<&str>>();

    let command_name = &command[0];
    let command_args = &command[1..];

    let mut options = OpenOptions::new();
    let out_file = options.create(true).append(true).open(&log_file)?;
    let err_file = options.create(true).append(true).open(&log_file)?;

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
        match File::create(&file_path) {
            Ok(mut file) => {
                file.write_all(contents.as_bytes()).unwrap_or_else(|_| {
                    panic!("Access denied? File can't be written: {}", &file_path)
                });
            }

            Err(err) => {
                panic!(
                    "Atomic write to: {} has failed! Cause: {}",
                    &file_path,
                    err.to_string()
                )
            }
        }
    }
}


#[test]
fn check_v4_instance_increment() {
    assert_eq!(next_instance_ipv4("0"), None);
    assert_eq!(
        next_instance_ipv4("127.0.0.0"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.1"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.2"),
        Some("127.0.0.3".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.253"),
        Some("127.0.0.254".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.0.254"),
        Some("127.0.2.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.252.254"),
        Some("127.0.254.2".to_string())
    );
    assert_eq!(
        next_instance_ipv4("127.0.254.253"),
        Some("127.0.254.254".to_string())
    );
    assert_eq!(next_instance_ipv4("127.0.254.254"), None);
}


#[test]
fn check_v4_workstation_increment() {
    assert_eq!(
        next_workstation_ipv4("127.0.0.0"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.1"),
        Some("127.0.0.2".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.2"),
        Some("127.0.0.3".to_string())
    );
    assert_eq!(
        next_workstation_ipv4("127.0.0.253"),
        Some("127.0.0.254".to_string())
    );
    assert_eq!(next_workstation_ipv4("127.0.0.254"), None);
    assert_eq!(next_workstation_ipv4("0"), None);
}
