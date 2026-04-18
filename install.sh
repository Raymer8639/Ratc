#!/usr/bin/env sh
set -x
cargo build --release

cargo install --force --path crates/ravm
cargo install --force --path crates/rac
