---
name: frb-issue-to-green-pr
description: "Use when implementing a GitHub issue, bug fix, or feature in flutter_rust_bridge end-to-end: develop the change, add regression coverage, prepare and open a PR, monitor CI until green, request and resolve Gemini review, and keep following up on a 5 minute cadence until the PR is ready."
---

# FRB Issue to Green PR

Use this skill as the top-level workflow when the user asks you to implement an issue, fix a bug, add a feature, or otherwise "handle it end to end" in `flutter_rust_bridge`, especially when they ask for an automatic PR, CI monitoring, or Gemini review.

This skill orchestrates the narrower FRB skills. Do not duplicate their detailed instructions in memory; read them when their phase begins.

## Required Subskills

Always read these first:

1. The applicable user-level FRB workflow or environment skill, if one exists. For example, Tom's environment may provide `tom-frb-env`; other users may have different local rules.
2. `frb-develop-feature` for the reproduce -> fix -> final regression workflow.

Read these when entering the matching phase:

- `frb-code-generation` before running generation or changing Rust APIs, codegen, or example APIs.
- `frb-test` before running local tests.
- `frb-lint` before lint or format checks.
- `frb-prepare-pr` before pushing or opening the PR.
- `frb-fix-ci` before diagnosing any CI failure.
- `gh-actions-live-logs` before reading GitHub Actions logs.
- `frb-debugging` when generated code is surprising or codegen behavior is unclear.

## Workflow

1. Understand the GitHub issue or requested change.
   - Fetch the issue body and comments if the user gave an issue link or number.
   - Identify the smallest observable failing behavior or missing capability.
   - Check `git status --short` and do not disturb unrelated user or multi-agent changes.

2. Develop with a fast feedback loop.
   - Follow `frb-develop-feature`: reproduce or prototype in `frb_example/dart_minimal` first when applicable.
   - Add a regression test for bugs.
   - Move the final coverage to `frb_example/pure_dart` with the required `TwinNormal` naming when applicable.
   - Keep generated-file edits produced by the appropriate generator, not by hand.

3. Verify locally with scope proportional to risk.
   - Run the relevant generation, focused tests, and lint commands from the subskills.
   - Prefer focused commands while iterating; broaden only when the change's blast radius requires it.
   - Record any skipped expensive tests or local environment blockers explicitly.

4. Commit promptly.
   - After each completed logical unit, stage only files you intentionally changed and create a small commit.
   - Use English commit messages.
   - Do not amend unless the user explicitly asks.

5. Prepare and open the PR.
   - Follow `frb-prepare-pr`.
   - Push with upstream tracking.
   - Before drafting a PR title, inspect the user's recent PR titles and mimic the repo style.
   - Create the PR with a concise body covering issue, fix, tests, and known limitations.

6. Monitor CI until terminal.
   - After the PR is opened or updated, do not leave the PR in an unknown queued or in-progress state.
   - If the user asked for 5 minute follow-up, create a thread heartbeat automation with a 5 minute cadence to re-check PR checks.
   - On each wake-up, inspect the latest PR checks, not stale runs.
   - If CI fails, read `frb-fix-ci` and `gh-actions-live-logs`, diagnose the latest relevant failure, fix it, commit, push, and continue monitoring.
   - If CI appears flaky, rerun only failed jobs when appropriate, then keep monitoring.

7. Handle Gemini review.
   - After pushing the PR, wait for Gemini's GitHub review or comments if the repository automation posts them.
   - Treat actionable Gemini feedback like review comments: inspect, fix if valid, commit, push, and reply or otherwise make the resolution visible.
   - If feedback is incorrect or not actionable, leave a concise PR comment explaining why.
   - Once you believe the code is ready and CI checks are passing, post a PR comment containing exactly `/gemini review` to request another Gemini pass.
   - Wait for the new Gemini response on GitHub, then resolve any actionable follow-up feedback.

8. Stop only when ready.
   - The PR checks are green or all remaining non-green checks are clearly unrelated and explained.
   - Gemini has no unresolved actionable feedback after the final `/gemini review` pass.
   - The branch is pushed, commits are present on the PR, and the final status is reported to the user with the PR URL.

## Automation Rules

- Prefer a heartbeat attached to the current thread for short follow-up intervals such as every 5 minutes.
- The heartbeat prompt must be self-contained: include the PR URL/number, branch, repository, and the requirement to inspect CI and Gemini review state, fix issues, commit, push, and continue until ready.
- Do not create a duplicate heartbeat if one already exists for the same PR; update the existing automation instead.
- Cancel or leave inactive any PR-specific heartbeat once the stop conditions are met.

## Failure Handling

- If blocked by permissions, missing credentials, a required external reviewer delay, or unavailable infrastructure, explain the concrete blocker and keep the PR state explicit.
- If a CI run is queued for a long time, keep the heartbeat active rather than stopping.
- If new user instructions arrive, let the newest instruction steer the workflow while preserving already-completed work.
