# Integrating with Windows and Linux

This guide groups together instructions for Windows and Linux desktop apps,
as they use the same build system.

The idea is the same as other platforms: we hook onto the existing projects
using scripts, and we will also be borrowing from the template. Go ahead
and download [rust.cmake](https://raw.githubusercontent.com/Desdaemon/flutter_rust_bridge_template/main/windows/rust.cmake)
into your `windows` and `linux` folders. Keep in mind that CMake will refuse
to use files that lie outside of its working directory, so there will be duplications
between the two build folders.

Next, add this line to your `CMakeLists.txt` files:

```diff
 # Generated plugin build rules, which manage building the plugins and adding
 # them to the application.
 include(flutter/generated_plugins.cmake)

+include(./rust.cmake)

 # === Installation ===
 # Support files are copied into place next to the executable, so that it can
```

## Linux

On Linux, you will need to bump the minimum CMake version to 3.12 to make use
of [Corrosion](https://github.com/corrosion-rs/corrosion), which is used by `rust.cmake`. Change this line in `linux/CMakeLists.txt`:

```diff
-cmake_minimum_required(VERSION 3.10)
+cmake_minimum_required(VERSION 3.12)
```

Alternatively, you can install Corrosion permanently on your system.
Refer to the [Linux troubleshooting notes](../template/setup_desktop.md) here.
