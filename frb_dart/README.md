# flutter_rust_bridge: High-level memory-safe binding generator for Flutter/Dart <-> Rust

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)
[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Documentation](https://docs.rs/flutter_rust_bridge/badge.svg)](https://docs.rs/flutter_rust_bridge)

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

## 🧭 Show me the code

What you write down (in Rust):

```rust
pub fn my_function(a: MyTreeNode, b: SomeOtherStruct) -> Result<Vec<u8>> {
    ... do my heavy computations ...
}

// you can use structs (even recursive)
pub struct TreeNode { pub value: String, pub children: Vec<MyTreeNode> }
```

With bindings automatically generated, you can simply use the following API in Flutter/Dart. Nothing more.

```dart
Future<Uint8List> myFunction(MyTreeNode a, SomeOtherStruct b);
```

<sub>**Remark**: Why `Future` in Flutter: Flutter is single-threaded. If not using future, just like what you do with plain-old Flutter bindings, your UI will be *stuck* as long as your Rust code is executing. If your Rust code run for a second, your UI will fully freeze for one second.</sub> 

## 💡 Quickstart

### Install

* Install dependency `cbindgen`: `cargo install cbindgen` <sub>(may [need latest version](https://github.com/fzyzcjy/flutter_rust_bridge/issues/53#issuecomment-939588321), thanks @gmorenz)</sub>
* Install dependency `ffigen`:  `dart pub global activate ffigen`, and [install LLVM](https://pub.dev/packages/ffigen#installing-llvm).
* Install this code generator binary by `cargo install flutter_rust_bridge_codegen`.
* Add `flutter_rust_bridge = "1.0"` (where `1.0` should be the latest version) to Rust's `Cargo.toml`.
* Add `flutter_rust_bridge: ^1.0` (same as above, should be latest version) to Flutter/Dart's `pubspec.yaml` under the section of `dependencies`.

### Run

```shell
flutter_rust_bridge_codegen --rust-input path/to/your/api.rs --dart-output path/to/file/being/bridge_generated.dart
```

(If you have problems, see "Troubleshooting" section.) (For more options, use `--help`; To see what types and function signatures can you write in Rust, have a look at [this example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs).) (For Windows, you may need `\\` instead of `/` for paths.)

### Enjoy

Use the class in the generated `.dart` file, as if it is a normal Flutter/Dart class! (The abstract class at the top of the generated file.)

Want to see a Flutter tutorial with UI? See [the tutorial section below](https://github.com/fzyzcjy/flutter_rust_bridge#-tutorial-a-flutterrust-app). Want pure-Dart example? [Here is](https://github.com/fzyzcjy/flutter_rust_bridge#-tutorial-pure-dart) another tutorial.

<sub>**Remark**: If you are interested, why `abstract`class can be used - it is because of the [factory](https://dart.dev/guides/language/language-tour#factory-constructors) language feature.</sub>

## 📪 Safety

This library has CI that runs [Valgrind](https://www.valgrind.org/) automatically on the setup that a Dart program calls a Rust program using this package, so memory problems should be found by Valgrind. <sub>(Notice that, even when running a simple hello-world Dart program, Valgrind will report hundreds of errors. See [this Dart lang issue](https://github.com/dart-lang/sdk/issues/47346) for more details. Therefore, I both look at "definitely lost" in Valgrind, and manually search things related to this library - if all reported errors are unrelated to this library then we are safe.)</sub>

In addition, Flutter integration tests are also done in CI. This ensures a real Flutter application using this library does not suffer from problems.

Most of the code are written in safe Rust. The `unsafe` code mainly comes from `support::box_from_leak_ptr` and `support::vec_from_leak_ptr`. They are used for pointers and arrays, and I follow the high-upvoted answers and official doc when writing down that few lines of code.

I use this library heavily in my own Flutter project (`yplusplus`, or `why++`). That app is in production and it works quite well. If I observe any problems, I will fix it in this library.

The CI also runs the `run_codegen` workflow, which ensure that the code generator can compile and generate desired results. Lastly, the CI also runs formatters and linters (`fmt`, `clippy`, `dart analyze`, `dart format`), and linters can also catch some common problems.

## 📚 Tutorial: A Flutter+Rust app

**Remark**: The `flutter_*_test` sections of the [CI workflow](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/test.yaml) can also be useful, if you want details of each command.

### Get example code

Please [install Flutter](https://flutter.dev/docs/get-started/install), [install Rust](https://www.rust-lang.org/learn/get-started), and have some familiarity with them. Then run `git clone https://github.com/fzyzcjy/flutter_rust_bridge`, and my example is in `frb_example/with_flutter`.

### (Optional) Run code generator

I have generated the source code already (in quickstart), so this step is optional. Even if you do it, you should not see anything changed.

Install it: `cargo install flutter_rust_bridge_codegen`.

Run it:

```
flutter_rust_bridge_codegen --rust-input frb_example/with_flutter/rust/src/api.rs --dart-output frb_example/with_flutter/lib/bridge_generated.dart --c-output frb_example/with_flutter/ios/Runner/bridge_generated.h
```

<sub>**Remark**: If you have problems, see "Troubleshooting" section. For Windows, you may need `\\` instead of `/` for paths.</sub>

### Run "Flutter+Rust" app

With this app, you will see a [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) plotted in Flutter UI and generated by Rust algorithm.

#### If Android

Run `cargo ndk -o ../android/app/src/main/jniLibs build`. Then run the Flutter app normally as is taught in official tutorial. For example, `flutter run`.

Remark: Since my quickstart app is so baremetal, I do not integrate the Rust building process into Flutter building process. But you can look at [this tutorial](https://stackoverflow.com/q/69515032/4619958) to easily do that.

#### If iOS

Modify `Cargo.toml` to change `cdylib` to `staticlib`. (Again, this is baremetal example so it is done manually. For your project, you can automate it.)

Run `cargo lipo && cp target/universal/debug/libflutter_rust_bridge_example.a ../ios/Runner` to build Rust and copy the static library. Then run the Flutter app normally as is taught in official tutorial. For example, `flutter run`. (Similarly, [this tutorial](https://stackoverflow.com/q/69515032/4619958) can automate the process.)

#### If desktop (Windows/Linux/MacOS)

Run it directly using `flutter run` assuming [Flutter desktop support](https://flutter.dev/desktop#set-up) has been configured. 

Flutter can run on Windows/Linux/MacOS without any problem, and this lib does nothing but generates some code like a human being. Therefore, this package should work well as long as you set up the Flutter desktop app's ffi functionality successfully.

The example in `with_flutter` demonstrates how to integrate Cargo with CMake on Linux and Windows, so it can be. More details can be seen [in the issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/66).

### (Optional) See more types that this library can generate

Have a look at the function arguments and return types in this file: [api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs). With this library, we have a generated API that resides at [generated_api.dart](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/dart/lib/generated_api.dart) (of course, that is auto generated, and you can use it in other Dart code).

### (Optional) Remarks

#### The `mod`

If you are adding this lib to your own existing code, please put `mod generated_wire;` (where `generated_wire` is the name of the wire file that you choose) into your `lib.rs` or `main.rs`. Only by doing this, Rust can understand that this generated file is a part of your project.

#### Version

Dart SDK `>=2.14.0` is needed not by this library, but by the latest version of the `ffigen` tool. Therefore, write `sdk: ">=2.14.0 <3.0.0"` in the `environment` section of `pubspec.yaml`. If you do not want that, consider installing a older version of the `ffigen` tool.

## 📚 Tutorial: Pure Dart

**Remark**: The `valgrind_test` section of the [CI workflow](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/test.yaml) can also be useful, if you want details of each command and want to see Valgrind configuration.

Unlike the previous tutorial, this one integrates Rust with pure Dart instead of Flutter.

### Get example code

Please [install Dart](https://dart.dev/get-dart), [install Rust](https://www.rust-lang.org/learn/get-started), and have some familiarity with them. Then run `git clone https://github.com/fzyzcjy/flutter_rust_bridge`, and my example is in `frb_example/pure_dart`.

### (Optional) Run code generator

Remark: I have generated the source code already (in quickstart), so this step is optional. Even if you do it, you should not see anything changed.

Install it: `cargo install flutter_rust_bridge_codegen`.

Run it: `flutter_rust_bridge_codegen --rust-input frb_example/pure_dart/rust/src/api.rs --dart-output frb_example/pure_dart/dart/lib/bridge_generated.dart` (See [CI workflow](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/codegen.yml) as a reference.) (For Windows, you may need `\\` instead of `/` for paths.)

### Run "Dart+Rust" app

You may run `frb_example/pure_dart/dart/lib/main.dart` as a normal Dart program, except that you should provide the dynamic linked library of the Rust code (for simplicity, here I only demonstrate the approach for dynamic linked library, but you can for sure use other methods). The detailed steps are as follows.

Run `cargo build` in `frb_example/pure_dart/rust` to build the Rust code into a `.so` file. Then run `dart frb_example/pure_dart/dart/lib/main.dart frb_example/pure_dart/rust/target/debug/libflutter_rust_bridge_example.so ` to run the Dart program with Rust `.so` file. (If you have problems, see "Troubleshooting" section.)  (If on MacOS, Rust may indeed generate `.dylib`, so change the last command to use `...dylib` instead of `...so`,)

P.S. You will only see some tests passing - no fancy UI or functionality in this example.

## Command line arguments

Simply add `--help` to see full documentation.

```shell
flutter_rust_bridge_codegen

USAGE:
    flutter_rust_bridge_codegen [FLAGS] [OPTIONS] --dart-output <dart-output> --rust-input <rust-input>

FLAGS:
        --skip-add-mod-to-lib    Skip automatically adding `mod bridge_generated;` to `lib.rs`
    -h, --help                   Prints help information
    -V, --version                Prints version information

OPTIONS:
    -r, --rust-input <rust-input>                              Path of input Rust code
    -d, --dart-output <dart-output>                            Path of output generated Dart code
    -c, --c-output <c-output>                                  Path of output generated C header
        --rust-crate-dir <rust-crate-dir>                      Crate directory for your Rust project
        --rust-output <rust-output>                            Path of output generated Rust code
        --class-name <class-name>                              Generated class name
        --dart-format-line-length <dart-format-line-length>    Line length for dart formatting
        --llvm-path <llvm-path>                                Path to the installed LLVM
```

## What this library is & isn't

This library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, you may refer to external materials to learn Flutter, learn Rust, learn [Flutter FFI](https://flutter.dev/docs/development/platform-integration/c-interop) (Dart FFI) and so on. With material on the Internet, you will know how to create a mobile application using Flutter, and how that app can call Rust functions via Dart FFI (in the C ABI). Then this package comes in, and ease you from the burden to write down tons of boilerplate code ;)

## Troubleshooting

#### Have problems when using `Linux`?

Try to run code generator with working directory at `/`. This seems to be a problem with Rust's builtin `Command`. See [#108](https://github.com/fzyzcjy/flutter_rust_bridge/issues/108) for more details.

#### Issue with store_dart_post_cobject?

If calling rust function gives the error below, please consider running **cargo build** again. This can happen when the generated rs file is not included when building is being done.
```sh
[ERROR:flutter/lib/ui/ui_dart_state.cc(209)] Unhandled Exception: Invalid argument(s): Failed to lookup symbol 'store_dart_post_cobject': target/debug/libadder.so: undefined symbol: store_dart_post_cobject
```

#### Error running `cargo ndk`: `ld: error: unable to find library -lgcc`

Downgrade Android NDK to version 22. This is an [ongoing issue](https://github.com/bbqsrc/cargo-ndk/issues/22) with `cargo-ndk`, a library unrelated to flutter_rust_bridge but solely used to build the examples, when using Android NDK version 23. (See [#149](https://github.com/fzyzcjy/flutter_rust_bridge/issues/149))

#### Other problems?

Don't hesitate to [open an issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/new?assignees=&labels=bug&template=bug_report.md&title=)! I usually reply within minutes or hours (except when sleeping, of course).

## Feature details

Here is a list of types that the code generator can generate:

[WIP] You can read the `pure_dart` example code currently, before I put the full list here.

Here are other functionalities:

[WIP] (e.g. [Stream is supported but not documented yet](https://github.com/fzyzcjy/flutter_rust_bridge/issues/179)).

## Advanced

### Customize handler's behavior

By default, the `DefaultHandler` is used. You can implement your own `Handler` doing whatever you want. In order to do this, create a variable named `FLUTTER_RUST_BRIDGE_HANDLER` in the Rust input file (probably using `lazy_static`). You may not need to create a brand new struct implementing `Handler`, but instead, use the `SimpleHandler` and customize its generic arguments such as its `Executor`.

### Setup/init FFI call

If you want that feature, have a look at `FlutterRustBridgeSetupMixin` in the Dart side.

## Appendix: Set up Flutter/Dart+Rust support

I suggest that you can start with the [Flutter example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter) first, and modify it to satisfy your needs. It can serve as a template for new projects. It is run against CI so we are sure it works.

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

Step 4: Follow the standard steps of "how iOS uses static libraries". For example, in XCode, edit `Strip Style` in `Build Settings` to `Debugging Symbols`. Also, add your `libyour_generate_file.a` to `Link Binary With Libraries` in `Build Phases`. Add `binding.h` to `Copy Bundle Resources`. Add `#import "binding.h"` to `Runner-Bridging-Header`. Last but not least, add a never-to-be-executed dummy function in Swift that calls any of the generated C bindings. This lib has already generated a dummy method for you, so you simply need to add `print("dummy_value=\(dummy_method_to_enforce_bundling())");` to swift file's `override func application(...) {}`, and this will prevent symbol stripping - especially in the release build for iOS (i.e. when building ipa file or releasing to App Store). Notice that, we have to use that `dummy_method_to_enforce_bundling()`, otherwise the symbols will not maintain in the release build, and Flutter will complain it cannot find the symbols.

Lastly, in order to build Rust automatically when you are building Flutter, follow [this tutorial](https://stackoverflow.com/q/69515032/4619958).

## Appendix: Future work

I plan to support the following features. Of course, if you want to have other features, feel free to make an issue or PR.

* Support `async` in Rust (currently only `async` in Dart). Should be quite easy to implement; I have not done it because my use case currently does not includ that, but feel free to PR.
* Beautify the generated code, possibly making the cases (camel/snake/...) consistent with the language guide.
* Make the code generator more robust to invalid inputs.

## Appendix: Contributing

Please look at [contributing guide](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/CONTRIBUTING.md).

## Contributors ✨

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-7-orange.svg?style=flat-square)](#contributors-)
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
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
