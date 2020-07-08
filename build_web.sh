#!/usr/bin/sh

rm -rf target/web
mkdir -p target/web

cp src_web/index.html src_web/gl.js src_web/live.js target/web
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/web-tetris.wasm target/web
