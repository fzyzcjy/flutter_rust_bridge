# Integrating with Desktop

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

> **Note** \
> This method depends on `FetchContent` and co. being available, which
> means that users with CMake versions older than 3.11 will have to install Corrosion
> permanently on their system. See [additional notes here](../template/setup_others.md).
