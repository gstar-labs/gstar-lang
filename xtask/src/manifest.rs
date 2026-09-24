//! The workspace lint policy, and each member's opt-in to it.

use anyhow::{Context as _, bail};
use std::path::Path;

/// Lints the workspace policy must set to `deny` or `forbid`, as
/// `(table, lint)` under `[workspace.lints]`.
pub const REQUIRED: &[(&str, &str)] = &[
    ("rust", "dead_code"),
    ("rust", "unreachable_pub"),
    ("rust", "missing_docs"),
    ("rust", "unused"),
    ("rust", "unsafe_code"),
    ("clippy", "all"),
    ("clippy", "pedantic"),
];

fn is_strict(value: Option<&toml::Value>) -> bool {
    let level = match value {
        Some(toml::Value::String(level)) => Some(level.as_str()),
        Some(toml::Value::Table(table)) => table.get("level").and_then(toml::Value::as_str),
        _ => None,
    };
    matches!(level, Some("deny" | "forbid"))
}

/// The [`REQUIRED`] entries that `workspace_manifest` does not set to `deny` or
/// `forbid`, as `table.lint`.
///
/// # Errors
/// Returns an error if the manifest is not valid TOML.
pub fn weakened_policy(workspace_manifest: &str) -> anyhow::Result<Vec<String>> {
    let doc: toml::Table = workspace_manifest
        .parse()
        .context("workspace Cargo.toml is not valid TOML")?;
    let lints = doc.get("workspace").and_then(|w| w.get("lints"));
    Ok(REQUIRED
        .iter()
        .filter(|(table, lint)| {
            !is_strict(lints.and_then(|l| l.get(*table)).and_then(|t| t.get(*lint)))
        })
        .map(|(table, lint)| format!("{table}.{lint}"))
        .collect())
}

/// True when a member manifest sets `[lints] workspace = true`.
///
/// # Errors
/// Returns an error if the manifest is not valid TOML.
pub fn opts_in(member_manifest: &str) -> anyhow::Result<bool> {
    let doc: toml::Table = member_manifest
        .parse()
        .context("member Cargo.toml is not valid TOML")?;
    Ok(doc
        .get("lints")
        .and_then(|l| l.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true))
}

/// Checks the workspace policy and every member's opt-in. `members` lists
/// `(name, manifest path)`.
///
/// # Errors
/// Returns an error naming every weakened entry, every member that opts out,
/// or that there are no members.
pub fn check_workspace(
    workspace_manifest: &str,
    members: &[(String, &Path)],
) -> anyhow::Result<()> {
    let weakened = weakened_policy(workspace_manifest)?;
    if !weakened.is_empty() {
        bail!("workspace lint policy weakened (each must be deny or forbid): {weakened:?}");
    }
    if members.is_empty() {
        bail!("no workspace members; the check would pass vacuously");
    }
    let mut opted_out = Vec::new();
    for (name, path) in members {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        if !opts_in(&text)? {
            opted_out.push(name.clone());
        }
    }
    opted_out.sort();
    if !opted_out.is_empty() {
        bail!("members missing `[lints] workspace = true`: {opted_out:?}");
    }
    Ok(())
}
