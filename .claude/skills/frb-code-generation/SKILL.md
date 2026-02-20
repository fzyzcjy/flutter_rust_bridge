---
name: frb-code-generation
description: Use when modifying Rust API, codegen, or example code in flutter_rust_bridge to determine which generation commands to run
---

# FRB Code Generation

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen may require remote execution.

## Overview

flutter_rust_bridge requires code generation when Rust APIs change. This skill maps change types to the minimal generation commands needed.

**Core principle:** Run only the generation commands needed for your change type.

> **After codegen:** Check your user-level `remote-testing` rules. If codegen was run remotely, pull changes back to local.

## Quick Reference

| Change Type | Command |
|-------------|---------|
| Example Rust API (`frb_example/**/rust/src/api/*.rs`) | `./frb_internal precommit-generate` |
| Flutter integrate examples | `./frb_internal precommit-integrate` |
| `frb_codegen/` generation logic | `./frb_internal precommit-generate` + `generate-internal-rust` |
| `frb_rust/src/` core API | `./frb_internal generate-internal-rust` |
| `frb_example/pure_dart` generator | `./frb_internal generate-internal-frb-example-pure-dart` |
| CLI help documentation | `./frb_internal generate-internal-book-help` |
| Non-generated (docs, comments, tests, `frb_dart/`) | No generation needed |
