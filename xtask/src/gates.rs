//! Gate bodies. Each one spawns a tool or calls a tested module; no logic
//! lives here.

use crate::proc;

/// Fails if any Rust file is not rustfmt-clean.
///
/// # Errors
/// Returns an error if `cargo fmt --check` reports a difference.
pub fn fmt() -> anyhow::Result<()> {
    proc::cargo(&["fmt", "--all", "--check"])
}
