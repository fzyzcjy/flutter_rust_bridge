# Quickstart

## 🧭 Show me the code

What you write down (in Rust):

```rust
pub fn my_function(a: MyTreeNode, b: SomeOtherStruct) -> Result<Vec<u8>> {
    ... do my heavy computations ...
}

// you can use structs (even recursive)
pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode> }
```

With bindings automatically generated, you can simply use the following API in Flutter/Dart. Nothing more.

```dart
Future<Uint8List> myFunction(MyTreeNode a, SomeOtherStruct b);
```

<sub>**Remark**: Why `Future` in Flutter: Flutter is single-threaded. If not using future, just like what you do with plain-old Flutter bindings, your UI will be *stuck* as long as your Rust code is executing. If your Rust code run for a second, your UI will fully freeze for one second.</sub> 

## Execute by yourself

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

## See more

Want to see more? Have a look at the next section for quickstarts or tutorials.