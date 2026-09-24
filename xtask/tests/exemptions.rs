//! Exemption lists: one `name: reason` per line.

use std::collections::BTreeSet;
use std::path::Path;
use xtask::exemptions::parse;

fn keys_of(list: &str) -> BTreeSet<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("exemptions")
        .join(list);
    let text = std::fs::read_to_string(&path).expect("read the exemption list");
    parse(&text)
        .expect("valid exemption list")
        .into_keys()
        .collect()
}

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

/// Growing an exemption list silently widens what a gate lets through, so each
/// list is pinned to its exact entries.
#[test]
fn the_reachability_exemptions_are_exactly_xtask() {
    assert_eq!(
        keys_of("reachability.txt"),
        BTreeSet::from(["xtask".to_owned()])
    );
}

#[test]
fn the_untested_code_exemptions_are_exactly_the_gate_bodies() {
    assert_eq!(
        keys_of("untested-code.txt"),
        BTreeSet::from(["xtask/src/gates.rs".to_owned()])
    );
}
