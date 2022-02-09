# Android setup

For Android, a few components are required to get started:

## Android NDK

> Android Studio > SDK Manager > SDK Tools > uncheck Hide Obsolete Packages > NDK (version 22)

The [Android NDK], or Native Development Kit, enables code written in other
languages to be run on the JVM via the [Java Native Interface], or JNI for short. In this case, we would like to pass the dynamic libraries created by Cargo to be included in the bundle when we run or build the project.

[An issue] regarding building Rust's `core` library against the latest NDK means that for now only NDK versions 22 and older can be used.

## `ANDROID_NDK` Gradle property

> ```shell
> echo "ANDROID_NDK=.." >> ~/.gradle/gradle.properties
> ```

Next, you need to make this NDK visible to Gradle. The way to do this depends on your current system and is unlikely to be portable, but generally you can add a `gradle.properties` in your `~/.gradle` folder like this:

```
ANDROID_NDK=(path to NDK)
```

or edit one of the `gradle.properties` that resides within the `android` folder.

[Android NDK]: https://developer.android.com/ndk
[Java Native Interface]: https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/jniTOC.html
[An issue]: https://github.com/rust-lang/rust/pull/85806