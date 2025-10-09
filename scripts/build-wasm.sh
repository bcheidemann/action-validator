#!/usr/bin/env bash

set -euo pipefail

WASM_PACK_FLAGS=""

# Parse command line arguments
if [ "${1:-}" = "--dev" ]; then
  WASM_PACK_FLAGS="--no-opt"
fi

npx wasm-pack build $WASM_PACK_FLAGS --out-dir target/wasm-pack/build --no-typescript --target nodejs --no-default-features --features js
rm -rf packages/core/snippets
cp -R target/wasm-pack/build/snippets packages/core/snippets
cp target/wasm-pack/build/action_validator_bg.wasm packages/core/
cp target/wasm-pack/build/action_validator.js packages/core/
