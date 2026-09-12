---
name: frb-pr-review
description: Review a flutter_rust_bridge PR before treating it as ready, including subagent checks for correctness and test weakening.
---

# 1 Human SoT

# 2 FRB PR Review

Use this before treating a non-trivial `flutter_rust_bridge` PR as ready, especially after CI-driven fixes, Flutter upgrades, generated drift, test changes, workflow changes, or broad merge conflict resolution.

## 2.1 Review Gate

Run the following review before final readiness:

1. Review the PR diff against its base:
   - Spawn a subagent to review correctness.
     - Ask it to inspect the PR diff against the PR base.
     - Focus on real behavior bugs, stale generated output, incorrect CI workarounds, coverage-only changes, and unrelated drift.
     - Require findings with file paths, line numbers, impact, and suggested fix.
   - Spawn a subagent to review test weakening.
     - Inspect changed or deleted assertions, relaxed thresholds, new skips, ignored failures, timeout changes, and coverage exclusions.
     - Require a concrete justification for each reduction in coverage; restore unjustified weakening and fix the underlying failure.
     - If `tom-style-tests` is installed, apply its common test principles and weakening gate; use `frb-test` for FRB execution commands.
     - Treat unjustified skipped tests, weaker assertions, broader ignores, fake timeouts, and coverage hiding as blockers.

2. Write a concise review conclusion.
   - Put the conclusion in the PR description or an agent-context draft when the user asks for a Markdown artifact.
   - Include the subagents used, accepted findings, dismissed findings, fixes made, and remaining risks.

## 2.2 Stop Condition

Do not call the PR ready until:

- Correctness review has no unresolved actionable findings and test-weakening review has no unjustified weakening.
- CI status is green, or remaining non-green checks are clearly unrelated and explained.
