# flutter_rust_bridge_codegen create/integrate command

As is seen in the [overview](overview) and [quickstart](../../quickstart),
this command automatically add Cargokit and flutter_rust_bridge-needed code into any Flutter project you specify.

Remark: Currently, the Cargokit source code has to be copied into the target repository,
as is [suggested officially](https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/).

## Platform selection

The `create` and `integrate` commands accept `--platforms <PLATFORMS>` to control which Flutter platforms should receive scaffold. When `--platforms` is omitted, flutter_rust_bridge follows the current Flutter toolchain: ordinary upstream Flutter projects do not receive HarmonyOS/OHOS scaffold, while OHOS-enabled Flutter toolchains may receive OHOS scaffold automatically.

To force OHOS scaffold even when generating on a non-OHOS Flutter toolchain, pass a platform list containing `ohos`, for example `--platforms android,ios,linux,macos,windows,ohos`.

:::info
When Dart's [native assets](native-assets) language feature is stabilized,
we may also utilize that approach.
:::
