---
name: frb-fix-codecov
description: Use when a flutter_rust_bridge PR has Codecov patch or project coverage drops, Codecov comments mention missing lines, or you need to decide whether to add tests or justified coverage-ignore markers.
---

# FRB Fix Codecov

## Overview

Use this skill to turn a vague Codecov PR comment into concrete file/line facts, then decide whether the
right fix is real coverage or a narrow, justified ignore marker.

## Workflow

### Step 1: Collect the Codecov Facts

Start from the latest PR head. Codecov comments can be edited and old comments/checks can describe stale
commits.

Fetch the PR metadata, Codecov comments/checks, and the full Codecov report into a local temporary directory
with the bundled analyzer:

```bash
python3 .claude/skills/frb-fix-codecov/codecov_analyzer.py analyze --pr <number>
```

The analyzer writes `pr.json`, `check-runs.json`, `codecov-report.json`, `codecov-comment.md`,
`codecov-checks.json`, `codecov-files-summary.json`, and `missing-patch-lines.txt` under
`/private/tmp/frb-codecov-<pr>-<timestamp>/` by default. Use `--output-dir <dir>` to choose a stable path for
debugging or examples.

The full Codecov `report?sha=` endpoint is preferred because it returns all files in one response. Codecov also has
`file_report/<urlencoded-path>?sha=<sha>` for one file; use it only when the full report is too large or you
need to re-check a path precisely.

Read both Codecov statuses:

- `codecov/patch`: coverage on changed lines only. This is the usual PR blocker.
- `codecov/project`: whole-project coverage delta. A green project status does not make a red patch status
  irrelevant.

The GitHub comment usually lists files and missing counts, not precise line numbers. Compute exact missing
patch lines by intersecting Codecov's missing lines with Git's added/changed lines. In Codecov's
`line_coverage`, FRB's uploaded custom Rust coverage JSON uses:

- `null`: not coverable or ignored
- `0`: covered
- positive number: missing

The analyzer computes exact missing patch lines by intersecting the saved Codecov report with
`git diff --unified=0 <base_sha> <head_sha>`. Read `missing-patch-lines.txt` first, then use the raw JSON files
when the summary does not explain the status.

If the computed output disagrees with Codecov:

- Check whether the Codecov comment/check describes a different `HEAD_SHA`.
- Check whether Git has both SHAs locally; fetch the PR head/base if needed.
- For a truncated file path in the comment, get the full path from the Codecov URL's `filepath=` query.
- Verify `Misc :: Codecov` downloaded all expected `*-coverage` artifacts before upload; use the
  `gh-actions-live-logs` skill for GitHub Actions logs.

### Step 2: Choose the Fix

#### Option A: Exclude Non-Meaningful Coverage

In this repo the Rust-side marker is `frb-coverage:ignore-start` /
`frb-coverage:ignore-end` (not `frb-codecov-ignore`). It is implemented by
`tools/frb_internal/lib/src/utils/codecov_transformer.dart`, which removes those Rust lines before upload.

Dart-side code uses normal Dart coverage markers:

```dart
// coverage:ignore-start
...
// coverage:ignore-end
```

Only use ignore markers when the code genuinely should not be judged by Codecov. Existing FRB examples
usually have an explanatory comment immediately above the marker. Acceptable reasons include:

- The code is definitely executed, but the coverage tool cannot see it, especially build-time or macro-time
  execution.
- llvm-cov reports a branch as uncovered even though a focused test exercises it.
- The branch is warning-only, defensive, tool-missing, OS-specific, or otherwise not worth forcing in normal
  CI.
- The line is generated glue, FFI shape glue, or syntactic boilerplate where testing it would assert
  implementation trivia rather than behavior.

When adding Rust ignores, keep the ignored range as small as possible and write the reason first:

```rust
// This is executed at build time, but llvm-cov does not observe that path.
// frb-coverage:ignore-start
...
// frb-coverage:ignore-end
```

Do not use ignore markers merely because adding a test is inconvenient. If the reason cannot be explained in
one short public comment, treat the line as coverage-relevant.

#### Option B: Add Meaningful Coverage

Investigate why each exact missing line is not covered.

Prefer real coverage when the uncovered line represents product behavior that should keep working. Good
candidates for tests:

- Pure functions, parsers, config decisions, path computations, generated text decisions, and error handling.
- Integration/codegen behavior where the missing line affects generated files or public workflow behavior.
- Regression-prone behavior introduced or modified by the PR.

Choose the smallest meaningful test first:

- Unit test the function directly when the behavior is local and stable.
- Add a codegen or integration test when only the end-to-end generated output proves the behavior.
- Add an E2E test when the missing line is only meaningful through a real FRB workflow.

Use the relevant skills after choosing the test path:

- `frb-develop-feature` for feature/bug regression coverage.
- `frb-test` for selecting local test commands.
- `tom-frb-env` before running FRB tests or setup commands in Tom's environment.

If the behavior matters but a focused test would be brittle, very expensive, or require a product decision,
summarize the exact missing lines, the suspected reason they are uncovered, and the realistic options, then
ask the human which tradeoff to take.

## Verification

After changing tests or ignore markers:

1. Re-read the touched files immediately before editing.
2. Commit the logical fix before long verification, following Tom's commit policy.
3. Run the focused local test command when practical.
4. Push and check the latest `codecov/patch` and `codecov/project` statuses on the new head.
5. If Codecov is still red, repeat the workflow from the latest exact report instead of reusing stale line
   numbers.
