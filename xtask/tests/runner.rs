//! `run_gates` with fake gates. Each test has its own counters, because gate
//! bodies are `fn` pointers and cannot capture.

// Fake gates must match `fn() -> anyhow::Result<()>` even when they cannot fail.
#![allow(clippy::unnecessary_wraps)]

use std::sync::atomic::{AtomicUsize, Ordering};
use xtask::{Check, Gate, Outcome, run_gates};

static UNKNOWN_RUNS: AtomicUsize = AtomicUsize::new(0);
fn counts_unknown_runs() -> anyhow::Result<()> {
    UNKNOWN_RUNS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[test]
fn an_unknown_name_fails_before_any_gate_runs() {
    let gates = [Gate {
        name: "a",
        check: Check::Active(counts_unknown_runs),
    }];
    let err = run_gates(&gates, Some("nope")).expect_err("unknown name must fail");
    assert!(
        format!("{err:#}").contains("unknown gate `nope`"),
        "{err:#}"
    );
    assert_eq!(UNKNOWN_RUNS.load(Ordering::SeqCst), 0);
}

fn fails_first() -> anyhow::Result<()> {
    anyhow::bail!("first broke")
}
fn fails_second() -> anyhow::Result<()> {
    anyhow::bail!("second broke")
}
static MIDDLE_RUNS: AtomicUsize = AtomicUsize::new(0);
fn counts_middle_runs() -> anyhow::Result<()> {
    MIDDLE_RUNS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[test]
fn every_gate_runs_and_every_failure_is_named() {
    let gates = [
        Gate {
            name: "first",
            check: Check::Active(fails_first),
        },
        Gate {
            name: "middle",
            check: Check::Active(counts_middle_runs),
        },
        Gate {
            name: "second",
            check: Check::Active(fails_second),
        },
    ];
    let err = run_gates(&gates, None).expect_err("failing gates fail the run");
    let message = format!("{err:#}");
    assert!(
        message.contains("first") && message.contains("second"),
        "{message}"
    );
    assert!(!message.contains("middle"), "{message}");
    assert_eq!(MIDDLE_RUNS.load(Ordering::SeqCst), 1);
}

static OTHER_RUNS: AtomicUsize = AtomicUsize::new(0);
static TARGET_RUNS: AtomicUsize = AtomicUsize::new(0);
fn counts_other_runs() -> anyhow::Result<()> {
    OTHER_RUNS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}
fn counts_target_runs() -> anyhow::Result<()> {
    TARGET_RUNS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[test]
fn only_runs_the_named_gate() {
    let gates = [
        Gate {
            name: "other",
            check: Check::Active(counts_other_runs),
        },
        Gate {
            name: "target",
            check: Check::Active(counts_target_runs),
        },
    ];
    let outcome = run_gates(&gates, Some("target")).expect("the named gate passes");
    assert_eq!(OTHER_RUNS.load(Ordering::SeqCst), 0);
    assert_eq!(TARGET_RUNS.load(Ordering::SeqCst), 1);
    assert_eq!(
        outcome,
        Outcome {
            passed: vec!["target"],
            skipped: vec![]
        }
    );
}

static BESIDE_SKIP_RUNS: AtomicUsize = AtomicUsize::new(0);
fn counts_beside_skip_runs() -> anyhow::Result<()> {
    BESIDE_SKIP_RUNS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[test]
fn a_skipped_gate_is_reported_not_run() {
    let gates = [
        Gate {
            name: "waiting",
            check: Check::Skip("activates in milestone \"M1 · Interpreter\""),
        },
        Gate {
            name: "live",
            check: Check::Active(counts_beside_skip_runs),
        },
    ];
    let outcome = run_gates(&gates, None).expect("a skip is not a failure");
    assert_eq!(
        outcome,
        Outcome {
            passed: vec!["live"],
            skipped: vec!["waiting"]
        }
    );
    assert_eq!(BESIDE_SKIP_RUNS.load(Ordering::SeqCst), 1);
}

#[test]
fn only_naming_a_skipped_gate_fails_with_its_milestone() {
    let gates = [Gate {
        name: "lean",
        check: Check::Skip("activates in milestone \"M1 · Core & Lean syntax\""),
    }];
    let err = run_gates(&gates, Some("lean")).expect_err("a skipped gate cannot pass by name");
    let message = format!("{err:#}");
    assert!(message.contains("not active yet"), "{message}");
    assert!(message.contains("M1 · Core & Lean syntax"), "{message}");
}
