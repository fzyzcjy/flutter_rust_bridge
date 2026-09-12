---
name: frb-ci
description: Monitor flutter_rust_bridge PR checks, wait for CI state changes, and retrieve GitHub Actions job logs.
---

# 1 Human SoT

# 2 Choose the CI Tools

- If `tom-ci` is installed, read its `docs/common/overview.md` and the matching manuals under `docs/common/manuals/`: `waiter.md`, `read-status.md`, `live-logs.md`, and `diagnose-a-failure.md`.
- Use only those common mechanisms with `fzyzcjy/flutter_rust_bridge`. Follow `frb-ci-filter` for dispatch and labels, `frb-fix-ci` for failure diagnosis, and `frb-pr-review` for readiness.
- Resolve personal skill paths from the active skill catalog. Do not require a particular user's filesystem layout.
- If `tom-ci` is unavailable, use the GitHub CLI workflow below; it does not require a personal skill.

# 3 Wait and Inspect

- With `tom-ci`, run its `scripts/wait_for_ci.py` with the PR URL after each push, rerun, or handled CI event. Follow `waiter.md` for bounded timeouts, persistent state, and exact-run monitoring.
- Treat the returned event as a wake-up signal. Inspect the latest head, checks, and run identity before deciding what to do.
- On timeout, start another bounded wait. Keep only one waiter active for the PR.
- Without `tom-ci`, query the PR with:

```bash
gh pr view <pr-url> --json headRefOid,mergeable,state,statusCheckRollup
```

- While checks are pending or the rollup is empty, wait between queries using the agent's sleep tool, with a bounded deadline for each monitoring session. Resume monitoring after the deadline if CI is still pending; do not create an unbounded shell loop or detached poller.
- Verify the workflow run belongs to the expected branch, commit, event, and attempt. For focused dispatches, verify the selected job actually ran; a skipped job or an unrelated green run is not success.
- Do not call the PR ready from an empty rollup or an intentionally partial run. Apply `frb-ci-filter` before final readiness.

# 4 Read Logs and Handle Failures

- Get the job ID from the check's `detailsUrl` or the run's jobs response.
- Save logs to the task's artifact directory before reading them:

```bash
gh api repos/fzyzcjy/flutter_rust_bridge/actions/jobs/<job-id>/logs > <artifact-dir>/job-<job-id>.log
```

- The per-job endpoint may provide partial logs while a job is running. If logs are unavailable, inspect job steps and retry later; do not infer the failure cause from an unavailable log.
- Use `gh run view --log` only after the run completes. Read the complete relevant failure log before diagnosing; a progress tail alone is insufficient.
- Follow `frb-fix-ci` before changing code or rerunning failures. Keep waiting after each action until checks are green or remaining non-green checks are clearly unrelated and explained.
- Follow the user's authorization rules for comments and reruns; waiting and reading logs do not authorize posting messages.
