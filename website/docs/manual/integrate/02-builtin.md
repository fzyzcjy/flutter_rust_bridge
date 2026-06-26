# flutter_rust_bridge_codegen create/integrate command

As is seen in the [overview](overview) and [quickstart](../../quickstart),
this command automatically adds the selected integration backend and flutter_rust_bridge-needed code into any Flutter project you specify.

Remark: Currently, the Cargokit source code has to be copied into the target repository,
as is [suggested officially](https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/).

## Integration backend

The `create` and `integrate` commands accept `--integration-backend <BACKEND>`.
The default backend is `cargokit`.
The `native-assets` backend uses Dart/Flutter build hooks and `flutter_rust_bridge_hooks` to build Rust as a bundled code asset.

```bash
flutter_rust_bridge_codegen create my_app --integration-backend cargokit
flutter_rust_bridge_codegen create my_app --integration-backend native-assets
flutter_rust_bridge_codegen integrate --integration-backend native-assets
```

Use `cargokit` for the current default behavior and for projects that need older Flutter/Dart versions.
Use `native-assets` only with a Flutter/Dart SDK that supports build hooks and code assets.

## Platform selection

The `create` and `integrate` commands accept `--platforms <PLATFORMS>` to control which Flutter platforms should receive scaffold. When `--platforms` is omitted, flutter_rust_bridge follows the current Flutter toolchain: ordinary upstream Flutter projects do not receive HarmonyOS/OHOS scaffold, while OHOS-enabled Flutter toolchains may receive OHOS scaffold automatically.

To force OHOS scaffold even when generating on a non-OHOS Flutter toolchain, pass a platform list containing `ohos`, for example `--platforms android,ios,linux,macos,windows,ohos`.

:::info
The `native-assets` backend is available through `--integration-backend native-assets`.
:::
