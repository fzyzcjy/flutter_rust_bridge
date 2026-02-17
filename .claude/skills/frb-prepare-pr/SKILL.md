---
name: frb-prepare-pr
description: Use when about to create a PR or push changes in flutter_rust_bridge to ensure code generation is up to date
---

# FRB Prepare for PR

## Overview

Before creating a PR, ensure generated code is up to date. This skill checks what generation is needed based on your changes.

**Core principle:** Run code generation if needed. Tests run on CI, so no need to run locally.

## Workflow

**REQUIRED: Read `frb-code-generation` skill first.** It determines what needs to be generated.

```
1. Read frb-code-generation skill
   |
   +-- No generation needed --> Ready for PR
   |
   +-- Generation needed --> 2. Run commands from frb-code-generation
                              |
                              +-- 3. Commit generated changes
                              |
                              +-- 4. Ready for PR
```

## Quick Checklist

1. [ ] **REQUIRED:** Read `frb-code-generation` skill
2. [ ] Run any required generation commands
3. [ ] Commit generated changes
4. [ ] Push and create PR

## What CI Will Do

CI automatically runs:
- Code generation check (fails if generated code is outdated)
- All tests (Rust, Dart, Flutter, Web)
- Lint and format checks

You don't need to run these locally unless debugging.

## Related Skills

- `frb-code-generation` - Determines which generation commands to run
- `frb-test` - For local debugging when CI fails
