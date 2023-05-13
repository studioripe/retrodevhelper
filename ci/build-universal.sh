#!/bin/sh
# This script builds a universal binary for macOS
rustup target install x86_64-apple-darwin

cargo build --target aarch64-apple-darwin --release
cargo build --target x86_64-apple-darwin --release

cd target
lipo -create -output ./retrodevhelper ./x86_64-apple-darwin/release/retrodevhelper ./aarch64-apple-darwin/release/retrodevhelper

tar -zcvf retrodevhelper-macos-universal.tar.gz ./retrodevhelper 