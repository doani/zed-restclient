# Contributing to REST Client

First off, thank you for considering contributing to the Zed REST Client! It's people like you that make the open-source community such an amazing place to learn, inspire, and create.

## Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Architecture Context
This extension uses a **Hybrid Architecture** due to Zed's WASM sandbox constraints:
1. A **WASM Frontend** (compiled to `wasm32-wasi`) handling Zed UI, Code Lenses, and file parsing.
2. A **Native Rust LSP/Sidecar** handling the actual TCP/HTTP networking.

If you are modifying network logic, look at the sidecar codebase. If you are modifying UI or parsing, look at the extension codebase.

## How to Contribute

**Important Rule: No Feature Without an Issue.**
Before you start working on a new feature, improvement, or bug fix, **you must open an issue first**. This allows us to discuss the idea, align on the approach, and prevent duplicated work.

1. **Discuss first:** If you plan to make significant changes or a new feature, please open an issue first to discuss the proposed changes. Do not create a feature branch without an associated issue.
2. **Rust Standards:** We use strict Rust standards. No `unwrap()` or `expect()` in production code. Use `Result` and appropriate error handling.
3. **Tests:** Ensure you write tests for your changes. Run `cargo test` and `cargo fmt` before submitting a PR.
4. **Branches:** Prefix your branches appropriately and reference the issue (`feat/issue-123-AmazingFeature`, `fix/issue-124-BugFix`, `docs/`).

## Local Setup & Development

To run and test the extension locally within the Zed Editor:

1. **Install Prerequisites:**
   Make sure you have Rust installed. You must also add the WASM target:
   
   If you installed Rust via **rustup** (recommended):
   ```bash
   rustup target add wasm32-wasip1
   ```
   
   If you installed Rust via your system's package manager (e.g., Arch Linux/CachyOS `pacman`):
   ```bash
   sudo pacman -S rust-wasm
   ```

2. **Load the Dev Extension in Zed:**
   - Open your Zed Editor.
   - Open the Command Palette (`cmd-shift-p` on Mac or `ctrl-shift-p` on Linux/Windows).
   - Search for the command: `zed: extensions` and hit Enter.
   - In the Extensions panel, click on the **Install Dev Extension** button.
   - Select the root folder of this repository (`zed-restclient`).
   
3. **Verify it Works:**
   - Zed will automatically compile the WASM extension using `cargo build --target wasm32-wasip1` in the background and load it.
   - Create a new file with the extension `.http` (e.g., `test.http`).
   - Type a simple request:
     ```http
     GET https://api.github.com
     ```
   - You should see the **▶ Send Request** Code Lens appear above the `GET` line!
   
*Note: During active development, if you change the Rust code of the extension or the sidecar, you need to open the Extensions panel in Zed and click **Reload** on the Dev Extension to compile and apply the changes.*
