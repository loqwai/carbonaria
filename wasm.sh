#!/bin/bash

build() {
  cargo build \
    --release \
    --target wasm32-unknown-unknown
}

bindgen() {
  wasm-bindgen \
    --out-name wasm_example \
    --out-dir target/wasm \
    --target web \
    target/wasm32-unknown-unknown/release/carbonaria.wasm
}

run_server() {
  basic-http-server target/wasm
}

main(){
  build \
  && bindgen \
  && run_server
}
main "$@"
