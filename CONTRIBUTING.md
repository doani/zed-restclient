# Contributing to Zed REST Client

First off, thank you for considering contributing to the Zed REST Client! It's people like you that make the open-source community such an amazing place to learn, inspire, and create.

## Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Architecture Context
This extension uses a **Hybrid Architecture** due to Zed's WASM sandbox constraints:
1. A **WASM Frontend** (compiled to `wasm32-wasi`) handling Zed UI, Code Lenses, and file parsing.
2. A **Native Rust LSP/Sidecar** handling the actual TCP/HTTP networking.

If you are modifying network logic, look at the sidecar codebase. If you are modifying UI or parsing, look at the extension codebase.

## How to Contribute
1. **Discuss first:** If you plan to make significant changes, please open an issue first to discuss the proposed changes.
2. **Rust Standards:** We use strict Rust standards. No `unwrap()` or `expect()` in production code. Use `Result` and appropriate error handling.
3. **Tests:** Ensure you write tests for your changes. Run `cargo test` and `cargo fmt` before submitting a PR.
4. **Branches:** Prefix your branches appropriately (`feat/`, `fix/`, `docs/`).

## Local Setup
*(To be expanded once the scaffolding is finalized in Epic 1)*
