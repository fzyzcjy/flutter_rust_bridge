# HarmonyOS setup

Before creating or building an OHOS project, set up the OpenHarmony Flutter
toolchain. The full environment guide is maintained by the OpenHarmony Flutter
team:

- [OpenHarmony Flutter environment setup, English](https://gitcode.com/CPF-Flutter/flutter_samples/blob/master/ohos/docs/03_environment/OpenHarmony-flutter-environment-setup.md)
- [OpenHarmony Flutter environment setup, Chinese](https://gitcode.com/CPF-Flutter/flutter_samples/blob/master/ohos/docs/03_environment/OpenHarmony-flutter%E7%8E%AF%E5%A2%83%E6%90%AD%E5%BB%BA%E6%8C%87%E5%AF%BC.md)

Follow that guide to install DevEco Studio, JDK 17, the OHOS-enabled Flutter
SDK, the OpenHarmony SDK, and the required `ohpm`, `hvigor`, `node`, and `hdc`
tools. After configuring the toolchain, run:

```shell
flutter doctor -v
```

Both Flutter and OpenHarmony should be reported as available. If the doctor
output reports a missing OpenHarmony component, finish the corresponding setup
step from the environment guide before continuing.

## Supported baseline

The repository's CI baseline is:

| Component | Supported baseline |
|---|---|
| Flutter OHOS | branch `oh-3.41.9-dev`, commit `6d7e5b43fb43bb85ba0a59e3469299ebcf45a637` |
| HarmonyOS command-line tools | `6.1.1.280` |
| JDK | 17 |
| Rust | stable; CI currently uses `1.93.1` |
| Integration backend | CargoKit |
| Required ABI/build mode | arm64 release (`ohos-arm64` / `arm64-v8a`) |

The CI workflow pins these values instead of following the latest OHOS SDK or
Flutter fork automatically. Other toolchain combinations can work, but are not
release-gated yet. x86_64, armv7, profile, Native Assets, plugin consumption,
and CI-controlled real-device execution remain experimental or unverified until
their test matrix is added. A dedicated test entrypoint provides a deterministic
synchronous Dart-to-Rust device smoke marker; broader FRB API coverage is not
yet a release gate.

## `OHOS_SDK_HOME`

In addition to the variables required by the OpenHarmony Flutter guide,
flutter_rust_bridge needs `OHOS_SDK_HOME` when CargoKit cross-compiles Rust code
for OHOS. Set it to the SDK's `native` directory, not to the SDK root.

On Windows, add or update an environment variable named `OHOS_SDK_HOME`.
It can be either a user environment variable or a system environment variable,
as long as the terminal and IDE used for building can read it. Its value should
be the SDK `native` directory, for example
`D:\Huawei\SDK\18\native`.

On macOS or Linux, add this to your shell profile, such as `~/.bashrc`,
`~/.zshrc`, or `~/.profile`, so the variable persists after restarting the
terminal:

```shell
export OHOS_SDK_HOME=/opt/Huawei/SDK/18/native
```

The `OHOS_SDK_HOME` path must not contain Chinese characters, spaces, or other
whitespace characters. Otherwise the Rust build invoked by CargoKit may fail
before or during native compilation.

This is especially important when using the SDK bundled with DevEco Studio:
some default DevEco Studio installation paths contain spaces, and the bundled
SDK inherits that path. Install or move DevEco Studio and the SDK to a directory
whose full path is plain ASCII without whitespace, or configure DevEco Studio to
use an SDK directory such as `D:\Huawei\SDK\18\native`.

Restart your terminal and IDE after changing the environment variable, then
confirm the value before building:

```shell
echo $OHOS_SDK_HOME
```

On Windows PowerShell, use:

```powershell
echo $env:OHOS_SDK_HOME
```

The repository's OHOS build gate runs an environment preflight before invoking
Flutter. It rejects a missing or unsafe `OHOS_SDK_HOME`, checks for
`llvm/bin/clang`, `llvm/bin/llvm-ar`, and `sysroot`, confirms that the active
Flutter SDK advertises the `ohos` platform, and verifies that JDK's `jar` tool is
available or that `unzip` can be used as a HAP inspection fallback. The
successful preflight log also prints the Flutter target, Rust target, HAP ABI,
and selected native SDK path.

You can perform the equivalent core checks locally before a build:

```shell
flutter create --help | grep -w ohos
test -x "$OHOS_SDK_HOME/llvm/bin/clang"
test -x "$OHOS_SDK_HOME/llvm/bin/llvm-ar"
test -d "$OHOS_SDK_HOME/sysroot"
jar --version || unzip -v
```

On Windows, check for `clang.exe` and `llvm-ar.exe` in PowerShell or File
Explorer instead of using the POSIX `test` commands.

## Creating an OHOS project

If your project uses FVM to manage Flutter versions, always pass
`--skip-fvm-install` when running flutter_rust_bridge commands for an OHOS
project.

HarmonyOS/OpenHarmony Flutter is a Huawei community fork of Flutter. Official
Flutter does not support OHOS, but FVM installs Flutter from the official
Flutter releases by default. Without `--skip-fvm-install`, FVM may try to fetch
an official Flutter SDK, which can get stuck or leave the OHOS project unable to
run.

After the HarmonyOS toolchain is ready, create the project with OHOS enabled:

```shell
flutter_rust_bridge_codegen create my_app --platforms ohos --skip-fvm-install
```

If you also want the usual Flutter mobile platforms, include them explicitly:

```shell
flutter_rust_bridge_codegen create my_app --platforms android,ios,ohos --skip-fvm-install
```

Use the same flag when integrating flutter_rust_bridge into an existing OHOS
project:

```shell
flutter_rust_bridge_codegen integrate --platforms ohos --skip-fvm-install
```

Then build or run the generated Flutter project with the OHOS Flutter toolchain,
for example:

```shell
cd my_app
flutter build hap --debug
```

To confirm that the Rust library was packaged into every HAP, inspect each
archive independently:

```bash
set -o pipefail
find build/ohos/hap -name '*.hap' -print0 |
  (
    hap_count=0
    while IFS= read -r -d '' hap; do
      jar tf "$hap" |
        grep 'libs/arm64-v8a/librust_lib_my_app\.so$' >/dev/null || {
        echo "Missing Rust library: $hap" >&2
        exit 1
      }
      ((hap_count += 1))
    done
    ((hap_count > 0)) || { echo 'No HAP files found' >&2; exit 1; }
  )
```

The command should succeed for every HAP. Each archive should contain the
library for your Rust crate, such as
`libs/arm64-v8a/librust_lib_my_app.so`. Cargo replaces hyphens in crate names
with underscores in the generated library filename; replace the example name in
the command when your crate uses a different name. A HAP that contains only
Flutter or system libraries cannot call the generated Rust bridge.

If the Rust build fails with an OHOS SDK error, check `OHOS_SDK_HOME` first. It
must point to the `native` SDK directory and the full path must not contain
Chinese characters or whitespace.

## Running the signed HAP smoke test on a device

The repository provides a repeatable device smoke command for a signed HAP.
The command refuses to replace an existing application, installs the HAP,
starts its ability, waits for a log marker that proves the Rust call completed,
saves the matching process logs, bounds every `hdc` invocation, and uninstalls
the test application during cleanup. If installation reports a failure, the
command checks whether the bundle nevertheless appeared before deciding whether
to uninstall it.

Build the quickstart with the dedicated smoke entrypoint, then sign the HAP
through your normal secure signing workflow:

```shell
cd frb_example/flutter_via_create
flutter build hap --debug \
  --target ohos/ohos_device_smoke_main.dart
```

Connect exactly one development device, confirm it is visible, and run:

```shell
hdc list targets
./frb_internal ohos-device-smoke \
  --hap /absolute/path/to/entry-default-signed.hap \
  --bundle com.example.flutter_via_create \
  --ability EntryAbility
```

When more than one device is connected, select one explicitly:

```shell
./frb_internal ohos-device-smoke \
  --hap /absolute/path/to/entry-default-signed.hap \
  --bundle com.example.flutter_via_create \
  --device-id DEVICE_UDID
```

The dedicated entrypoint emits `FRB_OHOS_SMOKE_RESULT=PASS` only after its
synchronous `greet` call returns the expected value. For a broader device test
fixture, pass its deterministic marker with `--expected-log`. The command also
checks that `--bundle` matches the HAP metadata before installation. Logs are
saved under `target/ohos_device_smoke` by default. Use a dedicated bundle name
whose application is not already installed on the device; the command
intentionally aborts instead of overwriting user application data. To use a
dedicated name, change `bundleName` in `ohos/AppScope/app.json5` before building
and pass exactly the same value to `--bundle`.

The command does not create or manage signing credentials. Configure a debug
signature in DevEco Studio or sign the HAP through your existing secure signing
workflow before running it. Do not commit `.p12`, provision profiles, passwords,
or generated signing configuration.

## HarmonyOS PC and OpenHarmony device types

Some Flutter OHOS templates generate an entry module that only declares the
`phone` device type. On an OpenHarmony PC or 2-in-1 SDK this can produce an empty
system-capability intersection during the Hvigor build. If the build reports
that `phone` is unsupported, update the application entry module's
`deviceTypes` for the target product, for example:

```json5
"deviceTypes": [
  "default"
]
```

Keep `runtimeOS`, `compatibleSdkVersion`, and the installed SDK component from
the same HarmonyOS or OpenHarmony toolchain family. Switching only `runtimeOS`
without matching SDK components can instead produce `SDK component missing`.
For an existing application, preserve its product-specific phone, tablet, PC,
and signing configuration rather than replacing the whole build profile.
