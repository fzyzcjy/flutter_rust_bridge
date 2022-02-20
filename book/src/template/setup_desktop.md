# Windows and Linux

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

## Troubleshooting: CMake on Linux

The minimum version of CMake required to use [Corrosion] is 3.12, which is not the version set
by default in `CMakeLists.txt`. You will need to modify this line in `linux/CMakeLists.txt`:

```diff
-cmake_minimum_required(VERSION 3.10)
+cmake_minimum_required(VERSION 3.12)
```

However, this has a separate issue of disallowing Flutter SDK installations via Snap to build,
as these are bundled with a non-upgradeable CMake 3.10. If possible, it is recommended to install
Flutter manually using the command line. This issue may be resolved once
[canonical/flutter-snap#61](https://github.com/canonical/flutter-snap/pull/61) lands.

A workaround is to ignore `rust.cmake` and manually configure CMake to build and bundle the Rust library, as suggested by
[this comment](https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1038751426)
in the case of Flutter on ARM Linux.

[Corrosion]: https://github.com/corrosion-rs/corrosion
[this guide]: https://github.com/corrosion-rs/corrosion#installation
