---
name: frb-prepare-pr
description: Use when about to create a PR or push changes in flutter_rust_bridge to ensure code generation is up to date
---

# FRB Prepare for PR

> **Note:** Check your user-level `remote-testing` rules before running commands. Codegen, lint, and tests may require remote execution.

## Overview

Before creating a PR, ensure generated code is up to date and lint passes.

**Core principle:** Generate → Lint → Commit → PR.

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
2. [ ] **REQUIRED:** Read `frb-lint` skill, run `./frb_internal lint --fix`
3. [ ] (Optional) Read `frb-test` skill, run relevant tests
4. [ ] Commit all changes
5. [ ] Push and create PR

## What CI Will Do

CI automatically runs:
- Code generation check (fails if generated code is outdated)
- All tests (Rust, Dart, Flutter, Web)
- Lint and format checks

Run lint locally to avoid CI failures. Tests are optional locally.

## Related Skills

- `frb-code-generation` - Determines which generation commands to run
- `frb-lint` - Lint and format checks
- `frb-test` - For local debugging when CI fails
- `creating-pull-requests` - Standard PR creation process
