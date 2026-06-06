# Manual Test Template

## Purpose

:::info
Write one or two concrete sentences explaining what behavior this manual test verifies. Name the user-visible behavior, platform, package, or workflow under test. Do not leave this as a generic phrase like "verify it works".
:::

TODO: Write what this test verifies.

## Source

:::info
Link the reason this test exists: issue, PR, feature request, release checklist, user report, or maintenance task. If there is no external link, state the local context in plain text.
:::

- Context: `TODO: issue, PR, feature, release, or user report`
- Related docs or skills: `TODO: paths or links`

## When To Run

:::info
State the trigger for this manual test. Examples: before a release, after changing a named subsystem, after upgrading Flutter/Rust/Dart, before closing a bug, or when validating a specific platform.
:::

TODO: Write when this test is required or useful.

## Preconditions

:::info
List everything that must already be true before starting. Include checkout state, expected branch family if relevant, submodules, generated files, devices, simulators, credentials, account state, network access, and external services. Never paste secrets. Keep this section about reusable requirements, not one particular run.
:::

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: `TODO: clean checkout with submodules initialized, generated files present, etc.`
- Required credentials or account state: `TODO: required capability, never paste secrets`
- Required device or simulator state: `TODO: device must be available, app uninstalled, cache reset, etc.`

## Environment

:::info
Record exact versions and runtime context needed to make the result reproducible. If a tool is not required, write `not required` instead of leaving it ambiguous.
:::

- OS: `TODO: macOS / Linux / Windows version, or not constrained`
- Flutter: `TODO: flutter --version, or not required`
- Dart: `TODO: dart --version, or not required`
- Rust: `TODO: rustc --version, or not required`
- Device or simulator: `TODO: device name, OS version, UDID if applicable, or not required`
- Browser or external service: `TODO: name and version if applicable, or not required`

## Preparation

:::info
Give copy-pasteable commands that take an existing repository root to the state needed by the test. Prefer repository tooling such as `./frb_internal`. If generated files, caches, or dependencies are required, include the exact commands here. Keep source acquisition and revision selection outside the reusable test case.
:::

```bash
git submodule update --init --recursive
```

If this test needs generated output or dependencies, list the exact setup commands here.

```bash
./frb_internal --help
```

## Test Data

:::info
Describe all inputs and fixtures. Include paths, account setup, generated files, sample API code, reset commands, or "none" if the test uses no special data.
:::

- Input files, API examples, account fixtures, or generated assets: `TODO: paths or setup notes, or none`
- Reset procedure before each run: `TODO: commands or UI actions`

## Steps

:::info
Write ordered mechanical steps. Each step should be executable by a human or agent without guessing. For UI steps, name buttons, screens, field values, and expected intermediate state. For command steps, include complete commands.
:::

1. TODO: Prepare the target package, example, device, account, or service.

   ```bash
   TODO_COMMAND
   ```

2. TODO: Run the command or manual action that verifies the behavior.

   ```bash
   TODO_COMMAND
   ```

3. If UI interaction is required, describe each click, field value, device action, or expected screen by name.

## Expected Result

:::info
Define the pass condition as observable output: command exit status, log text, generated artifact, UI state, screenshot content, or external service state. Avoid vague language.
:::

TODO: The command or manual flow should complete successfully and show the expected observable behavior.

```text
TODO: success output, UI text, artifact path, or other pass criteria
```

## Failure Criteria

:::info
List concrete conditions that mean the test failed or is blocked. Include setup failures that should be reported separately from product failures.
:::

The test fails if any of the following happens:

- The command exits non-zero unexpectedly.
- The UI, artifact, or log output does not match the expected result.
- The required device, account, or service state cannot be prepared.

## Results To Capture

:::info
Say exactly what artifacts the executor should save or link in the execution record. Include terminal logs, screenshots, recordings, generated files, device details, run URLs, or external service observations.
:::

- Full terminal log from the command.
- Screenshot or screen recording if UI behavior is involved.
- Device, simulator, or browser version.
- Any generated artifact path needed to inspect the result.

## Troubleshooting

:::info
Provide known recovery steps for common environment and setup failures. This section should help distinguish a blocked manual run from a real product failure.
:::

- If dependencies are missing, re-run the repository setup commands and record the exact failure.
- If a device or simulator is unavailable, record the discovered devices and mark the run as blocked.
- If credentials or external services are unavailable, record the missing capability without exposing secrets.

## Cleanup

:::info
List commands and manual actions needed to restore the checkout, stop services, reset devices, and undo external account state. If no cleanup is needed, say so explicitly.
:::

```bash
git status --short
```

Remove temporary local files, stop simulators or servers started for the test, and restore any external account or device state changed during the run.

## Future Automation

:::info
Explain whether this manual test could later become automated. If yes, name the likely CI job, package, or test harness. If no, explain what keeps it manual.
:::

If this scenario later becomes reliable and affordable to automate, replace or supplement this manual report with an automated test and link the PR here.
