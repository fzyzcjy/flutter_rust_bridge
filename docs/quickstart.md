# Quickstart

### Install

* Install dependency `cbindgen`: `cargo install cbindgen` <sub>(may [need latest version](https://github.com/fzyzcjy/flutter_rust_bridge/issues/53#issuecomment-939588321), thanks @gmorenz)</sub>
* Install dependency `ffigen`:  `dart pub global activate ffigen`, and [install LLVM](https://pub.dev/packages/ffigen#installing-llvm). If you are running macOS, check below under MacOS Caveats for instructions on how to install and use llvm.
* Install this code generator binary by `cargo install flutter_rust_bridge_codegen`.
* Add `flutter_rust_bridge = "1.0"` (where `1.0` should be the latest version) to Rust's `Cargo.toml`.
* Add `flutter_rust_bridge: ^1.0` (same as above, should be latest version) to Flutter/Dart's `pubspec.yaml` under the section of `dependencies`.

### Run

```shell
flutter_rust_bridge_codegen --rust-input path/to/your/api.rs --dart-output path/to/file/being/bridge_generated.dart
```

If you have problems (such as failure on MacOS), please see the ["Troubleshooting"](https://github.com/fzyzcjy/flutter_rust_bridge#troubleshooting) section below.

(For more options, use `--help`; To see what types and function signatures can you write in Rust, have a look at [this example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs).) (For Windows, you may need `\\` instead of `/` for paths.)

### Enjoy

Use the class in the generated `.dart` file, as if it is a normal Flutter/Dart class! (The abstract class at the top of the generated file.)

Want to see a Flutter tutorial with UI? See [the tutorial section below](https://github.com/fzyzcjy/flutter_rust_bridge#-tutorial-a-flutterrust-app). Want pure-Dart example? [Here is](https://github.com/fzyzcjy/flutter_rust_bridge#-tutorial-pure-dart) another tutorial.

<sub>**Remark**: If you are interested, why `abstract`class can be used - it is because of the [factory](https://dart.dev/guides/language/language-tour#factory-constructors) language feature.</sub>