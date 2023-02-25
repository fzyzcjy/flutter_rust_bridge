# Android setup

## JDK 8

Android Studio depends on the `javax` library being present in the Java runtime, and the only reliable way to ensure this is to install an older version of Java. On Unix-like systems, you can use [asdf](https://asdf-vm.com/) or similar tools to manage your Java versions, and the template defines a known working version of Java in the `.tool-versions` file.

## Android NDK

> Android Studio > SDK Manager > SDK Tools > uncheck Hide Obsolete Packages > NDK (version 22)

The [Android NDK], or Native Development Kit, enables code written in other
languages to be run on the JVM via the [Java Native Interface], or JNI for short. In this case, we would like to pass the dynamic libraries created by Cargo to be included in the bundle when we run or build the project.

After following the instructions above, the NDK should be installed in your `$ANDROID_HOME/ndk` folder, where `ANDROID_HOME` usually is:
- on Windows: `%APPDATA%\Local\Android\sdk`
- on MacOS: `~/Library/Android/sdk`
- on Linux: set via the environment variable `ANDROID_HOME`, or `~/Android/sdk`

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
cargo install cargo-ndk --version 2.6.0
```

[cargo-ndk] is a Cargo plugin for compiling code suitable for plugging into
the JNI without additional configuration. Run the above command to install.
Version 2.7.0 of cargo-ndk introduced changes that broke support for NDK
version 22 so 2.6.0 must be used for now. If you still want to use 2.7.0 with
a workaround, see [this article](./alternative_ndk.md)

Then run 
```shell
cargo ndk -o ../android/app/src/main/jniLibs build`
```
Then run the Flutter app normally with `flutter run`.

**Remark**: [This tutorial](https://stackoverflow.com/q/69515032/4619958) will help you automatically execute `cargo` builds when building Flutter app.

