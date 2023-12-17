# [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge): High-level memory-safe binding generator for Flutter/Dart <-> Rust

[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Stars](https://img.shields.io/github/stars/fzyzcjy/flutter_rust_bridge)](https://github.com/fzyzcjy/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)

[<img src="https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/flutter_favorite.png" width="200" />](https://flutter.dev/docs/development/packages-and-plugins/favorites)

![Logo](https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/logo.png)

Want to combine the best between [Flutter](https://flutter.dev/), a cross-platform hot-reload rapid-development UI toolkit, and [Rust](https://www.rust-lang.org/), a language empowering everyone to build reliable and efficient software? Here it comes!

## 🚀 Advantages

* **Memory-safe**: Never need to think about malloc/free.
* **Feature-rich**: `enum`s with values, platform-optimized `Vec`, possibly recursive `struct`, zero-copy big arrays, opaque types on arbitrary structs/classes, `Stream` (iterator) abstraction, error (`Result`) handling, cancellable tasks, concurrency control, and more. See full features [here](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html).
* **Async programming**: Rust code will never block the Flutter. Call Rust naturally from Flutter's main isolate (thread); sync mode also equally supported.
* **Lightweight**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. <sub>For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).</sub>
* **Cross-platform**: Android, iOS, Windows, Linux, MacOS, and Web.
* **Easy to code-review & convince yourself**: This package simply simulates how humans  write boilerplate code. If you want to convince yourself (or your team) that it is safe, there is not much code to look at. No magic at all! ([More about](https://fzyzcjy.github.io/flutter_rust_bridge/safety.html) safety concerns.)
* **Fast**: It is only a thin (though feature-rich) wrapper, without overhead such as protobuf serialization, thus performant. (More [benchmarks](https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1034536815) later) <small>(Throw away components like thread-pool to make it even faster)</small>
* **Pure-Dart compatible:** Despite the name, this package is 100% compatible with [pure](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/README.md) Dart.

## 💡 User Guide

Check out [the user guide](https://fzyzcjy.github.io/flutter_rust_bridge/) for [show-me-the-code](https://fzyzcjy.github.io/flutter_rust_bridge/quickstart.html), [tutorials](https://fzyzcjy.github.io/flutter_rust_bridge/tutorial_with_flutter.html), [features](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html) and much more.

## 📎 P.S. Achieve ~60 FPS, no matter how janky the Flutter app was due to build/layout

Here is my another open-source library :) https://github.com/fzyzcjy/flutter_smooth.

## ✨ Contributors

TODO
