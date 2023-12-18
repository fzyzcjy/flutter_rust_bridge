# Alternative NDK setup

This is only needed if you wish to use a version of the Android NDK higher than version 22 with versions of Rust that are lower than version 1.68.
This guide details how to prevent the [unable to find library -lgcc error].

## Android NDK
Install the latest NDK:

> Android Studio > SDK Manager > SDK Tools > NDK (Side by side)

Click on OK at the bottom right corner to start the installation.

## [cargo-ndk]
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

2. Create 4 text files named `libgcc.a` in the four folders mentioned above with these contents

   ```
   INPUT(-lunwind)
   ```

## More details on NDK with flutter_rust_bridge
For more details on how NDK works with `flutter_rust_bridge`, have a look at this article (integrate/android_tasks) please.

[cargo-ndk]: https://github.com/bbqsrc/cargo-ndk
[unable to find library -lgcc error]:https://github.com/bbqsrc/cargo-ndk/issues/22
[reference]: https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946
