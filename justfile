build-wasm:
  cd wasm && wasm-pack build --target nodejs

build-core:
    cargo build -p allfeat-sdk-core --release
