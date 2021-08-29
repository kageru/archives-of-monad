#!/bin/sh

cd foundry
git pull
cd ..
cargo check
cargo clippy
cargo fmt
cargo test
