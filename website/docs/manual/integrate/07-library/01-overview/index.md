import V1Notice from '@site/docs/snippets/_v1-notice.mdx';

# Overview

<V1Notice />
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
This section is a legacy, fine-grained guide for library authors who want to own
their prebuilt binary distribution and CI workflow.
For new Flutter packages that can require a recent Flutter/Dart SDK with build hooks
and code assets, also consider the [Native Assets backend](../../native-assets).

This guide does not currently cover web support, but provides the necessary ground work
to support web in the future.
Feel free to PR to add web support to this guide!
