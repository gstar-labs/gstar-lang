# Contributing to g-star

## The gate is the definition of done

`cargo xtask gate` runs every repository gate and reports every failure.
CI runs the same command on every pull request and in the merge queue, and
`main` accepts only changes that pass it. `cargo xtask gate --list` shows the
gates in order. A gate marked SKIP names the milestone in which it activates.

## Workflow

1. Create a branch named `feat/…`, `fix/…`, `rfc/…`, `docs/…` or `chore/…`.
2. Make the change, with tests. Run `cargo xtask gate`.
3. Open a pull request, and fill in the checklist.
4. After review, the pull request merges through the merge queue with a merge commit.

## Hard rules

- No unfinished-work markers in source files. Finish the work, or open an
  issue labelled `deferred`. In the rare genuine exception, mark the line
  `gstar:allow-stub (reason)`.
- Strict lints: clippy `all` and `pedantic` are errors, and every `allow`
  states why in its `reason = "…"`.
- Every function is reached by an integration test.
- No unused dependencies. A dependency lands in the change that first uses it.
- Tests are deterministic: no sleeps.

## Deferred work and decisions

Tracked as GitHub issues labelled `deferred`, `decision`, `accepted` or
`finding`, each in a milestone. The pull request that defers something opens
its issue. The pull request that resolves it closes the issue with `Fixes #N`.

## Changelog and versions

Every user-visible change adds an entry under `[Unreleased]` in `CHANGELOG.md`.
The `Bump:` line at the top of that section states the change's semver impact.
