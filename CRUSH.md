CRUSH.md — Commands and Conventions for Allfeat SDK

Build, Lint, Test
- Prereqs: Node >=20, pnpm >=8, Rust toolchain (workspace uses edition 2024), wasm-pack if building WASM.
- Install JS deps: pnpm install
- Build everything (JS packages via workspace recursion): pnpm run build
- Typecheck all JS/TS: pnpm run typecheck
- Lint (JS packages that define it; workspace passthrough): pnpm run lint
- Test (JS workspaces that define tests): pnpm run test
- Run a single JS package script: pnpm -F <package-name> <script>
  - Examples: pnpm -F @allfeat/types/midds build, pnpm -F @allfeat/papi-providers typecheck
- Clean: pnpm run clean

Rust-specific
- Build all crates: cargo build
- Release builds:
  - Client: just build-client  (cargo build -p allfeat-client --release)
  - MIDDS: just build-midds    (cargo build -p allfeat-midds-v2 --release)
  - MIDDS (runtime features): just build-midds-runtime
- Tests: cargo test (use cargo test <name> to run a single test; add -- --ignored for ignored tests)
- Format: cargo fmt --all
- Lint: cargo clippy --all-targets --all-features -D warnings
- WASM build for midds-v2 -> JS bindings: just build-midds-js (requires wasm-pack; builds with --features web)

TypeScript package details
- Packages: packages/midds-js (bundled with tsup), packages/papi-providers (tsc). Root workspace uses pnpm -r to recurse.
- Single test (if a package adds a test runner): pnpm -F <pkg> test -- <matcher>
- Typecheck only: pnpm -F <pkg> typecheck

Imports and Formatting
- JS/TS: ESM modules ("type": "module"); prefer named exports; path-based exports configured in package.json.
- Rust: follow module tree per crate; use prelude-style grouped imports; keep std/external/internal import groups separated and sorted.
- Formatting: use rustfmt (cargo fmt) for Rust; use tsconfig/tsup defaults for JS/TS; 2 spaces from .editorconfig.

Types and Naming
- Rust: snake_case for functions/vars, CamelCase for types/structs/enums, SCREAMING_SNAKE_CASE for consts. Prefer explicit types; avoid unwrap in library code; use Result<T, E>.
- TS: enable strict typing; prefer interfaces/types over any; use PascalCase for types/classes, camelCase for vars/functions; keep public API types in dist.

Error Handling
- Rust: use anyhow/thiserror patterns if added; otherwise define error enums per crate and return Result. Avoid panics in library code. Propagate with ?.
- JS/TS: throw Error with clear messages; narrow unknown; validate inputs (zod is available in @allfeat/midds-js).

Workspace notes
- Cargo workspace members: client, midds-v2 (and midds-v2/midds-v2-codegen).
- .cargo/config sets TS_RS_EXPORT_DIR -> packages/midds-js/src for ts-rs exports when feature "ts-rs" is enabled in allfeat-midds-v2.
- justfile provides common tasks; use just <task>. If just isn’t installed, install it or run the underlying cargo/wasm-pack commands.

Cursor/Copilot rules
- No Cursor or Copilot instruction files were found in this repo at the time of writing. If added later (e.g., .cursorrules or .github/copilot-instructions.md), mirror key rules here.
