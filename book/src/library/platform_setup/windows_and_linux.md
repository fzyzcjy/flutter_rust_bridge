# Windows & Linux
Windows and Linux both use CMake for their build system,
so this page will walk you through setting up CMake for Windows/Linux.

Also, this page will introduce the windows & linux build script
to compile your Rust library to these platforms.

## CMake
CMake happens to be by far the easiest build process to set up
of of all the Flutter supported platforms.

Replace all instances of `library_name` below with your library name.
Also, replace other variables (i.e. `YourGitHubAccount` and `repo_name`) as needed.

### Linux CMakeLists.txt (`/packages/flutter_library_name/linux/CMakeLists.txt`)
```cmake
set(LibraryVersion "library_name-v0.0.0") # generated; do not edit

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "flutter_library_name")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}")
set(ArchivePath "${LibRoot}.tar.gz")
if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/YourGitHubAccount/repo_name/releases/download/${LibraryVersion}/other.tar.gz"
    ${ArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LibRoot})
file(MAKE_DIRECTORY ${LibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${ArchivePath}
  WORKING_DIRECTORY ${LibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(flutter_library_name_bundled_libraries
  "${LibRoot}/${FLUTTER_TARGET_PLATFORM}/liblibrary_name.so"
  PARENT_SCOPE
)
```

### Windows CMakeLists.txt (`/packages/flutter_library_name/windows/CMakeLists.txt`)
```cmake
set(LibraryVersion "library_name-v0.0.0") # generated; do not edit

# TODO Remove this workaround once Flutter supports Windows ARM.
# https://github.com/flutter/flutter/issues/116196
set(FLUTTER_TARGET_PLATFORM windows-x64)

# The Flutter tooling requires that developers have a version of Visual Studio
# installed that includes CMake 3.14 or later. You should not increase this
# version, as doing so will cause the plugin to fail to compile for some
# customers of the plugin.
cmake_minimum_required(VERSION 3.14)

# Project-level configuration.
set(PROJECT_NAME "flutter_library_name")
project(${PROJECT_NAME} LANGUAGES CXX)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}")
set(ArchivePath "${LibRoot}.tar.gz")
if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/YourGitHubAccount/repo_name/releases/download/${LibraryVersion}/other.tar.gz"
    ${ArchivePath}
    TLS_VERIFY ON
  )
endif()

# Extract the binaries, overriding any already present.
file(REMOVE_RECURSE ${LibRoot})
file(MAKE_DIRECTORY ${LibRoot})
execute_process(
  COMMAND ${CMAKE_COMMAND} -E tar xzf ${ArchivePath}
  WORKING_DIRECTORY ${LibRoot}
)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(flutter_library_name_bundled_libraries
  "${LibRoot}/${FLUTTER_TARGET_PLATFORM}/library_name.dll"
  PARENT_SCOPE
)
```

### Platform-Specific Peculiarities
There exists a few differences between the Linux and Windows `CMakeLists.txt`s:
1. The minimum CMake version supported
2. At the time of writing, Windows CMake does not yet have a builtin `FLUTTER_TARGET_PLATFORM` variable; thus, we need to define a dummy version of the variable. See [here](https://github.com/flutter/flutter/issues/116196) for updates on this issue
3. On linux, dynamic library names follow the form of `liblibrary_name.so` and on Windows, dynamic library names follow the form of `library_name.dll`

## `.gitignore`
If you choose to have a .gitignore in your `linux/` and `windows/` directories, here is what the author of this page uses:
```gitignore
# Set up as allowlist
*

# Allowed files
!.gitignore
!CMakeLists.txt
```

## Build Script (`/scripts/build-other.sh`)
Replace `library_name` below as needed.
```bash
#!/bin/bash

# Setup
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Install build dependencies
cargo install cargo-zigbuild
cargo install cargo-xwin

zig_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo zigbuild --target "$TARGET" -r
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

win_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo xwin build --target "$TARGET" -r
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

# Build all the dynamic libraries
LINUX_LIBNAME=liblibrary_name.so
zig_build aarch64-unknown-linux-gnu linux-arm64 $LINUX_LIBNAME
zig_build x86_64-unknown-linux-gnu linux-x64 $LINUX_LIBNAME
WINDOWS_LIBNAME=library_name.dll
win_build aarch64-pc-windows-msvc windows-arm64 $WINDOWS_LIBNAME
win_build x86_64-pc-windows-msvc windows-x64 $WINDOWS_LIBNAME

# Archive the dynamic libs
tar -czvf other.tar.gz linux-* windows-*

# Cleanup
rm -rf linux-* windows-*
```
