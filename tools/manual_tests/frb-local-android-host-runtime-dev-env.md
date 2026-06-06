# Local Android Host Runtime Development Environment Coverage

## Purpose

Verify that the FRB local development workflow can run Android runtime validation with a host-managed Android Emulator and FRB/Flutter commands running inside the per-worktree Docker container.

## Source

- Context: Maintenance check for the local FRB Android runtime workflow added to `.claude/skills/frb-dev-env`.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-android-emulator-prepare/SKILL.md`, `.claude/skills/frb-docker/SKILL.md`, `.claude/skills/frb-test/SKILL.md`

## When To Run

Run this after changing the FRB dev environment helper, host Android Emulator workflow, Docker image Android SDK contents, Flutter Android tooling, CargoKit Android integration, or any behavior that must be validated on a running host Android Emulator from local Docker.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Docker must be able to pull or use the configured FRB development image. Android SDK licenses must be accepted before installing or using emulator packages.
- Required device or simulator state: prepared host Android Emulator environment with an AVD such as `FRB_API_35`; prepare it with `.claude/skills/frb-android-emulator-prepare/SKILL.md`.

## Environment

- OS: host macOS capable of running Docker and a host Android Emulator.
- Flutter: record `flutter --version` inside the Docker container.
- Dart: record `dart --version` inside the Docker container.
- Rust: record `rustc --version` and `cargo --version` inside the Docker container.
- Device or simulator: record the host emulator serial, device name, API level, and Docker-visible ADB endpoint.
- Browser or external service: not required.

## Preparation

Run preparation commands from the repository root of the checkout being tested.

```bash
git rev-parse --show-toplevel
git status --short
git submodule update --init --recursive
sw_vers
which adb
adb version
.claude/skills/frb-dev-env/frb_dev_env.py docker info
.claude/skills/frb-dev-env/frb_dev_env.py docker create
```

Prepare the SDK packages and AVD using `.claude/skills/frb-android-emulator-prepare/SKILL.md`, then start the emulator with a stable console port:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android env
.claude/skills/frb-dev-env/frb_dev_env.py android emulator --avd FRB_API_35 --port 5554
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in package `frb_example/flutter_via_create`.
- Reset procedure before each run: return to a clean checkout or record intentional local changes in the execution record. Use a single Android target when possible.

## Steps

1. Record host Android Emulator state. `emulator-5554` should be present and authorized.

   ```bash
   adb devices -l
   ```

2. Record Docker tool versions.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc '
   set -euo pipefail
   flutter --version
   dart --version
   rustc --version
   cargo --version
   '
   ```

3. Confirm Docker-side ADB can see the host emulator through Docker-local ADB connected directly to the emulator endpoint:

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l
   ```

4. Run the Android Flutter native integration test from Docker. For host console port `5554`, use Docker-visible serial `host.docker.internal:5555`:

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- \
     bash -lc './frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id host.docker.internal:5555"'
   ```
5. Confirm the host checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

## Expected Result

The Android host runtime coverage test passes when host ADB sees the emulator, Docker-side ADB sees the same emulator through `--android-emulator-adb`, and the Android Flutter native integration test completes successfully against the Docker-visible emulator serial.

```text
adb devices -l
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l
./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id host.docker.internal:5555"
```

## Failure Criteria

The test fails if any of the following happens:

- The host emulator is missing, unauthorized, offline, or the wrong serial is selected.
- Docker-side ADB cannot see the host-managed emulator through `--android-emulator-adb`.
- The Android Flutter native integration test exits non-zero unexpectedly after the target is available.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if the host has no prepared Android Emulator environment.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host macOS version, Android SDK path if using an emulator, ADB version, Docker image, and container name.
- Android emulator host serial, Docker-visible endpoint, model, transport, and API level when available.
- Flutter, Dart, Rust, and Cargo versions inside the Docker container.
- Final `git status --short` output.

## Troubleshooting

- If host `adb devices -l` shows `unauthorized`, unlock the emulator and accept the debugging prompt if one appears.
- If Docker cannot see a host emulator, verify host `adb devices -l` shows `emulator-5554`, then retry `.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l`.
- If the Flutter integration test fails with a VM service or DDS connection refused on `127.0.0.1`, ensure the command uses `--android-emulator-adb` so the ADB server and `adb forward` live inside Docker.
- If the emulator command is missing, read `.claude/skills/frb-android-emulator-prepare/SKILL.md` and prepare the host Android Emulator environment before retrying.
- If Flutter cannot find Android build tools inside Docker, inspect the Docker image Android SDK contents and update the image or mount strategy; do not install host Flutter or Gradle as a workaround.

## Cleanup

Optionally stop the emulator:

```bash
adb -s emulator-5554 emu kill
git status --short
```

Keep the per-worktree Docker container and the Android SDK/AVD caches unless the user explicitly wants to delete them.

## Future Automation

This scenario can be automated on a dedicated macOS Android host with a prepared emulator and Docker Desktop. Keep this as a manual report while emulator lifecycle and local Docker networking remain host-specific.
