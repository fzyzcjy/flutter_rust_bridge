---
name: frb-manual-test
description: Use when a flutter_rust_bridge bug cannot be reproduced by CI and needs an independent manual regression test report under tools/manual_tests, or when writing, reviewing, or updating those manual test reports.
---

# FRB Manual Test

Use this skill when a bug report needs a durable manual regression test because CI cannot realistically reproduce the bad behavior. Manual tests are for cases involving external devices, host-specific setup, interactive tooling, credentials, marketplace/account state, flaky infrastructure, or other conditions that cannot be encoded as a reliable CI job yet.

## Core Rule

A manual test report is an independent evidence artifact. For a bug fix in the `frb-issue-to-green-pr` workflow:

1. First try to create an intentional red CI reproduction PR with CI narrowing.
2. If that is not realistic, create a separate manual-test PR that adds `tools/manual_tests/<name>.md`.
3. Only after that evidence PR exists should the fix PR proceed.

The manual-test PR is not the fix PR. Its PR title and body should say that it adds manual regression coverage and does not fix the bug.

## File Location

Write reports under:

```text
tools/manual_tests/<name>.md
```

Use a short kebab-case name. Include a date prefix when the report is an example, a one-off reproduction, or does not yet have a stable feature name, such as:

```text
tools/manual_tests/2026-06-06-example.md
```

## Required Content

Each report must be mechanical enough that a human or future agent can run it without the original conversation:

- Purpose and scope.
- Source issue, PR, user report, or context link.
- Why this is manual instead of CI-backed.
- Baseline commit or release to test.
- Required environment: OS, Flutter, Dart, Rust, devices, simulators, browsers, network, credentials, and any external services.
- Repository preparation commands.
- Step-by-step reproduction commands or UI actions.
- Expected bad behavior before the fix, including exact error text when known.
- Expected fixed behavior after the fix.
- Cleanup steps.
- Evidence to capture: logs, screenshots, artifacts, run URLs, or command output.
- Notes on how this could later become an automated CI test.

## Writing Rules

- Keep steps copy-pasteable and ordered.
- Do not rely on "run the app normally" or "verify it works" without concrete commands or UI actions.
- Prefer repo tooling such as `./frb_internal` when possible.
- Include exact paths for files to edit, commands to run, and artifacts to inspect.
- If credentials or private services are needed, name the required capability without including secrets.
- If the test is destructive or changes host state, include an explicit cleanup section.

## PR Body

For the manual-test PR, include:

```markdown
## Changes

Add a manual regression test for <scenario>.

This PR intentionally does not fix the bug. It records mechanical reproduction steps because the scenario cannot be covered by CI yet.

## Validation

- Read through tools/manual_tests/<name>.md for mechanical completeness
```

For the later fix PR, link the manual-test PR and report whether the manual test was re-run after the fix.
