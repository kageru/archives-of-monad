#!/bin/sh

cargo check
cargo clippy
cargo fmt
cargo test
