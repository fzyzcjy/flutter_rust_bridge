# High-level memory-safe bindgen for Flutter/Dart &lt;-> Rust
[![Documentation](https://docs.rs/dart_rust_bridge/badge.svg)](https://docs.rs/flutter_rust_bridge)
[![Rust Package](https://img.shields.io/crates/v/dart_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/dart_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Valgrind test for Dart+Rust](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/valgrind.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/valgrind.yml)
[![Linters](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml)
[![Rust build and test](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml)
[![Flutter integration test for Flutter+Rust](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/flutter.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/flutter.yaml)
[![Run codegen](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/run_codegen.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/run_codegen.yml)

![Logo](docs/logo.png)

## Advantages

* **Memory-safe**: Never need to think about malloc/free.
* **Type support**: Unlike low-level binding generator which only provide primitives and pointers, this package provides things like `Vec<u8>`(`Uint8List`), `Vec<T>`(`List<T>`), any custom `struct`(`class`)s, and even use recursive structs (e.g. a tree node).
* **Zero-copy**: Pass big array of bytes from Rust to Dart without any memory copies.
* **Async programming**: You can simply call functions directly in main isolate (thread) of Dart/Flutter, and Rust code will not block the Flutter UI.
* **Easy to use**: All you need to do is write down your Rust code. The code generator will do everything and expose an API in the Dart/Flutter style for you.
* **Lightweight**: Simulate how human beings will write down boilerplate code . Thus compared with 
* **Plug-and-play**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).
* **Easy to code-review**: If you want to convince yourself (or your team) that it is safe, there are not much code for you to look at. The runtime is only hundreds of loc, and the generated code follows simple patterns. (Of course, if you find a bug, tell me!)
* **Pure-Dart compatible:** Despite the name, this package is 100% compatible with pure Dart. It does not require anything specific to Flutter. See [this pure-Dart example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/README.md).

## Show me the code

A very simple example. What you write down (in Rust):

```Rust
pub fn my_function(a: MyTreeNode, b: SomeOtherStruct) -> Result<Vec<u8>> {
    ... do my heavy computations ...
}

// you can use structs (even recursive)
pub struct TreeNode { pub value: i32, pub children: Vec<MyTreeNode> }
```

We will generate the bindings. Then you only need to use the following generated API in Dart/Flutter. Nothing more. It looks exactly like a normal Dart/Flutter function.

```Dart
Future<Uint8List> myFunction(MyTreeNode a, SomeOtherStruct b);
```

<sub>**Remark**: Why `Future` in Flutter: Flutter is single-threaded. If not using future, just like what you do with plain-old Flutter bindings, your UI will be *stuck* as long as your Rust code is executing. If your Rust code run for a second, your UI will fully freeze for one second.</sub> 

## Quickstart

### Get example code

Please [install Flutter](https://flutter.dev/docs/get-started/install), [install Rust](https://www.rust-lang.org/learn/get-started), and have some familiarity with them. Then run `git clone https://github.com/fzyzcjy/flutter_rust_bridge && cd frb_example/with_flutter` to get my example.

Or you can use your own code (if you find this Quickstart section too brief, have a look at the later Usage section).

### (Optional) Run code generator

Remark: I have generated the source code already (in quickstart), so this step is optional. Even if you do it, you should not see anything changed.

Install it: `cargo install flutter_rust_bridge_codegen`.

Run it: `flutter_rust_bridge_codegen frb_example/with_flutter/rust/flutter_rust_bridge.yaml` (the argument is the path to the configuration file).

### Run "Flutter+Rust" app

#### If Android

Run `cargo ndk -o ../android/app/src/main/jniLibs build`. Then run the Flutter app normally as is taught in official tutorial. For example, `flutter run`.

#### If iOS

Modify `Cargo.toml` to change `cdylib` to `staticlib`. (Again, this is baremetal example so it is done manually. For your project, you can automate it.)

Run `cargo lipo && cp target/universal/debug/libflutter_rust_bridge_example.a ../ios/Runner` to build Rust and copy the static library. Then run the Flutter app normally as is taught in official tutorial. For example, `flutter run`.

Remark: Since my quickstart app is so baremetal, I do not integrate the Rust building process into Flutter building process (but definitely you can do that). 

### (Optional) See more types that this library can generate

Have a look at the function arguments and return types in this file: [api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs). With this library, we have a generated API that resides at [generated_api.dart](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/dart/lib/generated_api.dart) (of course, that is auto generated, and you can use it in other Dart code).

## Usage in details

### Configuration

`flutter_rust_bridge.yaml` is nothing but specifying where is your source code, and where do you want the generated code to be put in. We give you full control of the location of every generated file.

An example is provided [in the example directory](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/rust/flutter_rust_bridge.toml). I suggest you follow it and modify for your needs.

For all keys and their meanings of the configuration yaml (`flutter_rust_bridge.yaml`), please refer to [its source code](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/config.rs).

### Add some code

#### The `mod`

Please put `mod generated_wire;` (where `generated_wire` is the name of the wire file that you choose) into your `lib.rs` or `main.rs`. Only by doing this, Rust can understand that this generated file is a part of your project.

#### Dependency

Add `flutter_rust_bridge = "1.0"` (where `1.0` should be the latest version) to Rust's `Cargo.toml`.

Add `flutter_rust_bridge: ^1.0` (same as above, should be latest version) to Flutter/Dart's `pubspec.yaml` under the section of `dependencies`.

#### Version

Dark SDK `>=2.14.0` is needed by the latest version of the `ffigen` tool. Therefore, write `sdk: ">=2.14.0 <3.0.0"` in the `environment` section of `pubspec.yaml`. If you do not want that, consider installing a older version of the `ffigen` tool.

### Run code generator

Same as the section in Quickstart. Simply run that binary.

### Run "Flutter+Rust" app

Same as the section in Quickstart. Simply build the Rust code (possibly integrated into the build process of Flutter), and run the Flutter app.

## What this library is & isn't

This library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, you may refer to external materials to learn Flutter, learn Rust, learn [Flutter FFI](https://flutter.dev/docs/development/platform-integration/c-interop) (Dart FFI) and so on. With material on the Internet, you will know how to create a mobile application using Flutter, and how that app can call Rust functions via Dart FFI (in the C ABI). Then this package comes in, and ease you from the burden to write down tons of boilerplate code ;)

## Set up Flutter/Dart+Rust support

I suggest that you can start with the [Flutter example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter) first, and modify it to satisfy your needs. It can serve as a template for new projects. It is run against CI [WIP] so we are sure it works.

Indeed, this library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, "how to create a Flutter app that can run Rust code" is actually out of the scope of this library, and there are already several tutorials on the Internet.

However, I can sketch the outline of what to do if you want to set up a new Flutter+Rust project as follows.

Step 1: Create a new Flutter project (or use an existing one). The Dart SDK should be `>=2.14.0` if you want to use the latest `ffigen` tool.

Step 2: Create a new Rust project, say, at directory `rust` under the Flutter project.

Step 3: Edit `Cargo.toml` and add:

```
[lib]
name = "flutter_rust_bridge_example" # whatever you like
crate-type = ["cdylib"] # <-- notice this type. `cdylib` for android, and `staticlib` for iOS. I write down a script to change it before build.
```

Step 4: Follow the standard steps of "how iOS uses static libraries". For example, in XCode, edit `Strip Style` in `Build Settings` to `Debugging Symbols`. Also, add your `libyour_generate_file.a` to `Link Binary With Libraries` in `Build Phases`. Add `binding.h` to `Copy Bundle Resources`. Add `#import "binding.h"` to `Runner-Bridging-Header`. Last but not least, add a never-to-be-executed dummy function in Swift that calls any of the generated C bindings, such as `func dummyMethodToAvoidSymbolStripping() { wire_passing_complex_structs(42, nil) }`, and this will prevent symbol stripping.

## Safety

This library has CI that runs [Valgrind](https://www.valgrind.org/) automatically on the setup that a Dart program calls a Rust program using this package, so memory problems should be found by Valgrind. (Notice that, even when running a simple hello-world Dart program, Valgrind will report hundreds of errors. See [this Dart lang issue](https://github.com/dart-lang/sdk/issues/47346) for more details. Therefore, I both look at "definitely lost" in Valgrind, and manually search things related to this library - if all reported errors are unrelated to this library then we are safe.)

In addition, Flutter integration tests are also done in CI. This ensures a real Flutter application using this library does not suffer from problems.

Most of the code are written in safe Rust. The `unsafe` code mainly comes from `support::box_from_leak_ptr` and `support::vec_from_leak_ptr`. They are used for pointers and arrays, and I follow the high-upvoted answers and official doc when writing down that few lines of code.

I use this library heavily in my own Flutter project (`yplusplus`, or `why++`). That app is in production and it works quite well. If I observe any problems, I will fix it in this library.

The CI also runs the `run_codegen` workflow, which ensure that the code generator can compile and generate desired results. Lastly, the CI also runs formatters and linters (`fmt`, `clippy`, `dart analyze`, `dart format`), and linters can also catch some common problems.

## Future work

I plan to support the following features. Of course, if you want to have other features, feel free to make an issue or PR.

* Support `async` in Rust (currently only `async` in Dart). Should be quite easy to implement; I have not done it because my use case currently does not includ that, but feel free to PR.
* Support [`Stream`](https://dart.dev/tutorials/language/streams)s, which is a powerful abstraction. Should also be easy to implement.
* Beautify the generated code, possibly making the cases (camel/snake/...) consistent with the language guide.
* Make the code generator more robust to invalid inputs.

## Advanced

### Using your own executor

`DefaultExecutor`: When Dart calls Rust, the `DefaultExecutor` use a simple thread pool  to execute the real Rust functions. By doing this, Rust function that needs to run for a long time (more than a few frames) will never make the UI stuck.

However, you can implement your own `Executor` doing whatever you want. In order to do this, implement the `Executor` trait, and call `set_executor` to set your own executor.