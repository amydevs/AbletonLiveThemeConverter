#!/usr/bin/env bash
wasm-pack build "$@" && cargo xtask generate-json-schema ./pkg/schema.json && ./scripts/addToFiles.sh schema.json