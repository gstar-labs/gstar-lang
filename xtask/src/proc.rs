//! Child processes for gate bodies.

use anyhow::{Context as _, bail};
use std::process::Command;

/// Runs `program` with `args`, inheriting stdio.
///
/// # Errors
/// Returns an error if the program cannot be spawned or exits non-zero.
pub fn run(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("failed to spawn `{program}`"))?;
    if !status.success() {
        let mut shown = program.to_owned();
        for arg in args {
            shown.push(' ');
            shown.push_str(arg);
        }
        bail!("`{shown}` failed: {status}");
    }
    Ok(())
}

/// Runs `cargo` with `args`, inheriting stdio.
///
/// # Errors
/// Returns an error if cargo cannot be spawned or exits non-zero.
pub fn cargo(args: &[&str]) -> anyhow::Result<()> {
    run("cargo", args)
}
