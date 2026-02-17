---
name: frb-develop-feature
description: Use when adding tests or developing new features in flutter_rust_bridge, when compilation is slow, when learning twin test naming conventions, or when debugging code generation
---

# FRB Develop Feature

## Overview

Guide for developing features and adding tests in flutter_rust_bridge.

## When to Use

- Adding a new function or feature
- Writing tests for new or existing functionality
- Compilation feels slow (use dart_minimal instead)
- Need to debug code generation behavior

## Quick Reference

| Task | Where | Command |
|------|-------|---------|
| Quick testing | `frb_example/dart_minimal` | Faster compile |
| Full test suite | `frb_example/pure_dart` | Comprehensive |
| Code gen (fast) | - | `./frb_internal generate-internal-frb-example-pure-dart && ./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart` |
| Code gen (full) | - | `./frb_internal precommit --mode slow` |
| Debug dumps | `rust/target/frb_dump/` | Set `dump_all: true` |

## Testing Bed Selection

**Problem:** `frb_example/pure_dart` has comprehensive coverage but slow compilation.

**Solution:** Use `frb_example/dart_minimal` for quick iteration:

| Scenario | Use |
|----------|-----|
| Quick experiments, active development | `dart_minimal` |
| Final testing, before PR submission | `pure_dart` |

`dart_minimal` has minimal dependencies → faster compile → quicker iteration cycles.

## How to Add a Test

> **Tip:** Write one test, get ~6 variants automatically via twin naming convention.

### Steps

1. In `./frb_example/pure_dart`, add your test in:
   - `rust/src/api/whatever.rs`
   - `test/api/whatever_test.dart`

2. Use twin test naming to enable automatic test generation:

   | Side | File | Convention |
   |------|------|------------|
   | Rust | `rust/src/api/whatever.rs` | Function suffix `_twin_normal` |
   | Dart | `test/api/whatever_test.dart` | Class suffix `TwinNormal` |

   **Why:** Internal scripts create "twin" tests running the same logic under different codegen modes (with/without Dart snapshot, different crate types). Mimic existing tests for exact patterns.

3. Run generation:
   ```bash
   # Full (slow) - all precommit checks
   ./frb_internal precommit --mode slow

   # Fast - only pure_dart generation
   ./frb_internal generate-internal-frb-example-pure-dart && \
   ./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart
   ```

## Debug Code Generation

When generated code looks wrong or you need to understand FRB internals:

1. Enable dumping in config (e.g., `frb_example/dart_minimal/rust/src/lib.rs`):
   ```rust
   flutter_rust_bridge::frb_generated_boilerplate!(dump_all: true);
   ```

2. Run code generation

3. Inspect `rust/target/frb_dump/` for intermediate representations

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-test` - How to run tests locally
- `frb-prepare-pr` - Preparing a PR for review
