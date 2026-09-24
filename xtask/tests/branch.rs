//! Pull-request branch names.

use xtask::branch::allowed;

#[test]
fn prefixed_and_dependabot_branches_are_allowed() {
    for name in [
        "feat/x",
        "fix/y",
        "rfc/0002-z",
        "docs/a",
        "chore/b",
        "dependabot/cargo/serde-1.0.0",
    ] {
        assert!(allowed(name), "{name} should be allowed");
    }
}

#[test]
fn unprefixed_or_empty_branches_are_rejected() {
    for name in ["wip", "feat/", "feature/x", "main", "spec/x", ""] {
        assert!(!allowed(name), "{name} should be rejected");
    }
}
