# Troubleshooting

## The generated store_dart_post_cobject() has the wrong signature / `'stdarg.h' file not found` in Linux

Try to run code generator with working directory at `/`, or add include path as is described in #108. This is a problem with Rust's builtin `Command`. See [#108](https://github.com/fzyzcjy/flutter_rust_bridge/issues/108) for more details.

## Issue with `store_dart_post_cobject`

If calling rust function gives the error below, please consider running **cargo build** again. This can happen when the generated rs file is not included when building is being done.
```sh
[ERROR:flutter/lib/ui/ui_dart_state.cc(209)] Unhandled Exception: Invalid argument(s): Failed to lookup symbol 'store_dart_post_cobject': target/debug/libadder.so: undefined symbol: store_dart_post_cobject
```

## Error running `cargo ndk`: `ld: error: unable to find library -lgcc`

Downgrade Android NDK to version 22. This is an [ongoing issue](https://github.com/bbqsrc/cargo-ndk/issues/22) with `cargo-ndk`, a library unrelated to flutter_rust_bridge but solely used to build the examples, when using Android NDK version 23. (See [#149](https://github.com/fzyzcjy/flutter_rust_bridge/issues/149))

## Fail to run `flutter_rust_bridge_codegen` on MacOS, "Please supply one or more path/to/llvm..."

If you are running macOS, you will need to specify a path to your llvm:
```shell
flutter_rust_bridge_codegen --rust-input path/to/your/api.rs --dart-output path/to/file/being/bridge_generated.dart --llvm-path /usr/local/homebrew/opt/llvm/
```
If you are on Intel, you can install llvm using `brew install llvm` and it will be installed at `/usr/local/homebrew/opt/llvm/` by default.

If you are on M1, you need to install the x86 versions of everything and run them through Rosetta 2, since Flutter does not support M1 yet. Start by installing Rosetta 2 if you haven't already:

```shell
/usr/sbin/softwareupdate --install-rosetta
```
Then, install an x86 version of brew to `/usr/local`:
```shell
arch -x86_64 zsh
cd /usr/local && mkdir homebrew
curl -L https://github.com/Homebrew/brew/tarball/master | tar xz --strip 1 -C homebrew
```
Then, you need to use the x86 brew to install the x86 version of llvm:
```shell
arch -x86_64 /usr/local/homebrew/bin/brew install llvm
```
Reference [this article](https://www.wisdomgeek.com/development/installing-intel-based-packages-using-homebrew-on-the-m1-mac/) for details.

And when you build with cargo, you need to select x86 as the target:

```shell
cargo build --target=x86_64-apple-darwin
```

## On M1 MacOS, ` Failed to load dynamic library '/opt/homebrew/opt/llvm/lib/libclang.dylib'`

Full possible error:

```
Invalid argument(s): Failed to load dynamic library '/opt/homebrew/opt/llvm/lib/libclang.dylib': dlopen(/opt/homebrew/opt/llvm/lib/libclang.dylib, 0x0001): tried: '/opt/homebrew/opt/llvm/lib/libclang.dylib' (mach-o file, but is an incompatible architecture (have 'arm64', need 'x86_64')), '/usr/lib/libclang.dylib' (no such file), '/opt/homebrew/Cellar/llvm/13.0.0_2/lib/libclang.dylib' (mach-o file, but is an incompatible architecture (have 'arm64', need 'x86_64')), '/usr/lib/libclang.dylib' (no such file)
```

Solution: Install the arm64 version of dart and put that in PATH instead of the x86_64 version that flutter ships. See https://github.com/dart-lang/ffigen/issues/260.

Related: https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1037718638

## Freezed file is sometimes not generated when it should be

If your `.freezed.dart` or `.g.dart` seems outdated, ensure you have run the `build_runner`.

Related: https://github.com/fzyzcjy/flutter_rust_bridge/issues/330

## `Can't create typedef from non-function type.`

Ensure min sdk version of Flutter `pubspec.yaml` is at least 2.13.0 to let `ffigen` happy.

https://github.com/fzyzcjy/flutter_rust_bridge/issues/334

## Generated code is so long

Indeed all generated code are necessary (if you find something that can be simplified, file an issue). Moreover, other code generation tools also generate long code - for example, when using Google protobuf, a very popular serialization library, I see >10k lines of Java code generated for a quite simple source proto file.

## Why need Dart `2.14.0`

Dart SDK `>=2.14.0` is needed not by this library, but by the latest version of the `ffigen` tool. Therefore, write `sdk: ">=2.14.0 <3.0.0"` in the `environment` section of `pubspec.yaml`. If you do not want that, consider installing a older version of the `ffigen` tool.

## Other problems?

Don't hesitate to [open an issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/new/choose)! I usually reply within minutes or hours (except when sleeping, of course).
