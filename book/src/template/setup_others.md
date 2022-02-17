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

## Apple Silicon hosts

For Apple Silicon hosts building Flutter apps, make sure to read [these notes](https://github.com/flutter/flutter/wiki/Developing-with-Flutter-on-Apple-Silicon)
on the official Flutter repo.

As of writing (Feb 2022) it is possible to build Flutter apps for other architectures using
the arm64 Dart SDK, albeit unofficially. Using [`flutter_m1_patcher`](https://pub.dev/packages/flutter_m1_patcher)
will replace the Dart SDK bundled with Flutter with your installation of Dart SDK via Homebrew.
You will need to repeat this process after every `flutter upgrade`:

```bash
dart pub global activate flutter_m1_patcher
flutterpatch
```

This process will be superseded by [flutter/flutter#60118](https://github.com/flutter/flutter/issues/60118) once
it lands.

[Corrosion]: https://github.com/corrosion-rs/corrosion
[this guide]: https://github.com/corrosion-rs/corrosion#installation
