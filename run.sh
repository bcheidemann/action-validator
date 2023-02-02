# exit on error
set -e

wasm-pack build --target nodejs --features js
node --trace-uncaught ./test.js
