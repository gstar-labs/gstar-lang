# g-star

**A verified-systems language for code written by agents, and for the humans who trust it.**

> **Experimental.** g-star is at its very beginning. Expect breaking changes and
> missing pieces. Don't use it for anything that matters yet.

Design goal: a program an agent writes is either proved to meet its written
contracts, or rejected with a machine-readable reason.

## What exists today

Only what is listed here works:

- The `gstar` command-line tool, which reports its version.
- The repository gate (`cargo xtask gate`): the definition of done for every change.

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
