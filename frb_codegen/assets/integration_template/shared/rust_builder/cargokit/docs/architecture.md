# Cargokit Architecture

Note: This is mostly relevant for plugins authors that want to see a bit under the hood rather then just following a tutorial.

In ideal conditions the end-developer using the plugin should not even be aware of Cargokit existence.

## Integration

Cargokit is meant to be included in Flutter plugin (or application) that contains the Rust crate to be built during the Flutter build process.

Cargokit can be either incuded as git submodule or git subtree (required for plugins - as pub does not support submodules for git dependencies).

For a step by step tutorial on integrating Cargokit with a Flutter plugin see https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/.

## build_tool

Build tool is the core of cargokit. It is a Dart command line package that facilitates the build of Rust crate. It is invoked during the Flutter build process to build (or download) Rust artifacts, but it can be also used as a standalone tool.

It handles the following commands:

### build-cmake

This is invoked from `cargokit.cmake` and it is used to build the Rust crate into a dynamic library on Linux and Windows (which use CMake as build system).

The command takes no additional arguments, everything is controlled during environment variables set by `cargokit.cmake`.

### build-gradle

This is invoked from `plugin.gradle` and it is used to build the Rust crate into a dynamic library on Android. The command takes no additional arguments, everything is controlled during environment variables set by `plugin.gradle`.

The build_tool installs NDK if needed, configures the Rust environment for cross compilation and then invokes `cargo build` with appropriate arguments and environment variables.

The build-tool also acts a linker driver.

### build-pod

This is invoked from plugin's podspec `script_phase` through `build_pod.sh`. Bundle tool will build the Rust crate into a static library that gets linked into the plugin Framework. In this case must have `:execution_position` set to `:before_compile`.

Cargokit will build binaries for all active architectures from XCode build and lipo them togherer.

When using Cargokit to integrate Rust code with an application (not a plugin) you can also configure the `Cargo.toml` to just build a dynamic library. When Cargokit finds that the crate only built a dylib and no static lib, it will attempt to replace the Cocoapod framework binary with the dylib. In this case the script `:execution_position` must be set to `:after_compile`. This is *not* recommended for plugins and it's quite experimental.

### gen-key, precompile-binaries, verify-binaries

These are used as when providing precompiled binaries for Plugin. See [precompiled_binaries.md](precompiled_binaries.md) for more information.

## Launching the build_tool during build.

During Flutter build, the build tool can not be launched directly using `dart run`. Rather it is launched through `run_build_tool.sh` and `run_build_tool.cmd`. Because the `build_tool` is shipped as part of plugin, we generally don't want to write into the plugin directory during build, which would happen if the `build_tool` was simply invoked through `dart run` (For example the `.dart_tool/package_config.json` file would get written inside the `build_tool` directory).

Instead the `run_build_tool` script creates a minimal Dart command line package in the build directory and references the `build_tool` as package. That way the `.dart_tool/package_config.json` file is created in the temporary build folder and not in the plugin itself. The script also precompiles the Dart code to speed up subsequent invocations.

## Configuring Cargokit

### Configuration for the Rust crate

Cargokit can be configured through a `cargokit.yaml` file, which can be used to control the build of the Rust package and is placed into the Rust crate next to `Cargo.toml`.

Here is an example `cargokit.yaml` with comments:
```yaml
cargo:
  debug: # Configuration of cargo execution during debug builds
    toolchain: stable # default
  release: # Configuration of cargo execution for release builds
    toolchain: nightly # rustup will be invoked with nightly toolchain
    extra_flags: # extra arguments passed to cargo build
      - -Z
      - build-std=panic_abort,std

# If crate ships with precompiled binaries, they can be configured here.
precompiled_binaries:
  # Uri prefix used when downloading precompiled binaries.
  url_prefix: https://github.com/superlistapp/super_native_extensions/releases/download/precompiled_

  # Public key for verifying downloaded precompiled binaries.
  public_key: 3a257ef1c7d72d84225ac4658d24812ada50a7a7a8a2138c2a91353389fdc514
```

### Configuration for the application consuming the plugin

A `cargokit_options.yaml` file can also be placed by developer using plugin to the root of the application package. In which case the file can be used to specify following options:

```yaml
# Enables verbose logging of Cargokit during build
verbose_logging: true

# Opts out of using precompiled binaries. If crate has configured
# and deployed precompiled binaries, these will be by default used whenever Rustup
# is not installed. With `use_precompiled_binaries` set to false, the build will
# instead be aborted prompting user to install Rustup.
use_precompiled_binaries: false
```

## Detecting Rustup

When the plugin doesn't come with precompiled libraries (or user opt-out), `build_tool` will need to invoke Rustup during build to ensure that required Rust targets and toolchain are installed for current build and to build the Rust crate.

Cargokit will attempt to detect Rustup in the default Rustup installation location (`~/.cargo/rustup`) as well as in PATH. This is done so that if user install Rustup but doesn't properly configure PATH, Cargokit will still work.

If `build_tool` doesn't find Rustup, it will about the build with a message showing instructions to install Rustup specific to current platform.

On macOS it will also detect a homebrew Rust installation in PATH and will prompt user to call `brew unlink rust` first to remove homebrew Rust installation from PATH, because it may interfere with Rustup.

Homebrew Rust installation can not be used by Cargokit, because it can only build for host platform. Cargokit needs to be able to cross compile the Rust crate for iOS and Android and thus needs full Rustup installation.
