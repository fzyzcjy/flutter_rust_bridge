---
name: frb-lint
description: Use when you need to run lint, format, or clippy checks in flutter_rust_bridge
---

# FRB Lint

> **Note:** Read `frb-dev-env` before running commands and follow the active user's environment rules.

## Quick Reference

| Command | Description |
|---------|-------------|
| `./frb_internal lint --fix` | Run all lints and auto-fix (recommended) |
| `./frb_internal lint` | Run all lints |
| `./frb_internal lint-dart` | Dart only |
| `./frb_internal lint-rust` | Rust only |

## What Lint Includes

It contains:
- `dart analyze`
- `dart format`
- `cargo clippy`
- `cargo fmt`
