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

## Quickstart

[TODO]

## Configurations

For all keys and their meanings of the configuration yaml (`dart_rust_bridge.yaml`), please refer to [its source code](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/config.rs).

An example is also provided [in the example directory](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/frb_example_rust/dart_rust_bridge.yaml).

