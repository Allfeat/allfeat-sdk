build-client-wasm:
  cd client && wasm-pack build --target nodejs --no-default-features --features js

build-client:
    cargo build -p allfeat-client --release

build-midds-wasm:
    cd midds && wasm-pack build --target nodejs --no-default-features --features js

build-midds:
    cargo build -p allfeat-midds --release
