# Manual Test Execution

## Test

:::info
Identify the manual test that was executed and why this run happened. Copy the purpose from the test report or summarize it in one sentence.
:::

- Manual test: `TODO: tools/manual_tests/name.md`
- Purpose: `TODO: short description copied or summarized from the test report`
- Source context: `TODO: issue, PR, release, or request if relevant`

## Run

:::info
Record when, by whom, and against exactly what commit or release the manual test was executed. Include enough environment detail for someone else to compare results.
:::

- Date and timezone: `TODO: YYYY-MM-DD HH:MM TZ`
- Executor: `TODO: human, agent, or automation`
- Commit or release tested: `TODO: sha or version`
- Environment: `TODO: OS, shell, device, simulator, browser, or service summary`

## Procedure

:::info
Record whether the written test was followed exactly. Any deviation matters, even if the result passed.
:::

- Followed report revision: `TODO: commit containing the manual test report`
- Deviations from written steps: `TODO: none, or list exact deviations`
- Cleanup completed: `TODO: yes/no; explain if no`

## Result

:::info
Use `pass`, `fail`, `blocked`, or `partial`. The summary should explain the result without requiring the reader to open logs first.
:::

- Status: `TODO: pass/fail/blocked/partial`
- Summary: `TODO: one or two sentences`

## Artifacts

:::info
Link or name the artifacts captured during execution. Use `not applicable` for artifact types that do not apply.
:::

- Execution markdown gist: `TODO: gist URL if this result belongs to a PR, issue, or release checklist; otherwise not applicable`
- Terminal log: `TODO: path or link`
- Screenshot or recording: `TODO: path or link, or not applicable`
- Generated artifacts: `TODO: path or link, or not applicable`
- Other evidence: `TODO: path or link, or not applicable`

## Follow-up

:::info
If the result is not a clean pass, state the next action. Link the issue or PR when one exists.
:::

- Required follow-up: `TODO: none, issue link, PR link, or next action`
- Notes: `TODO: anything future executors should know`
