# Platform setup
In this subsection, we will be exploring how to set up our Flutter wrapper package
to bundle the platform-specific Rust binaries so users of the library will be able
to actually use the library.
After creating the Flutter wrapper on the previous page, you may have noticed that
you get a runtime error when trying to use it as-is because of `DynamicLibrary`;
this is because the binaries are not yet distributed along with the package!

## Binary distribution
Unfortunately, at the time of writing, pub.dev has a hard 100 MB upload limit and
discourages distributing platform-specific binaries through pub.dev directly.
In the future, hopefully with Native Assets, there will be a way to distribute
our Rust binaries through pub.dev, or something similar, which will make
distribution as a library author *much* more convenient.

In the meantime, however, we will need to work around these limitations. There are
many ways to distribute the binaries ourselves, outside of pub.dev, but in this
subsection, we will cover using GitHub releases because it easily integrates with
our CI/CD solution, GitHub Actions (more on this [later](ci.md)).

## How it works
If you look in your Flutter wrapper's pubspec (`/packages/flutter_library_name/pubspec.yaml`),
you should notice the following section near the bottom
(if you don't see this, or it is incomplete, add it now!):
```yaml
flutter:
  plugin:
    platforms:
      ios:
        ffiPlugin: true
      macos:
        ffiPlugin: true
      android:
        ffiPlugin: true
      linux:
        ffiPlugin: true
      windows:
        ffiPlugin: true
```
This section of the pubspec tells Flutter that our package is using the newer
ffi plugins format instead of the older platform channels Flutter has.
This makes the work on our end much simpler, because instead of having to
specify platform channels for each platform supported, we now only have to
bundle the binaries with our package.

But, the key here is that we still must bundle our binaries along with our package.
To do so, we have to follow a certain procedure (*read this; it is important*):
1. We have a series of build scripts (`/scripts/build-*.sh`) that build all of our
platform specific binaries into `/platform-build` and package them up appropriately,
based on the target platform.
Example: on iOS/macOS, this bundle is an XCFramework, on Windows/Linux, it is a `.tar.gz`.
2. These binaries are uploaded to somewhere online; as mentioned previously, we will use
GitHub releases in this guide (which is [automated in ci](ci.md)).
3. When the Dart tooling builds our library (such as when an application consuming
our library is built), it invokes the platform specific build process.
We hijack this build process by downloading a copy of the binaries for the needed platform,
*if not already present on the filesystem*. This last part is the key; it allows us to run
integration tests locally and in CI by providing our own copy of the binaries instead of
forcing our build process to always fetch the binaries from GitHub releases.
4. After the binaries are stored locally (either by being copied to the proper folder(s)
or by fetching them from online), we extract them and place them in the needed locations.

Here are the relevant directories per platform.
This is helpful for if you want to test your library on a real device or emulator locally.
Also note: replace `library_name` below with your library name, and replace `library_tag` below with
`library_name-vVERSION` where `VERSION` is the current version in your Dart-only `pubspec.yaml`.
This setup is a bit of a pain to test locally with but I am not sure there is a better way at the moment
(other than creating a melos script to copy over all the binaries for you).
The idea here is that you will do most of your integration tests in CI.
- Binary archive locations (copy created archives from `/platform-build` to these locations to test locally)
  - iOS (`/plaform-build/LibraryName.xcframework.zip`): `/packages/flutter_library_name/ios/Frameworks/library_tag.zip` 
  - macOS (`/platform-build/LibraryName.xcframework.zip`): `/packages/flutter_library_name/macos/Frameworks/library_tag.zip` 
  - Android (`/platform-build/android.tar.gz`): `/packages/flutter_library_name/android/library_tag.tar.gz` 
  - Windows (`/platform-build/other.tar.gz`): `/packages/flutter_library_name/windows/library_tag.tar.gz` 
  - Linux (`/platform-build/other.tar.gz`): `/packages/flutter_library_name/linux/library_tag.tar.gz` 
- Extracted binary locations (not as important to know, but helps understand the build process)
  - iOS: `/packages/flutter_library_name/ios/Frameworks/` 
  - macOS: `/packages/flutter_library_name/macos/Frameworks/` 
  - Android: `/packages/flutter_library_name/android/src/main/jniLibs/`
    - If you know what an `aar` is, Flutter does something similar for android plug-ins under the hood
  - Windows: `/packages/flutter_library_name/windows/` 
  - Linux: `/packages/flutter_library_name/linux/` 

Always use melos to build the latest version(s) of the binaries (e.g. `melos run build:android`)
*before copying the binary archives from `/platform-build/` and testing locally*!
Also, *do not check the `/platform-build/` or `/target/` directories into source control*!
