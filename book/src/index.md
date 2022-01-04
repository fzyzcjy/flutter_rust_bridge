# [flutter_rust_bridge](https://fzyzcjy.github.io/flutter_rust_bridge/): High-level memory-safe binding generator for Flutter/Dart <-> Rust

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)
[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Documentation](https://docs.rs/flutter_rust_bridge/badge.svg)](https://docs.rs/flutter_rust_bridge)
[![Example](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml)

![Logo](https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/docs/logo.png)

Want to combine the best between `Flutter`, a cross-platform hot-reload rapid-development UI toolkit, and `Rust`, a language empowering everyone to build reliable and efficient software? Here it comes!

## 🚀 Advantages

* **Memory-safe**: Never need to think about malloc/free.
* **Type support**: Unlike low-level binding generator which only provide primitives and pointers, this package provides things like `Vec<u8>`(`Uint8List`), `Vec<T>`(`List<T>`), any custom `struct`(`class`)s, and even recursive structs (e.g. a tree node).
* **Zero-copy**: Pass big array of bytes from Rust to Dart without any memory copies.
* **Async programming**: Simply call functions directly in main isolate (thread) of Dart/Flutter, and Rust code will not block the Flutter UI.
* **Easy to use**: All you need to do is write down your Rust code. The code generator will do everything and expose an API in Dart/Flutter's style.
* **Lightweight**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. <sub>For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).</sub>
* **Easy to code-review & convince yourself**: This package simply simulates how human beings write down boilerplate code. If you want to convince yourself (or your team) that it is safe, there is not much code for you to look at. The runtime is only hundreds of loc, and the generated code follows simple patterns. No magic <sub>and also no blackbox macros</sub> at all! ([More about](https://github.com/fzyzcjy/flutter_rust_bridge#safety) safety concerns.)
* **Pure-Dart compatible:** Despite the name, this package is 100% compatible with pure Dart. It does not require anything specific to Flutter. See [this pure-Dart example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/README.md).

## 💡 User Guide

Check out [the user guide](https://fzyzcjy.github.io/flutter_rust_bridge/) for show-me-the-code, quickstart, tutorials, and more advanced features.

## Contributors ✨

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-9-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/fzyzcjy"><img src="https://avatars.githubusercontent.com/u/5236035?v=4?s=100" width="100px;" alt=""/><br /><sub><b>fzyzcjy</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=fzyzcjy" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=fzyzcjy" title="Documentation">📖</a> <a href="#example-fzyzcjy" title="Examples">💡</a> <a href="#ideas-fzyzcjy" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-fzyzcjy" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/Desdaemon"><img src="https://avatars.githubusercontent.com/u/36768030?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Viet Dinh</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Tests">⚠️</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/smw-wagnerma"><img src="https://avatars.githubusercontent.com/u/66412697?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Marcel</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=smw-wagnerma" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rustui"><img src="https://avatars.githubusercontent.com/u/90625190?v=4?s=100" width="100px;" alt=""/><br /><sub><b>rustui</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=rustui" title="Documentation">📖</a></td>
    <td align="center"><a href="https://adventures.michaelfbryan.com/"><img src="https://avatars.githubusercontent.com/u/17380079?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Michael Bryan</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Michael-F-Bryan" title="Code">💻</a></td>
    <td align="center"><a href="https://bus710.net"><img src="https://avatars.githubusercontent.com/u/8920680?v=4?s=100" width="100px;" alt=""/><br /><sub><b>bus710</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=bus710" title="Documentation">📖</a></td>
    <td align="center"><a href="https://scholar.google.com/citations?user=RbAto7EAAAAJ"><img src="https://avatars.githubusercontent.com/u/1213857?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sebastian Urban</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=surban" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/trobanga"><img src="https://avatars.githubusercontent.com/u/8888869?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Daniel</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=trobanga" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/AlienKevin"><img src="https://avatars.githubusercontent.com/u/22850071?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Kevin Li</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=AlienKevin" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=AlienKevin" title="Documentation">📖</a></td>
  </tr>
</table>
<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
