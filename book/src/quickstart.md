# Quickstart

## Show me the code

Write down Rust functions and types normally:

```rust,ignore
// A normal Rust function ...
pub fn my_function(a: TreeNode, b: MyEnum) -> Result<Vec<u8>> { /* ... */ }

// ... with rich types
pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode> }
pub enum MyEnum { Hello(String), World(bool) }
```

With bindings automatically generated, use it seamlessly in Flutter/Dart:

```dart
var output = await api.myFunction(
    TreeNode(value: "root", ...),
    MyEnumHello("tom"));
```

<sub>PS. [Why](feature/async_dart.md) `await` in Flutter</sub> 

## Execute by yourself

This section assumes you have knowledge for Flutter, Rust, FFI and so on,
and thus is extremely abbreviated.
For more in-depth guides, see [Create new projects from a template](template.md)
or [Integrating with existing projects](integrate.md).

### Install

* Install `cbindgen`: `cargo install cbindgen` <sub>(may [need latest version](https://github.com/fzyzcjy/flutter_rust_bridge/issues/53#issuecomment-939588321), thanks @gmorenz)</sub>
* Install `ffigen`:  `dart pub global activate ffigen`, and [install LLVM](https://pub.dev/packages/ffigen#installing-llvm).
* Install the codegen program: `cargo install flutter_rust_bridge_codegen`.
* Add `flutter_rust_bridge = "1.0"` (where `1.0` should be the latest version) to Rust's `Cargo.toml`.
* Add `flutter_rust_bridge: ^1.0` (same as above, should be latest version) to Flutter/Dart's `pubspec.yaml` under the section of `dependencies`.

### Run

```bash
flutter_rust_bridge_codegen \
    --rust-input path/to/api.rs \
    --dart-output path/to/bridge_generated.dart
```

### Enjoy

Use the class in the generated `.dart` file, as if it is a normal Flutter/Dart class.
