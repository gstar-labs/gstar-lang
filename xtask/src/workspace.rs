//! The Cargo workspace the runner operates on.

use std::path::PathBuf;

/// The workspace root, found by `cargo metadata` from the current directory.
///
/// # Errors
/// Returns an error if `cargo metadata` fails.
pub fn root() -> anyhow::Result<PathBuf> {
    let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec()?;
    Ok(metadata.workspace_root.into_std_path_buf())
}
