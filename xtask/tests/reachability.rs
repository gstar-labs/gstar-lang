//! Every workspace crate must be reachable from a product binary.

mod common;

use common::{Scratch, xtask_in};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use xtask::reachability::{ROOTS, check};

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

fn metadata_at(manifest: &Path) -> cargo_metadata::Metadata {
    cargo_metadata::MetadataCommand::new()
        .manifest_path(manifest)
        .no_deps()
        .exec()
        .expect("cargo metadata")
}

/// A workspace where `gstar-cli` depends on `a` normally and on `b` only as a
/// dev-dependency, with an exemption list that holds nothing but comments.
fn dev_dependency_workspace(name: &str) -> Scratch {
    let repo = Scratch::new(name);
    repo.write(
        "Cargo.toml",
        "[workspace]\nresolver = \"3\"\nmembers = [\"gstar-cli\", \"a\", \"b\"]\n",
    );
    repo.write(
        "gstar-cli/Cargo.toml",
        "[package]\nname = \"gstar-cli\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
         [dependencies]\na = { path = \"../a\" }\n\n\
         [dev-dependencies]\nb = { path = \"../b\" }\n",
    );
    repo.write("gstar-cli/src/main.rs", "fn main() {}\n");
    for member in ["a", "b"] {
        repo.write(
            &format!("{member}/Cargo.toml"),
            &format!("[package]\nname = \"{member}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
        );
        repo.write(&format!("{member}/src/lib.rs"), "//! A crate.\n");
    }
    repo.write(
        "xtask/exemptions/reachability.txt",
        "# Crates exempt from the reachability gate: `name: reason`.\n",
    );
    repo
}

#[test]
fn edges_keep_normal_dependencies_and_drop_dev_dependencies() {
    let repo = dev_dependency_workspace("reach-edges");
    let graph = xtask::reachability::edges(&metadata_at(&repo.path().join("Cargo.toml")));
    assert_eq!(
        graph,
        edges(&[("gstar-cli", &["a"]), ("a", &[]), ("b", &[])])
    );
}

#[test]
fn a_crate_reached_only_through_a_dev_dependency_fails_the_real_gate() {
    let repo = dev_dependency_workspace("reach-gate");
    let out = xtask_in(repo.path(), &["gate", "--only", "reachability"]);
    assert!(!out.status.success(), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unreachable crates: [\"b\"]; stale exemptions: []"),
        "{stderr}"
    );
}

#[test]
fn this_repository_passes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let list = std::fs::read_to_string(root.join("xtask/exemptions/reachability.txt"))
        .expect("read the exemption list");
    let exemptions = xtask::exemptions::parse(&list).expect("valid exemption list");
    let graph = xtask::reachability::edges(&metadata_at(&root.join("Cargo.toml")));
    check(&graph, ROOTS, &exemptions).expect("every crate is reachable or exempt");
}
