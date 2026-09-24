//! Exemption lists: one `name: reason` per line.

use xtask::exemptions::parse;

#[test]
fn entries_parse_with_their_reasons() {
    let map = parse("# comment\n\nxtask: development tool\n").expect("valid");
    assert_eq!(
        map.get("xtask").map(String::as_str),
        Some("development tool")
    );
    assert_eq!(map.len(), 1);
}

#[test]
fn an_entry_without_a_reason_is_rejected() {
    assert!(parse("xtask:\n").is_err());
    assert!(parse("xtask\n").is_err());
}

#[test]
fn a_duplicate_entry_is_rejected() {
    assert!(parse("a: one\na: two\n").is_err());
}
