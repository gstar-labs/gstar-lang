# Agent guide for g-star

## Project direction

g-star is pre-1.0. Breaking changes are acceptable when they follow the
project's semver rules. Prefer the clearer design over compatibility shims.

## Safety

Experimental software: work in disposable environments, and never point it at
sensitive systems or data.

## Development

- Build: `cargo build`
- Every gate: `cargo xtask gate`
- One gate: `cargo xtask gate --only <name>`; gate names: `cargo xtask gate --list`
- Missing tools: `cargo xtask gate --only preflight` prints how to install each one.

**Before ending any turn that changes the repository, run `cargo xtask gate`.**
If it cannot complete, report the blocker and the exact failing command.

## Layout

- `crates/gstar-cli/`: the `gstar` command-line tool.
- `xtask/`: the repository gates.
- `tools/versions.toml`: pinned external tools.

## Public docs policy

Public documents describe current behavior only. Keep development history,
release planning, and internal validation narratives out of them.

## Releasing

Release tags are created by the maintainer.
