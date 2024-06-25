# Cargokit

flutter_rust_bridge uses Cargokit for seamless integration of cargo build
with flutter applications and packages.

## Cargokit Details

Please refer to its documentation for how to use it.
Its GitHub repository is https://github.com/irondash/cargokit.
In addition, it has a companion blog about how to integrate it at https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/.

The following links may also be useful for customizations:

* Configuration: https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit
* Use precompiled binaries (instead of default compile-on-the-fly): https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md

[It seems that](https://github.com/irondash/cargokit/issues/39#issuecomment-1831584430),
after Dart [native assets](native-assets) is stablized,
cargokit will also utilize it.