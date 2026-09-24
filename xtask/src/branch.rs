//! Pull-request branch names.

/// Allowed prefixes. `dependabot/` is Dependabot's own.
pub const PREFIXES: &[&str] = &["feat/", "fix/", "rfc/", "docs/", "chore/", "dependabot/"];

/// True when `name` is an allowed prefix followed by at least one character.
#[must_use]
pub fn allowed(name: &str) -> bool {
    PREFIXES
        .iter()
        .any(|p| name.strip_prefix(p).is_some_and(|rest| !rest.is_empty()))
}
