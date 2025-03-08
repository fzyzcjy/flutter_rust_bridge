# Precompiled Rust Binaries

By default, flutter_rust_bridge uses Cargokit to do Rust compilation.
Its default mode is to compile when the users build their apps.

However, if we want to ship pre-compiled Rust binaries in the Flutter package,
this can be done by following https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md.
