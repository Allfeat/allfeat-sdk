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
