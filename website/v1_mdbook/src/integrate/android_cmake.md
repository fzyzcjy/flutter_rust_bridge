# CMake with Gradle

If you have taken a look at either the `windows` or `linux` folder,
you will see a file named `CMakeLists.txt`. This is the definition file
for the CMake toolchain that Flutter uses to build Windows and Linux apps.
You can also use this strategy on Gradle, but this setup is beyond the scope
of this guide and reserved for advanced tinkers.

Refer to the [Add C and C++ code to your project](https://developer.android.com/studio/projects/add-native-code)
page on the official Android docs, modify around C-specific parts and use a
tool like [`Corrosion`](https://github.com/corrosion-rs/corrosion) to integrate
with Cargo. The advantage of this setup is that you can reuse your C tools
*and* benefit from various techniques such as caching builds.
