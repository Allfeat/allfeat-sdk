# ATS-ZKP-WASM – WebAssembly Bindings for Allfeat ZKP

A WebAssembly module exposing the **ATS-ZKP** cryptographic primitives and zero-knowledge proof helpers to JavaScript/TypeScript applications.
It is designed to be used in **Next.js, Node.js, and browser environments** to compute musical metadata hashes, commitments, and zk-SNARK proofs without exposing Arkworks internals.

## Overview

The `ats-zkp-wasm` crate is a thin WASM façade on top of [`ats-zkp`](../zkp).
It provides a minimal, **JS-friendly API** with hex strings and plain objects as inputs/outputs, and gives you four high-level functions:

- **`build_bundle(title, audioBytes, creators, timestampBigInt)` -> `{ bundle }`**
  Computes:
  - `hash_title`, `hash_audio`, `hash_creators`
  - a fresh random `secret`
  - Poseidon `commitment` and `nullifier`
  Returns everything as **hex strings**. Note: `bundle.timestamp` is the **timestamp encoded as an `Fr` hex**, ready to pass to proof/verify.

- **`calculate_commitment(title, audioBytes, creators, secretHex)` -> `commitmentHex`**
  Computes the Poseidon hash commitment from the provided inputs using an **existing secret**:
  - `hash_title`, `hash_audio`, `hash_creators` (computed internally)
  - `commitment` = Poseidon(`hash_title`, `hash_audio`, `hash_creators`, `secret`)
  Returns the **commitment as a hex string**. Use this when you already have a secret (e.g., from a previous `build_bundle` call) and need to recompute or verify the commitment.

- **`prove(pkHex, secretHex, publicsArray)` -> `{ proof, publics }`**
  Generates a Groth16 proof using the **compressed PK** (0x-hex) and **6 public inputs** in this exact order:
  `[hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]`.

- **`verify(vkHex, proofHex, publicsArray)` -> `boolean`**
  Verifies a proof using the **compressed VK** (0x-hex) and the same 6 publics (0x-hex) in the **same order**.

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

3. Run the example with Node.js:

```bash
node example.js
```

This will:

- Read a sample audio file (`sample-audio.mp3`)
- Build a ZKP input bundle (`build_bundle`)
- Generate and print a Groth16 proof (`prove`)

## Integration in TypeScript/JavaScript Applications

### Next.js Integration

1. Build the WASM module:

```bash
wasm-pack build --target bundler --out-dir pkg
```

2. Copy the `pkg` folder to your Next.js project (e.g., `src/lib/ats-cert-generator`)

3. Configure Next.js for WASM support in `next.config.ts`:

```typescript
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  webpack: (config) => {
    // Enable async WebAssembly
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };

    // Ensure .wasm files are handled as async webassembly
    config.module.rules.push({
      test: /\.wasm$/,
      type: "webassembly/async",
    });

    return config;
  },
};

export default nextConfig;
```

4. Use in your Next.js components:

```typescript
// app/components/ZkpDemo.tsx (or any client component)
"use client";

import { useState, useRef } from "react";

export default function ZkpDemo() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string>("");
  const fileRef = useRef<HTMLInputElement | null>(null);

  const onRun = async () => {
    if (typeof window === "undefined") return; // guard SSR

    try {
      setLoading(true);
      setResult("");

      // 1) Dynamically load the wasm-bindgen JS and init the WASM
      const init = (await import("@/lib/ats-zkp-wasm/ats_zkp_wasm.js")).default;
      const { build_bundle, prove, verify } = await import(
        "@/lib/ats-zkp-wasm/ats_zkp_wasm.js"
      );
      await init(); // VERY IMPORTANT: initialize the wasm module

      // 2) Read the audio file as Uint8Array
      const file = fileRef.current?.files?.[0];
      if (!file) {
        setResult("Please choose an audio file first.");
        return;
      }
      const buf = new Uint8Array(await file.arrayBuffer());

      // 3) Prepare inputs (creators must match JsCreator)
      const creators = [
        { fullName: "Alice", email: "alice@example.com", roles: ["AT"] },
      ];
      const title = "Song Title";
      const timestamp = BigInt(Math.floor(Date.now() / 1000)); // u64-safe

      // 4) Build the bundle (hashes, secret, commitment, nullifier)
      const { bundle } = build_bundle(title, buf, creators, timestamp);

      // 5) Prove using your PK (hex string, 0x-prefixed, compressed)
      //    You can import it or fetch it from your API/secrets manager.
      const { PK } = await import("@/lib/ats-zkp-wasm/pk.js");
      const publics = [
        bundle.hash_title,
        bundle.hash_audio,
        bundle.hash_creators,
        bundle.commitment,
        bundle.timestamp, // already Fr-hex from build_bundle
        bundle.nullifier,
      ];
      const { proof, publics: publicsProof } = prove(PK, bundle.secret, publics);

      // 6) Verify using your VK (hex string, 0x-prefixed, compressed)
      const { VK } = await import("@/lib/ats-zkp-wasm/vk.js");
      const ok = verify(VK, proof, publicsProof);

      setResult(
        [
          `secret: ${bundle.secret} (keep this PRIVATE)`,
          `proof: ${proof}`,
          `verify: ${ok}`,
        ].join("\n")
      );
    } catch (err) {
      // Errors are forwarded from Rust via `JsValue::from_str`, so they arrive as strings
      console.error("ZKP flow failed:", err);
      setResult(`Error: ${String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <input ref={fileRef} type="file" accept="audio/*" />
      <button onClick={onRun} disabled={loading}>
        {loading ? "Running..." : "Run ZKP demo"}
      </button>
      <pre style={{ whiteSpace: "pre-wrap" }}>{result}</pre>
    </div>
  );
}
```
