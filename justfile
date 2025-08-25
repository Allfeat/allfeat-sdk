build-client:
    cargo build -p allfeat-client --release

build-midds:
    cargo build -p allfeat-midds-v2 --release

build-midds-runtime:
    cargo build -p allfeat-midds-v2 --release --features runtime

build-midds-js:
    cd midds-v2 && wasm-pack build --target nodejs --features web

gen-metadata-melodie:
    subxt metadata --url ws://127.0.0.1:9944 > ./client/artifacts/melodie_metadata.scale
