# OHOS real-device smoke test

## Purpose

Verify that a signed arm64 HAP produced from the CargoKit create example can be
installed on an OHOS/HarmonyOS device, start `EntryAbility`, load the packaged
Rust library, complete a Dart-to-Rust call, and clean up the test application.

## Source

- Context: [flutter_rust_bridge PR #3352](https://github.com/fzyzcjy/flutter_rust_bridge/pull/3352)
- Related docs or skills: `website/docs/guides/miscellaneous/harmony-os.md`,
  `.claude/skills/frb-manual-test/SKILL.md`

## When To Run

Run after changing OHOS HAP packaging, CargoKit OHOS integration, the
`ohos-device-smoke` command, the OHOS Flutter toolchain, or the dedicated smoke
entrypoint. Run before claiming real-device coverage in a PR or release.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: dependencies resolved and the dedicated entrypoint
  present at
  `frb_example/flutter_via_create/ohos/ohos_device_smoke_main.dart`.
- Required credentials or account state: local debug signing capability; never
  commit the keystore, certificate, provisioning profile, or passwords.
- Required device or simulator state: exactly one target visible through `hdc`,
  developer mode enabled, and the HAP bundle not already installed.

## Environment

- OS: macOS, Linux, or Windows supported by the selected OHOS Flutter SDK.
- Flutter: an OHOS-enabled Flutter version compatible with this checkout.
- Dart: the version bundled with that Flutter SDK.
- Rust: stable with the `aarch64-unknown-linux-ohos` target installed.
- Device or simulator: arm64 OHOS/HarmonyOS device reachable through `hdc`.
- Browser or external service: not required.

## Preparation

```bash
git submodule update --init --recursive
hdc list targets
rustup target add aarch64-unknown-linux-ohos
```

Set `OHOS_SDK_HOME` to the installed native SDK directory. Configure a dedicated
bundle name in `frb_example/flutter_via_create/ohos/AppScope/app.json5`, then
configure local debug signing without committing the signing material.

## Test Data

- Input: the signed debug HAP built from `frb_example/flutter_via_create`.
- Reset: uninstall the dedicated bundle before the run if it belongs to an
  earlier test; never remove an unrelated or user-owned application.

## Steps

1. Build the dedicated smoke entrypoint.

   ```bash
   cd frb_example/flutter_via_create
   flutter build hap --debug \
     --target ohos/ohos_device_smoke_main.dart
   ```

2. Confirm the HAP contains the expected arm64 Rust library and note the signed
   HAP path and configured bundle name.

   ```bash
   unzip -l /absolute/path/to/entry-default-signed.hap | \
     grep 'libs/arm64-v8a/librust_lib_flutter_via_create.so'
   ```

3. From the repository root, execute the device smoke command with the exact
   bundle recorded in the HAP metadata.

   ```bash
   ./frb_internal ohos-device-smoke \
     --hap /absolute/path/to/entry-default-signed.hap \
     --bundle com.example.frb_ohos_smoke \
     --ability EntryAbility
   ```

4. Query installed bundles and confirm the dedicated bundle was removed.

   ```bash
   bundles="$(hdc shell bm dump -a)" || exit 1
   if printf '%s\n' "$bundles" | grep -Fq 'com.example.frb_ohos_smoke'; then
     echo 'bundle still installed' >&2
     exit 1
   fi
   ```

## Expected Result

The build and smoke commands exit zero, the HAP contains the Rust shared
library, the device log contains the exact current-launch marker, and cleanup
removes the dedicated bundle.

```text
OHOS device smoke passed: ... marker=FRB_OHOS_SMOKE_RESULT=PASS ...
```

## Failure Criteria

The test fails or is blocked if any of the following occurs:

- The device cannot be uniquely selected.
- The HAP bundle differs from `--bundle` or is already installed.
- HAP installation, ability startup, Rust library loading, or marker detection
  fails.
- The marker comes only from logs retained before the current process launch.
- The cleanup command fails or leaves the test bundle installed.

## Results To Capture

- Full terminal log for build, archive inspection, smoke, and cleanup commands.
- `flutter --version`, `dart --version`, `rustc --version`, and device ID.
- Signed HAP path and SHA-256 digest; do not upload signing credentials.
- Saved hilog file from `target/ohos_device_smoke`.
- A device screenshot is optional because the deterministic log marker is the
  pass criterion.

## Troubleshooting

- If Flutter reports an SDK component missing, verify that the configured
  `runtimeOS`, compatible SDK, native SDK, and device type exist locally.
- If Dart dependency resolution fails, use the repository-supported Flutter
  version; do not commit a lowered SDK constraint.
- If installation is refused, verify the HAP metadata and use a dedicated
  bundle rather than enabling replacement.
- If no device is available, record `hdc list targets` and mark the run blocked.

## Cleanup

The smoke command uninstalls only the bundle it installed. After an install
failure, it queries the device again and removes the bundle if it appeared.
Restore any temporary SDK, bundle, or signing configuration and verify the
checkout afterwards.

```bash
git status --short
```

## Future Automation

Move this test to a dedicated arm64 OHOS/HarmonyOS device runner once secure HAP
signing and exclusive device allocation are available in CI.
