---
name: frb-manual-test
description: Use when writing, executing, reviewing, or updating flutter_rust_bridge manual test reports under tools/manual_tests for scenarios that require human or agent-driven manual verification.
---

# FRB Manual Test

Use this skill for manual test reports in `tools/manual_tests/`. These reports describe repeatable checks that a human or agent can execute when automated tests are insufficient, unavailable, too expensive, or not the right interface for the behavior being verified.

Manual tests are normal software-engineering test artifacts. They may cover devices, host-specific setup, interactive UI behavior, credentials or account state, release packaging, marketplace flows, external services, or long-running workflows. They are not tied to any single PR workflow.

## Core Rule

A manual test report must be mechanical enough that a future human or agent can execute it without the original conversation.

When writing a new manual test report, do not start from a blank file. Copy the bundled template first, then edit it for the scenario:

```bash
cp .claude/skills/frb-manual-test/template.md tools/manual_tests/<name>.md
```

After copying, replace every placeholder and remove sections that truly do not apply. Keep the resulting report as a normal test case:

1. Define the purpose and scope.
2. State preconditions and environment.
3. Give exact setup and execution steps.
4. Define pass/fail criteria.
5. Say what results and artifacts to capture.
6. Include cleanup and maintenance notes.

## File Location

Write reports under:

```text
tools/manual_tests/<name>.md
```

Use a short kebab-case name. Include a date prefix when the report is an example, a one-off check, or does not yet have a stable feature name, such as:

```text
tools/manual_tests/2026-06-06-some-scenario.md
```

## Required Content

Each report should include:

- Purpose and scope.
- Source context: issue, PR, feature, release, or user report when available.
- When to run the test: before release, after changing a subsystem, after dependency upgrades, before closing a bug, or on demand.
- Preconditions: required branch, commit, release, account state, devices, generated files, caches, or services.
- Environment: OS, Flutter, Dart, Rust, devices, simulators, browsers, network, credentials, and external services.
- Test data and fixtures, including how to create or reset them.
- Repository preparation commands.
- Step-by-step execution commands or UI actions.
- Expected result and pass/fail criteria.
- Troubleshooting notes for common setup failures.
- Cleanup steps.
- Results to capture: logs, screenshots, artifacts, command output, device details, or screen recordings.
- Execution record section where the runner can append date, executor, commit, result, and artifact links.
- Automation notes if the test could later become an automated CI or integration test.

## Writing Rules

- Keep steps copy-pasteable and ordered.
- Do not rely on "run the app normally", "check manually", or "verify it works" without concrete commands or UI actions.
- Prefer repo tooling such as `./frb_internal` when possible.
- Include exact paths for files to edit, commands to run, and artifacts to inspect.
- If credentials or private services are needed, name the required capability without including secrets.
- If the test is destructive or changes host state, include an explicit cleanup section.
- Keep the report evergreen: update commands, device names, and expected output when the product or tooling changes.

## Executing A Manual Test

When executing a manual test, do not only say that it "passed". Record:

- Date and timezone.
- Executor: human name, agent, or automation.
- Commit or release tested.
- Environment actually used.
- Result: pass, fail, blocked, or partial.
- Result artifact paths or links.
- Any deviations from the written steps.
- Follow-up issue or PR if the result is not clean.

Append this to the report's execution record when the report is meant to keep run history. If the run history would be noisy, put the execution result in the PR, issue, or release checklist instead and keep the report itself as the stable procedure.
