# g-star

**A verified-systems language for code written by agents, and for the humans who trust it.**

> **Experimental.** g-star is at its very beginning. Expect breaking changes and
> missing pieces. Don't use it for anything that matters yet.

g-star is being designed so that a program an agent writes is either proved to
meet its written contracts, or rejected with a machine-readable reason: a
replayed counterexample, an unproven obligation, or a precise error. Programs
live in a content-addressed store of nodes that git can diff at the node level.
Types are dependent and refined, and proof obligations go to SMT solvers. The
core calculus is formalized in Lean.

## What exists today

- The `gstar` command-line tool, which reports its version.
- The repository gate (`cargo xtask gate`): the definition of done for every change.

Everything else (the store, the checker, the interpreter and the book) is
still ahead. Each piece will be listed here once it works.

## Building from source

Requires the Rust toolchain pinned in `rust-toolchain.toml`.

```sh
cargo build
cargo run --package gstar-cli -- --version
```

## Checking a change

```sh
cargo xtask gate --only preflight   # lists any missing tool and how to install it
cargo xtask gate                    # every gate, as CI runs it
```

## License

Licensed under either of the [Apache License, Version 2.0](LICENSE-APACHE) or the
[MIT license](LICENSE-MIT), at your option.
