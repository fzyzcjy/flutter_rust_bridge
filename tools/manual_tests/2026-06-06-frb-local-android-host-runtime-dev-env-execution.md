# Manual Test Execution

## Test

- Manual test: `tools/manual_tests/frb-local-android-host-runtime-dev-env.md`
- Purpose: Verify that FRB/Flutter Android runtime validation can run inside the per-worktree Docker container against a host-managed Android Emulator.
- Source context: PR #3180 Android host runtime development environment workflow.

## Run

- Date and timezone: `2026-06-06 13:50 CST`
- Executor: Codex agent
- Commit or release tested: `69651aa9c0` plus local PR changes for `--android-emulator-adb`, Docker arm64 x86_64 Android SDK runtime libraries, and Android example Gradle fixes.
- Environment: macOS 26.4 build 25E246 on arm64; Docker container `frb-03f4cd3355c1` from `fzyzcjy/flutter_rust_bridge_dev:latest`; host Android Emulator `emulator-5554` with Android 15 API 35, model `sdk_gphone64_arm64`, device `emu64a`.

## Procedure

- Followed report revision: local working tree revision of `tools/manual_tests/frb-local-android-host-runtime-dev-env.md` after updating the emulator path to `--android-emulator-adb`.
- Deviations from written steps: Used only the host Android Emulator path. The physical-device host ADB server path was not executed.
- Cleanup completed: yes; the Flutter test uninstalled `com.example.flutter_via_create` during teardown. The reusable Docker container and host SDK/AVD caches were intentionally kept.

## Result

- Status: pass
- Summary: Docker-local ADB connected to `host.docker.internal:5555`, Flutter detected the emulator as an Android device, installed the APK, forwarded the VM service port inside Docker, and completed `integration_test/simple_test.dart` with `All tests passed!`.

## Artifacts

- Terminal log: `/private/tmp/frb-android-host-runtime-manual-test-20260606-1350.log`
- Screenshot or recording: not applicable
- Generated artifacts: not applicable
- Other evidence: host `adb devices -l` showed `emulator-5554 device`; Docker `adb devices -l` showed `host.docker.internal:5555 device`.

## Follow-up

- Required follow-up: none
- Notes: The earlier host ADB server approach can list emulator devices from Docker, but full Flutter tests fail because Flutter's dynamic VM service `adb forward` binds on the host while Flutter runs in Docker. Use `--android-emulator-adb` for host emulator runtime tests.
