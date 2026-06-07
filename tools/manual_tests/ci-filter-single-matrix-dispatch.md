# CI Filter Single Matrix Dispatch

## Purpose

Verify that a pull request with the `ci-manual-dispatch` label avoids the automatic full CI surface, while a `workflow_dispatch` run with `ci_filter` can run one exact matrix entry.

This test specifically exercises one Linux Dart native matrix item:

```text
test_dart_native[image=ubuntu-24.04,package=frb_example--dart_minimal]
```

## Source

- Context: Manual coverage for the focused CI filtering workflow added to `.github/workflows/ci.yaml`.
- Related docs or skills: `.claude/skills/frb-ci-filter/SKILL.md`, `.github/workflows/ci.yaml`, `tools/frb_internal/lib/src/makefile_dart/ci_plan/`.

## When To Run

Run this after changing CI planning, `ci_filter` parsing, workflow dispatch wiring, manual dispatch label behavior, or matrix definitions in `.github/workflows/ci.yaml`.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`.
- Required branch state: clean checkout based on the latest default branch.
- Required credentials or account state: GitHub CLI authenticated with permission to push a branch, create a PR, edit PR labels, dispatch workflows, and read workflow runs.
- Required device or simulator state: none.

## Environment

- OS: macOS, Linux, or Windows with Git and GitHub CLI.
- Flutter: not required locally.
- Dart: not required locally if Docker-based `./frb_internal` execution is available; record the local or container Dart version if used for smoke checks.
- Rust: not required locally.
- Device or simulator: none.
- Browser or external service: GitHub Actions.

## Preparation

Run from the repository root:

```bash
git fetch origin master
git switch -c codex/ci-filter-dummy-YYYYMMDD-HHMMSS origin/master
git status --short
git submodule update --init --recursive
```

If the local environment does not have the FRB toolchain, run the smoke check through the repository Docker helper:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- \
  bash -lc "./frb_internal plan-ci --filter 'test_dart_native[image=ubuntu-24.04,package=frb_example--dart_minimal]'"
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: a temporary dummy branch containing an empty commit or other low-risk no-op change.
- Reset procedure before each run: create a fresh branch from the latest `origin/master` and use a new PR, or close/delete any prior dummy PR before reusing the branch name.

## Steps

1. Create and push a dummy branch from the latest default branch.

   ```bash
   git fetch origin master
   git switch -c codex/ci-filter-dummy-YYYYMMDD-HHMMSS origin/master
   git commit --allow-empty -m "Create dummy PR for CI filter manual test"
   git status --short
   git push -u origin codex/ci-filter-dummy-YYYYMMDD-HHMMSS
   ```

2. Create a temporary dummy PR for the dummy branch and apply the `ci-manual-dispatch` label before treating CI status as meaningful.

   ```bash
   gh pr create \
     --repo fzyzcjy/flutter_rust_bridge \
     --base master \
     --head codex/ci-filter-dummy-YYYYMMDD-HHMMSS \
     --title "Manual CI filter dummy PR" \
     --body "Temporary dummy PR for executing the CI filter manual test. This PR should be closed after evidence is captured." \
     --label ci-manual-dispatch
   ```

   If the label is not present after creation, add it immediately:

   ```bash
   gh pr edit <PR_NUMBER> --repo fzyzcjy/flutter_rust_bridge --add-label ci-manual-dispatch
   ```

3. Confirm the automatic PR-triggered CI did not schedule the heavy full CI surface.

   ```bash
   gh pr checks <PR_NUMBER> --repo fzyzcjy/flutter_rust_bridge
   gh run list --repo fzyzcjy/flutter_rust_bridge --branch codex/ci-filter-dummy-YYYYMMDD-HHMMSS --event pull_request --limit 5
   ```

4. Dispatch exactly one Linux Dart native matrix item.

   ```bash
   gh workflow run ci.yaml \
     --repo fzyzcjy/flutter_rust_bridge \
     --ref codex/ci-filter-dummy-YYYYMMDD-HHMMSS \
     -f 'ci_filter=test_dart_native[image=ubuntu-24.04,package=frb_example--dart_minimal]'
   ```

5. Identify the workflow dispatch run and watch its jobs.

   ```bash
   gh run list --repo fzyzcjy/flutter_rust_bridge --branch codex/ci-filter-dummy-YYYYMMDD-HHMMSS --event workflow_dispatch --limit 5
   gh run view <RUN_ID> --repo fzyzcjy/flutter_rust_bridge --json status,conclusion,url,headSha,attempt
   gh api repos/fzyzcjy/flutter_rust_bridge/actions/runs/<RUN_ID>/jobs --paginate
   ```

## Expected Result

The manual test passes when all of the following are true:

- The PR has the `ci-manual-dispatch` label.
- The automatic pull request CI run only executes the lightweight planning/status surface and does not run unrelated heavy matrix jobs.
- The workflow dispatch run is named with the selected filter.
- The `Plan :: CI` job succeeds and prints a plan where `test_dart_native.enable` is `true`.
- Exactly one `Test :: Dart :: Native` matrix job is scheduled for `ubuntu-24.04` and `frb_example--dart_minimal`.
- Unselected CI jobs are skipped or absent from the scheduled job set.
- The selected matrix job exits successfully.

Expected selected job name:

```text
Test :: Dart :: Native (ubuntu-24.04, frb_example--dart_minimal)
```

## Failure Criteria

The test fails if any of the following happens:

- The filter is accepted locally but rejected by the `Plan :: CI` job.
- The workflow dispatch run schedules more than the selected `test_dart_native` matrix item, excluding lightweight planner/status jobs.
- The automatic PR CI runs the full heavy CI surface while `ci-manual-dispatch` is present.
- The selected `Test :: Dart :: Native (ubuntu-24.04, frb_example--dart_minimal)` job is missing.
- The selected matrix job fails for a reason attributable to CI planning or filtering.

Mark the run as blocked, not failed, if GitHub Actions capacity prevents the selected job from starting before the observation timeout.

## Results To Capture

- PR URL and PR number.
- PR labels at the time of dispatch.
- Exact `ci_filter` string.
- Local `plan-ci` smoke-check output.
- Pull request event run URL and job summary.
- Workflow dispatch run URL and job summary.
- Selected job URL, status, conclusion, runner image, and key success log lines.
- Any unexpected scheduled, skipped, failed, or queued jobs.

## Troubleshooting

- If the PR starts full CI before the label is visible, add the label, cancel stale full runs, and record the race in the execution result.
- If `gh workflow run` cannot find `ci.yaml`, verify the branch contains the workflow changes and use `--ref` with the pushed branch name.
- If the filter matches no entries, run `./frb_internal plan-ci --filter '<FILTER>'` locally and compare the matrix keys with `.github/workflows/ci.yaml`.
- If jobs remain queued because of GitHub Actions capacity, record the queued state and rerun or continue watching later.

## Cleanup

After recording the result:

```bash
gh pr edit <PR_NUMBER> --repo fzyzcjy/flutter_rust_bridge --remove-label ci-manual-dispatch
git status --short
```

Close the dummy PR and delete the remote branch after the execution evidence is captured. Do not merge the dummy PR.

## Future Automation

This scenario could become an automated workflow test if GitHub Actions supports a low-cost self-test path for dispatching a workflow and asserting its own planned job set. Until then, a manual PR-level run is the closest end-to-end validation because the behavior depends on GitHub event labels, workflow dispatch inputs, and matrix scheduling.
