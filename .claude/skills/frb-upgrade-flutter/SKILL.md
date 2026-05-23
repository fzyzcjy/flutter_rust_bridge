---
name: frb-upgrade-flutter
description: >-
  Upgrade flutter_rust_bridge to a new Flutter stable release. Use when changing Flutter/Dart versions,
  devcontainer Docker images, CI/post-release pins, generated Flutter scaffolds, or platform compatibility.
---

# FRB Upgrade Flutter

Use this as the single-file workflow for Flutter stable bumps in `flutter_rust_bridge`.

## Start Here

1. Confirm the target Flutter stable release from official Flutter sources.
2. Also read:
   - `frb-docker` before changing `.devcontainer/**` or publishing the dev image.
   - `frb-code-generation` before accepting generated or scaffold drift.
   - `frb-cargokit` or `frb-cargokit-dev` before changing copied `cargokit` files.
   - `frb-fix-ci` when CI starts failing.

## Non-Negotiables

- Keep `.github/workflows/ci.yaml` and `.github/workflows/post_release.yaml` toolchain env values in sync.
- Treat `.devcontainer/Dockerfile` `ARG` values as the source of truth for dev image tags.
- Publish or dry-run the dev Docker image before depending on a new derived image tag.
- Do not hand-edit generated files as the final state.
- Classify scaffold drift by source: integration template, Apple scaffold, Cargokit, or example output.

## Workflow

### Step 1: Review the Flutter Release

Use official Flutter sources. Record the target Flutter version, bundled Dart version, release date,
and release-note items likely to affect FRB.

Scan for:

- Dart SDK constraint changes
- Android Gradle Plugin, Kotlin, Java, Android SDK, or NDK changes
- iOS/macOS project generation changes, especially Swift Package Manager or CocoaPods defaults
- Web renderer, Chrome, DevTools, or test-driver changes
- Host architecture changes such as Apple Silicon or Windows ARM support

### Step 2: Plan Small PRs

Prefer small PRs in this order:

1. Workflow skill/docs update, if the process needs clarification.
2. Dev Docker image upgrade and publish.
3. CI and post-release version pin update.
4. Generated/scaffold drift.
5. Real compatibility fixes.

### Step 3: Inventory Current Pins

Run these before planning the bump:

```shell
rg -n \
  "FRB_MAIN_|FLUTTER_VERSION|DART_VERSION|RUST_VERSION|setup-flutter|setup-dart|cirruslabs/flutter"
rg -n "flutter_rust_bridge_dev|3\\.[0-9]+"
```

Inspect at least:

- `.devcontainer/Dockerfile`
- `.github/workflows/ci.yaml`
- `.github/workflows/post_release.yaml`
- `.github/workflows/publish_dev_docker.yaml`
- `tools/frb_internal/test/src/makefile_dart/test_dev_docker_metadata.dart`
- `pubspec.yaml`, package `pubspec.yaml` files, and checked-in `pubspec.lock` files
- `frb_codegen/assets/integration_template/**`
- `tools/frb_internal/assets/apple_scaffold/**`
- `frb_example/**`

### Step 4: Upgrade the Devcontainer First

Update `.devcontainer/Dockerfile`:

- `ARG FLUTTER_VERSION`
- Required Rust, Rust nightly, Node, Playwright, Chrome, system package, Java, or Android tooling changes

Update metadata tests that assert the derived dev image tag.

Build and smoke-test locally when practical:

```shell
docker build -f .devcontainer/Dockerfile -t frb-dev .devcontainer
docker run --rm -v "$PWD:/workspace" -w /workspace frb-dev bash -lc './frb_internal --help'
docker run --rm -v "$PWD:/workspace" -w /workspace frb-dev bash -lc '
set -euo pipefail
flutter --version
dart --version
node --version
npm --version
cargo --version
wasm-pack --version
"${CHROME_BIN}" --version
'
```

If local build is too expensive, dry-run the workflow:

```shell
gh workflow run publish_dev_docker.yaml --ref <branch> -f publish=false
```

After the Dockerfile change reaches the publish branch, publish and verify both `linux/amd64` and
`linux/arm64`:

```shell
gh workflow run publish_dev_docker.yaml --ref master
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:latest
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:flutter-<flutter>-rust-<rust>-nightly-<nightly>
```

BuildKit attestation manifests can appear as `unknown/unknown`; those are not platform images.

### Step 5: Sync CI and Post-Release Pins

Update top-level env values together in `.github/workflows/ci.yaml` and
`.github/workflows/post_release.yaml`:

- `FRB_MAIN_FLUTTER_VERSION`
- `FRB_MAIN_DART_VERSION`
- `FRB_MAIN_RUST_VERSION` if the Flutter or tooling bump requires newer Rust
- `FRB_RUSTFMT_NIGHTLY_VERSION` only if formatting or nightly-only `rust-src` behavior requires it

`post_release.yaml` intentionally says it should stay in sync with `ci.yaml`. It verifies released
quickstart and codegen installation modes, so do not leave it pinned to old Flutter/Dart versions.

Scan workflow assumptions:

- `flutter-actions/setup-flutter`
- `dart-lang/setup-dart`
- Java setup for Android jobs
- Linux desktop package prerequisites
- iOS simulator names and macOS runner labels
- Windows ARM runner coverage
- Chrome/chromedriver setup for web jobs
- Post-release `codegen_install_mode` coverage for `cargo-install`, `cargo-binstall`, `scoop`, and
  `homebrew`
- Any commented job that says it was waiting for a CI Flutter upgrade

### Step 6: Regenerate and Classify Drift

Read `frb-code-generation` first.

Expect drift in:

- `pubspec.lock` Dart SDK constraints
- `flutter create` / `flutter integrate` scaffold output
- Android Gradle, Kotlin, Java, NDK, and manifest files
- iOS/macOS Xcode project files, CocoaPods files, or SwiftPM package files
- Windows/Linux desktop scaffold files
- Generated `frb_generated.*` files if Dart formatting, analyzer behavior, or codegen dependencies changed

Classify by source:

- Template-driven drift belongs in `frb_codegen/assets/integration_template/**`.
- Apple scaffold drift may belong in `tools/frb_internal/assets/apple_scaffold/**`.
- Cargokit drift may belong in the upstream Cargokit repo.
- Example-only drift should come from the relevant `./frb_internal generate-*` or `precommit-*` command.

Run focused generation first when possible, then broaden:

```shell
./frb_internal precommit-generate
./frb_internal precommit-integrate
```

If multiple generated-output CI failures rotate across packages, stop package-by-package fixes and run
a clean full `./frb_internal precommit-generate`.

### Step 7: Validate Locally

Read `frb-lint` and `frb-test` for exact command guidance. Tom's FRB environment runs tests locally,
usually through the per-worktree Docker container.

Recommended minimum validation:

```shell
./frb_internal lint --fix
./frb_internal test-dart-native --package frb_example/pure_dart
./frb_internal test-dart-native --package frb_example/pure_dart_pde
./frb_internal test-flutter-native --package frb_example/flutter_via_create
./frb_internal test-flutter-web --package frb_example/gallery
```

For CI or Docker plumbing-only changes, dev image dry-run and focused metadata tests may be enough
before opening the PR. Let CI cover the full matrix.

### Step 8: Triage CI in Dependency Order

Read `frb-fix-ci` before deep debugging.

1. Dev Docker publish or dry-run failures
2. Lint/setup failures caused by incompatible tool versions
3. Generate and Generate Internal failures
4. Integrate scaffold failures
5. Build and platform tests
6. Post-release quickstart failures
7. Coverage, benchmark, website, and upload jobs

When a platform starts failing after the Flutter bump, compare against release notes before patching
symptoms. Flutter stable bumps often intentionally change generated platform projects.

## PR Notes

For the Docker/CI upgrade PR, include:

- Old and new Flutter/Dart versions
- Whether the dev image was dry-run or published
- Exact version tag of the dev image
- Generated/scaffold sources that changed
- Local validation commands and CI status
- Any known remaining platform-specific follow-up
