# Wrapping up

Congratulations! You have successfully added a Rust component to
your Flutter app using `flutter_rust_bridge` and configured
`flutter run` (more on web later) to build your Rust library and link it to the app.

As a reminder, you need to run these commands every time your Rust code changes *and*
before you run `flutter run`:

```bash
{{#include command.sh.txt}}
```

## Renaming the Rust bridge module

If you would like to use the `--rust-output` flag of `flutter_rust_bridge_codegen`,
keep in mind that you will have to update `$crate/src/lib.rs` to point to the correct
file, for example if you use this command instead:

```bash
flutter_rust_bridge_codegen \
    ..
    --rust-output $crate/src/my_bridge.rs
```

then you need to modify this in `lib.rs`:

```diff
- mod bridge_generated;
+ mod my_bridge;
```

## `flutter_rust_bridge_serve`-less workflows

If you don't need to run Flutter Web in development
mode and would rather build in release mode once
in a while, [read here](../build_wasm.md) for instructions on how to build your WASM
binary without `flutter_rust_bridge_serve`.