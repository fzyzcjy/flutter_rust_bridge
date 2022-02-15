# Summary

# Core Doc

- [Introduction](index.md)
- [🧭 Quickstart](quickstart.md)
- [📚 Tutorial: A Flutter+Rust app](tutorial_with_flutter.md)
- [Feature details](feature_details.md)

# User Guide
- [Creating a new project](template.md)
  - [Android setup](template/setup_android.md)
  - [iOS setup](template/setup_ios.md)
  - [Web setup](template/setup_web.md)
  - [Other platforms](template/setup_others.md)
- [Template tour](tour.md)
  - [native/src/api.rs](tour/api.md)
  - [`justfile`](tour/justfile.md)
  - [Build scripts](tour/scripts.md)
  - [`rust.cmake`](tour/cmake.md)
- [Generating code](generate.md)
  - [Installing codegen](generate/install.md)
  - [Adding new code](generate/adding_code.md)
  - [Using build_runner](generate/build_runner.md)
  - [Wrapping up](generate/finish.md)
- [Integrating with existing projects](integrate.md)
  - [Creating a new crate](integrate/new_crate.md)
  - [Installing dependencies](integrate/deps.md)
  - [Integrating with Android](integrate/android.md)
    - [Hooking onto tasks](integrate/android_tasks.md)
    - [CMake with Gradle](integrate/android_cmake.md)
  - [Integrating with iOS](integrate/ios.md)
    - [Retrieving build scripts](integrate/ios_scripts.md)
    - [Configuring build settings](integrate/ios_config.md)
    - [Adding build phases](integrate/ios_phases.md)
    - [Generating bindings](integrate/ios_gen.md)
    - [Using dummy headers](integrate/ios_headers.md)
    - [Linking the library](integrate/ios_linking.md)
  - [Integrating with MacOS](integrate/macos.md)
  - [Integrating with Desktop](integrate/desktop.md)
  - [Integrating with Web](integrate/web.md)
  - [Using the dynamic library](integrate/usage.md)

# More Doc

- [Tutorial: Pure Dart](tutorial_pure_dart.md)
- [Safety concerns](safety.md)
- [Troubleshooting](troubleshooting.md)
- [Command line arguments](command_line.md)
- [Set up Flutter/Dart+Rust support from scratch](set_up_from_scratch.md)
- [Articles](article.md)
  - [Async in Rust](article/async_in_rust.md)

