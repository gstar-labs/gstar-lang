//! Repository gates for g-star, run with `cargo xtask gate`.
//!
//! The registry is the definition of done: `cargo xtask gate` runs every gate,
//! reports every failure, and exits non-zero if any gate failed.

pub mod gates;
pub mod manifest;
pub mod proc;
pub mod tools;
pub mod workspace;

use anyhow::bail;

/// What running a registered gate does.
#[derive(Clone, Copy)]
pub enum Check {
    /// Runs the check; an error describes the failure.
    Active(fn() -> anyhow::Result<()>),
    /// Registered, but the gate's input does not exist yet. The text names the
    /// stage milestone that activates it.
    Skip(&'static str),
}

/// One repository gate.
pub struct Gate {
    /// Stable name, used by `--only` and in reports.
    pub name: &'static str,
    /// What running the gate does.
    pub check: Check,
}

/// The gates a successful run passed and skipped, in run order.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    /// Gates that ran and passed.
    pub passed: Vec<&'static str>,
    /// Gates that are registered but not active yet.
    pub skipped: Vec<&'static str>,
}

const LATER_IN_SKELETON: &str = "activates later in milestone \"M1 · Skeleton\"";
const CORE: &str = "activates in milestone \"M1 · Core & Lean syntax\"";
const STORE: &str = "activates in milestone \"M1 · Surface & store\"";
const KERNEL: &str = "activates in milestone \"M1 · Kernel & SMT\"";
const SPECS: &str = "activates in milestone \"M1 · Specs & elaboration\"";
const INTERP: &str = "activates in milestone \"M1 · Interpreter\"";
const BOOK: &str = "activates in milestone \"M1 · Book & release\"";
const RELEASE: &str =
    "activates after the first 0.1.0 release tags, in milestone \"M1 · Book & release\"";

fn active(name: &'static str, run: fn() -> anyhow::Result<()>) -> Gate {
    Gate {
        name,
        check: Check::Active(run),
    }
}

fn skip(name: &'static str, reason: &'static str) -> Gate {
    Gate {
        name,
        check: Check::Skip(reason),
    }
}

/// Every registered gate, in run order (cheap first).
///
/// The order and the skipped set are pinned by `tests/registry.rs`.
#[must_use]
pub fn registry() -> Vec<Gate> {
    vec![
        active("preflight", gates::preflight),
        active("fmt", gates::fmt),
        active("lint", gates::lint),
        active("lint-optin", gates::lint_optin),
        skip("no-stubs", LATER_IN_SKELETON),
        skip("reachability", LATER_IN_SKELETON),
        skip("deps", LATER_IN_SKELETON),
        skip("trusted-base", CORE),
        skip("duplication", STORE),
        skip("core-sync", CORE),
        skip("semver", RELEASE),
        active("test", gates::test),
        skip("lean", CORE),
        skip("difftest", KERNEL),
        skip("book", BOOK),
        skip("diagnostics", SPECS),
        skip("stability", BOOK),
        skip("fuzz", INTERP),
        skip("untested-code", LATER_IN_SKELETON),
        skip("solver-work", BOOK),
    ]
}

/// Runs `gates` in order, or only the one named `only`. Every selected gate
/// runs, even after a failure, and all failures are reported together.
///
/// # Errors
/// Before any gate runs: `only` names no gate, or names a skipped gate.
/// Afterwards: an error naming every gate that failed.
pub fn run_gates(gates: &[Gate], only: Option<&str>) -> anyhow::Result<Outcome> {
    if let Some(name) = only {
        match gates.iter().find(|g| g.name == name) {
            None => bail!("unknown gate `{name}`"),
            Some(Gate {
                check: Check::Skip(reason),
                ..
            }) => {
                bail!("gate `{name}` is not active yet: {reason}")
            }
            Some(_) => {}
        }
    }
    let mut outcome = Outcome::default();
    let mut failed: Vec<&str> = Vec::new();
    for gate in gates
        .iter()
        .filter(|g| only.is_none_or(|name| name == g.name))
    {
        println!("== {} ==", gate.name);
        match gate.check {
            Check::Skip(reason) => {
                println!("SKIP: {reason}");
                outcome.skipped.push(gate.name);
            }
            Check::Active(run) => match run() {
                Ok(()) => outcome.passed.push(gate.name),
                Err(err) => {
                    eprintln!("FAIL {}: {err:#}", gate.name);
                    failed.push(gate.name);
                }
            },
        }
    }
    println!(
        "gates: {} passed, {} skipped, {} failed",
        outcome.passed.len(),
        outcome.skipped.len(),
        failed.len()
    );
    if !failed.is_empty() {
        bail!("failed gates: {}", failed.join(", "));
    }
    Ok(outcome)
}
