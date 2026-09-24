//! Exemption lists: one `name: reason` per line. Blank lines and lines
//! starting with `#` are ignored. Every entry needs a reason.

use anyhow::bail;
use std::collections::BTreeMap;

/// Parses an exemption list.
///
/// # Errors
/// Returns an error for an entry without a reason, and for a duplicate entry.
pub fn parse(text: &str) -> anyhow::Result<BTreeMap<String, String>> {
    let mut entries = BTreeMap::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((name, reason)) = line.split_once(':') else {
            bail!("line {}: expected `name: reason`, got `{line}`", index + 1);
        };
        let (name, reason) = (name.trim(), reason.trim());
        if name.is_empty() || reason.is_empty() {
            bail!(
                "line {}: every exemption needs a name and a reason",
                index + 1
            );
        }
        if entries.insert(name.to_owned(), reason.to_owned()).is_some() {
            bail!("line {}: `{name}` is listed twice", index + 1);
        }
    }
    Ok(entries)
}
