#!/bin/bash

set +e

cargo build --release  --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/life_web.wasm demo
