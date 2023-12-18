[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Stars](https://img.shields.io/github/stars/fzyzcjy/flutter_rust_bridge)](https://github.com/fzyzcjy/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)

[<img src="https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/flutter_favorite.png" width="200" />](https://flutter.dev/docs/development/packages-and-plugins/favorites)

![Logo](https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/logo.png)

## What's new in V2

Please visit [this page](https://fzyzcjy.github.io/flutter_rust_bridge/guides/whats-new) for information and update guide.

## 🚀 Advantages

* **Officially `Flutter Favorite`**: This package is [officially Flutter Favorite](https://docs.flutter.dev/packages-and-plugins/favorites), and is in the first batch of 7 packages at its [rebooting](https://medium.com/flutter/whats-new-in-flutter-3-16-dba6cb1015d1).
* **Seamless communication**: Call Rust from Dart, as if calling the same language.
  * **Arbitrary types**: Use arbitrary Rust and Dart types, even if they are not serializable or non-clone.
  * **Async & sync** x Rust & Dart: Multi modes for various needs - Avoid blocking the main thread, or sync API (e.g. used in Widget.build); Async runtime for IO bound tasks, or thread pools for CPU-heavy computations.
  * **Two-way road**: Not only can Dart call Rust - Rust can also call Dart.
  * **Auto-translatable types**: Lots of types can be further translated to Dart native types, e.g. complex `enum`s and `struct`s, zero-copy big arrays, errors (`Result`), and `Stream`s (iterator).
  * **Safety**: Focus on your code, and forget memory safety, malloc/free, or undefined behavior completely.
  * **Other features**: Support whole folders as input, and the output folder will preserve hierarchy. Methods (not only functions).
* **Quick setup, but fully customizable**: Run a one-liner command, then get a ready-to-use project (or integrate into existing projects). Provide sensible defaults, but everything can be customized.
* **Solid CI**: We have CI for Valgrind, sanitizers (ASAN/MSAN/LSAN), testing for each platform, benchmarking, etc.
* **Fast**: It is only a thin (though feature-rich) wrapper, benchmarked on CI, and even has multiple codecs for best performance under different workloads.
* **Lightweight**: You are free to use your favorite Flutter and Rust libraries and toolchains (e.g. runner, debugger).
* **Cross-platform**: Android, iOS, Windows, Linux, MacOS, and Web.
* **Pure-Dart compatible:** Despite the name, this package is 100% compatible with pure Dart.

<details>
<summary>Why Flutter + Rust?</summary>

Firstly, super briefly introduce each component (you can find much more in a lot of blogs and posts):

* **[Flutter](https://flutter.dev/)**: Cross-platform, hot-reload, rapid-development, flexible UI toolkit.
  * "The most popular cross-platform mobile SDK" (by StackOverflow [[1]](https://stackoverflow.blog/2022/02/21/why-flutter-is-the-most-popular-cross-platform-mobile-sdk/)[[2]](https://survey.stackoverflow.co/2023/#technology-most-popular-technologies)).
* **[Rust](https://www.rust-lang.org/)**: Highly efficient and performant, reliable, productive.
  * "The most desired programming language" for 8 years (by StackOverflow and GitHub [[1]](https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/)[[2]](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages)).

Typical scenarios to combine them include:

* **UI framework for Rust**: When you want a UI framework for your Rust system.
* **Use arbitrary Rust libraries in Flutter**: When the desired functionality only has a library in Rust, not Dart (Flutter).
* **Need high-performance code for Flutter**: Rust makes it easy and performant to write multi-thread code, algorithms, data-intensive operations, etc.
* ...

</details>
