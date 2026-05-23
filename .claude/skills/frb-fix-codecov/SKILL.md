---
name: frb-fix-codecov
description: Use when a flutter_rust_bridge PR has Codecov patch or project coverage drops, Codecov comments mention missing lines, or you need to decide whether to add tests or justified coverage-ignore markers.
---

# FRB Fix Codecov

## Overview

Triage Codecov as a coverage signal, not as a generic CI failure. First identify the exact missing patch
lines from the latest Codecov report, then decide whether those lines deserve real coverage or a justified
ignore marker.

## Triage

Start from the latest PR head, not an old Codecov comment. Codecov comments may be edited and checks may
arrive after the main workflow finishes.

Useful commands:

```bash
gh pr view <number> --repo fzyzcjy/flutter_rust_bridge \
  --json headRefOid,comments,statusCheckRollup,url

gh api repos/fzyzcjy/flutter_rust_bridge/commits/<head-sha>/check-runs --paginate \
  | jq '.check_runs[]
      | select(.name | startswith("codecov/"))
      | {name, conclusion, title: .output.title, summary: .output.summary, text: .output.text, details_url}'
```

Read both Codecov statuses:

- `codecov/patch`: coverage on changed lines only. This is the usual PR blocker.
- `codecov/project`: whole-project coverage delta. A green project status does not make a red patch status
  irrelevant.

The GitHub comment usually lists only files and counts, for example `2 Missing`, not precise line numbers.
For exact lines, open the Codecov `details_url`, use the file links in the comment, and inspect the red
uncovered lines in the file diff/tree view. If the web view is unavailable, fetch the coverage artifacts from
the `Misc :: Codecov` run and inspect the uploaded `codecov.json` / `lcov.info` against the PR diff.

In FRB CI, coverage is uploaded only by `Misc :: Codecov` after downloading all `*-coverage` artifacts. If
upstream coverage jobs were cancelled or missing, treat the report as partial until you verify the artifact
download in that job. Use the `gh-actions-live-logs` skill when reading GitHub Actions logs.

## Understand FRB Coverage Inputs

Rust coverage is transformed before upload by
`tools/frb_internal/lib/src/utils/codecov_transformer.dart`.

Important effects:

- `frb-coverage:ignore-start` / `frb-coverage:ignore-end` remove Rust lines from the custom Codecov JSON.
- Some syntax-only Rust lines are filtered by transformer patterns.
- Dart-side coverage uses normal Dart/lcov markers such as `// coverage:ignore-start` and
  `// coverage:ignore-end`.
- `codecov.yml` ignores examples, tests, generated Dart files, and `frb_example/`; do not expect those paths
  to repair patch coverage for `frb_codegen`, `frb_rust`, or `frb_dart` changes.

## Decide the Fix

Prefer real coverage when the uncovered line represents behavior that should keep working.

Good candidates for tests:

- Pure functions, parsers, config decisions, path computations, generated text decisions, and error handling
  that can be exercised in Rust or Dart unit tests.
- Regression-prone behavior that caused the PR change.
- Logic where a small focused test can call the function directly after making the function test-visible or
  private-in-module accessible from an existing `#[cfg(test)]` module.

Use the relevant testing skills after choosing the test:

- `frb-develop-feature` for adding feature or bug regression coverage.
- `frb-test` for selecting local test commands.
- `tom-frb-env` before running FRB tests or setup commands in Tom's environment.

Ignore coverage only when the line is genuinely hard or misleading to execute and the source already makes
that reason clear. Acceptable examples seen in this repo include:

- Build-time or macro-time code that is certainly executed but invisible to llvm-cov.
- OS-specific fallback or missing-tool branches that are impractical or flaky to force in normal CI.
- Warning-only or defensive branches where testing would mostly assert implementation trivia.

When adding Rust ignores, use:

```rust
// Short reason why coverage cannot see this or why the branch is not worth executing.
// frb-coverage:ignore-start
...
// frb-coverage:ignore-end
```

When adding Dart ignores, use the native Dart coverage markers:

```dart
// coverage:ignore-start
...
// coverage:ignore-end
```

Do not add ignore markers merely because adding a test is inconvenient. If the line is product behavior and
no focused test path is obvious, summarize the missing lines and ask the human which tradeoff they want.

## PR #3065 Pattern

In `fzyzcjy/flutter_rust_bridge#3065`, Codecov initially reported patch coverage below 100% with four
missing changed lines across:

- `frb_codegen/src/library/codegen/polisher/auto_upgrade.rs`
- `frb_codegen/src/library/integration/integrator.rs`

The eventual fix was mixed:

- Add focused Rust unit tests for newly testable logic.
- Add `frb-coverage:ignore-start/end` around a branch that tests exercised but llvm-cov still reported as
  uncovered.

Use that as the default mental model: first try to cover meaningful behavior, then narrowly ignore only the
coverage-tool artifact.

## Verification

After changing code or markers:

1. Re-read the touched files before editing and stage only your files.
2. Commit the logical fix before running long tests, following Tom's commit policy.
3. Run the focused local test command that covers the changed package when practical.
4. Push and check the latest `codecov/patch` and `codecov/project` statuses on the new head.
5. If Codecov is still red, repeat from the exact latest missing-line report rather than reusing stale line
   numbers.
