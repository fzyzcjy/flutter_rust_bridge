# High-level memory-safe bindgen for Flutter/Dart &lt;-> Rust
[![Documentation](https://docs.rs/dart_rust_bridge/badge.svg)](https://docs.rs/flutter_rust_bridge)
[![Rust Package](https://img.shields.io/crates/v/dart_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/dart_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Rust](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/rust.yml)
[![Lints](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/linter.yml)
[![Dart](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/dart.yml/badge.svg?branch=master)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/dart.yml)

## Advantages

* **Memory-safe**: Never need to think about malloc/free.
* **Zero-copy**: Pass big objects from Rust to Dart without any memory copies.
* **Type support**: Unlike low-level binding generator which only provide primitives and pointers, this package provides things like `Vec<u8>`(`Uint8List`), `Vec<T>`(`List<T>`), any custom `struct`(`class`)s, and even use recursive structs (e.g. a tree node).
* **Async programming**: You can simply call functions directly in main isolate (thread) of Dart/Flutter, and Rust code will not block the Flutter UI.
* **Easy to use**: All you need to do is write down your Rust code. The code generator will do everything and expose an API in the Dart/Flutter style for you.
* **Lightweight**: Simulate how human beings will write down boilerplate code . Thus compared with 
* **Plug-and-play**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).
* **Easy to code-review**: If you want to convince yourself (or your team) that it is safe, there are not much code for you to look at. The runtime is only hundreds of loc, and the generated code follows simple patterns. (Of course, if you find a bug, tell me!)

## Show me the code

What you write down (in Rust).

```Rust
pub fn my_function(a: MyTreeNode, b: SomeOtherStruct) -> Result<Something> {
    ... do my heavy computations ...
}

// you can use structs (even recursive)
pub struct TreeNode { pub value: i32, pub children: Vec<MyTreeNode> }
```

We will generate the bindings. Then you only need to use the following generated API in Dart/Flutter. Nothing more. It looks exactly like a normal Dart/Flutter function.

```Dart
Future<Something> myFunction(MyTreeNode a, SomeOtherStruct b);
```

## Quickstart

[WIP, espeically because currently I only make a Dart example. Should add a Flutter example.]

## Configurations

For all keys and their meanings of the configuration yaml (`dart_rust_bridge.yaml`), please refer to [its source code](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/config.rs).

An example is also provided [in the example directory](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/frb_example_rust/dart_rust_bridge.yaml).

## Safety

This library has CI that runs [Valgrind](https://www.valgrind.org/) automatically for FFI calls, so memory problems should be found by Valgrind.

Most of the code are written in safe Rust. The `unsafe` code mainly comes from `support::box_from_leak_ptr` and `support::vec_from_leak_ptr`. They are used for pointers and arrays, and I follow the high-upvoted answers and official doc when writing down that few lines of code.

The CI also runs formatters and linters (`fmt`, `clippy`, `dart analyze`, `dart format`), and linters can also catch some common problems.

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