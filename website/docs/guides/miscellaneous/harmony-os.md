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

CargoKit is the backend covered by the repository's OHOS CI baseline. With an
OHOS-enabled Flutter SDK, the Native Assets backend can also be used through a
community-maintained hooks package; see
[Native Assets (community-maintained)](#native-assets-community-maintained).
That path is neither maintained nor release-gated by flutter_rust_bridge, so
prefer CargoKit for production builds until the OHOS toolchain is upstreamed.

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

The OHOS Flutter fork may ship an older Dart SDK than the repository's internal
development tools. Keep the OHOS Flutter executable first on `PATH` for Flutter
builds, and put a Dart 3.13 or newer executable directory earlier on `PATH`
than the fork's `bin` directory when running `frb_internal` commands.

When DevEco Studio is installed on macOS, use its bundled Hvigor wrapper and
Node runtime for Hvigor and `ohpm`. Put the matching Hvigor directory before a
standalone command-line-tools installation; mixing Hvigor releases can fail
with errors such as `WithParamReplacement is not a function`. Adjust these
paths for your installation:

```shell
export HVIGOR_HOME=/Applications/DevEco-Studio.app/Contents/tools/hvigor
export NODE_HOME=/Applications/DevEco-Studio.app/Contents/tools/node
export PATH="$HVIGOR_HOME/bin:$NODE_HOME/bin:$PATH"
command -v hvigorw
hvigorw --version
```

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

## Native Assets (community-maintained)

:::caution
This recipe is contributed by the community. It is not maintained, reviewed, or
covered by flutter_rust_bridge CI. It replaces the standard Native Assets hooks
package with a personal fork of the standard hooks package maintained by
[@star4277](https://github.com/star4277), and build hooks execute third-party
code during `flutter build` and `flutter run`. Review the pinned sources before
using them, and prefer CargoKit for production builds until the OHOS toolchain
is upstreamed. Upstream discussion:
[#3524](https://github.com/fzyzcjy/flutter_rust_bridge/issues/3524).
:::

The flutter_rust_bridge generator does not perform the steps below
automatically.

### Verified versions

The fork was validated with the following versions:

| Component | Verified value |
| --- | --- |
| flutter_rust_bridge | `2.14.0-beta.2` |
| Hooks package | Fork of `flutter_rust_bridge_hooks` at `2.14.0-beta.2` |
| OHOS Flutter | `3.47.7+ohos-0.0.1-pre`, revision `650641656f`, Dart `3.13.5` |
| Rust | `1.93.1` |

Other combinations can work, but are not verified. The fork is a snapshot of
the hooks package; later flutter_rust_bridge releases may change the hook API.
Check the
[fork README](https://github.com/star4277/native_toolchain_rust_ohos/blob/main/frb_hooks_ohos/README.md)
for newer snapshots and setup notes.

### 1. Generate the Native Assets project

Run the normal Native Assets setup on the OHOS Flutter SDK. For an application:

```shell
flutter_rust_bridge_codegen create my_app \
  --integration-backend native-assets \
  --platforms ohos \
  --skip-fvm-install
```

For an existing project, run the matching `integrate` command with the same
flags. For a plugin, omit `--platforms` in both commands: the `package_ffi`
template does not accept platform filtering. The generated `hook/build.dart`
already imports `flutter_rust_bridge_hooks`; the remaining steps only change
where that package and its build dependencies come from. If the project already
had a `hook/build.dart`, confirm that it calls
`FlutterRustBridgeNativeAssetsBuilder` as described in
[Native assets](../../manual/integrate/03-native-assets.md).

Re-running `create` or `integrate` with this version of flutter_rust_bridge
restores the hosted hooks dependency. Re-apply steps 2 to 4 afterwards.

### 2. Replace the hooks dependency

In the package that contains `hook/build.dart` (the application, or the plugin
package), replace the existing `flutter_rust_bridge_hooks` entry with a Git
dependency pinned to a commit:

```yaml
dependencies:
  flutter_rust_bridge_hooks:
    git:
      url: https://github.com/star4277/native_toolchain_rust_ohos.git
      path: frb_hooks_ohos
      ref: 93d6e1b8cc939b50d1849124683f34b23a3bfd49
```

Do not replace the commit with `main` or another branch. Build hooks are
executed, so a mutable ref lets later commits change what your build runs.

### 3. Add the overrides at the consumer root

`hooks` and `code_assets` must come from the OHOS-compatible fork, and pub
resolves one source per package. Add all three entries to the
`dependency_overrides` section of the package graph root: the application root,
or the Pub workspace root. Do not put them in a workspace member.

```yaml
dependency_overrides:
  hooks:
    git:
      url: https://gitcode.com/CPF-Flutter/fluttertpc_dart_native.git
      path: pkgs/hooks
      ref: 124484c19678e7fbc21de3ef6726a5e6bd51832e
  code_assets:
    git:
      url: https://gitcode.com/CPF-Flutter/fluttertpc_dart_native.git
      path: pkgs/code_assets
      ref: 124484c19678e7fbc21de3ef6726a5e6bd51832e
  flutter_rust_bridge_hooks:
    git:
      url: https://github.com/star4277/native_toolchain_rust_ohos.git
      path: frb_hooks_ohos
      ref: 93d6e1b8cc939b50d1849124683f34b23a3bfd49
```

The `flutter_rust_bridge_hooks` override is required when a workspace member
already depends on a hosted or local copy of that package. In a plain
application it is harmless.

A plugin's `example/` application resolves independently. Add the same three
overrides to `example/pubspec.yaml`, or to the `example` workspace root when
the example belongs to a workspace.

Re-resolve dependencies:

```shell
flutter pub get
```

If the lockfile still resolves the hosted hooks package, force the source
change:

```shell
flutter pub upgrade flutter_rust_bridge_hooks native_toolchain_rust
```

### 4. Add the OHOS Rust targets

In the `rust-toolchain.toml` next to the Rust crate (the directory configured
as `rust_crate_dir`), add these entries to the existing `targets` array and
keep the existing `channel` and other settings:

```toml
"aarch64-unknown-linux-ohos",
"x86_64-unknown-linux-ohos",
"armv7-unknown-linux-ohos",
```

### 5. Build

```shell
flutter build hap --debug
```

The fork discovers the native SDK from Flutter's C compiler configuration when
`OHOS_SDK_HOME` is filtered out of the build hook environment. If the variable
is available it must point to the native SDK directory, as described in
[`OHOS_SDK_HOME`](#ohos_sdk_home) above.

### Restoring the standard setup

Switch back to a standard Flutter SDK with these steps:

- remove the three `dependency_overrides` entries and the Git
  `flutter_rust_bridge_hooks` dependency from the package and its example;
- restore the hosted `flutter_rust_bridge_hooks` dependency for your
  flutter_rust_bridge version;
- remove the three OHOS entries from the `rust-toolchain.toml` `targets` array;
- run `flutter pub get` and rebuild.

### Troubleshooting

- **Version solving mentions `flutter_rust_bridge_hooks` from git and from
  hosted**: an override is in a workspace member instead of the workspace root.
  Move it to the root and remove the member entry.
- **Hook or `native_toolchain_rust` errors after upgrading
  flutter_rust_bridge**: the fork is a snapshot of `2.14.0-beta.2`. Use that
  version, or check the fork README for a newer snapshot.
- **`dart pub publish` warns about overridden dependencies**: the overrides are
  for local builds. Keep `publish_to: none` for applications and review the
  warning before publishing a plugin.
- **Fetch failures**: both `github.com` and `gitcode.com` must be reachable.

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

The dedicated entrypoint emits `FRB_OHOS_SMOKE_RESULT=PASS` after completing a
synchronous Dart-to-Rust string call. For a broader device test fixture, pass
its deterministic marker with `--expected-log`. The command also checks that
`--bundle` matches the HAP metadata before installation. Logs are saved under
`target/ohos_device_smoke` by default. Use a dedicated bundle name
whose application is not already installed on the device; the command
intentionally aborts instead of overwriting user application data. To use a
dedicated name, change `bundleName` in `ohos/AppScope/app.json5` before building
and pass exactly the same value to `--bundle`.

The command does not create or manage signing credentials. Configure a debug
signature in DevEco Studio or sign the HAP through your existing secure signing
workflow before running it. Do not commit `.p12`, provision profiles, passwords,
or generated signing configuration.

The build gate defaults to `ohos-arm64` and checks the matching `arm64-v8a`
Rust library. To build another supported target, set
`FRB_OHOS_TARGET_PLATFORM` before running the repository build command:

```shell
FRB_OHOS_TARGET_PLATFORM=ohos-x64 ./frb_internal build-flutter \
  --package frb_example/flutter_via_create \
  --target ohos
```

The supported target mapping is `ohos-arm64`/`arm64-v8a` and
`ohos-x64`/`x86_64`. The current HarmonyOS Hvigor toolchain rejects the
`armeabi-v7a` ABI, so `ohos-arm` is rejected before Flutter starts instead of
failing late during HAP configuration. Keep the existing CMake mapping only as
forward-compatible scaffolding for a future toolchain that supports armv7.

The build gate uses release mode by default. To validate the same packaging
checks against a different Flutter mode, set `FRB_OHOS_BUILD_MODE` to `debug`,
`profile`, or `release`:

```shell
FRB_OHOS_BUILD_MODE=profile ./frb_internal build-flutter \
  --package frb_example/flutter_via_create \
  --target ohos
```

The mode value is validated before Flutter starts so a typo cannot silently
produce a different artifact.

## HarmonyOS PC and OpenHarmony device types

Some Flutter OHOS templates generate an entry module that only declares the
`phone` device type. On an OpenHarmony PC or 2-in-1 SDK this can produce an empty
system-capability intersection during the Hvigor build. The checked-in
quickstart entry module includes the broad `default` and `tablet` types. For an
existing application, update the application entry module's `deviceTypes` for
the target product when the generated `phone` type is unsupported, for example:

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
