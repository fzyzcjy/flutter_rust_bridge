# iOS & macOS
Flutter libraries targeting iOS and macOS use cocoapods for dependencies,
so this page will walk you through setting that up with FRB.

The simplist way the author of this page found to integrate with cocoapods for all
Apple platforms (iOS, iOS Simulator, and macOS) is to create an XCFramework.
While you don't need to know what an XCFramework is to follow this guide, if you
want to understand how this all works behind the scenes,
I'd recommend doing a quick Google search on "What is an XCFramework?".

Also, this page will introduce the iOS & macOS build script (`build-apple.sh`)
to compile your Rust library for all Apple platforms. Note: unlike all of the other build scripts
presented in this guide (which we can run on any host OS), `build-apple.sh` must be run on macOS.

## Directory Tree
We will need to create several files for both iOS and macOS to:
- Prevent our Rust symbols from being accidentally stripped
- Bundle our "XCFramework" with our Flutter library

### `ios/Classes/EnforceBundling.swift` and `macos/Classes/EnforceBundling.swift`
```swift
public func dummyMethodToEnforceBundling() -> Int64 {
  return dummy_method_to_enforce_bundling()
}
let dummyVar = dummyMethodToEnforceBundling();
```

### `ios/Frameworks/.gitkeep` and `macos/Frameworks/.gitkeep`
No file contents here; simply add a blank file (i.e., `touch .gitkeep` in `bash`).

### `ios/.gitignore`
```gitignore
Flutter/
Runner/
Frameworks/*
!Frameworks/.gitkeep
```

### `macos/.gitignore`
```gitignore
Flutter/
Frameworks/*
!Frameworks/.gitkeep
```

### `ios/flutter_library_name.podspec` and `macos/flutter_library_name.podspec` (for Cocoapods)
We cannot use the CMake approach taken on other platforms with Cocoapods,
so we do something a bit different here. `.podspec` files are actually just ruby files;
due to this observation, we can access the system shell to make arbitrary changes.
While we could download and extract our Rust binaries for iOS/macOS in ruby directly,
it is much more straightforward to simply use bash/zsh.

Replace all instances of `library_name` and `LibraryName` below with your library name.
Also, replace other variables (i.e. `YourGitHubAccount` and `repo_name`) as needed.

Note: the same exact `flutter_library_name.podspec` is used for both iOS and macOS;
you can thank the `XCFramework` for this simplicity.
```ruby
release_tag_name = 'library_name-v0.0.0' # generated; do not edit

# We cannot distribute the XCFramework alongside the library directly,
# so we have to fetch the correct version here.
framework_name = 'LibraryName.xcframework'
remote_zip_name = "#{framework_name}.zip"
url = "https://github.com/YourGitHubAccount/repo_name/releases/download/#{release_tag_name}/#{remote_zip_name}"
local_zip_name = "#{release_tag_name}.zip"
`
cd Frameworks
rm -rf #{framework_name}

if [ ! -f #{local_zip_name} ]
then
  curl -L #{url} -o #{local_zip_name}
fi

unzip #{local_zip_name}
cd -
`

Pod::Spec.new do |spec|
  spec.name          = 'library_name'
  spec.version       = '0.0.1'
  spec.license       = { :file => '../LICENSE' }
  spec.homepage      = 'https://github.com/YourGitHubAccount/repo_name'
  spec.authors       = { 'Your Name' => 'your-email@example.com' }
  spec.summary       = 'iOS/macOS Flutter bindings for library_name'

  spec.source              = { :path => '.' }
  spec.source_files        = 'Classes/**/*'
  spec.public_header_files = 'Classes/**/*.h'
  spec.vendored_frameworks = "Frameworks/#{framework_name}"

  spec.ios.deployment_target = '11.0'
  spec.osx.deployment_target = '10.11'
end
```

## Build Script (`/scripts/build-apple.sh`)
Replace `library_name` and `LibraryName` below as needed.
```bash
#!/bin/bash

# Setup
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Build static libs
for TARGET in \
        aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim \
        x86_64-apple-darwin aarch64-apple-darwin
do
    rustup target add $TARGET
    cargo build -r --target=$TARGET
done

# Create XCFramework zip
FRAMEWORK="LibraryName.xcframework"
LIBNAME=liblibrary_name.a
mkdir mac-lipo ios-sim-lipo
IOS_SIM_LIPO=ios-sim-lipo/$LIBNAME
MAC_LIPO=mac-lipo/$LIBNAME
lipo -create -output $IOS_SIM_LIPO \
        ../target/aarch64-apple-ios-sim/release/$LIBNAME \
        ../target/x86_64-apple-ios/release/$LIBNAME
lipo -create -output $MAC_LIPO \
        ../target/aarch64-apple-darwin/release/$LIBNAME \
        ../target/x86_64-apple-darwin/release/$LIBNAME
xcodebuild -create-xcframework \
        -library $IOS_SIM_LIPO \
        -library $MAC_LIPO \
        -library ../target/aarch64-apple-ios/release/$LIBNAME \
        -output $FRAMEWORK
zip -r $FRAMEWORK.zip $FRAMEWORK

# Cleanup
rm -rf ios-sim-lipo mac-lipo $FRAMEWORK
```
