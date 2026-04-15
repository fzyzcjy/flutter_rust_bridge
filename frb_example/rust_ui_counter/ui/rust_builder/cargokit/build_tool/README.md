# build_tool

`build_tool` is the Dart command-line package that powers Cargokit.

It is responsible for:

- loading crate and user configuration
- resolving build targets from platform-specific environment variables
- preparing Rust toolchains and targets through `rustup`
- building local artifacts with `cargo`
- downloading and verifying signed precompiled binaries
- materializing output artifacts where Gradle, CMake, and CocoaPods expect them

## Commands

### `build-cmake`

Builds the Rust library for Linux or Windows.

This command is normally launched by `cmake/cargokit.cmake` and expects its
inputs through `CARGOKIT_*` environment variables.

### `build-gradle`

Builds the Rust library for Android.

This command is normally launched by `gradle/plugin.gradle`. It resolves the
requested Android ABIs, configures the Android NDK toolchain, and places the
produced `.so` files into Gradle's `jniLibs` output structure.

### `build-pod`

Builds the Rust library for iOS or macOS CocoaPods integration.

For static libraries, artifacts for active architectures are combined with
`lipo`. For dynamic-library integration, Cargokit updates the framework binary
produced by CocoaPods/Xcode.

### `gen-key`

Generates an Ed25519 key pair used to sign and verify precompiled binaries.

```sh
dart run bin/build_tool.dart gen-key
```

### `precompile-binaries`

Builds release artifacts, signs them, and uploads them to a GitHub release.

Required inputs:

- `--repository`
- `--manifest-dir`
- `PRIVATE_KEY` environment variable
- `GITHUB_TOKEN` environment variable

Example:

```sh
dart run bin/build_tool.dart precompile-binaries \
  --repository=owner/repo \
  --manifest-dir=../rust
```

### `verify-binaries`

Checks whether all expected signed precompiled binaries exist for a crate.

```sh
dart run bin/build_tool.dart verify-binaries --manifest-dir=../rust
```

## Configuration Files

### Crate configuration: `cargokit.yaml`

Placed next to `Cargo.toml`.

Supported sections:

- `cargo`
- `precompiled_binaries`

Example:

```yaml
cargo:
  release:
    toolchain: nightly
    extra_flags:
      - -Z
      - build-std=panic_abort,std

precompiled_binaries:
  url_prefix: https://github.com/example/repo/releases/download/precompiled_
  public_key: <public-key-hex>
```

### App configuration: `cargokit_options.yaml`

Placed in the consuming Flutter app and discovered by walking upward from
`CARGOKIT_ROOT_PROJECT_DIR`.

Supported fields:

- `verbose_logging`
- `use_precompiled_binaries`

## Development

```sh
dart analyze
dart test
```

The package is usually executed indirectly through `run_build_tool.sh` or
`run_build_tool.cmd`, which create a temporary wrapper package so the vendored
copy of Cargokit does not get polluted with generated `.dart_tool` files.
