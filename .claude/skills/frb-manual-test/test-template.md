# Manual Test Template

## Purpose

<What this test verifies.>

## Source

- Context: `<issue, PR, feature, release, or user report>`
- Related docs or skills: `<paths or links>`

## When To Run

<When this test is required or useful.>

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Commit or release under test: `<commit-or-version>`
- Required branch state: `<clean checkout with submodules initialized, generated files present, etc.>`
- Required credentials or account state: `<required capability, never paste secrets>`
- Required device or simulator state: `<device must be available / app uninstalled / cache reset / etc.>`

## Environment

- OS: `<macOS / Linux / Windows version>`
- Flutter: `<flutter --version>`
- Dart: `<dart --version>`
- Rust: `<rustc --version>`
- Device or simulator: `<device name, OS version, UDID if applicable>`
- Browser or external service: `<name and version, if applicable>`

## Preparation

```bash
git clone https://github.com/fzyzcjy/flutter_rust_bridge.git
cd flutter_rust_bridge
git checkout <commit-or-version-under-test>
git submodule update --init --recursive
```

If this test needs generated output or dependencies, list the exact setup commands here.

```bash
./frb_internal --help
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: `<paths or setup notes>`
- Reset procedure before each run: `<commands or UI actions>`

## Steps

1. <Prepare the target package, example, device, account, or service.>

   ```bash
   <command>
   ```

2. <Run the command or manual action that verifies the behavior.>

   ```bash
   <command>
   ```

3. If UI interaction is required, describe each click, field value, device action, or expected screen by name.

## Expected Result

<The command or manual flow should complete successfully and show the expected observable behavior.>

```text
<success output, UI text, artifact path, or other pass criteria>
```

## Failure Criteria

The test fails if any of the following happens:

- The command exits non-zero unexpectedly.
- The UI, artifact, or log output does not match the expected result.
- The required device, account, or service state cannot be prepared.

## Results To Capture

- Full terminal log from the command.
- Screenshot or screen recording if UI behavior is involved.
- Device, simulator, or browser version.
- Any generated artifact path needed to inspect the result.

## Troubleshooting

- If dependencies are missing, re-run the repository setup commands and record the exact failure.
- If a device or simulator is unavailable, record the discovered devices and mark the run as blocked.
- If credentials or external services are unavailable, record the missing capability without exposing secrets.

## Cleanup

```bash
git status --short
```

Remove temporary local files, stop simulators or servers started for the test, and restore any external account or device state changed during the run.

## Future Automation

If this scenario later becomes reliable and affordable to automate, replace or supplement this manual report with an automated test and link the PR here.
