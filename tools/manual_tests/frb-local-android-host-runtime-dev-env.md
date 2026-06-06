# Local Android Host Runtime Development Environment Coverage

## Purpose

Verify that the FRB local development workflow can run Android runtime validation with the Android target managed by the host and FRB/Flutter commands running inside the per-worktree Docker container. This test covers both physical-device host ADB connectivity and the host Android Emulator path that Docker cannot provide directly on macOS.

## Source

- Context: Maintenance check for the local FRB Android runtime workflow added to `.claude/skills/frb-dev-env`.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-android-emulator-prepare/SKILL.md`, `.claude/skills/frb-docker/SKILL.md`, `.claude/skills/frb-test/SKILL.md`

## When To Run

Run this after changing the FRB dev environment helper, Android host ADB workflow, Docker image Android SDK contents, Flutter Android tooling, CargoKit Android integration, or any behavior that must be validated on a running Android device or emulator from local Docker.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Docker must be able to pull or use the configured FRB development image. Android SDK licenses must be accepted before installing or using emulator packages.
- Required device or simulator state: one Android target managed by the host. A physical phone with USB debugging authorization is sufficient for the physical-device path. The emulator path requires a prepared host Android Emulator environment with an AVD such as `FRB_API_35`; prepare it with `.claude/skills/frb-android-emulator-prepare/SKILL.md`.

## Environment

- OS: host macOS capable of running Docker and a host Android device or emulator.
- Flutter: record `flutter --version` inside the Docker container.
- Dart: record `dart --version` inside the Docker container.
- Rust: record `rustc --version` and `cargo --version` inside the Docker container.
- Device or simulator: record the Android serial, device name, API level, and whether it is a physical device or emulator.
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

For a physical Android device, connect it to the host, enable USB debugging, accept the host authorization prompt, and record:

```bash
adb devices -l
```

For a host Android Emulator, first prepare the SDK packages and AVD using `.claude/skills/frb-android-emulator-prepare/SKILL.md`, then start the emulator with a stable console port:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android env
.claude/skills/frb-dev-env/frb_dev_env.py android emulator --avd FRB_API_35 --port 5554
```

For a physical Android device, start a host ADB server that Docker can reach in a separate foreground terminal:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android adb-server
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in package `frb_example/flutter_via_create`.
- Reset procedure before each run: return to a clean checkout or record intentional local changes in the execution record. Use a single Android target when possible.

## Steps

1. Record host Android target state. For the emulator path, `emulator-5554` should be present and authorized.

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

3. Confirm Docker-side ADB can see the selected Android target. For the emulator path, use Docker-local ADB connected directly to the host emulator endpoint:

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l
   ```

   For a physical device connected to host USB, use the host ADB server path instead:

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-host-adb -- adb devices -l
   ```

4. Run the Android Flutter native integration test from Docker. For the emulator path with host console port `5554`, use Docker-visible serial `host.docker.internal:5555`:

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- \
     bash -lc './frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id host.docker.internal:5555"'
   ```

   For a physical device, replace `<SERIAL>` with the device serial and use `--android-host-adb`. This path validates physical-device connectivity, but prefer the emulator path above for full Flutter runtime checks when possible because Flutter's `adb forward` binds dynamic VM service ports where the ADB server runs.

5. Confirm the host checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

## Expected Result

The Android host runtime coverage test passes when host ADB sees the selected Android target, Docker-side ADB sees the same target through the appropriate mode, and the Android Flutter native integration test completes successfully against the requested serial.

```text
adb devices -l
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l
./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id host.docker.internal:5555"
```

## Failure Criteria

The test fails if any of the following happens:

- The host ADB server cannot start in Docker-reachable mode.
- The Android target is missing, unauthorized, offline, or the wrong serial is selected.
- Docker-side ADB cannot see the host-managed Android target through the selected ADB mode.
- The Android Flutter native integration test exits non-zero unexpectedly after the target is available.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if the host has no physical Android device and no prepared Android Emulator environment.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host macOS version, Android SDK path if using an emulator, ADB version, Docker image, and container name.
- Android target serial, model, transport, and API level when available.
- Flutter, Dart, Rust, and Cargo versions inside the Docker container.
- Final `git status --short` output.

## Troubleshooting

- If host `adb devices -l` shows `unauthorized`, unlock the Android device or emulator and accept the USB debugging prompt.
- If host ADB cannot start with `could not install *smartsocket* listener`, retry from a normal host terminal or with an execution context that can bind local TCP ports. If the error says `listening on specified hostname currently unsupported`, use `adb -a -L tcp:5037 server nodaemon` rather than `tcp:0.0.0.0:5037`.
- If Docker cannot see a host emulator, verify host `adb devices -l` shows `emulator-5554`, then retry `.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l`.
- If Docker cannot see a physical device, ensure `.claude/skills/frb-dev-env/frb_dev_env.py android adb-server` is still running and using the same port as `docker exec --android-host-adb`.
- If the Flutter integration test fails with a VM service or DDS connection refused on `127.0.0.1`, rerun the emulator path with `--android-emulator-adb` rather than `--android-host-adb`.
- If the emulator command is missing, read `.claude/skills/frb-android-emulator-prepare/SKILL.md` and prepare the host Android Emulator environment before retrying.
- If Flutter cannot find Android build tools inside Docker, inspect the Docker image Android SDK contents and update the image or mount strategy; do not install host Flutter or Gradle as a workaround.

## Cleanup

Stop the foreground host ADB server with `Ctrl-C` after the run. For an emulator target, optionally stop the emulator:

```bash
adb -s emulator-5554 emu kill
git status --short
```

Keep the per-worktree Docker container and the Android SDK/AVD caches unless the user explicitly wants to delete them.

## Future Automation

This scenario can be automated on a dedicated macOS Android host with a prepared emulator and Docker Desktop. Keep this as a manual report while USB authorization, emulator lifecycle, host TCP binding, and local Docker networking remain host-specific.
