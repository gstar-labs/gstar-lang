//! The gate registry is pinned: identity, order, and which gates are skipped.

use xtask::{Check, registry};

/// A gate removed, renamed or reordered would retire a check while
/// every remaining gate keeps passing.
#[test]
fn registry_is_exactly_these_gates_in_order() {
    let names: Vec<&str> = registry().iter().map(|g| g.name).collect();
    assert_eq!(
        names,
        vec![
            "preflight",
            "fmt",
            "lint",
            "lint-optin",
            "no-stubs",
            "reachability",
            "deps",
            "trusted-base",
            "duplication",
            "core-sync",
            "semver",
            "test",
            "lean",
            "difftest",
            "book",
            "diagnostics",
            "stability",
            "fuzz",
            "untested-code",
            "solver-work",
        ]
    );
}

/// Activating a gate, or skipping an active one, shows up here.
#[test]
fn skipped_gates_are_exactly_these() {
    let skipped: Vec<&str> = registry()
        .iter()
        .filter(|g| matches!(g.check, Check::Skip(_)))
        .map(|g| g.name)
        .collect();
    assert_eq!(
        skipped,
        vec![
            "trusted-base",
            "duplication",
            "core-sync",
            "semver",
            "lean",
            "difftest",
            "book",
            "diagnostics",
            "stability",
            "fuzz",
            "untested-code",
            "solver-work",
        ]
    );
}

/// A skip that doesn't name its public stage milestone could stay forever.
#[test]
fn every_skip_names_a_stage_milestone() {
    for gate in registry() {
        if let Check::Skip(reason) = gate.check {
            assert!(
                reason.starts_with("activates ") && reason.contains("milestone \"M1 · "),
                "gate `{}` skip reason names no stage milestone: {reason}",
                gate.name
            );
        }
    }
}

#[test]
fn gate_names_are_unique_and_non_empty() {
    let names: Vec<&str> = registry().iter().map(|g| g.name).collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        names.len(),
        "duplicate gate name in {names:?}"
    );
    assert!(names.iter().all(|n| !n.is_empty()));
}
