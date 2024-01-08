# Android
There are a few different ways to integrate with our Android binaries when building for Android. None are particularly outstanding:
- An "Ivy Repository"
  - Works great, but impossible to test changes on an emulator locally or in CI :(
- Raw Groovy & Gradle
  - Works in theory, but tedious to have to write all needed logic in Groovy/Gradle
- Starting an OS Shell
  - Similar to iOS & macOS, see that section for more details on this
  - Wouldn't work on Windows development machines unfortunately; the started shell would not be `bash`
- CMake
  - We call to CMake from Gradle to take care of fetching and processing our Android binaries
  - A somewhat odd approach, but works cross-platform and has code re-use from Windows/Linux!

Due to the above reasoning, we cover how to use CMake on this page. But do note, there are other possibilities out there.

## CMake (`/packages/flutter_library_name/android/CMakeLists.txt`)
Unlike windows and linux CMakeLists.txt, the android equivalent does
_not actually build anything_, which may come as a surprise.
Instead, its sole purpose is to download & extract our Android binaries
in a cross-platform friendly way. Here is our android `CMakeLists.txt`:
```cmake
set(LibraryVersion "library_name-v0.0.0") # generated; do not edit
set(PROJECT_NAME "project_name")

# Unlike the Windows & Linux CMakeLists.txt, this Android equivalent is just here
# to download the Android binaries into src/main/jniLibs/ and does not build anything.
# The binary download/extraction is difficult to do concisely in Groovy/Gradle,
# at least across host platforms, so we are just reusing our Linux/Windows logic.

# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(PROJECT_NAME)

# Download the binaries if they are not already present.
set(LibRoot "${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs")
set(ArchivePath "${CMAKE_CURRENT_SOURCE_DIR}/${LibraryVersion}.tar.gz")
if(NOT EXISTS ${ArchivePath})
  file(DOWNLOAD
    "https://github.com/YourGitHubAccount/repo_name/releases/download/${LibraryVersion}/android.tar.gz"
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
```
Replace all instances of `library_name` above with your library name.
Also, replace other variables (i.e. `YourGitHubAccount` and `repo_name`) as needed.

## `build.gradle` Changes
Replace the `android {...}` section at the bottom of `build.gradle` with the following:
```gradle
android {
    compileSdkVersion 31

    defaultConfig {
        minSdkVersion 16
    }

    // Trigger the binary download/update over in CMakeLists.txt
    externalNativeBuild {
        cmake {
            path "CMakeLists.txt"
        }
    }
}
```

## `.gitignore`
Add the following to `android/.gitignore`
```gitignore
# Ignore Rust binaries
src/main/jniLibs/
*.tar.gz
```

## Build Script (`/scripts/build-android.sh`)
```bash
#!/bin/bash

# Setup
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Create the jniLibs build directory
JNI_DIR=jniLibs
mkdir -p $JNI_DIR

# Set up cargo-ndk
cargo install cargo-ndk
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
        i686-linux-android

# Build the android libraries in the jniLibs directory
cargo ndk -o $JNI_DIR \
        --manifest-path ../Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release 

# Archive the dynamic libs
cd $JNI_DIR
tar -czvf ../android.tar.gz *
cd -

# Cleanup
rm -rf $JNI_DIR
```
