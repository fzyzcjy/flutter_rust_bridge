# Precompiled Binaries

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

```
dart run build_tool gen-key
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

The github action should be run at every commit to main branch (and possibly other branches).

The action needs two secrets - private key for signing binaries and GitHub token for uploading binaries as releases. Here is example action that precompiles and uploads binaries for all supported targets.

```yaml
on:
  push:
    branches: [ main ]

name: Precompile Binaries

jobs:
  Precompile:
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os:
          - ubuntu-latest
          - macOS-latest
          - windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: dart-lang/setup-dart@v1
      - name: Install GTK
        if: (matrix.os == 'ubuntu-latest')
        run: sudo apt-get update && sudo apt-get install libgtk-3-dev
      - name: Precompile
        if: (matrix.os == 'macOS-latest') || (matrix.os == 'windows-latest')
        run: dart run build_tool precompile-binaries -v --manifest-dir=../../rust --repository=superlistapp/super_native_extensions
        working-directory: super_native_extensions/cargokit/build_tool
        env:
          GITHUB_TOKEN: ${{ secrets.RELEASE_GITHUB_TOKEN }}
          PRIVATE_KEY: ${{ secrets.RELEASE_PRIVATE_KEY }}
      - name: Precompile (with Android)
        if: (matrix.os == 'ubuntu-latest')
        run: dart run build_tool precompile-binaries -v --manifest-dir=../../rust --repository=superlistapp/super_native_extensions --android-sdk-location=/usr/local/lib/android/sdk --android-ndk-version=24.0.8215888 --android-min-sdk-version=23
        working-directory: super_native_extensions/cargokit/build_tool
        env:
          GITHUB_TOKEN: ${{ secrets.RELEASE_GITHUB_TOKEN }}
          PRIVATE_KEY: ${{ secrets.RELEASE_PRIVATE_KEY }}
```

By default the `built_tool precompile-binaries` commands build and uploads the binaries for all targets buildable from current host. This can be overriden using the `--target <rust-triple>` argument.

Android binaries will be built when `--android-sdk-location` and `--android-ndk-version` arguments are provided.

