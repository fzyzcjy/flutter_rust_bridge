# Continuous Integration & Deployment (CI/CD)

The CI/CD detailed here, using GitHub Actions, automates a lot of the busy work
that you would otherwise need to maintain your library. These workflows include:

- Automatic dependency updates with dependabot
- Continuous Integration (CI)
  - Unit tests and code checks on pushes/PRs to `main`
  - Integration tests on real & emulated devices on pushes/PRs to `main`
- Continuous Deployment (CD)
  - Manual version/release creation with Melos through a workflow dispatch
    - You can set this up to be automated, but in most cases you _don't_ want a new release on every commit to main
  - Automated publishing of new versions to GitHub releases and pub.dev

## Dependabot (`/.github/dependabot.yaml`)

It is highly recommended that you set up dependabot to automatically submit PRs when
your dependencies fall out of date.

Replace `library_name` below with your library name.

```yaml
version: 2
enable-beta-ecosystems: true
updates:
  - package-ecosystem: pub
    directory: "/packages/library_name"
    schedule:
      interval: weekly
  - package-ecosystem: pub
    directory: "/packages/library_name/example"
    schedule:
      interval: weekly
  - package-ecosystem: pub
    directory: "/packages/flutter_library_name"
    schedule:
      interval: weekly
  - package-ecosystem: pub
    directory: "/packages/flutter_library_name/example"
    schedule:
      interval: weekly
  - package-ecosystem: cargo
    directory: "/packages/library_name/native"
    schedule:
      interval: weekly
```

## Continuous Integration (`/.github/workflows/build.yml`)

Replace `library_name` and `LibraryName` below with your library name.

```yaml
name: Build & Test

on:
  pull_request:
  push:
    branches:
      - main
  schedule:
    # runs the CI everyday at 10AM
    - cron: "0 10 * * *"

jobs:
  # General build, check, and test steps
  build_and_test:
    runs-on: ubuntu-latest

    steps:
      # Setup
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable
          components: rustfmt, clippy

      # Rust
      - name: Check Rust format
        working-directory: ./packages/library_name/native/src
        run: rustfmt --check lib.rs
      - name: Rust code analysis
        run: cargo clippy -- -D warnings
      - name: Run Rust tests
        run: cargo test
      - name: Build Rust code for Dart tests
        run: cargo build -r

      # Dart/Flutter
      - name: Check Dart format
        run: melos run check-format --no-select
      - name: Dart code analysis
        run: melos run analyze --no-select
      - name: Run Dart tests
        run: melos run test

  macos_integration_test:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Build the XCFramework
        run: melos run build:apple
      - name: Copy the XCFramework to the needed location
        run: |
          CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`
          cp platform-build/LibraryName.xcframework.zip packages/flutter_library_name/macos/Frameworks/$CURR_VERSION.zip
          echo Copied file!

      - name: Run Flutter integration tests
        working-directory: packages/flutter_library_name/example
        run: flutter test -d macos integration_test

  windows_integration_test:
    runs-on: windows-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: goto-bus-stop/setup-zig@v2
      - uses: KyleMayes/install-llvm-action@v1
        with:
          version: "15"
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Build the binaries
        run: melos run build:other
      - name: Copy the binaries to the needed location
        shell: bash
        run: |
          CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`
          cp platform-build/other.tar.gz packages/flutter_library_name/windows/$CURR_VERSION.tar.gz
          echo Copied file!

      - name: Run Flutter integration tests
        working-directory: packages/flutter_library_name/example
        run: flutter test -d windows integration_test

  linux_integration_test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
      - name: Install dependencies for flutter integration test
        run: sudo apt update && sudo apt-get install -y libglu1-mesa ninja-build clang cmake pkg-config libgtk-3-dev liblzma-dev
      - uses: pyvista/setup-headless-display-action@v1
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: goto-bus-stop/setup-zig@v2
      - uses: KyleMayes/install-llvm-action@v1
        with:
          version: "15"
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Build the binaries
        run: melos run build:other
      - name: Copy the binaries to the needed location
        run: |
          CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`
          cp platform-build/other.tar.gz packages/flutter_library_name/linux/$CURR_VERSION.tar.gz
          echo Copied file!

      - name: Run Flutter integration tests
        working-directory: packages/flutter_library_name/example
        run: flutter test -d linux integration_test

  ios_integration_test:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable

      - name: Start iOS Simulator
        run: |
          DEVICE_ID=$(xcrun xctrace list devices | grep iPhone | head -1 | awk '{print $NF}' | tr -d '()')
          echo "DEVICE_ID=$DEVICE_ID" >> $GITHUB_ENV
          xcrun simctl boot $DEVICE_ID

      - name: Build the XCFramework
        run: melos run build:apple
      - name: Copy the XCFramework to the needed location
        run: |
          CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`
          cp platform-build/LibraryName.xcframework.zip packages/flutter_library_name/ios/Frameworks/$CURR_VERSION.zip
          echo Copied file!

      - name: Run Flutter integration tests
        working-directory: packages/flutter_library_name/example
        run: flutter test -d ${{ env.DEVICE_ID }} integration_test

  android_integration_test:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable
      - uses: nttld/setup-ndk@v1
        with:
          ndk-version: r25b
      - uses: actions/setup-java@v3
        with:
          distribution: zulu
          java-version: "11.x"

      - name: Build the binaries
        run: melos run build:android
      - name: Copy the binaries to the needed location
        run: |
          CURR_VERSION=library_name-v`awk '/^version: /{print $2}' packages/library_name/pubspec.yaml`
          cp platform-build/android.tar.gz packages/flutter_library_name/android/$CURR_VERSION.tar.gz
          echo Copied file!

      - name: Run Flutter integration tests
        uses: Wandalen/wretry.action@master # sometimes android tests are flaky
        with:
          attempt_limit: 5
          action: reactivecircus/android-emulator-runner@v2
          with: |
            api-level: 33
            target: google_apis
            arch: x86_64
            ram-size: 1024M
            disk-size: 2048M
            script: cd packages/flutter_library_name/example && flutter test -d `flutter devices | grep android | tr ' ' '\n' | grep emulator-` integration_test
```

## Continuous Deployment

There are two files you need for CD:

1. Create new versions/releases with Melos
2. Publish new releases to GitHub releases and pub.dev

### Create new versions with Melos (`/.github/workflows/create-release.yml`)

You can create new releases of your library with this workflow by going to the
"Actions" tab in your GitHub repo and manually starting this workflow with an
appropriate option. The options are:

- `--` -> call `melos version` with no additional parameters
- `--prerelease` -> create a prerelease version instead of normal release (e.g., `1.0.0-dev.0`)
- `--graduate` -> graduate a prerelease version to a normal release (e.g., `1.0.0-dev.0` becomes `1.0.0`)

You will need to set a repository secret of `BOT_ACCESS_TOKEN` to your GitHub personal access token (PAT)
to allow for pushes to main from this Action.

Change `YourName` and `your-email@example.com` below as appropriate.

```yaml
name: Create Release(s)

on:
  workflow_dispatch:
    inputs:
      version_parameters:
        description: 'Parameters to pass to "melos version"'
        required: true
        default: " "
        type: choice
        options:
          - "--"
          - "--prerelease"
          - "--graduate"

jobs:
  create_release:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
        with:
          token: ${{ secrets.BOT_ACCESS_TOKEN }}
          fetch-depth: 0
      - name: Setup git
        run: |
          git config user.name "YourName"
          git config user.email "your-email@example.com"
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2

      - name: Create the new version(s)
        run: melos version --yes ${{ inputs.version_parameters }}

      - name: Push created version commit
        run: git push
      - name: Push modified tags
        run: git push --tags
```

### Publish new releases to GitHub releases and pub.dev (`/.github/workflows/publish-release.yml`)

In order for this workflow to execute correctly and publish packages to pub.dev,
you need to have the contents of your pub credentials JSON file in a GitHub repo secret.

First you need to sign-in into your pub account locally by
running the following command: `dart pub login`.

After the authorization is completed, open the credentials file, which can be found:

- On Linux, at `~/.config/dart/pub-credentials.json`
- On macOS, at `~/Library/Application Support/dart/pub-credentials.json`
- On Windows, at `C:\Users\YourUsername\AppData\Roaming\dart\pub-credentials.json`
  
And copy the contents of this `pub-credentials.json` file to a new GitHub repo secret named `PUB_CRED_JSON`.

This workflow is set to execute whenever new version tags are pushed up to GitHub.

```yaml
name: Publish Release(s)

on:
  push:
    tags:
      - "*"

jobs:
  publish_github_release:
    # macOS because we can cross-compile to all other platforms from it
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - uses: goto-bus-stop/setup-zig@v2
      - uses: KyleMayes/install-llvm-action@v1
        with:
          version: "15"
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: stable
      - uses: nttld/setup-ndk@v1
        with:
          ndk-version: r25b

      - name: Build all library binaries
        run: melos run build

      - name: Create GitHub release
        uses: softprops/action-gh-release@v1
        with:
          files: platform-build/*

  publish_pub_release:
    needs: publish_github_release
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
      - uses: subosito/flutter-action@v2
      - uses: bluefireteam/melos-action@v2
      - name: Setup pub.dev credentials
        run: |
          mkdir -p $HOME/.config/dart
          cat << EOF > $HOME/.config/dart/pub-credentials.json
          ${{ secrets.PUB_CRED_JSON }}
          EOF
      - name: Dry-run publish to pub.dev
        run: melos publish -y --dry-run
      - name: Publish to pub.dev
        run: melos publish -y --no-dry-run
```
