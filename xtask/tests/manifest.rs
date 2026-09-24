//! The workspace lint policy and member opt-in.

use xtask::manifest::{opts_in, weakened_policy};

const WHOLE: &str = r#"
[workspace.lints.rust]
dead_code = "deny"
unreachable_pub = "deny"
missing_docs = "deny"
unused = { level = "deny", priority = -1 }
unsafe_code = "forbid"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
"#;

#[test]
fn the_whole_policy_is_not_weakened() {
    assert!(weakened_policy(WHOLE).expect("valid").is_empty());
}

#[test]
fn a_deleted_entry_is_reported() {
    let text = WHOLE.replace("pedantic = { level = \"deny\", priority = -1 }\n", "");
    assert_eq!(
        weakened_policy(&text).expect("valid"),
        vec!["clippy.pedantic"]
    );
}

#[test]
fn an_entry_lowered_to_warn_is_reported() {
    let text = WHOLE.replace("missing_docs = \"deny\"", "missing_docs = \"warn\"");
    assert_eq!(
        weakened_policy(&text).expect("valid"),
        vec!["rust.missing_docs"]
    );
}

#[test]
fn an_entry_in_the_wrong_table_does_not_count() {
    let text = "[workspace.lints.clippy]\ndead_code = \"deny\"\n";
    assert!(
        weakened_policy(text)
            .expect("valid")
            .contains(&"rust.dead_code".to_owned())
    );
}

#[test]
fn opt_in_is_detected() {
    assert!(opts_in("[package]\nname = \"a\"\n\n[lints]\nworkspace = true\n").expect("valid"));
    assert!(!opts_in("[package]\nname = \"a\"\n").expect("valid"));
    assert!(!opts_in("[lints]\nworkspace = false\n").expect("valid"));
}

#[test]
fn this_repository_keeps_the_whole_policy() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let text = std::fs::read_to_string(root.join("Cargo.toml")).expect("read workspace manifest");
    assert!(weakened_policy(&text).expect("valid").is_empty());
}
