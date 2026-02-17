---
name: frb-develop-feature
description: Use when adding tests or developing new features in flutter_rust_bridge, covers test patterns and testing bed setup
---

# FRB Develop Feature

## Overview

Guide for developing features and adding tests in flutter_rust_bridge.

## When to Use

Use this skill when:
- Adding a new function or feature to flutter_rust_bridge
- Writing new tests for existing or new functionality
- Need a quick testing environment during development

## Use dart_minimal as Testing Bed

`frb_example/pure_dart` is large and slow to compile due to its comprehensive test coverage. Use `frb_example/dart_minimal` as a quick testing bed:

- **Minimal dependencies**: Only includes essential crates, so compilation is much faster
- **Ad-hoc testing**: Add functions directly to examine outputs and behavior during development
- **Faster iteration**: Quick compile-test cycles without waiting for full pure_dart rebuilds

When to use each:
- `dart_minimal`: Quick experiments, debugging specific behavior, during active development
- `pure_dart`: Final testing, comprehensive coverage, before submitting PRs

## How to Add a Test

> **Tip:** This package has scripts to automatically create more tests based on the test you write, so you write one test and get (usually) six tests ;)

### Steps

1. In `./frb_example/pure_dart`, add your test in:
   - `rust/src/api/whatever.rs`
   - `test/api/whatever_test.dart`

2. Use twin test naming conventions to enable automatic test generation:

   **Rust side** (`rust/src/api/whatever.rs`):
   - Function name suffix `_twin_normal` → generates variants for different configurations

   **Dart side** (`test/api/whatever_test.dart`):
   - Test class suffix `TwinNormal` → auto-generates multiple test configurations

   These suffixes allow the internal scripts to create "twin" tests that run the same logic under different codegen modes (e.g., with/without Dart snapshot, different crate types). Mimic existing tests in the codebase for the exact patterns.

3. Run generation:
   ```bash
   # Full (slow) - runs all precommit checks including lints, formatting, and generation
   ./frb_internal precommit --mode slow

   # Faster alternative - only generates what's needed for pure_dart:
   # 1. generate-internal-frb-example-pure-dart: generates internal test infrastructure
   # 2. generate-run-frb-codegen-command-generate: runs codegen for the specific package
   ./frb_internal generate-internal-frb-example-pure-dart && ./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart
   ```

## Debug with Dumped Data

When investigating code generation issues or complex behavior, enable data dumping:

1. Set `dump_all: true` in your configuration (e.g., in `frb_example/dart_minimal/rust/src/lib.rs` or relevant config file)

2. Run your test or code generation

3. Check the dumped data at:
   ```
   rust/target/frb_dump/
   ```

The dump contains intermediate representations, generated code fragments, and other debugging information useful for understanding how FRB processes your code.

## Related Skills

- `frb-code-generation` - What generation commands to run
- `frb-test` - How to run tests locally
- `frb-prepare-pr` - Preparing a PR for review (pre-commit checks, CI considerations)
