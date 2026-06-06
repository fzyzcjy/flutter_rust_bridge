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
- `frb-pr-review` before treating a non-trivial PR as ready.
- `frb-fix-ci` before diagnosing any CI failure.
- `frb-manual-test` before writing a manual regression test report under `tools/manual_tests/`.
- `gh-actions-live-logs` before reading GitHub Actions logs.
- `frb-debugging` when generated code is surprising or codegen behavior is unclear.

## Workflow

1. Start a 5 minute completion loop.
   - Create or update a thread heartbeat automation at a 5 minute cadence before doing substantial work.
   - The heartbeat must say to continue this skill until all stop conditions are met; update it with the PR URL, branch, and repository once they are known.

2. Understand the GitHub issue or requested change.
   - Fetch the issue body and comments if the user gave an issue link or number.
   - Identify the smallest observable failing behavior or missing capability.
   - Check `git status --short` and do not disturb unrelated user or multi-agent changes.

3. Develop with the project feature or bug-fix workflow.
   - Read and follow `frb-develop-feature`; treat it as the source of truth for reproduction, iteration, local verification, regression coverage, and final example placement.
   - For bug fixes, before changing fix code or opening the fix PR, create an independent evidence PR that proves the bad behavior.
   - If CI can reproduce the bad behavior, the evidence PR must be an intentional red CI reproduction PR: unchanged fix code, minimal reproducer or workflow adjustment, forced CI narrowing to only the relevant job family, and a failure whose error matches the user's report. Mark the branch name, PR title, and PR body clearly as an intentional red reproduction, not a real fix PR.
   - If CI cannot realistically reproduce the bad behavior, read `frb-manual-test` and instead create an independent PR that adds or updates `tools/manual_tests/NAME.md` with a normal manual test procedure and mechanical execution steps an agent or human can run.
   - Do not proceed to the fix PR until either the intentional red CI reproduction PR exists with a matching failed run, or the manual-test PR exists with precise manual test steps.
   - Save the evidence PR URL, red CI run URL when applicable, job name or manual-test path, and matching error text in the fix PR reproduction report.
   - Before considering the change ready, explicitly pass the `frb-develop-feature` Final Placement Gate: final regression coverage belongs in `frb_example/pure_dart` with generated `pure_dart_pde` coverage, not only in `frb_example/dart_minimal`.
   - Keep generated-file edits produced by the appropriate generator, not by hand.

4. Commit each completed logical unit immediately.
   - Make atomic commits as soon as a minimal unit is written; do not wait until the end of the task.
   - Stage only files intentionally changed for this task.
   - Always create a new commit unless the user explicitly asks to amend.

5. Prepare and open the PR.
   - Follow `frb-prepare-pr`.
   - Re-check that no final regression or feature coverage remains only in `frb_example/dart_minimal`. If it does, stop PR preparation and migrate it to `frb_example/pure_dart` first.
   - Push with upstream tracking.
   - Before drafting a PR title, inspect the user's recent PR titles and mimic the repo style.
   - Create the PR according to the active PR workflow and repository/user PR body rules.
   - If the work comes from a GitHub issue, ensure the PR body includes the appropriate closing keyword such as `Close #1234`, unless the active PR workflow explicitly requires an empty body.

6. Handle Gemini review.
   - Follow `frb-pr-review` for the full PR review gate, including correctness review, test-weakening review, and Gemini.
   - After pushing the PR and once you believe the code is reasonably ready, post a PR comment containing exactly `/gemini review` to request a Gemini pass; do not wait for CI to be green before requesting this first self-initiated review.
   - Wait for Gemini's GitHub review or comments if the repository automation posts them.
   - Treat actionable Gemini feedback like review comments: inspect, fix if valid, commit, push, and reply or otherwise make the resolution visible.
   - If feedback is incorrect or not actionable, leave a concise PR comment explaining why.
   - After substantial follow-up fixes, request another Gemini pass when you again believe the code is reasonably ready.
   - Wait for each new Gemini response on GitHub, then resolve any actionable follow-up feedback.

7. Monitor CI until terminal.
   - After the PR is opened or updated, do not leave the PR in an unknown queued or in-progress state.
   - On each wake-up, inspect the latest PR checks, not stale runs.
   - For bug-fix PRs with an intentional red CI reproduction PR, explicitly find the same job family or CI path that failed in the reproduction branch and verify that it is now green on the fix PR.
   - For bug-fix PRs with a manual-test report PR, state whether the manual regression was re-run, who or what ran it, and whether the observed behavior now matches the fixed expectation.
   - If CI fails, read `frb-fix-ci` and `gh-actions-live-logs`, diagnose the latest relevant failure, fix it, commit, push, and continue monitoring.
   - If CI appears flaky, rerun only failed jobs when appropriate, then keep monitoring.

8. Stop only when ready.
   - The PR checks are green or all remaining non-green checks are clearly unrelated and explained.
   - Gemini has no unresolved actionable feedback after the latest self-initiated `/gemini review` pass.
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
