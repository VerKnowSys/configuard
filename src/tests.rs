use crate::utils::{find_last_ipv4, next_instance_ipv4, next_workstation_ipv4};




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
