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
cargo install cargo-ndk --version 2.6.0
```

[cargo-ndk] is a Cargo plugin for compiling code suitable for plugging into
the JNI without additional configuration. Run the above command to install.
Version 2.7.0 of cargo-ndk introduced changes that breaked support for NDK
version 22 so 2.6.0 must be used for now.

## Alternative NDK setup

You can alternatively use the latest version of the Android NDK which is greater than 22.
However, this requires a hack to prevent the [unable to find library -lgcc error].

### Android NDK
Install the latest NDK:

> Android Studio > SDK Manager > SDK Tools > NDK (Side by side)

Click on OK at the bottom right corner to start the installation.

### [cargo-ndk]
You should install `cargo-ndk` version 2.7.0 or above which works for
Android NDK versions greater than 22.

```
cargo install cargo-ndk --version ^2.7.0
```

A workaround may be under development in the cargo-ndk project. Until it is finished,
you need to manually create four text files to redirect calls from libgcc to libunwind ([reference]):

1. Find out all the 4 folders containing file `libunwind.a`.
   * On Windows, it is similar to:
      ```
      C:\Users\Administrator\AppData\Local\Android\Sdk\ndk\24.0.8215888\toolchains\llvm\prebuilt\windows-x86_64\lib64\clang\14.0.1\lib\linux\x86_64\
      ```
   
   * On macOS Monterey, it is similar to:
      ```
      ~/Library/Android/sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/darwin-x86_64/lib64/clang/14.0.1/lib/linux/x86_64/
      ```
   The three other folders end with `aarch64`, `arm`,    `i386` instead of `x86_64`.

2. Create 4 text files named `libgcc.a` in the four folders mentioned above with this contents
   
   ```
   INPUT(-lunwind)
   ```

[Android NDK]: https://developer.android.com/ndk
[Java Native Interface]: https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/jniTOC.html
[An issue]: https://github.com/rust-lang/rust/pull/85806
[cargo-ndk]: https://github.com/bbqsrc/cargo-ndk
[unable to find library -lgcc error]:https://github.com/bbqsrc/cargo-ndk/issues/22
[reference]: https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946