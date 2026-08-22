---
name: frb-pr-review
description: Review a flutter_rust_bridge PR before treating it as ready, including subagent checks for correctness and test weakening.
---

# FRB PR Review

Use this before treating a non-trivial `flutter_rust_bridge` PR as ready, especially after CI-driven fixes, Flutter upgrades, generated drift, test changes, workflow changes, or broad merge conflict resolution.

## Review Gate

Run exactly one review path before final readiness:

1. If the user's local skill catalog provides `sdev-review-single-general`, you MUST run its complete workflow against the PR diff.
   - This is the primary review path.
   - Do not also run the fallback review below unless `sdev-review-single-general` requires it.

2. Otherwise, and only when `sdev-review-single-general` is unavailable, run this fallback review:
   - Spawn a subagent to review correctness.
     - Ask it to inspect the PR diff against the PR base.
     - Focus on real behavior bugs, stale generated output, incorrect CI workarounds, coverage-only changes, and unrelated drift.
     - Require findings with file paths, line numbers, impact, and suggested fix.
   - Spawn a subagent to review test weakening.
     - Use the test-weakening gate described in `sdev-pass-test` as the source of truth.
     - Do not duplicate that workflow here; read `sdev-pass-test` for detection, classification, and restoration details.
     - Treat unjustified skipped tests, weaker assertions, broader ignores, fake timeouts, and coverage hiding as blockers.

3. After either review path, write a concise review conclusion.
   - Put the conclusion in the PR description or an agent-context draft when the user asks for a Markdown artifact.
   - Include the subagents used, accepted findings, dismissed findings, fixes made, and remaining risks.

## Stop Condition

Do not call the PR ready until:

- The selected review path has no unresolved actionable findings.
- When the fallback path was used, correctness review has no unresolved actionable findings and test-weakening review has no unjustified weakening.
- CI status is green, or remaining non-green checks are clearly unrelated and explained.
