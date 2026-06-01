---
name: frb-fix-main-ci
description: Use when flutter_rust_bridge CI fails on the default branch/main/master, especially to distinguish ordinary CI repair from a regression introduced after a previously green default-branch commit.
---

# FRB Fix Main CI

Use this when CI is failing on the default branch (`main` / `master`) or on a commit that is already part of the default-branch history.

## First Step

Read `frb-fix-ci` first and apply its ordinary CI triage:

- Check the latest relevant run, not stale status.
- Use `gh-actions-live-logs` when reading GitHub Actions job logs.
- Classify flakes, generated diffs, lint drift, dependency-order failures, and propagated downstream symptoms before deep debugging.
- Prefer fixing prerequisite jobs such as `Generate`, `Integrate`, or high-relevance `Generate Internal` before chasing later build/test symptoms.

## Main-Branch Difference

A default-branch failure is usually a regression until proven otherwise. There should usually be an older default-branch commit that passed the same CI path.

After the quick `frb-fix-ci` checks, answer these questions before writing a fix:

1. What is the failing default-branch commit and workflow run?
2. What is the nearest older default-branch commit with a green run for the same workflow or job family?
3. Which commits or merged PRs sit between the green baseline and the red commit?
4. Does the failure reproduce at the red commit but not at the green baseline, using the same `./frb_internal ...` command and the FRB dev environment rules?
5. Which source change most directly explains the regression?

## Regression Workflow

1. Capture the exact failing jobs and the first useful error from CI logs.
2. Find the newest passing default-branch run for the same workflow/job family.
3. Compare the commit range between green and red:
   ```bash
   git log --oneline --decorate <green-sha>..<red-sha>
   git diff --stat <green-sha>..<red-sha>
   ```
4. If the range is broad or the culprit is unclear, bisect with the smallest meaningful reproduction command.
5. Fix the source of the regression, not just the generated or downstream symptom.
6. Add or adjust regression coverage when the bug is behavioral. For pure CI/tooling drift, document the before/after evidence in the PR.

## What Not To Do

- Do not treat default-branch CI as "just rerun until green" unless logs strongly indicate infra flake.
- Do not only repair checked-in generated files if the generator, template, version pin, or environment change is the real source.
- Do not chase downstream job failures while an upstream generation or integration job is still red.
- Do not assume the newest red commit is the sole cause without checking the nearest green baseline and intervening changes.

## PR Expectations

For a PR that fixes default-branch CI, include:

- Failing commit SHA and workflow/job URL.
- Nearest green baseline SHA or run, if available.
- The reproduction command used locally or in Docker/Tart.
- The suspected or confirmed culprit commit/PR.
- Why the fix addresses the source of the regression.

Use `frb-prepare-pr` before opening the PR.
