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
tools/manual_tests/NAME.md
```

When adding a report, do not start from a blank file. Copy the bundled test template first:

```bash
cp .claude/skills/frb-manual-test/test-template.md tools/manual_tests/NAME.md
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
cp .claude/skills/frb-manual-test/execution-template.md EXECUTION_RESULT_PATH.md
```

Always fill out an execution markdown file before summarizing the result. Do not only say that the test "passed".

If the execution result belongs to a PR, issue, release checklist, or other reviewable workflow, upload the filled execution markdown as a GitHub gist and link that gist in the PR description, PR comment, issue comment, or release checklist. The local execution markdown should still be kept in the run artifacts directory so the run can be audited without relying only on chat history.

If the result belongs only in chat, still fill out the execution markdown locally and include its path in the chat response.

Run cleanup from the manual test report before declaring the execution complete, unless cleanup is intentionally skipped and documented in the execution record.
