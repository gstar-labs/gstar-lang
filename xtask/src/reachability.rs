//! Every workspace crate must be reachable from a product binary through
//! normal or build dependencies. Dev-dependencies don't count as wiring.

use anyhow::bail;
use cargo_metadata::{DependencyKind, Metadata};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// The product binaries every crate must be reachable from.
pub const ROOTS: &[&str] = &["gstar-cli", "gstar-mcp"];

/// The dependency graph between workspace members: each member maps to the
/// members it depends on through normal or build dependencies.
/// Dev-dependencies are dropped, and so are dependencies outside the
/// workspace. Every member is a key, even one with no edges.
#[must_use]
pub fn edges(metadata: &Metadata) -> BTreeMap<String, BTreeSet<String>> {
    let members: BTreeSet<String> = metadata
        .workspace_packages()
        .iter()
        .map(|pkg| pkg.name.to_string())
        .collect();
    metadata
        .workspace_packages()
        .iter()
        .map(|pkg| {
            let deps = pkg
                .dependencies
                .iter()
                .filter(|dep| dep.kind != DependencyKind::Development)
                .map(|dep| dep.name.clone())
                .filter(|name| members.contains(name))
                .collect();
            (pkg.name.to_string(), deps)
        })
        .collect()
}

/// Checks `edges` (member → members it depends on) against `roots` and
/// `exemptions`.
///
/// # Errors
/// Returns an error if no root is a member (the check would pass vacuously),
/// naming every unreachable member that is not exempt, or every stale
/// exemption: one naming a crate that isn't a member, or is reachable.
pub fn check(
    edges: &BTreeMap<String, BTreeSet<String>>,
    roots: &[&str],
    exemptions: &BTreeMap<String, String>,
) -> anyhow::Result<()> {
    let mut queue: VecDeque<&str> = roots
        .iter()
        .copied()
        .filter(|r| edges.contains_key(*r))
        .collect();
    if queue.is_empty() {
        bail!(
            "no product root among {roots:?} is a workspace member; the check would pass vacuously"
        );
    }
    let mut reached: BTreeSet<&str> = queue.iter().copied().collect();
    while let Some(member) = queue.pop_front() {
        for dep in edges.get(member).into_iter().flatten() {
            if edges.contains_key(dep) && reached.insert(dep.as_str()) {
                queue.push_back(dep.as_str());
            }
        }
    }
    let unreachable: Vec<&str> = edges
        .keys()
        .map(String::as_str)
        .filter(|m| !reached.contains(m) && !exemptions.contains_key(*m))
        .collect();
    let stale: Vec<&str> = exemptions
        .keys()
        .map(String::as_str)
        .filter(|e| !edges.contains_key(*e) || reached.contains(e))
        .collect();
    if !unreachable.is_empty() || !stale.is_empty() {
        bail!("unreachable crates: {unreachable:?}; stale exemptions: {stale:?}");
    }
    Ok(())
}
