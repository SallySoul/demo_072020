#!/usr/bin/env sh

rm -rf target/web
mkdir -p target/web

cp src_web/index.html src_web/gl.js target/web
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/demo_072020.wasm target/web
