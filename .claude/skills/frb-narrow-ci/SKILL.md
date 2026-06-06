---
name: frb-narrow-ci
description: Use when temporarily narrowing flutter_rust_bridge GitHub Actions CI so only the relevant test, command, job path, or job family runs, especially for intentional red reproduction PRs or expensive CI iteration.
---

# FRB Narrow CI

Use this skill when CI feedback is too expensive or slow and the current task only needs a focused CI signal.

Temporary CI narrowing is allowed only when it is obvious, documented, and later reverted unless the PR is explicitly about CI behavior.

## Intentional Red Reproduction PRs

For intentional red CI reproduction PRs, narrowing is mandatory.

- Narrow CI to only the single relevant test, command, or job path that proves the reported bad behavior.
- Do not run the full CI matrix, broad job families, or unrelated tests.
- If the current workflow cannot already select that single target, temporarily modify the reproduction PR's CI configuration so only that reproducer runs.
- Keep fix code unchanged; the reproduction PR exists only to demonstrate the failure.
- The reproduction PR body must state:
  - the exact narrowed test, command, or job path expected to run
  - the expected failure text or behavior
  - which expensive or unrelated CI paths were intentionally disabled
- Do not treat this narrowed red run as fix readiness evidence. It is only the baseline proof that the bad behavior is observable.

## Iteration Branches

For ordinary CI debugging or iteration branches, narrowing is optional.

- Narrow only to the smallest job family needed for the current investigation.
- Keep the narrowing obvious and temporary; prefer a standalone commit that can be reverted cleanly.
- Say in the PR or follow-up status that CI is intentionally narrowed while iterating.
- Do not treat a narrowed CI run as final readiness evidence.
- Before final review or merge readiness, revert temporary workflow narrowing and run the normal CI surface again.

## Common Patterns

- Add an always-false `if` guard to jobs that are unrelated to the current investigation.
- Reduce matrix entries so only the target platform, package, or command remains.
- Restrict workflow triggers or job dependencies only when that is the clearest way to isolate the target.

Keep the temporary CI diff easy to review and easy to remove.
