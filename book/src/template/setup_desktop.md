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
-    GIT_TAG v0.4.4 # Optionally specify a version tag or branch here
-)
-
-FetchContent_MakeAvailable(Corrosion)
```

## Troubleshooting: CMake on Linux

Unless you have certain use-cases that you require from the latest versions of [Corrosion], it is recommended to
use **v0.4.x** in your build scripts since CMake versions supplied by the system and/or Flutter snap installations
trail behind the master branch. This will relax the CMake requirement to v15, which should be generally available
from most package maintainers and Flutter snaps.

A workaround is to ignore `rust.cmake` and manually configure CMake to build and bundle the Rust library, as suggested by
[this comment](https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1038751426)
in the case of Flutter on ARM Linux.

[Corrosion]: https://github.com/corrosion-rs/corrosion
[this guide]: https://github.com/corrosion-rs/corrosion#installation
