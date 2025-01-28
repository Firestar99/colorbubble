#!/bin/bash
cargo watch -s "wasm-pack build --target web --dev && simple-http-server -i"
