//! The unfinished-work marker scan. Every fixture below is a string literal,
//! and literals are exactly what the scan ignores, so this file never reports
//! itself.

mod common;

use common::Scratch;
use xtask::markers::{Lang, blank_literals, offending_lines, scan};

#[test]
fn rust_string_contents_are_blanked_and_line_breaks_kept() {
    let src = "let a = \"TODO x\";\nlet b = r#\"multi\nFIXME\"#;\n// kept\n";
    let blanked = blank_literals(src, Lang::Rust);
    assert_eq!(blanked.lines().count(), src.lines().count());
    assert!(
        !blanked.contains("TODO") && !blanked.contains("FIXME"),
        "{blanked}"
    );
    assert!(blanked.contains("// kept"));
}

#[test]
fn rust_char_literals_are_blanked_but_lifetimes_are_not() {
    let blanked = blank_literals("fn f<'a>(x: &'a str) -> char { '\\'' }", Lang::Rust);
    assert!(
        blanked.contains("<'a>") && blanked.contains("&'a str"),
        "{blanked}"
    );
}

#[test]
fn apostrophes_in_comments_do_not_open_literals() {
    let src = "// don't stop\nlet s = \"x\"; // HACK here\n";
    let hits = offending_lines("a.rs", src, Lang::Rust);
    assert_eq!(hits, vec!["a.rs:2: let s = \"x\"; // HACK here"]);
}

#[test]
fn markers_in_literals_pass_and_markers_in_comments_fail() {
    let src = "const P: &str = \"TODO|FIXME\";\n/* TODO later */\n";
    assert_eq!(
        offending_lines("a.rs", src, Lang::Rust),
        vec!["a.rs:2: /* TODO later */"]
    );
}

#[test]
fn identifiers_containing_a_marker_word_are_not_markers() {
    assert!(offending_lines("a.rs", "let todo_list = 1;\n", Lang::Rust).is_empty());
}

#[test]
fn a_marker_with_a_reason_exempts_its_line_and_without_one_does_not() {
    let marked = "// TODO kept gstar:allow-stub (fixture)\n";
    let bare = "// TODO kept gstar:allow-stub\n";
    assert!(offending_lines("a.rs", marked, Lang::Rust).is_empty());
    assert_eq!(offending_lines("a.rs", bare, Lang::Rust).len(), 1);
}

#[test]
fn toml_and_lean_literals_and_comments_are_understood() {
    assert!(
        offending_lines("a.toml", "k = \"TODO\"\nm = '''\nFIXME\n'''\n", Lang::Toml).is_empty()
    );
    assert_eq!(
        offending_lines("a.toml", "# for now\n", Lang::Toml).len(),
        1
    );
    assert!(offending_lines("A.lean", "def x' := \"TODO\"\n", Lang::Lean).is_empty());
    assert_eq!(
        offending_lines("A.lean", "-- FIXME\n/- XXX -/\n", Lang::Lean).len(),
        2
    );
}

#[test]
fn todo_macro_calls_are_markers() {
    assert_eq!(
        offending_lines("a.rs", "fn f() { todo!() }\n", Lang::Rust).len(),
        1
    );
}

#[test]
fn scan_reports_untracked_files_and_skips_ignored_ones() {
    let repo = Scratch::new("markers-scan");
    repo.git(&["init", "-q", "-b", "main"]);
    repo.write("src/lib.rs", "//! Clean.\n");
    repo.write("src/new.rs", "// XXX untracked\n");
    repo.write(".gitignore", "/target/\n");
    repo.write("target/gen.rs", "// XXX generated\n");
    let hits = scan(repo.path()).expect("scan runs");
    assert_eq!(hits, vec!["src/new.rs:1: // XXX untracked"]);
}

/// Directories excluded privately must be neither scanned nor
/// named in the output.
#[test]
fn scan_never_reads_or_names_privately_excluded_paths() {
    let repo = Scratch::new("markers-exclude");
    repo.git(&["init", "-q", "-b", "main"]);
    repo.write(".git/info/exclude", "/private/\n");
    repo.write("private/notes.rs", "// TODO secret\n");
    repo.write("src/lib.rs", "//! Clean.\n");
    let hits = scan(repo.path()).expect("scan runs");
    assert!(hits.is_empty(), "{hits:?}");
}

#[test]
fn scan_fails_when_there_is_nothing_to_scan() {
    let repo = Scratch::new("markers-empty");
    repo.git(&["init", "-q", "-b", "main"]);
    let err = scan(repo.path()).expect_err("no source files");
    assert!(format!("{err:#}").contains("vacuously"), "{err:#}");
}
