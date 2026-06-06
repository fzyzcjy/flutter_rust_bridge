---
name: frb-manual-test
description: Use when adding, executing, reviewing, or updating flutter_rust_bridge manual test reports under tools/manual_tests for scenarios that require human or agent-driven manual verification.
---

# FRB Manual Test

Use this skill for manual test reports in `tools/manual_tests/`. These reports describe repeatable checks that a human or agent can execute when automated tests are insufficient, unavailable, too expensive, or not the right interface for the behavior being verified.

Manual tests are normal software-engineering test artifacts. They may cover devices, host-specific setup, interactive UI behavior, credentials or account state, release packaging, marketplace flows, external services, or long-running workflows. They are not tied to any single PR workflow.

## Add Manual Tests

Manual test reports live under:

```text
tools/manual_tests/<name>.md
```

When adding a report, do not start from a blank file. Copy the bundled test template first:

```bash
cp .claude/skills/frb-manual-test/test-template.md tools/manual_tests/<name>.md
```

Use a short kebab-case name. Include a date prefix when the report is an example, a one-off check, or does not yet have a stable feature name, such as:

```text
tools/manual_tests/2026-06-06-some-scenario.md
```

After copying, edit the report in place:

- Replace every placeholder.
- Remove sections that truly do not apply.
- Keep commands and UI actions mechanical enough that a future human or agent can run them without the original conversation.
- Prefer repo tooling such as `./frb_internal` when possible.
- Do not rely on "run the app normally", "check manually", or "verify it works" without concrete commands or UI actions.
- If credentials or private services are needed, name the required capability without including secrets.

The detailed content requirements live in `test-template.md`; keep that template as the source of truth for report shape.

## Execute Manual Tests

When executing a manual test, first read the target report under `tools/manual_tests/` and follow its preparation, steps, expected result, failure criteria, troubleshooting, and cleanup sections.

Before reporting the run result, use the bundled execution template:

```bash
cp .claude/skills/frb-manual-test/execution-template.md <execution-result-path>.md
```

If the result belongs in a PR comment, issue comment, release checklist, or chat response instead of a file, copy the template text there and fill it out. Do not only say that the test "passed".

Fill the execution template with:

- Date and timezone.
- Executor: human name, agent, or automation.
- Manual test path.
- Commit or release tested.
- Environment actually used.
- Result: pass, fail, blocked, or partial.
- Result artifact paths or links.
- Deviations from the written steps.
- Follow-up issue or PR if the result is not clean.

Run cleanup from the manual test report before declaring the execution complete, unless cleanup is intentionally skipped and documented in the execution record.
