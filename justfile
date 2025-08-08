build-client:
    cargo build -p allfeat-client --release

build-midds:
    cargo build -p allfeat-midds --release

build-js-bindings:
    cd wasm-bindings && wasm-pack build --out-dir js/dist && cd js/dist && rm README.md && rm package.json

gen-metadata-melodie:
    subxt metadata --url ws://127.0.0.1:9944 > ./client/artifacts/melodie_metadata.scale
