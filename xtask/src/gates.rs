//! Gate bodies. Each one spawns a tool or calls a tested module; no logic
//! lives here.

use crate::proc;
use crate::{manifest, tools, workspace};
use anyhow::bail;

/// Fails if any Rust file is not rustfmt-clean.
///
/// # Errors
/// Returns an error if `cargo fmt --check` reports a difference.
pub fn fmt() -> anyhow::Result<()> {
    proc::cargo(&["fmt", "--all", "--check"])
}

/// Fails if a pinned external tool is not on `PATH`.
///
/// # Errors
/// Returns an error naming every missing tool, with its pinned version and
/// install line.
pub fn preflight() -> anyhow::Result<()> {
    let pins = tools::load_pins(&workspace::root()?)?;
    let missing = tools::missing(&pins, tools::on_path);
    if !missing.is_empty() {
        bail!("{}", tools::missing_message(&pins, &missing));
    }
    Ok(())
}

/// Fails on any clippy lint, across every target.
///
/// # Errors
/// Returns an error if clippy reports a lint.
pub fn lint() -> anyhow::Result<()> {
    proc::cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])
}

/// Fails if the workspace lint policy lost an entry, or a member doesn't opt in.
///
/// # Errors
/// Returns an error describing every problem found.
pub fn lint_optin() -> anyhow::Result<()> {
    let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec()?;
    let policy = std::fs::read_to_string(metadata.workspace_root.join("Cargo.toml"))?;
    let members: Vec<(String, &std::path::Path)> = metadata
        .workspace_packages()
        .iter()
        .map(|pkg| (pkg.name.to_string(), pkg.manifest_path.as_std_path()))
        .collect();
    manifest::check_workspace(&policy, &members)
}

/// Runs the whole test suite under nextest, then the doc tests, which nextest
/// doesn't run.
///
/// # Errors
/// Returns an error if any test fails.
pub fn test() -> anyhow::Result<()> {
    proc::cargo(&["nextest", "run", "--workspace"])?;
    proc::cargo(&["test", "--workspace", "--doc"])
}
