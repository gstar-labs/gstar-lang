#!/usr/bin/env bash
# Installs the external tools pinned in tools/versions.toml (CI only).
# Versions come from `cargo xtask tool-version`; this script holds no logic.
set -euo pipefail
bin="$HOME/.local/gstar-tools/bin"
mkdir -p "$bin"
echo "$bin" >> "$GITHUB_PATH"
export PATH="$bin:$PATH"
version() { cargo xtask tool-version "$1"; }

for tool in cargo-nextest cargo-shear cargo-deny cargo-llvm-cov similarity-rs; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    cargo install --locked --root "$HOME/.local/gstar-tools" "$tool@$(version "$tool")"
  fi
done

if ! command -v z3 >/dev/null 2>&1; then
  gh release download "z3-$(version z3)" --repo Z3Prover/z3 --pattern '*x64-glibc*.zip' --dir /tmp/z3
  unzip -q /tmp/z3/*.zip -d /tmp/z3 && cp /tmp/z3/*/bin/z3 "$bin/"
fi

if ! command -v cvc5 >/dev/null 2>&1; then
  gh release download "cvc5-$(version cvc5)" --repo cvc5/cvc5 --pattern '*Linux-x86_64-static.zip' --dir /tmp/cvc5
  unzip -q /tmp/cvc5/*.zip -d /tmp/cvc5 && cp /tmp/cvc5/*/bin/cvc5 "$bin/"
fi

if ! command -v elan >/dev/null 2>&1; then
  curl -sSf https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh \
    | sh -s -- -y --default-toolchain none
  echo "$HOME/.elan/bin" >> "$GITHUB_PATH"
fi
