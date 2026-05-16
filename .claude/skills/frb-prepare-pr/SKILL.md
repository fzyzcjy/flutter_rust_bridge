---
name: frb-prepare-pr
description: Use when about to create a PR or push changes in flutter_rust_bridge to ensure code generation is up to date
---

# FRB Prepare for PR

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen, lint, and tests may require remote execution.

## Overview

Before creating a PR, ensure generated code is up to date and lint passes.

**Core principle:** Generate → Lint → Commit → PR.

> **After codegen:** Check your user-level `remote-testing` rules. If codegen was run remotely, pull changes back to local.

## Workflow

```
1. Read frb-code-generation skill
   |
   +-- Run required generation commands
   |
   +-- 2. Read frb-lint skill --> Run lint --fix
   |
   +-- 3. (Optional) Read frb-test skill --> Run relevant tests
   |
   +-- 4. Commit all changes
   |
   +-- 5. Create PR (use creating-pull-requests skill)
```

## Quick Checklist

1. [ ] **REQUIRED:** Read `frb-code-generation` skill, run commands if needed
2. [ ] **REQUIRED for integrate output diffs:** If the PR changes Flutter integrate example outputs, platform scaffolds, or copied `cargokit` files under `frb_example/**`, confirm whether `frb_codegen/assets/integration_template/` is the source that should change, then run `./frb_internal precommit-integrate`
3. [ ] **REQUIRED:** Read `frb-lint` skill, run `./frb_internal lint --fix`
4. [ ] (Optional) Read `frb-test` skill, run relevant tests
5. [ ] Commit all changes
6. [ ] Push and create PR

## What CI Will Do

CI automatically runs:
- Code generation check (fails if generated code is outdated)
- All tests (Rust, Dart, Flutter, Web)
- Lint and format checks

Run lint locally to avoid CI failures. Tests are optional locally.

If your PR fixes Flutter integrate example outputs and the real bug is inside the embedded `cargokit` submodule, do not stop at copied example files. Push the `cargokit` fix to `fzyzcjy/cargokit` and update the submodule ref in this repo before pushing the PR branch.

If the PR changes integrate-generated example output but not `frb_codegen/assets/integration_template/`, explicitly verify that the output-only change is intentional. Most integrate scaffold behavior should be fixed in the template and regenerated, not patched only in `frb_example/**`.

## Related Skills

- `frb-code-generation` - Determines which generation commands to run
- `frb-lint` - Lint and format checks
- `frb-test` - For local debugging when CI fails
- `creating-pull-requests` - Standard PR creation process
