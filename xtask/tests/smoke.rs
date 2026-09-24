//! Runs the real `xtask` binary on paths that don't recurse into cargo.

use std::process::{Command, Output};

fn xtask(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(args)
        .output()
        .expect("run xtask")
}

#[test]
fn gate_list_prints_every_registered_gate_in_order() {
    let out = xtask(&["gate", "--list"]);
    assert!(out.status.success(), "{out:?}");
    let listed: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    let registered: Vec<&str> = xtask::registry().iter().map(|g| g.name).collect();
    assert_eq!(listed, registered);
}

#[test]
fn an_unknown_gate_is_rejected_before_any_gate_runs() {
    let out = xtask(&["gate", "--only", "no-such-gate"]);
    assert!(!out.status.success(), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown gate `no-such-gate`"));
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("=="),
        "a gate ran first"
    );
}

/// A skipped gate named with `--only` fails, on the real binary.
#[test]
fn a_skipped_gate_named_with_only_exits_non_zero() {
    let out = xtask(&["gate", "--only", "lean"]);
    assert!(!out.status.success(), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stderr).contains("not active yet"));
}
