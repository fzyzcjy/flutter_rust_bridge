---
name: frb-lint
description: Use when you need to run lint, format, or clippy checks in flutter_rust_bridge
---

# FRB Lint

## Quick Reference

| Command | Description |
|---------|-------------|
| `./frb_internal lint` | Run all lints (Dart + Rust) |
| `./frb_internal lint --fix` | Run all lints and auto-fix |
| `./frb_internal lint-dart` | Dart only (analyze + format) |
| `./frb_internal lint-rust` | Rust only (clippy + fmt) |

## What Lint Includes

**Dart:**
- `dart analyze --fatal-infos`
- `dart format --set-exit-if-changed`

**Rust:**
- `cargo clippy -- -D warnings`
- `cargo +nightly fmt --check`

## With --fix

**Dart:**
- `dart fix --apply`
- `dart format .`

**Rust:**
- `cargo fix --allow-dirty`
- `cargo clippy --fix --allow-dirty`
- `cargo +nightly fmt`
