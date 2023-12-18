# flutter_rust_bridge_codegen create/integrate command

As is seen in the [overview](overview) and [quickstart](../../quickstart),
this command automatically add Cargokit and flutter_rust_bridge-needed code into any Flutter project you specify.

Remark: Currently, the Cargokit source code has to be copied into the target repository,
as is [suggested officially](https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/).

:::info
When Dart's [native assets](native-assets) language feature is stabilized,
we may also utilize that approach.
:::
