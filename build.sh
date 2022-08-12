#!/bin/sh

wasm-pack build --release --target web
cp pkg/*.js pkg/*.wasm pkg/*.ts svelte/src/wasm/
