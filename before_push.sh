#!/bin/sh

cd foundry
git pull
cd ..
cargo test
cargo check
cargo clippy
cargo fmt
