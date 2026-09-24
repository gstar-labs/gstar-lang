//! Gate bodies. Each one spawns a tool or calls a tested module; no logic
//! lives here.

use crate::proc;
use crate::{tools, workspace};
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
