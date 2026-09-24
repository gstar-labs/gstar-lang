//! Pinned external tools (`tools/versions.toml`) and PATH detection.

use anyhow::{Context as _, bail};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// One pinned tool, keyed by its executable name in `tools/versions.toml`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolPin {
    /// The pinned version.
    pub version: String,
    /// How to install it; `{version}` is replaced by the pinned version.
    pub install: String,
}

/// Parses the text of `tools/versions.toml`.
///
/// # Errors
/// Returns an error if the text is not valid, or if it lists no tools, in
/// which case preflight would pass vacuously.
pub fn parse_pins(text: &str) -> anyhow::Result<BTreeMap<String, ToolPin>> {
    let pins: BTreeMap<String, ToolPin> =
        toml::from_str(text).context("tools/versions.toml is not valid")?;
    if pins.is_empty() {
        bail!("tools/versions.toml lists no tools; preflight would pass vacuously");
    }
    Ok(pins)
}

/// Reads and parses `tools/versions.toml` under `root`.
///
/// # Errors
/// Returns an error if the file cannot be read or parsed.
pub fn load_pins(root: &Path) -> anyhow::Result<BTreeMap<String, ToolPin>> {
    let path = root.join("tools/versions.toml");
    let text =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    parse_pins(&text)
}

/// The install instruction for `pin`, with its version filled in.
#[must_use]
pub fn install_line(pin: &ToolPin) -> String {
    pin.install.replace("{version}", &pin.version)
}

/// The pinned tools that `present` reports missing, in name order.
#[must_use]
pub fn missing(pins: &BTreeMap<String, ToolPin>, present: impl Fn(&str) -> bool) -> Vec<&str> {
    pins.keys()
        .map(String::as_str)
        .filter(|tool| !present(tool))
        .collect()
}

/// The preflight failure: every missing tool, with its pinned version and
/// install line.
#[must_use]
pub fn missing_message(pins: &BTreeMap<String, ToolPin>, missing: &[&str]) -> String {
    let mut message = String::from("missing tools:");
    for name in missing {
        if let Some(pin) = pins.get(*name) {
            message.push_str("\n  ");
            message.push_str(name);
            message.push(' ');
            message.push_str(&pin.version);
            message.push_str(": ");
            message.push_str(&install_line(pin));
        }
    }
    message
}

/// True when an executable named `tool` is in a directory on `PATH`.
///
/// Scans `PATH` instead of running `<tool> --version`, because cargo
/// subcommand binaries can exit non-zero when run directly.
#[must_use]
pub fn on_path(tool: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join(tool).is_file()))
}
