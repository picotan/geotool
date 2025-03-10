#!/bin/sh

cargo lipo --release
cargo build --target=aarch64-apple-ios --release
cargo build --target=aarch64-apple-ios-sim --release
cargo build --target=aarch64-apple-darwin --release
cargo build --target=aarch64-apple-ios 
cargo build --target=aarch64-apple-ios-sim 
cargo build --target=aarch64-apple-darwin 
cbindgen --config cbindgen.toml --crate geocore --output geocore.h --lang c++

