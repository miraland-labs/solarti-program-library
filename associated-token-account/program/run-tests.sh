#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")"
cargo clippy
cargo build
# MI
# cargo build-sbf
cargo build-sbf "$@"

if [[ $1 = -v ]]; then
  export RUST_LOG=solana=debug,miraland=debug
fi

cargo test
# MI
# cargo test-sbf
cargo test-sbf "$@"
