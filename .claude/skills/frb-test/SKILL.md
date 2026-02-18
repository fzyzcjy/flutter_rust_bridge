---
name: frb-test
description: Use when needing to run tests locally in flutter_rust_bridge, or when debugging test failures before pushing
---

# FRB Testing

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
| Web tests | `./frb_internal test-dart-web --package <name>` |

## When to Test Locally

**Run:** Debugging test failure, developing new tests, verifying fix before push

**Skip:** Documentation only, CI will catch it, just checking compilation
