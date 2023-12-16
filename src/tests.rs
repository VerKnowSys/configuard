use crate::{
    common::{read_all_entries, read_all_used_ipv4, render_all_entries, render_entry},
    utils::{find_last_ipv4, next_workstation_ipv4},
};

const ENTRIES_DIR: &str = "./tests/entries";


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


#[test]
fn check_matchers() {
    use regex::Regex;
    let file_name_match = Regex::new(r"^[a-zA-Z0-9 -\.]{3,}$").unwrap();
    assert!(file_name_match.is_match("T0skdjf42"));
    assert!(file_name_match.is_match("8sfdsfd222f"));
    assert!(file_name_match.is_match("8s fdsf d22 2f"));
    assert!(file_name_match.is_match("8s.fdsf.d22.2f"));
    assert!(file_name_match.is_match("-8s-fdsf-d 2 2.2f-"));
    assert!(file_name_match.is_match("ADA"));
    assert!(!file_name_match.is_match("8s"));
    assert!(!file_name_match.is_match(""));
    assert!(!file_name_match.is_match("8"));
    assert!(!file_name_match.is_match("zażółć"));
    assert!(!file_name_match.is_match("../../../etc"));
}


#[test]
fn test_read_all_entries() {
    let all_the_things = read_all_entries(ENTRIES_DIR);
    assert_eq!(all_the_things.0.len(), 3);
    assert_eq!(all_the_things.1.len(), 3);
}


#[test]
fn test_render_one() {
    let user_ips = "123.456.88.11";
    let user_public_key = "user-public-key";
    let frist_entry = read_all_entries(ENTRIES_DIR).0.first().unwrap().clone();
    let entry = render_entry(&frist_entry, user_ips, user_public_key);
    assert!(entry.contains("file: dmilith2"));
    assert!(entry.contains("modified_at: 20"));
    assert!(entry.contains("[Peer]"));
    assert!(entry.contains("PublicKey = user-public-key"));
    assert!(entry.contains("AllowedIPs = 123.456.88.11"));
}


#[test]
fn test_render_all_entries() {
    let all_the_things = render_all_entries(ENTRIES_DIR);
    assert_eq!(all_the_things.len(), 467);
    assert!(!all_the_things.is_empty());
    assert!(all_the_things.contains("123.45.67.89"));
    assert!(all_the_things.contains("123.45.67.90"));
    assert!(all_the_things.contains("123.45.67.91"));
    assert!(all_the_things.contains("dmilith3"));
}


#[test]
fn test_read_all_used_ipv4() {
    let all_entries = read_all_used_ipv4("tests/entries");
    assert_eq!(all_entries.len(), 3);
    assert!(all_entries.contains(&String::from("123.45.67.89")))
}


#[test]
fn test_used_ips_in_context() {
    let all_used_ipv4s = read_all_used_ipv4("tests/entries");
    let the_last_ipv4 = match find_last_ipv4(all_used_ipv4s) {
        Some(ipv4) => ipv4,
        None => String::from("123.45.1.1"), /* if list of entries is empty, assign next address after router */
    };
    let the_next = match next_workstation_ipv4(&the_last_ipv4) {
        Some(ipv4) => ipv4,
        None => panic!("Address pool exhausted!"),
    };
    assert_eq!(the_last_ipv4, "123.45.67.91");
    assert_eq!(the_next, "123.45.67.92");
}
