# ATS-ZKP – Allfeat Time-Stamp Zero-Knowledge Proofs

[![Rust](https://github.com/allfeat/allfeat-sdk/workflows/Rust/badge.svg)](https://github.com/allfeat/allfeat-sdk)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A Rust crate implementing **zero-knowledge circuits and cryptographic utilities** for Allfeat’s timestamping protocol.
It provides deterministic hashing of musical metadata into BN254 field elements (`Fr`) and verification of **Groth16 zk-SNARKs**.

## Overview

The ATS-ZKP crate defines the cryptographic backbone for timestamping and commitment proofs:

- 🎶 **Title Hashing** – SHA-256 → BN254 field element
- 👩‍🎤 **Creator Hashing** – Canonicalized concatenation of creators (name, email, roles, IPI, ISNI)
- 🎧 **Audio Hashing** – SHA-256 streaming hash of files (std-only)
- 🔒 **Commitments** – Poseidon-based commitments over audio/title/creators/secret
- ✅ **zk-SNARK Proofs** – Circuits and helpers for Groth16 setup, proving, and verification

## Key Features

### 🔑 Cryptographic Hashing
- **SHA-256 → Fr** reduction via `Fr::from_be_bytes_mod_order`
- Normalized, deterministic encoding of creators and titles
- Poseidon sponge utilities (`h2`, `h4`) for commitment building

### 🧩 Circuits & Proofs
- Groth16 circuits defined with Arkworks
- Helper structs for `PublicInputs` and `Witness`
- APIs for setup, prove, and verify (including serialization to/from hex)

### 🌐 WASM/No-Std Compatibility
- Core hashing runs in `no_std`
- File hashing (`hash_audio`) only available with `std`
- Can be compiled to **WASM** and called from JavaScript

## Quick Start

### Installation

```toml
[dependencies]
allfeat-ats-zkp = { path = "ats/zkp" }

# For std environments (with file hashing support)
allfeat-ats-zkp = { version = "0.1.0", features = ["std"] }

# For WASM/no-std environments
allfeat-ats-zkp = { version = "0.1.0", default-features = false }
```

### Basic Usage

#### Title Hashing

```rust
use allfeat_ats_zkp::hash_title;

let h = hash_title("Bohemian Rhapsody");
println!("Title hash: {:?}", h);
```

#### Creator Hashing

```rust
use allfeat_ats_zkp::{Creator, Roles, hash_creators};

let creators = vec![
    Creator {
        full_name: "Freddie Mercury",
        email: "freddie@example.org",
        roles: Roles { author: true, composer: true, ..Default::default() },
        ipi: Some("12345678901"),
        isni: None,
    }
];

let h = hash_creators(&creators);
println!("Creators hash: {:?}", h);
```

#### Proof Roundtrip

```rust
use allfeat_ats_zkp::{setup, prove, prepare_vk, verify, Witness, PublicInputs};
use rand::thread_rng;

let mut rng = thread_rng();

// example witness + public inputs
let w = Witness { secret: Default::default() };
let p = PublicInputs { /* fill fields */ ..Default::default() };

// setup
let (pk, vk) = setup(&mut rng, (w, p)).unwrap();

// prove
let (proof, publics) = prove(&pk, w, p, &mut rng).unwrap();

// verify
let pvk = prepare_vk(&vk);
assert!(verify(&pvk, &proof, &publics).unwrap());
```

## Architecture

The `ats/zkp` crate is organized into four main modules, each responsible for a distinct part of the zero-knowledge proof pipeline:

### Core Modules

| Module    | Description                                                                 |
|-----------|-----------------------------------------------------------------------------|
| `utils`   | Low-level field utilities: Fr ↔ hex conversion, padding, Poseidon helpers. |
| `hashing` | Deterministic SHA-256 → BN254 field element hashing for titles, creators, and audio files. |
| `circuit` | Arkworks R1CS definition of the Allfeat circuit: witness + public inputs, Groth16 constraints. |
| `api`     | High-level proving system: setup, prove, verify, plus serialization to bytes/hex. |

### Commitment Scheme

The final hash commitment is constructed as:

```text
commitment = Poseidon(hash_audio, hash_title, hash_creators, secret)
nullifier  = Poseidon(commitment, timestamp)
```
