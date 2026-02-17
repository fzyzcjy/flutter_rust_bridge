---
name: frb-test
description: Use when needing to run tests locally in flutter_rust_bridge, or when debugging test failures before pushing
---

# FRB Local Testing

## Overview

This skill maps change types to the appropriate local test commands. Tests run on CI by default, so local testing is optional but useful for debugging.

**Core principle:** Run tests locally only when debugging. Trust CI for verification.

## Quick Reference

| Change Type | Command |
|-------------|---------|
| `frb_rust/` | `./frb_internal test-rust --package frb_rust` |
| `frb_codegen/` | `./frb_internal test-rust --package frb_codegen` |
| Specific Rust package | `./frb_internal test-rust --package <path>` |
| `frb_dart/` | `./frb_internal test-dart-native --package frb_dart` |
| Dart example | `./frb_internal test-dart-native --package frb_example/<name>` |
| Flutter example | `./frb_internal test-flutter-native --package frb_example/flutter_<name>` |
| Web tests | `./frb_internal test-dart-web --package <name>` |

## When to Run Tests Locally

### Run Local Tests

- Debugging a specific test failure
- Developing new test cases
- Verifying fix before pushing
- CI is slow and you want quick feedback

### Skip Local Tests

- Documentation changes only
- CI will catch it anyway
- Just want to see if code compiles

## Commands Explained

### Rust Tests

```bash
# All Rust packages
./frb_internal test-rust

# Specific package
./frb_internal test-rust --package frb_codegen
./frb_internal test-rust --package frb_example/pure_dart/rust
```

### Dart Native Tests

```bash
# frb_dart package
./frb_internal test-dart-native --package frb_dart

# Dart example
./frb_internal test-dart-native --package frb_example/pure_dart
```

### Flutter Tests

```bash
./frb_internal test-flutter-native --package frb_example/flutter_via_create
```

### Web Tests

```bash
./frb_internal test-dart-web --package frb_dart
./frb_internal test-flutter-web --package frb_example/flutter_via_create
```

## Common Scenarios

### Modified frb_rust, want to verify

```bash
./frb_internal test-rust --package frb_rust
```

### Modified example API, want to verify Dart side

```bash
./frb_internal test-dart-native --package frb_example/pure_dart
```

### Debugging a failing CI test

```bash
# Reproduce locally with same package
./frb_internal test-rust --package frb_example/pure_dart/rust
```

## Note

- Tests run automatically on CI
- Local testing is for debugging, not required before PR
- If CI fails, check the error message and run the corresponding local test
