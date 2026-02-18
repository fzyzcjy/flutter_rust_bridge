---
name: frb-code-generation
description: Use when modifying Rust API, codegen, or example code in flutter_rust_bridge to determine which generation commands to run
---

# FRB Code Generation

## Overview

flutter_rust_bridge requires code generation when Rust APIs change. This skill maps change types to the minimal generation commands needed.

**Core principle:** Run only the generation commands needed for your change type.

## Quick Reference

| Change Type | Command |
|-------------|---------|
| Example Rust API (`frb_example/**/rust/src/api/*.rs`) | `./frb_internal precommit-generate` |
| Flutter integrate examples | `./frb_internal precommit-integrate` |
| `frb_codegen/` generation logic | `./frb_internal precommit-generate` + `generate-internal-rust` |
| `frb_rust/src/` core API | `./frb_internal generate-internal-rust` |
| `frb_example/pure_dart` generator | `./frb_internal generate-internal-frb-example-pure-dart` |
| CLI help documentation | `./frb_internal generate-internal-book-help` |
| Non-generated files (docs, comments, tests, `frb_dart/`) | No generation needed |

## When Unsure

If uncertain which commands to run, use the full precommit:

```bash
./frb_internal precommit --mode slow
```

This runs all generation, linting, and tests. Slower but safe.

## When to Run Generation

### Always Run Generation

- Modified `frb_example/**/rust/src/api/*.rs` (example API definitions)
- Modified `frb_codegen/src/` (codegen logic)
- Modified `frb_rust/src/` (core runtime, affects generated bindings)

### Never Run Generation

- Documentation only (`.md` files, comments)
- Test files only
- CI/CD configuration
- Non-API Rust code in examples

## Commands Explained

### `precommit-generate`

Runs `frb_codegen generate` for all Dart example packages.

```bash
./frb_internal precommit-generate
```

**When to use:** After modifying example Rust APIs.

### `precommit-integrate`

Re-creates Flutter integrate examples from scratch.

```bash
./frb_internal precommit-integrate
```

**When to use:** After modifying `frb_codegen create` or `frb_codegen integrate` logic.

### `generate-internal-rust`

Runs `frb_codegen internal-generate` to regenerate Rust bindings.

```bash
./frb_internal generate-internal-rust
```

**When to use:** After modifying `frb_codegen/` or `frb_rust/src/`.

### `generate-internal-frb-example-pure-dart`

Regenerates `frb_example/pure_dart` test code.

```bash
./frb_internal generate-internal-frb-example-pure-dart
```

**When to use:** After modifying the pure_dart generator.

### `generate-internal-book-help`

Regenerates CLI help documentation for website.

```bash
./frb_internal generate-internal-book-help
```

**When to use:** After adding/modifying CLI commands or arguments.

## Common Scenarios

### Added a new function to example API

```bash
# Edit frb_example/pure_dart/rust/src/api/simple.rs
./frb_internal precommit-generate
```

### Modified core frb_rust API

```bash
# Edit frb_rust/src/*.rs
./frb_internal generate-internal-rust
```

### Changed codegen template or logic

```bash
# Edit frb_codegen/src/*.rs
./frb_internal precommit-generate
./frb_internal generate-internal-rust
```

### Only documentation changes

```bash
# No generation needed
```
