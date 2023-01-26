#!/bin/bash

prerequisit_wasm32() {
  rustup show | grep wasm32-unknown-unknown &> /dev/null

  if [ $? == 0 ]; then
    return 0;
  fi

  echo "rust WASM toolchain not installed. Run the following command to install:"
  echo ""
  echo "rustup target install wasm32-unknown-unknown"
  return 1
}

prerequisit_wasm_bindgen() {
  wasm-bindgen --version &> /dev/null

  if [ $? == 0 ]; then
    return 0;
  fi

  echo "wasm-bindgen not installed. Run the following command to install:"
  echo ""
  echo "cargo install wasm-bindgen-cli"
  return 1
}

prerequisit_basic_http_server() {
  basic-http-server --version &> /dev/null

  if [ $? == 0 ]; then
    return 0;
  fi

  echo "basic-http-server not installed. Run the following command to install:"
  echo ""
  echo "cargo install basic-http-server"
  return 1
}

prerequisits() {
  prerequisit_wasm32 && prerequisit_wasm_bindgen && prerequisit_basic_http_server
}

build() {
  cargo build \
    --release \
    --no-default-features \
    --target wasm32-unknown-unknown
}

bindgen() {
  wasm-bindgen \
    --out-name wasm_example \
    --out-dir target/wasm \
    --target web \
    target/wasm32-unknown-unknown/release/carbonaria.wasm
}

copy_assets() {
  cp assets/index.html target/wasm/index.html
  cp -r assets target/wasm/
}

run_server() {
  basic-http-server target/wasm
}

main(){
  prerequisits \
  && build \
  && bindgen \
  && copy_assets \
  && run_server
}
main "$@"
