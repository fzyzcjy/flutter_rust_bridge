# Setup

## Dependencies
To start developing your Dart/Flutter library, you will need to download some dependencies locally.

### Required
The rest of this guide assumes you have the following tools installed on any development machines:
- [Flutter](https://docs.flutter.dev/get-started/install)
- [rustup](https://rustup.rs)
- [Melos](https://melos.invertase.dev) (needed for our monorepo, see [here](melos.md))
  - `dart pub global activate melos` to install once Dart/Flutter are installed

### Optional
If you would like to build your binaries (for Flutter devices) locally in addition to CI 
(say, to test on a real device or emulator), you will additionally need the following:
- To compile to macOS/iOS targets
  - macOS
- To cross-compile to Android targets
  - [Android NDK](https://developer.android.com/ndk/downloads)
    - Most NDK versions should work nowadays due to fixes in `cargo-ndk`
      - Previously, NDK version 21 (`r21e`) was the only one that could be used easily
        - You might see reference to this elsewhere, but that is largely out of date
      - NDK version 25 (`r25b`) was working at the time of writing this documentation
- To cross-compile to Windows/Linux targets
  - [Zig](https://ziglang.org/learn/getting-started/#installing-zig)
  - llvm (with `clang-cl`!)
    - Need to run `brew install llvm` on macOS since Apple's llvm doesn't have it

## Repository Structure
We will be using the following structure for our repository, assuming our library name is `library_name`:
- `.github/` for CI/CD (with GitHub Actions) & dependabot
- `packages/` where our Flutter/Dart packages will live
  - `library_name/` the Dart-only (library) package using flutter_rust_bridge (FRB)
    - `native/` the Rust library used by Dart
    - `test/` unit tests for our Dart-only library
    - `example/` an example project showing how to use `library_name` from Dart-only
      - `test/` (optional) tests for the example; can be used to ensure example continues to work in CI
  - `flutter_library_name/` the Flutter (library) package wrapping around `library_name` for ease of use
    - `android/`, `ios/`, `linux/`, `macos/`, & `windows/` for platform-specific wrappers in order to bundle our library binaries with Flutter applications
    - `test/` unit tests for our Flutter library (note: there might not be any if your Flutter library does not add any Flutter-specific functionality; in that case, add a dummy test in so CI is happy)
    - `example/` an example project showing how to use `flutter_library_name` from within a Flutter application
      - `integration_test/` integration tests to ensure your Flutter library, example, and platform-specific configuration are all working together correctly
- `scripts/` build Flutter binaries and handle release creation
- `platform-build/` the output (build) folder for all created Flutter binaries
- `analysis_options.yaml` to enable consistent Dart analysis in our Dart/Flutter libraries
- `Cargo.toml` so IDEs can find our Rust project under `packages/library_name/native`
- `melos.yaml` to configure the monorepo, see more [here](library/melos.md)
