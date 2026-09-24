//! Functions never entered by an integration test, from llvm-cov JSON.

use std::collections::BTreeMap;
use xtask::coverage::check;

/// One llvm-cov function record: mangled name, count, file, start line/column.
fn record(name: &str, count: u64, file: &str, line: u64, col: u64) -> String {
    format!(
        r#"{{"name":"{name}","count":{count},"filenames":["{file}"],"regions":[[{line},{col},{line},9,{count},0,0,0]]}}"#
    )
}

fn export(records: &[String]) -> String {
    format!(
        r#"{{"type":"llvm.coverage.json.export","data":[{{"functions":[{}]}}]}}"#,
        records.join(",")
    )
}

const MAIN: &str = "_ZN5gstar4main17h0123456789abcdefE";
const HELPER: &str = "_ZN5gstar6helper17h0123456789abcdefE";
const CLOSURE: &str = "_ZN5gstar4main28_$u7b$$u7b$closure$u7d$$u7d$17h0123456789abcdefE";

fn none() -> BTreeMap<String, String> {
    BTreeMap::new()
}

#[test]
fn entered_functions_pass() {
    let json = export(&[record(MAIN, 3, "/r/crates/a/src/main.rs", 1, 1)]);
    check(&json, "/r", &none()).expect("covered");
}

#[test]
fn a_never_entered_function_is_named_with_its_location() {
    let json = export(&[
        record(MAIN, 1, "/r/crates/a/src/main.rs", 1, 1),
        record(HELPER, 0, "/r/crates/a/src/main.rs", 7, 1),
    ]);
    let err = check(&json, "/r", &none()).expect_err("helper untested");
    let message = format!("{err:#}");
    assert!(
        message.contains("crates/a/src/main.rs:7 gstar::helper"),
        "{message}"
    );
}

#[test]
fn a_zero_count_duplicate_of_an_entered_function_is_covered() {
    let json = export(&[
        record(HELPER, 0, "/r/crates/a/src/lib.rs", 4, 1),
        record(HELPER, 2, "/r/crates/a/src/lib.rs", 4, 1),
    ]);
    check(&json, "/r", &none()).expect("one instance entered");
}

#[test]
fn an_entered_function_stays_covered_when_a_zero_count_duplicate_follows() {
    let json = export(&[
        record(HELPER, 2, "/r/crates/a/src/lib.rs", 4, 1),
        record(HELPER, 0, "/r/crates/a/src/lib.rs", 4, 1),
    ]);
    check(&json, "/r", &none()).expect("one instance entered");
}

#[test]
fn closures_and_foreign_files_are_ignored() {
    let json = export(&[
        record(MAIN, 1, "/r/crates/a/src/main.rs", 1, 1),
        record(CLOSURE, 0, "/r/crates/a/src/main.rs", 2, 5),
        record(HELPER, 0, "/deps/registry/src/dep/lib.rs", 1, 1),
    ]);
    check(&json, "/r", &none()).expect("closure and dependency ignored");
}

#[test]
fn test_files_are_not_first_party() {
    let json = export(&[
        record(MAIN, 1, "/r/crates/a/src/main.rs", 1, 1),
        record(HELPER, 0, "/r/crates/a/tests/version.rs", 3, 1),
        record(HELPER, 0, "/r/xtask/tests/common/mod.rs", 3, 1),
        record(HELPER, 0, "/r/crates/src/stray.rs", 3, 1),
    ]);
    check(&json, "/r", &none()).expect("only crate and runner sources count");
}

#[test]
fn only_test_files_fail_rather_than_passing_vacuously() {
    let json = export(&[record(HELPER, 1, "/r/crates/a/tests/version.rs", 1, 1)]);
    let err = check(&json, "/r", &none()).expect_err("vacuous");
    assert!(format!("{err:#}").contains("vacuously"), "{err:#}");
}

#[test]
fn an_exempt_file_is_skipped_and_a_stale_exemption_fails() {
    let json = export(&[
        record(MAIN, 1, "/r/crates/a/src/main.rs", 1, 1),
        record(HELPER, 0, "/r/xtask/src/gates.rs", 3, 1),
    ]);
    let mut exempt = BTreeMap::new();
    exempt.insert("xtask/src/gates.rs".to_owned(), "gate bodies".to_owned());
    check(&json, "/r", &exempt).expect("exempt file skipped");
    exempt.insert("xtask/src/gone.rs".to_owned(), "deleted".to_owned());
    let err = check(&json, "/r", &exempt).expect_err("stale exemption");
    assert!(format!("{err:#}").contains("xtask/src/gone.rs"), "{err:#}");
}

#[test]
fn no_first_party_functions_fails_rather_than_passing_vacuously() {
    let json = export(&[record(HELPER, 1, "/elsewhere/lib.rs", 1, 1)]);
    let err = check(&json, "/r", &none()).expect_err("vacuous");
    assert!(format!("{err:#}").contains("vacuously"), "{err:#}");
}
