#!/usr/bin/env bash
# Installs the external tools pinned in tools/versions.toml (CI only).
# Versions come from `cargo xtask tool-version`; this script holds only install
# commands. A tool counts as installed only when it is in the tool directory
# (restored from the cache), never because the runner image has some version.
set -euo pipefail
shopt -s inherit_errexit nullglob
root="$HOME/.local/gstar-tools"
bin="$root/bin"
elan_bin="$HOME/.elan/bin"
mkdir -p "$bin"
echo "$bin" >> "$GITHUB_PATH"
echo "$elan_bin" >> "$GITHUB_PATH"
export PATH="$bin:$elan_bin:$PATH"
version() { cargo xtask tool-version "$1"; }

# Downloads the single release asset of repository $1, tag $2, matching $3 into
# a fresh directory, and prints its path.
fetch() {
  local dir
  dir="$(mktemp -d)"
  gh release download "$2" --repo "$1" --pattern "$3" --dir "$dir"
  local files=("$dir"/*)
  if [ "${#files[@]}" -ne 1 ]; then
    echo "expected one $1 asset matching $3, got ${#files[@]}" >&2
    return 1
  fi
  echo "${files[0]}"
}

for tool in cargo-nextest cargo-shear cargo-deny cargo-llvm-cov similarity-rs; do
  if [ ! -x "$bin/$tool" ]; then
    cargo install --locked --root "$root" "$tool@$(version "$tool")"
  fi
done

if [ ! -x "$bin/z3" ]; then
  z3_version="$(version z3)"
  archive="$(fetch Z3Prover/z3 "z3-$z3_version" "z3-$z3_version-x64-glibc-*.zip")"
  unzip -q -j -o "$archive" '*/bin/z3' -d "$bin"
fi

if [ ! -x "$bin/cvc5" ]; then
  archive="$(fetch cvc5/cvc5 "cvc5-$(version cvc5)" 'cvc5-Linux-x86_64-static.zip')"
  unzip -q -j -o "$archive" '*/bin/cvc5' -d "$bin"
fi

if [ ! -x "$elan_bin/elan" ]; then
  archive="$(fetch leanprover/elan "v$(version elan)" 'elan-x86_64-unknown-linux-gnu.tar.gz')"
  tar -xzf "$archive" -C "$(dirname "$archive")" elan-init
  "$(dirname "$archive")/elan-init" -y --default-toolchain none
fi
