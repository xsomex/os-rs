#!/bin/sh

cd kernel
cargo build --target x86_64-unknown-none --target-dir target
cd ../builder
cargo run
cd ..
