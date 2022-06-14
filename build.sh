#!/bin/bash
cargo build --lib --release

cargo build --bin html-to-seed --release

cd html-to-seed-web
trunk build --release
cd ..