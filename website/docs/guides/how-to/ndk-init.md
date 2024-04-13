# Android NDK Init

This page is only needed to read if you want to use Android NDK in Rust code,
and see "android context was not initialized" error, or want to know more about`ndk_context` initialization.

Related issues:
[#1323](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1323),
[#1868](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1868).

## Method 1

On android, when attempting to use crates that interact with the JavaVM through the JNI (like oboe-rs via cpal), you may
get panics that typically have this message:

```
[ERROR:flutter/runtime/dart_vm_initializer.cc(41)] Unhandled Exception: FfiException(PANIC_ERROR, android context was not initialized, null)
```

This is due to a interesting quirk of Rust NDK interaction, where
the [`ndk_context`](https://github.com/rust-mobile/ndk-context) crate does not have it's JVM and Android context
initialized. Typically, in a normal application, the Android JVM would `System.loadLibrary()` the library through the
Activity inside the JVM. It looks for symbols related to the JNI and executes them in accordance with the JNI standard.
This would initialize the `ndk_context` normally via `JNI_OnLoad`. However, using the DartVM this step is skipped while
loading the library, as the DartVM is not the JVM. So, the Android specific variables are not initialized, and therefore
you cannot interact with the system via the Java interface.

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

```
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

## Method 2

Alternative method, if you require direct access to the current Android context, without creating another one.

### MainActivity.kt

Add these lines to your FlutterActivity subclass, as well as create a Plugin class:

```kotlin
package com.example.frontend

import android.content.Context
import androidx.annotation.NonNull
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

class MainActivity : FlutterActivity() {
    override fun configureFlutterEngine(
        @NonNull flutterEngine: FlutterEngine,
    ) {
        super.configureFlutterEngine(flutterEngine)
        flutterEngine.plugins.add(MyPlugin())
    }
}

class MyPlugin : FlutterPlugin, MethodCallHandler {
    companion object {
        init {
            System.loadLibrary("rust_lib_frontend")
        }
    }

    external fun init_android(ctx: Context)

    override fun onAttachedToEngine(
        @NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding,
    ) {
        init_android(flutterPluginBinding.applicationContext)
    }

    override fun onMethodCall(
        @NonNull call: MethodCall,
        @NonNull result: Result,
    ) {
        result.notImplemented()
    }

    override fun onDetachedFromEngine(
        @NonNull binding: FlutterPlugin.FlutterPluginBinding,
    ) {
    }
}
```

### Rust

#### Cargo.toml

```
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
```

#### lib.rs

```rust
#[cfg(target_os = "android")]
use {
    jni::{objects::JClass, objects::JObject, JNIEnv},
    mylib::setup_android,
};

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_example_spareshare_MyPlugin_init_1android(
    env: JNIEnv,
    _class: JClass,
    ctx: JObject,
) {
    setup_android(env, ctx);
}
```