# Tutorial: Pure Dart

**Remark**: The `valgrind_test` section of the [CI workflow](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/test.yaml) can also be useful, if you want details of each command and want to see Valgrind configuration.

Unlike the previous tutorial, this one integrates Rust with pure Dart instead of Flutter.

## Get example code

Please [install Dart](https://dart.dev/get-dart), [install Rust](https://www.rust-lang.org/learn/get-started), and have some familiarity with them. Then run `git clone https://github.com/fzyzcjy/flutter_rust_bridge`, and my example is in `frb_example/pure_dart`.

## (Optional) Manually run code generator

Remark: Bridge is automatically generated upon running `cargo build` using build-script in build.rs file, so this step is optional. Even if you do it, you should not see anything changed.

Install it: `cargo install flutter_rust_bridge_codegen`.

Run it: `flutter_rust_bridge_codegen --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart` (See [CI workflow](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/codegen.yml) as a reference.) (For Windows, you may need `\\` instead of `/` for paths.)

## Run "Dart+Rust" app

You may run `frb_example/pure_dart/dart/lib/main.dart` as a normal Dart program, except that you should provide the dynamic linked library of the Rust code (for simplicity, here I only demonstrate the approach for dynamic linked library, but you can for sure use other methods). The detailed steps are as follows.

Run `cargo build` in `frb_example/pure_dart/rust` to build the Rust code into a `.so` file. Then run `dart frb_example/pure_dart/dart/lib/main.dart frb_example/pure_dart/rust/target/debug/libflutter_rust_bridge_example_pure_dart.so` to run the Dart program with Rust `.so` file. (If you have problems, see "Troubleshooting" section.)  (If on MacOS, Rust may indeed generate `.dylib`, so change the last command to use `...dylib` instead of `...so`,)

P.S. You will only see some tests passing - no fancy UI or functionality in this example.
