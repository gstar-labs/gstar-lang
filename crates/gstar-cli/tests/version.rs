//! Runs the real `gstar` binary.

use std::process::{Command, Output};

fn gstar(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_gstar"))
        .args(args)
        .output()
        .expect("run gstar")
}

#[test]
fn version_prints_the_package_version() {
    let out = gstar(&["--version"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("gstar {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn an_unknown_flag_fails() {
    let out = gstar(&["--no-such-flag"]);
    assert!(!out.status.success(), "{out:?}");
}
