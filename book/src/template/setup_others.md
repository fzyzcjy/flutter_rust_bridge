# Other platforms

For all remaining platforms, there are no required setup steps to take, apart from those listed in [Desktop support for Flutter](https://docs.flutter.dev/desktop). If you need to check your progress, run `flutter doctor -v` and it will display the status of your toolchain and any actionable steps. The rest of this page
documents additional hints for each of the platforms that might be useful for newcomers to
Flutter and/or Rust.

## Windows and Linux

Windows and Linux share the same build system (CMake), making setup for these two
platforms the easiest even from scratch. The template uses [Corrosion] to expedite the process, which has to clone and initialize the builder first. If you are running builds continuously, it might be a good idea to follow [this guide] to learn how to install [Corrosion] permanently onto your system. Once that's done, go ahead and modify `rust.cmake` in `windows` and `linux`:

```diff
-# find_package(Corrosion REQUIRED)
+find_package(Corrosion REQUIRED)

-include(FetchContent)
-
-FetchContent_Declare(
-    Corrosion
-    GIT_REPOSITORY https://github.com/AndrewGaspar/corrosion.git
-    GIT_TAG origin/master # Optionally specify a version tag or branch here
-)
-
-FetchContent_MakeAvailable(Corrosion)
```

[Corrosion]: https://github.com/corrosion-rs/corrosion
[this guide]: https://github.com/corrosion-rs/corrosion#installation

## Simulator target for iOS

As of XCode 13, the simulator is a separate target from the default iOS target (`aarch64-apple-ios-sim`).
Among other things, this means that [cargo-lipo] is no longer sufficient nor necessary to create the appropriate binaries. This template uses the techniques demonstrated in the [rust-on-ios] repository and adapted for this particular use-case, but here's the basic gist (details explained in a separate page):

1. Build the *static library* as part of the build process
2. Link the library to the binary
3. Import and use generated dummy headers, to prevent symbols being dropped

[cargo-lipo]: https://github.com/TimNN/cargo-lipo
[rust-on-ios]: https://github.com/brotskydotcom/rust-on-ios