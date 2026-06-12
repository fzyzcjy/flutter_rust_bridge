# Cargokit

flutter_rust_bridge uses Cargokit for seamless integration of cargo build
with flutter applications and packages.

Cargokit remains the default `flutter_rust_bridge_codegen create/integrate` backend for compatibility with existing projects and older Flutter/Dart SDKs.
The [Native Assets](native-assets) backend is available through `--integration-backend native-assets`.

## Cargokit Details

Please refer to its documentation for how to use it.
The original GitHub repository is https://github.com/irondash/cargokit.
That repository has been archived; flutter_rust_bridge now uses the fork maintained by the flutter_rust_bridge team for template and submodule updates.
In addition, it has a companion blog about how to integrate it at https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/.

The following links may also be useful for customizations:

* Configuration: https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit
* Use precompiled binaries (instead of default compile-on-the-fly): https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md

For new projects that can require a recent Flutter/Dart SDK, consider the [Native Assets](native-assets) backend.
