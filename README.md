# 📊 allfeat-sdk-wasm

> ⚠️ **Experimental SDK – Work in Progress**

This is the bleeding-edge WebAssembly (WASM) SDK for the **Allfeat** blockchain ecosystem.
It is designed to bridge **Rust-native blockchain interactions** with the flexibility of **JavaScript/TypeScript development**.

---

## 🚧 Disclaimer

> **This SDK is highly experimental and under active development.** > **Not production-ready. Breaking changes will happen. Use with caution.**

---

## ✨ What it does

This SDK enables you to:

- Interact with the **Allfeat blockchain** from **JavaScript/TypeScript** via WebAssembly
- Leverage the **power of Subxt** (Rust client for Substrate chains)
- Use the **native types** and metadata of the Allfeat chain with full Rust guarantees
- Call extrinsics (e.g., `remark`) and sign them via browser extensions like **polkadot-js**
- Submit signed transactions directly from a JS front-end (Nuxt, React, etc.)

---

## 📚 Features (planned/in-progress)

- ✅ `system.remark` extrinsic
- 🧹 Custom calls from the Allfeat chain (e.g., MIDDS, Allstamp)
- 🔐 Custom signer injection
- 🧠 Native decoding + signature validation in Rust
- 🧪 Transaction preview, dry-run, and metadata reflection

---

## 📌 Status

| Feature        | Status                            |
| -------------- | --------------------------------- |
| WASM Binding   | ✅ Initial impl                   |
| Signer support | ✅ Custom `signPayload` injection |
| Tx submission  | ✅ Basic support                  |
| MIDDS support  | 🚧 Coming soon                    |
| Error handling | 🚧 In progress                    |

## 🧠 Why Rust + WASM?

This SDK leverages:

- 🦠 **Rust** for memory safety and type-rich blockchain access
- 🛆 **Subxt** for native call building, metadata handling, and signing logic
- 🔸 **WebAssembly** to expose those capabilities to the JS world

---

## 🤝 Contributing

PRs welcome — the SDK is evolving rapidly alongside the Allfeat chain.
