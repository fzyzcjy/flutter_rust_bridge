# High-level memory-safe bindgen for Flutter/Dart &lt;-> Rust
[![Github Actions](https://github.com/fzyzcjy/dart_rust_bridge/workflows/dart_rust_bridge/badge.svg)](https://github.com/fzyzcjy/dart_rust_bridge/actions?query=workflow%3Adart_rust_bridge)
[![Documentation](https://docs.rs/dart_rust_bridge/badge.svg)](https://docs.rs/dart_rust_bridge)
[![Rust Package](https://img.shields.io/crates/v/dart_rust_bridge.svg)](https://crates.io/crates/dart_rust_bridge)
[![Flutter Package][https://img.shields.io/pub/v/dart_rust_bridge.svg]][https://pub.dev/packages/dart_rust_bridge]

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
    Ok(... do my heavy computations ...)
}

// you can use structs (even recursive)
pub struct TreeNode { pub value: i32, pub children: Vec<MyTreeNode> }
```

We will generate the bindings. Then you only need to use the following generated API in Dart/Flutter. Nothing more.

```Dart
Future<Something> myFunction({required MyTreeNode a, required SomeOtherStruct b}) async { ... auto generated implementation ... }
```

## Quickstart

[WIP, espeically because currently I only make a Dart example. Should add a Flutter example.]

## Configurations

For all keys and their meanings of the configuration yaml (`dart_rust_bridge.yaml`), please refer to [its source code](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/config.rs).

An example is also provided [in the example directory](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/frb_example_rust/dart_rust_bridge.yaml).

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