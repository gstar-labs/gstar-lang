//! Child-process helpers.

#[test]
fn a_succeeding_command_is_ok() {
    xtask::proc::run("true", &[]).expect("`true` succeeds");
}

#[test]
fn a_failing_command_is_named() {
    let err = xtask::proc::run("false", &[]).expect_err("`false` fails");
    assert!(format!("{err:#}").contains("`false` failed"), "{err:#}");
}

#[test]
fn a_missing_program_is_a_spawn_error() {
    let err = xtask::proc::run("gstar-no-such-program", &[]).expect_err("cannot spawn");
    assert!(
        format!("{err:#}").contains("failed to spawn `gstar-no-such-program`"),
        "{err:#}"
    );
}

#[test]
fn cargo_runs_cargo() {
    xtask::proc::cargo(&["--version"]).expect("cargo --version succeeds");
}
