# Android setup

For Android, a few components are required to get started:

## Rust targets

If you have not already done so, cross-compiling to Android requires some additional
targets which can easily be added:

```shell
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

## JDK 8

Android Studio depends on the `javax` library being present in the Java runtime, and the only reliable way to ensure this is to install an older version of Java. On Unix-like systems, you can use [asdf](https://asdf-vm.com/) or similar tools to manage your Java versions, and the template defines a known working version of Java in the `.tool-versions` file.

## Android NDK

> Android Studio > SDK Manager > SDK Tools > uncheck Hide Obsolete Packages > NDK (version 22)

The [Android NDK], or Native Development Kit, enables code written in other
languages to be run on the JVM via the [Java Native Interface], or JNI for short. In this case, we would like to pass the dynamic libraries created by Cargo to be included in the bundle when we run or build the project.

After following the instructions above, the NDK should be installed in your `$ANDROID_SDK_HOME/ndk` folder, where ANDROID_SDK_HOME usually is:
- on Windows: `%APPDATA%\Local\Android\sdk`
- on MacOS: `~/Library/Android/sdk`
- on Linux: set via the environment variable ANDROID_SDK_HOME, or `~/Android/sdk`

[An issue] regarding building Rust's `core` library against the latest NDK means that as of writing only NDK versions 22 and older can be used.

## `ANDROID_NDK` Gradle property

```shell
echo "ANDROID_NDK=(path to NDK)" >> ~/.gradle/gradle.properties
```

Next, you need to make this NDK visible to Gradle. The way to do this depends on your current system and is unlikely to be portable, but generally you can add a `gradle.properties` in your `~/.gradle` folder like this:

```
ANDROID_NDK=(path to NDK)
```

or edit one of the `gradle.properties` that resides within the `android` folder.

## [cargo-ndk]
```shell
cargo install cargo-ndk
```

[cargo-ndk] is a Cargo plugin for compiling code suitable for plugging into
the JNI without additional configuration. Run the above command to install.


[Android NDK]: https://developer.android.com/ndk
[Java Native Interface]: https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/jniTOC.html
[An issue]: https://github.com/rust-lang/rust/pull/85806
[cargo-ndk]: https://github.com/bbqsrc/cargo-ndk