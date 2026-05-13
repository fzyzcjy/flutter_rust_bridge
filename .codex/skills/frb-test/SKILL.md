---
name: frb-test
description: Use when needing to run tests locally in flutter_rust_bridge, or when debugging test failures before pushing
---

# FRB Testing

> **Note:** Check your user-level `remote-testing` rules before running commands. Tests may require remote execution.

## Overview

Maps change types to local test commands. Tests run on CI by default, so local testing is optional but useful for debugging.

## Quick Reference

| Change Type | Command |
|-------------|---------|
| All Rust packages | `./frb_internal test-rust` |
| `frb_rust/` | `./frb_internal test-rust-package --package frb_rust` |
| `frb_codegen/` | `./frb_internal test-rust-package --package frb_codegen` |
| Specific Rust package | `./frb_internal test-rust-package --package <path>` |
| `frb_dart/` | `./frb_internal test-dart-native --package frb_dart` |
| Dart example | `./frb_internal test-dart-native --package frb_example/<name>` |
| Flutter example | `./frb_internal test-flutter-native --package frb_example/flutter_<name>` |
| Web test | `./frb_internal test-dart-web --package frb_example/<name>` |

## Web Tests

Web tests compile Dart tests to WebAssembly and run in a headless browser.

### Supported Packages

- `frb_dart`
- `frb_example/dart_minimal`
- `frb_example/pure_dart`
- `frb_example/pure_dart_pde`

### Requirements

- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown --toolchain nightly-2025-02-01`
- `rust-src` component: `rustup component add rust-src --toolchain nightly-2025-02-01`
- `wasm-pack`: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- Chrome/Chromium browser

### Running in Docker

When running web tests in Docker containers, puppeteer needs `--no-sandbox` flag. The test runner auto-detects Docker environment via `/.dockerenv`.

## When to Test Locally

**Run:** Debugging test failure, developing new tests, verifying fix before push

**Skip:** Documentation only, CI will catch it, just checking compilation

## CI Discipline

CI is slow and often queues for a long time. Before pushing a non-trivial FRB change, run meaningful local tests in the appropriate development environment and understand the result locally first. Scale the local test scope with the risk and blast radius of the change. Do not use CI as the first feedback loop for changes that can be verified locally.

Choose tests by blast radius:
- Runtime changes: run the relevant Rust package tests
- Generated or example API changes: run the focused Dart or Flutter example test first
- Codegen changes: run generation and the affected example tests

After pushing to a PR branch, monitor GitHub Actions until the run reaches a terminal state. If CI fails, inspect the failing job logs and either fix the issue or clearly report why it is unrelated or flaky. Do not leave a PR in an unknown queued or in-progress state when the user asked for CI validation.
