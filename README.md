<h1 align="center" style="border-bottom: none">
    <b><a href="https://github.com/fzyzcjy/flutter_rust_bridge">flutter_rust_bridge</a></b>
</h1>

<h2 align="center">High-level memory-safe binding generator for Flutter/Dart <-> Rust</h2>

[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Stars](https://img.shields.io/github/stars/fzyzcjy/flutter_rust_bridge)](https://github.com/fzyzcjy/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Example](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)

![Logo](https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/logo.png)

Want to combine the best between [Flutter](https://flutter.dev/), a cross-platform hot-reload rapid-development UI toolkit, and [Rust](https://www.rust-lang.org/), a language empowering everyone to build reliable and efficient software? Here it comes!

## 🚀 Advantages

- **Memory-safe**: Never need to think about malloc/free.
- **Feature-rich**: `enum`s with values, platform-optimized `Vec`, possibly recursive `struct`, zero-copy big arrays, opaque types on arbitrary structs/classes, `Stream` (iterator) abstraction, error (`Result`) handling, cancellable tasks, concurrency control, and more. See full features [here](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html).
- **Async programming**: Rust code will never block the Flutter. Call Rust naturally from Flutter's main isolate (thread); sync mode also equally supported.
- **Lightweight**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. <sub>For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).</sub>
- **Cross-platform**: Android, iOS, Windows, Linux, MacOS, and Web.
- **Easy to code-review & convince yourself**: This package simply simulates how humans write boilerplate code. If you want to convince yourself (or your team) that it is safe, there is not much code to look at. No magic at all! ([More about](https://fzyzcjy.github.io/flutter_rust_bridge/safety.html) safety concerns.)
- **Fast**: It is only a thin (though feature-rich) wrapper, without overhead such as protobuf serialization, thus performant. (More [benchmarks](https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1034536815) later) <small>(Throw away components like thread-pool to make it even faster)</small>
- **Pure-Dart compatible:** Despite the name, this package is 100% compatible with [pure](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/README.md) Dart.

## 💡 User Guide

Check out [the user guide](https://fzyzcjy.github.io/flutter_rust_bridge/) for [show-me-the-code](https://fzyzcjy.github.io/flutter_rust_bridge/quickstart.html), [tutorials](https://fzyzcjy.github.io/flutter_rust_bridge/tutorial_with_flutter.html), [features](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html) and much more.

## 📎 P.S. Achieve ~60 FPS, no matter how janky the Flutter app was due to build/layout

Here is my another open-source library :) https://github.com/fzyzcjy/flutter_smooth.

## ✨ Contributors

Thanks goes to these wonderful people!

[![All Contributors](https://img.shields.io/github/all-contributors/fzyzcjy/flutter_rust_bridge?color=ee8449)](#contributors-)

<a href="https://github.com/fzyzcjy/flutter_rust_bridge/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=fzyzcjy/flutter_rust_bridge" />
</a>

More specifically, thanks for all these contributions:

- [Desdaemon](https://github.com/Desdaemon): Support not only simple enums but also enums with fields which gets translated to native enum or freezed class in Dart. Support the Option type as nullable types in Dart. Support Vec of Strings type. Support comments in code. Add marker attributes for future usage. Add Linux and Windows support for with-flutter example, and make CI works for that. Avoid parameter collision. Overhaul the documentation and add several chapters to demonstrate configuring a Flutter+Rust project in all five platforms. Refactor command module. Precompiled binary CI workflow. Fix bugs. Add support for the Web platform, parallel to the existing mobile/desktop platforms, via WASM and JavaScript as intermediate values. GitHub retry actions. Implement draft of opaque types. Refactor Boxed and Option.
- [rogurotus](https://github.com/rogurotus): Add Rust opaque types, enabling arbitrary Rust structs to be used as opaque Dart objects by generating wrappers and raw Arc pointers, as well as Dart opaque types, allowing to use any Dart objects in Rust code. Extend `SyncReturn` for more types. Fix generation bug. Fix SyncReturn. Migrate to dart-sys. Update CI. Fix linters.
- [ngasull](https://github.com/ngasull): Make sync mode support whatever types that classical async mode supports. Bump sdk.
- [SecondFlight](https://github.com/SecondFlight): Allow structs and enums to be imported from other files within the crate by creating source graph. Auto-create relevant dir. Fix `store_dart_post_cobject` error with ffigen 6.0.
- [Unoqwy](https://github.com/Unoqwy): Add struct mirrors, such that types in the external crates can be imported and used without redefining and copying.
- [antonok-edm](https://github.com/antonok-edm): Avoid converting syn types to strings before parsing to improve code and be more robust.
- [lattice0](https://github.com/lattice0): Support methods, such that Rust struct impls can be converted to Dart class methods. StreamSink at any argument.
- [sagudev](https://github.com/sagudev): Make code generator a `lib`. Add error types. Depend on `cbindgen`. Fix LLVM paths. Update deps. Fix CI errors.
- [surban](https://github.com/surban): Support unit return type. Skip unresolvable modules. Ignore prefer_const_constructors. Non-final Dart fields.
- [Roms1383](https://github.com/Roms1383): Fix build_runner calling bug. Remove global `ffigen` dependency. Improve version check. Fix enum name-variant conflicts. Support Chrono date time and UUID types. Migrate to Rust 1.64 workspace. Update and refactor CI. Update header comments. Code cleanup.
- [GregoryConrad](https://github.com/GregoryConrad): Add doc to setup frb inside a Dart/Flutter library.
- [trobanga](https://github.com/trobanga): Add support for `[T;N]` structs. Add `usize` support. Add a cmd argument. Separate dart tests.
- [dbsxdbsx](https://github.com/dbsxdbsx): Allow generating multiple Rust and Dart files. Fix lint. Update doc. Add logging.
- [SoLongAndThanksForAllThePizza](https://github.com/SoLongAndThanksForAllThePizza): Refactor and enhance SyncReturn to support more types.
- [huang12zheng](https://github.com/huang12zheng): Support type aliases and nested ones. Tweak code generation.
- [hsfzxjy](https://github.com/hsfzxjy): Fix SyncReturn use-after-free bug.
- [Cupnfish](https://github.com/Cupnfish): Support arrays as function parameters. Allow multi mirror.
- [temeddix](https://github.com/temeddix): Fix broken CI. Custom num workers. Update doc.
- [alanlzhang](https://github.com/alanlzhang): Add generation for Dart metadata. Enhance module parser.
- [Zaitam](https://github.com/Zaitam): Fix when method return struct.
- [AlienKevin](https://github.com/AlienKevin): Add flutter example for macOS. Add doc for Android NDK bug.
- [alexthe2](https://github.com/alexthe2): Add Option Datetime. Add empty structs. Improve doc.
- [banool](https://github.com/banool): Fix pubspec parsing. Fix symbol-stripping doc.
- [efc-mw](https://github.com/efc-mw): Improve Windows encoding handling.
- [valeth](https://github.com/valeth): Rename callFfi's port.
- [sccheruku](https://github.com/sccheruku): Prevent double-generating utility.
- [jsonmona](https://github.com/jsonmona): Add import.
- [w-ensink](https://github.com/w-ensink): Improve doc. Fix CI. Refactor. Add tests.
- [codercengiz](https://github.com/codercengiz): Fix mirroring bug.
- [Michael-F-Bryan](https://github.com/Michael-F-Bryan): Detect broken bindings.
- [bus710](https://github.com/bus710): Add a case in troubleshooting.
- [Demezy](https://github.com/Demezy): Mention troubleshooting.
- [Syndim](https://github.com/Syndim): Add a bracket to box.
- [TENX-S](https://github.com/TENX-S): Improve doc. Reproduce a bug.
- [CicadaCinema](https://github.com/CicadaCinema): Bump version. Improve doc.
- [w1th0utnam3](https://github.com/w1th0utnam3): Improve message.
- [vincent-herlemont](https://github.com/vincent-herlemont): Loosen version.
- [zaynetro](https://github.com/zaynetro): Improve doc.
- [raphaelrobert](https://github.com/raphaelrobert): Remove oudated doc.
- [elliotsayes](https://github.com/elliotsayes): Improve doc.
- [tmpfs](https://github.com/tmpfs): Improve doc.
- [thomas725](https://github.com/thomas725): Improve doc.
- [juzi5201314](https://github.com/juzi5201314): Improve doc.
- [Voklen](https://github.com/Voklen): Improve doc.
- [svenstaro](https://github.com/svenstaro): Improve doc.
- [utilForever](https://github.com/utilForever): Fix typos.
- [Stonks3141](https://github.com/Stonks3141): Fix doc credit.
- [feber](https://github.com/feber): Fix doc link.
- [rustui](https://github.com/rustui): Fix a typo.
- [eltociear](https://github.com/eltociear): Fix a typo.
