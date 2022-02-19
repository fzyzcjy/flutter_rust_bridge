# Wrapping up

Congratulations! You have successfully added a Rust component to your Flutter app
using `flutter_rust_bridge` and configured `flutter run` to build your Rust library
and link it to the app.

As a reminder, you need to run these commands every time your Rust code changes *and*
before you run `flutter run`:

```bash
{{#include command.sh.txt}}
# if using Dart codegen
flutter pub run build_runner build
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
-mod bridge_generated;
+mod my_bridge;
```
