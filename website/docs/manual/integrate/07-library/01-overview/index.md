# Overview
This section of the user guide will walk you through the entire process of
making a Dart-only library base with a Flutter library built on top of it.

In the end, if you choose to publish to pub.dev, users can simply run:
- `flutter pub add flutter_library_name` for Flutter
- `dart pub add library_name` for Dart-only (say for a CLI or server app)

But you also don't need to publish your library either!
You can just use your library internally in a monorepo,
[as described later on](overview/melos).

## High Level Design
We will build out our library from scratch, piece by piece, allowing you to include
only what you want in your library.
It is intended that whenever a code snippet/file is shown, you read and understand the 
content of the snippet so that you can work with your library more easily in the future.
We will create the following components:
- Dart-only library
- Flutter library wrapping the Dart-only library
- CI/CD with GitHub Actions
  - Dart/Flutter unit & integration testing
    - Test your code on all supported Flutter platforms automatically!
  - Automate release creation
    - Release library binaries through GitHub releases
    - Create pub.dev releases automatically on GitHub release
- Custom build system to cross-compile to all supported Flutter platforms
  - You need a Mac to compile to macOS/iOS (at least locally)
    - But CI can handle your compilation/releases if you don't have a Mac!
  - Can cross-compile to all other Flutter platforms, no matter your dev device

For a full working example (that this guide was created based on!),
take a look at [mimir](https://github.com/GregoryConrad/mimir) (which is created by the author of this chapter of document).
It incorporates all functionality present in this guide and some more.

## Warning!
Please note, this entire section will be outdated & need to be overhauled once
["Native Assets"](https://github.com/dart-lang/sdk/issues/50565) are added to Dart.
A lot of the techniques described here are merely workarounds until Dart supports Native Assets.

Also, this guide does not currently cover web support, but provides the necessary ground work
to support web in the future.
It *may* be worth waiting for Native Assets before trying to come up with a custom solution for WASM.
Feel free to PR to add web support to this guide!
