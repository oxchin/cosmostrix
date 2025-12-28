#!/usr/bin/env bash
set -euo pipefail

if ! command -v pkg >/dev/null 2>&1; then
  echo "This script is intended to run inside Termux." >&2
  exit 1
fi

pkg update -y
pkg install -y rust git clang

cargo test --all
cargo build --profile release

# Non-interactive sanity run
./target/release/cosmostrix --info
