//! Every workspace crate must be reachable from a product binary.

use std::collections::{BTreeMap, BTreeSet};
use xtask::reachability::check;

fn edges(pairs: &[(&str, &[&str])]) -> BTreeMap<String, BTreeSet<String>> {
    pairs
        .iter()
        .map(|(from, to)| {
            (
                (*from).to_owned(),
                to.iter().map(|s| (*s).to_owned()).collect(),
            )
        })
        .collect()
}

fn exempt(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect()
}

#[test]
fn reachable_members_pass() {
    let graph = edges(&[("gstar-cli", &["core"]), ("core", &[])]);
    check(&graph, &["gstar-cli"], &exempt(&[])).expect("all reachable");
}

#[test]
fn an_unreachable_member_is_named() {
    let graph = edges(&[("gstar-cli", &[]), ("orphan-crate", &[])]);
    let err = check(&graph, &["gstar-cli"], &exempt(&[])).expect_err("unreachable");
    assert!(format!("{err:#}").contains("orphan-crate"), "{err:#}");
}

#[test]
fn an_exempt_member_passes() {
    let graph = edges(&[("gstar-cli", &[]), ("xtask", &[])]);
    check(&graph, &["gstar-cli"], &exempt(&[("xtask", "tool")])).expect("exempt");
}

#[test]
fn a_stale_exemption_fails() {
    let graph = edges(&[("gstar-cli", &["core"]), ("core", &[])]);
    let err = check(
        &graph,
        &["gstar-cli"],
        &exempt(&[("core", "no longer needed")]),
    )
    .expect_err("stale");
    assert!(format!("{err:#}").contains("stale"), "{err:#}");
    let err = check(
        &graph,
        &["gstar-cli"],
        &exempt(&[("gone", "deleted crate")]),
    )
    .expect_err("stale");
    assert!(format!("{err:#}").contains("gone"), "{err:#}");
}

#[test]
fn no_product_root_fails_rather_than_passing_vacuously() {
    let graph = edges(&[("core", &[])]);
    let err = check(&graph, &["gstar-cli"], &exempt(&[])).expect_err("no root");
    assert!(format!("{err:#}").contains("vacuously"), "{err:#}");
}
