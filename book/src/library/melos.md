# Monorepo with Melos

This page covers the basics of how to setup and use [Melos](https://melos.invertase.dev),
a Dart/Flutter focused monorepo tool, so you can use it to manage your own monorepo.

While you can, in theory, go about this guide without using Melos,
_using Melos will save you a lot of time and headache_.
There is some slight upfront cost of configuring and learning how Melos works,
but it pays off substantially in the long-term.

The rest of this guide assumes you are using Melos, so here are the steps to setup Melos,
along with some common commands.

It is also highly recommended that you read the Melos documentation linked above, as this
page only covers the bare minimum and it is likely you will want to do more than listed here.

## `/melos.yaml`

Here is a sample of Melos' configuration file to get you started:

```yaml
name: library_name

repository: https://github.com/YourGitHubAccount/library_name

packages:
  - packages/**

scripts:
  analyze:
    exec: flutter analyze .
    description: Analyze a specific package in this project.

  check-format:
    exec: dart format --set-exit-if-changed .
    description: Check the format of a specific package in this project.

  format:
    exec: dart format .
    description: Format a specific package in this project.

  version:
    description: Updates version numbers in all build files
    run: bash scripts/version.sh

  build:
    run: melos run build:apple && melos run build:android && melos run build:other
    description: Build all native libraries for the project.

  build:apple:
    run: bash scripts/build-apple.sh
    description: Build the XCFramework for iOS and macOS.

  build:android:
    run: bash scripts/build-android.sh
    description: Build the .tar.gz for Android.

  build:other:
    run: bash scripts/build-other.sh
    description: Build the .tar.gz for all other platforms.

  test:
    run: melos run test:dart --no-select && melos run test:flutter --no-select
    description: Run all Dart & Flutter tests in this project.

  test:dart:
    run: melos exec -c 1 --fail-fast -- "dart test test"
    description: Run Dart tests for a specific package in this project.
    select-package:
      flutter: false
      dir-exists: test

  test:flutter:
    run: melos exec -c 1 --fail-fast -- "flutter test test"
    description: Run Flutter tests for a specific package in this project.
    select-package:
      flutter: true
      dir-exists: test
```

You can run the melos "scripts" defined in this file with `melos run ...`,
e.g. `melos run build:android` to build a .tar.gz for Android devices.
Also, when you first setup your Melos repo, you will need to run `melos bootstrap` (or `melos bs` for short).
To clean your repo in the future, you can run `melos clean && melos bs`.

## `/scripts/version.sh`

Every time you need to make a new release of your library, Melos will take care of the heavy lifting for you.
Melos creates new versions via the simple command, `melos version`.
`melos version` creates and manages git tags, in addition to automatically incrementing the version numbers appropriately.

Since we are distributing our binaries separately from the Dart/Flutter packages on pub.dev, we take advantage of
a special "melos script" defined in the configuration file, named "version".
In this versioning script, we change the version numbers for our Flutter build process so that consumers of our library
will always get the binaries associated with their version.

Replace all instances of `library_name` below with your library name.

```bash
#!/bin/bash

CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_library_name/ios/flutter_library_name.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_library_name/macos/flutter_library_name.podspec
rm packages/flutter_library_name/macos/*.bak packages/flutter_library_name/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_library_name/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_library_name/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_library_name/
```

## Conventional Commits

For Melos versioning to work, which our monorepo relies on to distribute binaries properly,
you need to use "conventional commits."
If you are not familiar with conventional commits, that is ok.
Simply read up on conventional commits in the [Melos guide](https://melos.invertase.dev/guides/automated-releases#versioning).
