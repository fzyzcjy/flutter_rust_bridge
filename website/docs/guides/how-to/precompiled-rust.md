# Precompiled Rust Binaries

> You can view the original documentation at [cargokit](https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md)

Because Cargokit builds the Rust crate during Flutter build, it is inherently
dependend on the Rust toolchain being installed on the developer's machine.

To decrease the friction, it is possible for Cargokit to use precompiled binaries instead.

This is how the process of using precompiled binaries looks from the perspective of the build on developer machine:

1. Cargokit checks if there is `cargokit_options.yaml` file in the root folder of target application. If there is one, it will be checked for `use_precompiled_binaries` options to see if user opted out of using precompiled binaries. In which case Cargokit will insist on building from source. Cargokit will also build from source if the configuration file is absent, but user has Rustup installed.

2. Cargokit checks if there is `cargokit.yaml` file placed in the Rust crate. If there is one, it will be checked for `precompiled_binaries` section to see if crate supports precompiled binaries. The configuration section must contain a public key and URL prefix.

3. Cargokit computes a `crate-hash`. This is a SHA256 hash value computed from all Rust files inside crate, `Cargo.toml`, `Cargo.lock` and `cargokit.yaml`. This uniquely identifies the crate and it is used to find the correct precompiled binaries.

4. Cargokit will attempt to download the precompiled binaries for target platform and `crate_hash` combination and a signature file for each downloaded binary. If download succeeds, the binary content will be verified against the signature and public key included in `cargokit.yaml` (which is part of Rust crate and thus part of published Flutter package).

5. If the verification succeeds, the precompiled binaries will be used. Otherwise the binary will be discarded and Cargokit will insist on building from source.

## Providing precompiled binaries

Note that this assumes that precompiled binaries will be generated during github actions and deployed as github releases.

### Use `build_tool` to generate a key-pair:

```shell
dart run cargokit/build_tool/bin/build_tool.dart gen-key
```

This will print the private key and public key. Store the private key securely. It needs to be provided as a secret to github action.

The public key should be included in `cargokit.yaml` file in the Rust crate.

### Provide a `cargokit.yaml` file in the Rust crate

The file must be placed alongside Cargo.toml.

```yaml
precompiled_binaries:
  # Uri prefix used when downloading precompiled binaries.
  url_prefix: https://github.com/<repository-owner>/<repository-name>/releases/download/precompiled_

  # Public key for verifying downloaded precompiled binaries.
  public_key: <public key from previous step>
```

### Configure a github action to build and upload precompiled binaries.

The action needs two secrets - private key for signing binaries and GitHub token for uploading binaries as releases. Here is example action that precompiles and uploads binaries for all supported targets.

```yaml
name: Precompile Binaries

on:
  workflow_dispatch:
    inputs:
      TAG:
        description: 'Set a Tag'
        required: true
        default: ''
  push:
    tags:
      - v*

env:
  GITHUB_REPOSITORY: <repository-owner>/<repository-name>
  RUST_CRATE_DIR: <rust-crate-directory>
  NDK_VERSION: 27.0.12077973
  MIN_SDK_VERSION: 21

jobs:
  precompile:
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os:
          - ubuntu-latest
          - macos-latest
          - windows-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Dart
        uses: dart-lang/setup-dart@v1

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Linux dependencies
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev

      - name: Setup Java (for Android)
        if: matrix.os == 'ubuntu-latest'
        uses: actions/setup-java@v4
        with:
          java-version: '17'
          distribution: 'temurin'

      - name: Setup Android SDK (Ubuntu)
        if: matrix.os == 'ubuntu-latest'
        uses: android-actions/setup-android@v3

      - name: Accept Android SDK licenses
        if: matrix.os == 'ubuntu-latest'
        run: yes | $ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager --licenses

      - name: Install Android NDK
        if: matrix.os == 'ubuntu-latest'
        run: |
          $ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager "ndk;${{ env.NDK_VERSION }}"

      - name: Setup build_tool dependencies
        run: |
          cd cargokit/build_tool
          dart pub get

      - name: Precompile binaries (macOS)
        if: matrix.os == 'macos-latest'
        run: |
          dart run cargokit/build_tool/bin/build_tool.dart precompile-binaries -v --manifest-dir=${{ env.RUST_CRATE_DIR }} --repository=${{ env.GITHUB_REPOSITORY }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          PRIVATE_KEY: ${{ secrets.PRECOMPILE_BINARIES_PRIVATE_KEY }}

      - name: Precompile binaries (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          dart run cargokit/build_tool/bin/build_tool.dart precompile-binaries -v --manifest-dir=${{ env.RUST_CRATE_DIR }} --repository=${{ env.GITHUB_REPOSITORY }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          PRIVATE_KEY: ${{ secrets.PRECOMPILE_BINARIES_PRIVATE_KEY }}

      - name: Precompile binaries (Ubuntu with Android)
        if: matrix.os == 'ubuntu-latest'
        run: |
          dart run cargokit/build_tool/bin/build_tool.dart precompile-binaries -v --manifest-dir=${{ env.RUST_CRATE_DIR }} --repository=${{ env.GITHUB_REPOSITORY }} --android-sdk-location=$ANDROID_HOME --android-ndk-version=${{ env.NDK_VERSION }} --android-min-sdk-version=${{ env.MIN_SDK_VERSION }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          PRIVATE_KEY: ${{ secrets.PRECOMPILE_BINARIES_PRIVATE_KEY }}
```

By default the `built_tool precompile-binaries` commands build and uploads the binaries for all targets buildable from current host. This can be overriden using the `--target <rust-triple>` argument.

Android binaries will be built when `--android-sdk-location` and `--android-ndk-version` arguments are provided.

**You should modify the action file to suit your needs.**


