# Troubleshooting

## The generated store_dart_post_cobject() has the wrong signature / `'stdarg.h' file not found` in Linux / `stdbool.h` / ...

Try to run code generator with working directory at `/`, or set the environment variable:
```bash
export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
```
as described in [ffigen #257](https://github.com/dart-lang/ffigen/issues/257), or add include path as is described in [#108](https://github.com/fzyzcjy/flutter_rust_bridge/issues/108). This is a problem with Rust's builtin `Command`. See also: [#472](https://github.com/fzyzcjy/flutter_rust_bridge/issues/472) & [#494](https://github.com/fzyzcjy/flutter_rust_bridge/issues/494).

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
You can install llvm using `brew install llvm` and it will be installed at `/usr/local/homebrew/opt/llvm/` by default.

## Freezed file is sometimes not generated when it should be

If your `.freezed.dart` or `.g.dart` seems outdated, ensure you have run the `build_runner`.

Related: https://github.com/fzyzcjy/flutter_rust_bridge/issues/330

## `Can't create typedef from non-function type.`

Ensure min sdk version of Flutter `pubspec.yaml` is at least 2.17.0 to let `ffigen` happy.

https://github.com/fzyzcjy/flutter_rust_bridge/issues/334

## Imported from both `bridge_definitions.dart` and `bridge_generated.io.dart`

If you use a Rust type with `Kind` in it's name it may conflict with some generated types which can cause a duplicate import error. The workaround is to avoid using `Kind` as a suffix for a type name in Rust. See issue #757 for more details.

## Error on iOS TestFlight only (`store_dart_post_cobject`)

You may have an iOS app that works fine in Debug and Release modes locally but when deployed to TestFlight an error occurs trying to locate the `store_dart_post_cobject` - this is because the nested XCode project for the native bindings maybe be stripping symbols from the linked product.

Select the scheme (eg: `Product > Scheme > native-staticlib`) and go to *Build Settings* then under the `Deployment` section change `Strip Linked Product` to `No`; you may also need to change `Strip Style` to `Debugging Symbols`.

## Generated code is so long

Indeed all generated code are necessary (if you find something that can be simplified, file an issue). Moreover, other code generation tools also generate long code - for example, when using Google protobuf, a very popular serialization library, I see >10k lines of Java code generated for a quite simple source proto file.

## Why need Dart `2.17.0`

Dart SDK `>=2.15.0` is supported by this library, but by the latest version of the `ffigen` tool requires `>=2.17.0`. Therefore, write `sdk: ">=2.17.0 <3.0.0"` in the `environment` section of `pubspec.yaml`. If you do not want that, consider installing a older version of the `ffigen` tool.

## Why doesn't `flutter_rust_bridge_serve` work on Firefox?

This is a known issue stemming from Firefox's stricter rules regarding cross-origin requests. Use Chromium for testing, and check out
[this guide on enabling `crossOriginIsolated`](https://web.dev/cross-origin-isolation-guide/) for your production servers.

## "android context was not initialized", or `ndk_context` initialization.

Related issue: [#1323](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1323).

On android, when attempting to use crates that interact with the JavaVM through the JNI (like oboe-rs via cpal), you may get panics that typically have this message:

```
[ERROR:flutter/runtime/dart_vm_initializer.cc(41)] Unhandled Exception: FfiException(PANIC_ERROR, android context was not initialized, null)
```

This is due to a interesting quirk of Rust NDK interaction, where the [`ndk_context`](https://github.com/rust-mobile/ndk-context) crate does not have it's JVM and Android context initialized. Typically, in a normal application, the Android JVM would `System.loadLibrary()` the library through the Activity inside the JVM. It looks for symbols related to the JNI and executes them in accordance with the JNI standard. This would initialize the `ndk_context` normally via `JNI_OnLoad`. However, using the DartVM this step is skipped while loading the library, as the DartVM is not the JVM. So, the Android specific variables are not initialized, and therefore you cannot interact with the system via the Java interface.

### MainActivity.kt 

Add these lines to your FlutterActivity subclass:

```kotlin
package com.example.frontend

import io.flutter.embedding.android.FlutterActivity

// https://github.com/dart-lang/sdk/issues/46027
class MainActivity : FlutterActivity() {
    // this `init` block, where "foo" is the name of your library
    // ex: if it's libfoo.so, then use "foo"
    init {
        System.loadLibrary("foo")
    }
}
```

This handles loading the library before Dart does, and also executes the JNI related initialization.

### Rust

#### Cargo.toml

```toml
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
ndk-context = "0.1"
```

#### lib.rs

```rust
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint {
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}
```

This is the bit of JNI glue that allows for `ndk_context` to be initialized.

## "Could not resolve symbol __cxa_pure_virtual", or libc++_shared issues.

At the time of writing this, linking with `libc++_static` or not linking at all may lead to symbol resolution errors when launching the flutter application, after loading your dynamic library. Adding a fix is quite easy, create a build.rs script in the root of your Rust code:

### build.rs

```rust
fn main() {
    #[cfg(target_os = "android")]
    println!("cargo:rustc-link-lib=c++_shared");
}
```

Then, in each `jniLibs` architecture directory, put the corresponding `libc++_shared.so` from the Android NDK. `libc++_shared.so` is typically located in `$ANDROID_NDK/toolchains/llvm/prebuilt/`. You will have to search for it, as it's different for each operating system.

* arm-linux-androideabi -> armeabi-v7a
* aarch64-linux-android -> arm64-v8a
* i686-linux-android -> x86
* x86_64-linux-android -> x86_64

## Issues on Web?

Check out [Limitations on WASM](./wasm_limitations.md) for some common problems and solutions
to adapt existing code to WASM.

## Other problems?

Don't hesitate to [open an issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/new/choose)! I usually reply within minutes or hours (except when sleeping, of course).
