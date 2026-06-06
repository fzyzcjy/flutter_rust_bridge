# Local Tart Apple Development Environment Coverage

## Purpose

Verify that the per-worktree FRB Tart macOS VM can run Apple-platform local validation that Docker cannot provide: macOS Flutter native testing, iOS Simulator testing, and Apple release build smoke checks. This test also records that Tart is not the Android emulator strategy.

## Source

- Context: Maintenance check for the local FRB Tart Apple-platform development workflow.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-tart-prepare/SKILL.md`, `.claude/skills/frb-test/SKILL.md`

## When To Run

Run this after changing the Tart helper, Tart base VM, Apple scaffold, Flutter/Xcode/macOS/iOS versions, CargoKit Apple integration, or any behavior that must be validated with macOS or an iOS Simulator.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Tart must be installed and the immutable base VM `frb-tart-base` must exist, unless the run intentionally sets `FRB_TART_BASE_VM` to another prepared base VM.
- Required device or simulator state: an available iOS Simulator runtime inside the Tart VM. The executor must choose one available simulator UDID from `xcrun simctl list devices available`.

## Environment

- OS: host macOS capable of running Tart, plus guest macOS from `sw_vers` inside the VM.
- Flutter: record `flutter --version --no-version-check` inside the Tart VM.
- Dart: record `dart --version` inside the Tart VM.
- Rust: record `rustc --version` and `cargo --version` inside the Tart VM.
- Device or simulator: record the selected iOS Simulator name, runtime, and UDID.
- Browser or external service: not required.

## Preparation

Run preparation commands from the repository root of the checkout being tested.

```bash
git rev-parse --show-toplevel
git status --short
git submodule update --init --recursive
sw_vers
tart --version
.claude/skills/frb-dev-env/frb_dev_env.py tart info
.claude/skills/frb-dev-env/frb_dev_env.py tart create
.claude/skills/frb-dev-env/frb_dev_env.py tart start --wait 300
.claude/skills/frb-dev-env/frb_dev_env.py tart ip --wait 180
```

Confirm the VM can see the mounted worktree and prepare the VM-local copy used for heavy Apple builds.

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- bash -lc 'pwd && git status --short && ./frb_internal --help'
.claude/skills/frb-dev-env/frb_dev_env.py tart upload
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- bash -lc 'set -euo pipefail; flutter config --no-version-check; git -C /Users/admin/flutter remote set-url origin file:///Users/admin/flutter; git -C /Users/admin/flutter remote -v'
```

The Flutter remote override is VM-local and prevents `flutter doctor -v`, which is invoked by `./frb_internal test-flutter-native`, from blocking on remote upstream checks when the local validation host has restricted network access. Record the resulting `origin file:///Users/admin/flutter` evidence in the execution result.

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in package `frb_example/flutter_via_create`.
- Reset procedure before each run: return to a clean checkout or record intentional local changes in the execution record. Reuse the VM-local uploaded copy unless the run intentionally deletes it.

## Steps

1. Record the Apple toolchain and FRB tool versions inside the Tart VM.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec -- bash -lc '
   set -euo pipefail
   sw_vers
   xcodebuild -version
   xcrun simctl list runtimes
   flutter --version --no-version-check
   dart --version
   rustc --version
   cargo --version
   '
   ```

2. Run the macOS Flutter native integration test from the VM-local copy.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id macos'
   ```

3. List available iOS simulators and choose one bootable UDID. Record the chosen simulator name, runtime, and UDID in the execution result.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl list devices available
   ```

4. Boot the selected iOS Simulator and wait for it to finish booting.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec -- bash -lc 'xcrun simctl boot <UDID> || true'
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl bootstatus <UDID> -b
   ```

5. Run the iOS Flutter native integration test from the VM-local copy.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id <UDID>'
   ```

6. Smoke-test Apple release build commands from the VM-local copy.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal build-flutter --target macos
   .claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal build-flutter --target ios
   ```

7. Confirm the host checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

## Expected Result

The Tart Apple environment coverage test passes when every command exits successfully, the macOS and iOS native test commands complete against the requested devices, and the Apple build smoke commands copy release artifacts into `target/build_flutter_output` inside the VM-local uploaded copy.

```text
./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id macos'
./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id <UDID>'
./frb_internal build-flutter --target macos
./frb_internal build-flutter --target ios
```

## Failure Criteria

The test fails if any of the following happens:

- Tart is unavailable, the base VM is unavailable, or the per-worktree VM cannot be created or started.
- The mounted worktree or VM-local uploaded copy cannot be prepared.
- The macOS native test command exits non-zero unexpectedly.
- No iOS Simulator can be booted, or the iOS native test command exits non-zero unexpectedly after a simulator is booted.
- The macOS or iOS build command exits non-zero unexpectedly.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if the host does not support Tart virtualization or the configured base VM is missing.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host macOS version, Tart version, VM name, base VM name, and guest macOS version.
- Xcode, Flutter, Dart, Rust, and Cargo versions inside the VM.
- Selected iOS Simulator name, runtime, and UDID.
- Paths under the VM-local copy for `target/build_flutter_output` artifacts.
- Final host `git status --short` output.

## Troubleshooting

- If submodules are uninitialized, rerun `git submodule update --init --recursive` and record the output.
- If the per-worktree VM is missing, rerun `.claude/skills/frb-dev-env/frb_dev_env.py tart create` and record the helper output.
- If the VM is running but commands cannot find the checkout, rerun `.claude/skills/frb-dev-env/frb_dev_env.py tart upload` and record the mounted path and VM-local path.
- If no iOS simulator is available, record `xcrun simctl list devices available` and `xcrun simctl list runtimes`, then mark the run blocked.
- If the simulator is already booted, `xcrun simctl boot <UDID>` may report that state; continue to `bootstatus` and record the message.
- If a generated file changes, record the exact `git status --short` output and inspect whether the tested command intentionally regenerated it.

## Cleanup

The per-worktree Tart VM may be kept for future local FRB Apple-platform development. Shutdown the selected simulator if it should not remain booted.

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- bash -lc 'xcrun simctl shutdown <UDID> || true'
git status --short
.claude/skills/frb-dev-env/frb_dev_env.py tart info
```

If cleanup requires stopping or deleting the VM, run:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart stop
.claude/skills/frb-dev-env/frb_dev_env.py tart delete
```

## Future Automation

The command sequence can be automated on a dedicated Apple host with Tart and a prepared base VM. Keep this as a manual report while VM lifecycle, simulator selection, host storage, and Xcode runtime availability remain machine-specific.
