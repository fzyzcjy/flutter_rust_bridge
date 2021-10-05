# High-level memory-safe bindgen for Flutter/Dart &lt;-> Rust
[![Documentation](https://docs.rs/dart_rust_bridge/badge.svg)](https://docs.rs/flutter_rust_bridge)
[![Rust Package](https://img.shields.io/crates/v/dart_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/dart_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Valgrind](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/valgrind.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/valgrind.yml)
[![Rust](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml)
[![Lints](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml)
[![Dart](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/dart.yml/badge.svg?branch=master)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/dart.yml)
[![Run codegen](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/run_codegen.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/run_codegen.yml)

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

Please [install Flutter](https://flutter.dev/docs/get-started/install), [install Rust](https://www.rust-lang.org/learn/get-started), and have some familiarity with them. Then run `git clone https://github.com/fzyzcjy/flutter_rust_bridge && cd frb_example/complex` to get my example.

Or you can use your own code (if you find this Quickstart section too brief, have a look at the later Usage section).

### Run code generator

Install it: `cargo install flutter_rust_bridge_codegen`.

Run it: `flutter_rust_bridge_codegen frb_example/complex/rust/flutter_rust_bridge.yaml` (the argument is the path to the configuration file).

In quickstart, since I have generated the source code, you should not see anything changed.

### Run Flutter app

[WIP]

### Want to see more types that this library can generate?

Have a look at the function arguments and return types in this file: [api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs). With this library, we have a generated API that resides at [generated_api.dart](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/dart/lib/generated_api.dart) (of course, that is auto generated, and you can use it in other Dart code).

## Usage in details

### Configuration

`flutter_rust_bridge.yaml` is nothing but specifying where is your source code, and where do you want the generated code to be put in. We give you full control of the location of every generated file.

An example is provided [in the example directory](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/complex/rust/flutter_rust_bridge.toml). I suggest you follow it and modify for your needs.

For all keys and their meanings of the configuration yaml (`flutter_rust_bridge.yaml`), please refer to [its source code](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/config.rs).

### Add some code

#### The `mod`

Please put `mod generated_wire;` (where `generated_wire` is the name of the wire file that you choose) into your `lib.rs` or `main.rs`. Only by doing this, Rust can understand that this generated file is a part of your project.

#### Dependency

Add `flutter_rust_bridge = "1.0"` (where `1.0` should be the latest version) to Rust's `Cargo.toml`.

Add `flutter_rust_bridge: ^1.0` (same as above, should be latest version) to Flutter/Dart's `pubspec.yaml` under the section of `dependencies`.

### Run code generator

Same as the section in Quickstart. Simply run that binary.

### Run Flutter app

[WIP]

## Set up Flutter/Dart+Rust support

I suggest that you can start with the [Flutter example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/complex) first, and modify it to satisfy your needs. It can serve as a template for new projects. It is run against CI [WIP] so we are sure it works.

Indeed, this library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, "how to create a Flutter app that can run Rust code" is actually out of the scope of this library, and there are already several tutorials on the Internet.

However, I can sketch the outline of what to do if you want to set up a new Flutter+Rust project as follows.

Step 1: Create a new Flutter project (or use an existing one)

Step 2: Create a new Rust project, say, at directory `rust` under the Flutter project.

Step 3: Edit `Cargo.toml` and add:

```
[lib]
name = "flutter_rust_bridge_example" # whatever you like
crate-type = ["cdylib"] # <-- notice this type. `cdylib` for android, and `staticlib` for iOS. I write down a script to change it before build.
```

[WIP]

## Safety

This library has CI that runs [Valgrind](https://www.valgrind.org/) automatically on the setup that a Dart program calls a Rust program using this package, so memory problems should be found by Valgrind. (Notice that, even when running a simple hello-world Dart program, Valgrind will report hundreds of errors. See [this Dart lang issue](https://github.com/dart-lang/sdk/issues/47346) for more details. Therefore, I both look at "definitely lost" in Valgrind, and manually search things related to this library - if all reported errors are unrelated to this library then we are safe.)

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