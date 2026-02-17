---
name: frb-prepare-for-pr
description: Use when about to create a PR or push changes in flutter_rust_bridge to ensure code generation is up to date
---

# FRB Prepare for PR

## Overview

Before creating a PR, ensure generated code is up to date. This skill checks what generation is needed based on your changes.

**Core principle:** Run code generation if needed. Tests run on CI, so no need to run locally.

## Workflow

```
1. Check: Did you modify generated code or generation-related files?
   |
   +-- No --> Ready for PR
   |
   +-- Yes --> 2. Run frb-code-generation skill
               |
               +-- 3. Commit generated changes
               |
               +-- 4. Ready for PR
```

## When Generation is Needed

Check your changes against the `frb-code-generation` skill:

| If you modified... | Run this command |
|-------------------|------------------|
| Example Rust API | `./frb_internal precommit-generate` |
| `frb_codegen/` | `./frb_internal precommit-generate` + `generate-internal-rust` |
| `frb_rust/src/` | `./frb_internal generate-internal-rust` |
| Flutter integrate examples | `./frb_internal precommit-integrate` |

## When Generation is NOT Needed

- Documentation only (`.md`, comments)
- Test files only
- CI/CD configuration
- Non-API code

## Quick Checklist

Before PR:

1. [ ] Did you modify Rust API or codegen? → Run `frb-code-generation` skill
2. [ ] Commit any generated file changes
3. [ ] Push and create PR

## What CI Will Do

CI automatically runs:
- Code generation check (fails if generated code is outdated)
- All tests (Rust, Dart, Flutter, Web)
- Lint and format checks

You don't need to run these locally unless debugging.

## Related Skills

- `frb-code-generation` - Determines which generation commands to run
- `frb-test` - For local debugging when CI fails
