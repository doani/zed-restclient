# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-30
### Added
- **Initial Release**: Basic REST Client extension for the Zed editor.
- **In-Editor Requests**: Send HTTP requests directly from `.http` or `.rest` files using integrated Code Lenses ("Send Request").
- **Native Sidecar Backend**: A robust, Rust-based language server handles requests securely and efficiently.
- **Variables Support**: Basic support for defining and resolving variables directly within your HTTP files.
- **GitHub Actions**: Automated release pipeline for building and distributing the native sidecar binary for macOS (Intel/ARM), Linux, and Windows.
