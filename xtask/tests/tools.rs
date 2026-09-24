//! Pinned tools and preflight detection.

use std::collections::BTreeMap;
use xtask::tools::{ToolPin, install_line, missing, missing_message, on_path, parse_pins};

const TWO: &str = r#"
[alpha]
version = "1.2.3"
install = "get alpha@{version}"

[beta]
version = "4.5"
install = "get beta"
"#;

#[test]
fn pins_parse_by_executable_name() {
    let pins = parse_pins(TWO).expect("valid pins");
    assert_eq!(
        pins.get("alpha"),
        Some(&ToolPin {
            version: "1.2.3".into(),
            install: "get alpha@{version}".into()
        })
    );
    assert_eq!(pins.len(), 2);
}

#[test]
fn an_empty_pin_file_fails_rather_than_passing_vacuously() {
    let err = parse_pins("").expect_err("no tools listed");
    assert!(format!("{err:#}").contains("vacuously"), "{err:#}");
}

#[test]
fn an_unknown_field_is_rejected() {
    let err = parse_pins("[alpha]\nversion = \"1\"\ninstall = \"x\"\nurl = \"y\"\n")
        .expect_err("unknown field");
    assert!(format!("{err:#}").contains("not valid"), "{err:#}");
}

#[test]
fn install_lines_fill_in_the_version() {
    let pin = ToolPin {
        version: "1.2.3".into(),
        install: "get alpha@{version}".into(),
    };
    assert_eq!(install_line(&pin), "get alpha@1.2.3");
}

#[test]
fn only_absent_tools_are_missing() {
    let pins = parse_pins(TWO).expect("valid pins");
    assert_eq!(missing(&pins, |t| t == "beta"), vec!["alpha"]);
    assert!(missing(&pins, |_| true).is_empty());
}

/// A machine may lack tools the maintainer's shell has.
#[test]
fn the_message_names_each_missing_tool_with_version_and_install_line() {
    let pins: BTreeMap<String, ToolPin> = parse_pins(TWO).expect("valid pins");
    let message = missing_message(&pins, &["alpha", "beta"]);
    assert!(
        message.contains("alpha 1.2.3: get alpha@1.2.3"),
        "{message}"
    );
    assert!(message.contains("beta 4.5: get beta"), "{message}");
}

#[test]
fn the_repository_pin_file_is_valid_and_complete() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let pins = xtask::tools::load_pins(&root).expect("tools/versions.toml is valid");
    for tool in [
        "cargo-deny",
        "cargo-llvm-cov",
        "cargo-nextest",
        "cargo-shear",
        "cvc5",
        "elan",
        "lake",
        "similarity-rs",
        "z3",
    ] {
        assert!(pins.contains_key(tool), "{tool} is not pinned");
    }
    assert!(
        pins.values().all(|p| !p.version.contains('<')),
        "an unfilled version remains"
    );
}

/// `on_path` is otherwise only called from the exempt `gates.rs`,
/// so an integration test must enter it directly.
#[test]
fn on_path_finds_a_real_executable_but_not_a_fake_one() {
    assert!(on_path("sh"), "sh should be on PATH");
    assert!(!on_path("gstar-no-such-tool"));
}
