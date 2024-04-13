# Troubleshooting

## Linker complains undefined symbols

e.g. `ld: Undefined symbols: std::__1::basic_string ...`

Please change the following line in iOS/macos podspec file:

```
'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/librust_lib.a',
```

To something like:

```
'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/librust_lib.a -lc++',
```

Or other libraries that you need (e.g. `-framework SystemConfiguration`, `-framework AudioToolbox`, etc).

Please refer to these issues for more
details: [cargokit#52](https://github.com/irondash/cargokit/issues/52), [bridge#1610](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1610).

Another example, which solves the error when using `cpal`, can be found in [#1835](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1835#issuecomment-2020299630).

## Issues with glibc (e.g. `version GLIBC_2.33 not found`)

As is discussed in [this issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1753#issuecomment-1948241309)
and [this issue](https://github.com/irondash/cargokit/issues/55):

> This is a https://github.com/canonical/flutter-snap/issues/109 with Flutter Snap not being able to do LTO builds. Just
> install the Flutter regular way (without snap) and it should work.

## Issue with `store_dart_post_cobject`

If calling rust function gives the error below, please consider running **cargo build** again. This can happen when the
generated rs file is not included when building is being done.

```sh
[ERROR:flutter/lib/ui/ui_dart_state.cc(209)] Unhandled Exception: Invalid argument(s): Failed to lookup symbol 'store_dart_post_cobject': target/debug/libadder.so: undefined symbol: store_dart_post_cobject
```

## Error running `cargo ndk`: `ld: error: unable to find library -lgcc`

Downgrade Android NDK to version 22. This is an [ongoing issue](https://github.com/bbqsrc/cargo-ndk/issues/22)
with `cargo-ndk`, a library unrelated to flutter_rust_bridge but solely used to build the examples, when using Android
NDK version 23. (See [#149](https://github.com/fzyzcjy/flutter_rust_bridge/issues/149))

## Fail to run `flutter_rust_bridge_codegen` on MacOS, "Please supply one or more path/to/llvm..."

If you are running macOS, you will need to specify a path to your llvm:

```shell
flutter_rust_bridge_codegen --rust-input path/to/your/api.rs --dart-output path/to/file/being/bridge_generated.dart --llvm-path /usr/local/homebrew/opt/llvm/
```

You can install llvm using `brew install llvm` and it will be installed at `/usr/local/homebrew/opt/llvm/` by default.

## Freezed file is sometimes not generated when it should be

If your `.freezed.dart` or `.g.dart` seems outdated, ensure you have run the `build_runner`.

Related: https://github.com/fzyzcjy/flutter_rust_bridge/issues/330

## `Can't create typedef from non-function type.`

Ensure min sdk version of Flutter `pubspec.yaml` is at least 2.17.0 to let `ffigen` happy.

https://github.com/fzyzcjy/flutter_rust_bridge/issues/334

## Imported from both `bridge_definitions.dart` and `bridge_generated.io.dart`

If you use a Rust type with `Kind` in it's name it may conflict with some generated types which can cause a duplicate
import error. The workaround is to avoid using `Kind` as a suffix for a type name in Rust. See issue #757 for more
details.

## Error on iOS TestFlight only (`store_dart_post_cobject`)

You may have an iOS app that works fine in Debug and Release modes locally but when deployed to TestFlight an error
occurs trying to locate the `store_dart_post_cobject` - this is because the nested XCode project for the native bindings
maybe be stripping symbols from the linked product.

Select the scheme (eg: `Product > Scheme > native-staticlib`) and go to _Build Settings_ then under the `Deployment`
section change `Strip Linked Product` to `No`; you may also need to change `Strip Style` to `Debugging Symbols`.

## Generated code is so long

Indeed all generated code are necessary (if you find something that can be simplified, file an issue). Moreover, other
code generation tools also generate long code - for example, when using Google protobuf, a very popular serialization
library, I see >10k lines of Java code generated for a quite simple source proto file.

## Why need Dart `2.17.0`

Dart SDK `>=2.15.0` is supported by this library, but by the latest version of the `ffigen` tool requires `>=2.17.0`.
Therefore, write `sdk: ">=2.17.0 <3.0.0"` in the `environment` section of `pubspec.yaml`. If you do not want that,
consider installing a older version of the `ffigen` tool.

## Why doesn't `flutter_rust_bridge_serve` work on Firefox?

This is a known issue stemming from Firefox's stricter rules regarding cross-origin requests. Use Chromium for testing,
and check out
[this guide on enabling `crossOriginIsolated`](https://web.dev/cross-origin-isolation-guide/) for your production
servers.

## "Could not resolve symbol \_\_cxa_pure_virtual", or libc++\_shared issues.

At the time of writing this, linking with `libc++_static` or not linking at all may lead to symbol resolution errors
when launching the flutter application, after loading your dynamic library. Adding a fix is quite easy, create a
build.rs script in the root of your Rust code:

### build.rs

```rust
fn main() {
    #[cfg(target_os = "android")]
    println!("cargo:rustc-link-lib=c++_shared");
}
```

Then, in each `jniLibs` architecture directory, put the corresponding `libc++_shared.so` from the Android
NDK. `libc++_shared.so` is typically located in `$ANDROID_NDK/toolchains/llvm/prebuilt/`. You will have to search for
it, as it's different for each operating system.

- arm-linux-androideabi -> armeabi-v7a
- aarch64-linux-android -> arm64-v8a
- i686-linux-android -> x86
- x86_64-linux-android -> x86_64

## Issues on Web?

Check out [Limitations on WASM](./miscellaneous/wasm-limitations) for some common problems and solutions
to adapt existing code to WASM.

## Other problems?

Don't hesitate to [open an issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/new/choose)! I usually reply
within minutes or hours (except when sleeping, of course).
