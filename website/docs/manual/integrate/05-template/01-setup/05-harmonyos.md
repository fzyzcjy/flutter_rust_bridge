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

## `OHOS_SDK_HOME`

In addition to the variables required by the OpenHarmony Flutter guide,
flutter_rust_bridge needs `OHOS_SDK_HOME` when CargoKit cross-compiles Rust code
for OHOS. Set it to the SDK's `native` directory, not to the SDK root.

For example, on Windows:

```powershell
[Environment]::SetEnvironmentVariable("OHOS_SDK_HOME", "D:\Huawei\SDK\18\native", "User")
$env:OHOS_SDK_HOME = "D:\Huawei\SDK\18\native"
```

On macOS or Linux:

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

## Creating an OHOS project

After the HarmonyOS toolchain is ready, create the project with OHOS enabled:

```shell
flutter_rust_bridge_codegen create my_app --platforms ohos
```

If you also want the usual Flutter mobile platforms, include them explicitly:

```shell
flutter_rust_bridge_codegen create my_app --platforms android,ios,ohos
```

Then build or run the generated Flutter project with the OHOS Flutter toolchain,
for example:

```shell
cd my_app
flutter build hap --debug
```

If the Rust build fails with an OHOS SDK error, check `OHOS_SDK_HOME` first. It
must point to the `native` SDK directory and the full path must not contain
Chinese characters or whitespace.
