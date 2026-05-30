---
name: frb-pr-review
description: Review a flutter_rust_bridge PR before treating it as ready, including subagent checks for correctness and test weakening plus Gemini review.
---

# FRB PR Review

Use this before treating a non-trivial `flutter_rust_bridge` PR as ready, especially after CI-driven fixes, Flutter upgrades, generated drift, test changes, workflow changes, or broad merge conflict resolution.

## Review Gate

Run independent review before final readiness:

1. Spawn a subagent to review correctness.
   - Ask it to inspect the PR diff against the PR base.
   - Focus on real behavior bugs, stale generated output, incorrect CI workarounds, coverage-only changes, and unrelated drift.
   - Require findings with file paths, line numbers, impact, and suggested fix.

2. Spawn a subagent to review test weakening.
   - Use the test-weakening gate described in `sdev-pass-test` as the source of truth.
   - Do not duplicate that workflow here; read `sdev-pass-test` for detection, classification, and restoration details.
   - Treat unjustified skipped tests, weaker assertions, broader ignores, fake timeouts, and coverage hiding as blockers.

3. Ask Gemini for an external review.
   - Post `/gemini review` when the PR is reasonably ready.
   - Inspect Gemini's review/comments.
   - Fix valid feedback, commit, push, and explain invalid feedback in a concise PR comment.
   - Request another Gemini pass after substantial follow-up fixes.

4. Write a concise review conclusion.
   - Put the conclusion in the PR description or an agent-context draft when the user asks for a Markdown artifact.
   - Include the subagents used, Gemini status, accepted findings, dismissed findings, fixes made, and remaining risks.

## Stop Condition

Do not call the PR ready until:

- Correctness review has no unresolved actionable findings.
- Test-weakening review has no unjustified weakening.
- Gemini has no unresolved actionable feedback after the latest requested pass.
- CI status is green, or remaining non-green checks are clearly unrelated and explained.
