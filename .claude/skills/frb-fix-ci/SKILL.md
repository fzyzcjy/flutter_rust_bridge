---
name: frb-fix-ci
description: Use when CI fails in flutter_rust_bridge with test failures, git diff errors, or flaky tests - before deep investigation
---

# FRB Fix CI

## Overview

CI failures in flutter_rust_bridge often have simple fixes. Try these approaches in order before deep investigation.

**Core principle:** Start with lazy fixes (re-run, copy diff, --fix) before expensive investigation.

## Quick Reference

| Symptom | First Try |
|---------|-----------|
| Flaky test (passes sometimes) | Re-run CI (push empty commit) |
| Git diff shown in CI | `git apply` the diff (5 sec) OR regenerate (slower) |
| `./frb_internal` command failing | Add `--fix` flag |
| Can't reproduce locally | Use same `./frb_internal` command from CI |

## Strategies (in order of effort)

### 1. Re-run for Flaky Tests

Sometimes CI fails due to timing issues, not real bugs.

```bash
git commit --allow-empty -m "chore: trigger CI"
git push
```

If it passes on retry → flaky, not your bug.

### 2. Fix Git Diff Errors

When CI shows a diff, you have two options:

**Option A: `git apply` (faster - 5 seconds)**

CI already ran the generator. Just apply what it computed:

```bash
# Copy the diff from CI, then:
pbpaste | git apply   # macOS
```

**Option B: Regenerate (slower but more "proper")**

Run the generator locally:
```bash
./frb_internal precommit-generate
```

Both are correct. Option A is faster; Option B is more thorough.

### 3. Use --fix Flag

Most `./frb_internal` commands support `--fix`:

```bash
./frb_internal precommit --fix
./frb_internal test-dart --fix
```

### 4. Reproduce Locally

CI shows the command it ran. Run the same command:

```bash
# CI shows: ./frb_internal test-dart --package frb_example/pure_dart
./frb_internal test-dart --package frb_example/pure_dart
```

## Common Mistakes

- Investigating root cause when a simple re-run would work
- Not using `--fix` flag when available
- Not trying `git apply` first when CI provides a diff

## Related Skills

- `frb-code-generation` - Which generation commands to run
- `frb-debugging` - Deep debugging when simple fixes don't work
