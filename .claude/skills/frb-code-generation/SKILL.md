---
name: frb-code-generation
description: Use when modifying Rust API, codegen, or example code in flutter_rust_bridge to determine which generation commands to run
---

# FRB Code Generation

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen may require remote execution.

## Overview

flutter_rust_bridge requires code generation when Rust APIs change. This skill maps change types to the minimal generation commands needed.

**Core principle:** Run only the generation commands needed for your change type.

**Validation rule:** If regenerated outputs cause previously green non-`Generate` jobs to fail, treat the generated outputs as suspect until they are validated from a clean environment.

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

## Important Rules

For CI diagnosis rules about generated-file format/lint failures, repeated package-level `Generate` drift, or `Generate :: FRB Codegen :: Command Integrate` failures, see `frb-fix-ci` first. This skill is for command selection, not CI failure-propagation diagnosis.

For `pure_dart` / `pure_dart_pde` generation issues, treat `frb_example/pure_dart` as the upstream source and `frb_example/pure_dart_pde` as the derived copy. See `frb-fix-ci` for the CI diagnosis workflow.

If CI repair has already entered repeated package-level drift, stop choosing narrower commands and switch to `frb-fix-ci`.

Do not manually patch generated files as the final fix. The final accepted result should be produced by the corresponding generation command in a clean matching environment.
