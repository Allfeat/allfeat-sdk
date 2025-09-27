# ATS-ZKP-WASM – WebAssembly Bindings for Allfeat ZKP

A WebAssembly module exposing the **ATS-ZKP** cryptographic primitives and zero-knowledge proof helpers to JavaScript/TypeScript applications.
It is designed to be used in **Next.js, Node.js, and browser environments** to compute musical metadata hashes, commitments, and zk-SNARK proofs without exposing Arkworks internals.

## Overview

The `ats-zkp-wasm` crate is a thin WASM façade on top of [`ats-zkp`](../zkp).
It provides a minimal, **JS-friendly API** with hex strings and plain objects as inputs/outputs.

### Exposed Functions

- 🎶 **`hashTitle`** – Hash a song title into a BN254 field element (`Fr`) hex string
- 👩‍🎤 **`hashCreators`** – Hash a list of creators into an `Fr` hex string
- 🔒 **`genCommitment`** – Compute Poseidon commitments (commitment + nullifier)
- ✅ **`proveAndVerify`** – Run Groth16 proof generation and verification (demo flow)

All heavy logic remains in `ats-zkp`; this crate only exports the essential functions to JS.

## Prerequisites

Install `wasm-pack`:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Building

### For Next.js / Bundler (default)

```bash
wasm-pack build --target bundler --out-dir pkg
```

### For Node.js

```bash
wasm-pack build --target nodejs --out-dir pkg-node
```

### For Browser (ES Modules)

```bash
wasm-pack build --target web --out-dir pkg-web
```

## Testing

Run Rust tests:

```bash
cargo test
```

Run WASM tests:

```bash
wasm-pack test --node
```

## Running the JavaScript Example

An example script is included in `js-example/example.js`.

1. First, build the Node.js bindings:

```bash
wasm-pack build --target nodejs --out-dir pkg-node
```

2. Move into the js-example/ folder:

```bash
cd js-example
```

3. Install dependencies (if you haven’t already):

```bash
npm install
```

4. Run the example with Node.js:

```bash
node example.js
```

This will:

- Read a sample audio file (`sample-audio.mp3`)
- Build a ZKP input bundle (`build_bundle`)
- Generate and print a Groth16 proof (`prove`)
