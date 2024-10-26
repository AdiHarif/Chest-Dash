#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rust_gamedev_project.wasm .
zip -r rustgame.zip index.html rust_gamedev_project.wasm assets
