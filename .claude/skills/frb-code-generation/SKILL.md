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

For CI diagnosis rules about generated-file format/lint failures or `Generate :: FRB Codegen :: Command Integrate` failures, see `frb-fix-ci` first. Keep generated-output fixes rooted in generation logic, templates, or workflow rather than hand-editing generated files.

For `pure_dart` / `pure_dart_pde` generation issues, treat `frb_example/pure_dart` as the upstream source and `frb_example/pure_dart_pde` as the derived copy. See `frb-fix-ci` for the CI diagnosis workflow.

If the same package or path keeps getting `refresh` / `regenerate` / `sync` style commits, or if accepting regenerated outputs causes previously green non-`Generate` jobs to fail, stop treating this as a pure codegen-command selection problem. Switch to `frb-fix-ci` and diagnose failure propagation before regenerating again.

If the same `Generate :: FRB Codegen :: Command Generate` symptom starts rotating across different example packages with very similar generated Dart diffs, do not keep selecting per-package generate commands one by one. Switch to `frb-fix-ci` and validate clean remote `./frb_internal precommit-generate` as the source of truth for the whole `Generate` chain.

Practical cutoff:
after two similar package-level generated-output syncs, stop and re-check clean remote `./frb_internal precommit-generate` before accepting a third one.

When CI repair has already entered repeated package-level `Generate` drift, this is no longer a command-minimization problem. Stop choosing narrower package commands and switch to `frb-fix-ci` escalation via clean remote `./frb_internal precommit-generate`.

If Flutter integrate examples, example platform files, and downstream Flutter build/test jobs regress together, assume the issue may be in `frb_codegen/assets/integration_template/` or `cargokit` rather than in the generated outputs themselves. If the real fix belongs in the embedded `cargokit` submodule, edit it directly, push to `fzyzcjy/cargokit`, and then update the submodule ref. Use `frb-fix-ci` for that workflow.

Do not manually patch generated files as the final fix. The final accepted result should be produced by the corresponding generation command in a clean matching environment.
