//! Runs real gates inside scratch workspaces, end to end.

mod common;

use common::{Scratch, xtask_in};

const RUST_POLICY: &str = "[workspace.lints.rust]
dead_code = \"deny\"
unreachable_pub = \"deny\"
missing_docs = \"deny\"
unused = { level = \"deny\", priority = -1 }
unsafe_code = \"forbid\"
";

fn workspace(repo: &Scratch, pedantic: &str, member_opts_in: bool) {
    repo.write(
        "Cargo.toml",
        &format!(
            "[workspace]\nresolver = \"3\"\nmembers = [\"a\"]\n\n{RUST_POLICY}\n\
             [workspace.lints.clippy]\nall = {{ level = \"deny\", priority = -1 }}\n\
             pedantic = {{ level = \"{pedantic}\", priority = -1 }}\n\
             allow_attributes_without_reason = \"deny\"\n"
        ),
    );
    let lints = if member_opts_in {
        "\n[lints]\nworkspace = true\n"
    } else {
        ""
    };
    repo.write(
        "a/Cargo.toml",
        &format!("[package]\nname = \"a\"\nversion = \"0.1.0\"\nedition = \"2024\"\n{lints}"),
    );
    repo.write("a/src/lib.rs", "//! A crate.\n");
}

#[test]
fn lint_optin_passes_the_whole_policy() {
    let repo = Scratch::new("optin-whole");
    workspace(&repo, "deny", true);
    let out = xtask_in(repo.path(), &["gate", "--only", "lint-optin"]);
    assert!(out.status.success(), "{out:?}");
}

#[test]
fn lint_optin_rejects_a_weakened_policy() {
    let repo = Scratch::new("optin-weak");
    workspace(&repo, "warn", true);
    let out = xtask_in(repo.path(), &["gate", "--only", "lint-optin"]);
    assert!(!out.status.success(), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("weakened") && stderr.contains("clippy.pedantic"),
        "{stderr}"
    );
}

#[test]
fn lint_optin_names_a_member_that_opts_out() {
    let repo = Scratch::new("optin-member");
    workspace(&repo, "deny", false);
    let out = xtask_in(repo.path(), &["gate", "--only", "lint-optin"]);
    assert!(!out.status.success(), "{out:?}");
    assert!(String::from_utf8_lossy(&out.stderr).contains("[\"a\"]"));
}
