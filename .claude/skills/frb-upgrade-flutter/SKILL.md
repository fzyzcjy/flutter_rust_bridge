---
name: frb-upgrade-flutter
description: Use when upgrading flutter_rust_bridge to a new Flutter stable release across Docker and CI
---

# FRB Upgrade Flutter

Use this skill when bumping flutter_rust_bridge to a new Flutter stable release or when preparing the
project after Flutter publishes a new stable channel version.

## Source of Truth

- Flutter release information: official Flutter SDK archive and release notes.
- Devcontainer image definition: `.devcontainer/Dockerfile`.
- Dev Docker publish workflow: `.github/workflows/publish_dev_docker.yaml`.
- CI toolchain versions: top-level `env` in `.github/workflows/ci.yaml`.
- Dev image metadata parser tests:
  `tools/frb_internal/test/src/makefile_dart/test_dev_docker_metadata.dart`.
- Generated integration templates and example outputs:
  `frb_codegen/assets/integration_template/**`, `tools/frb_internal/assets/apple_scaffold/**`,
  and `frb_example/**`.

Do not hardcode a dev image tag by hand. The Dockerfile `ARG` values define the version tag used by
the publish workflow and by `./frb_internal precommit-autofix-in-dev-container`.

## Upgrade Shape

Prefer a short PR chain instead of one oversized PR:

1. Add or refresh this workflow skill when the process itself is unclear.
2. Upgrade and publish the dev Docker image.
3. Switch CI to the new Flutter/Dart versions.
4. Accept generated and scaffold drift only after identifying the source of truth.
5. Fix remaining real compatibility failures.

## Phase 1: Confirm the Target Release

1. Read the official Flutter SDK archive and release notes.
2. Record the target Flutter version, bundled Dart version, release date, and release-note items likely
   to affect FRB.
3. For a major stable bump, specifically scan for:
   - Dart SDK constraint changes.
   - Android Gradle Plugin, Kotlin, Java, Android SDK, or NDK changes.
   - iOS/macOS project generation changes, especially Swift Package Manager or CocoaPods defaults.
   - Web renderer, Chrome, DevTools, or test-driver changes.
   - Host architecture changes such as Apple Silicon or Windows ARM support.

## Phase 2: Inventory Current Pins

Run:

```shell
rg -n \
  "FRB_MAIN_|FLUTTER_VERSION|DART_VERSION|RUST_VERSION|setup-flutter|setup-dart|cirruslabs/flutter"
rg -n "flutter_rust_bridge_dev|3\\.[0-9]+"
```

At minimum, inspect:

- `.devcontainer/Dockerfile`
- `.github/workflows/ci.yaml`
- `.github/workflows/publish_dev_docker.yaml`
- `pubspec.yaml`, package `pubspec.yaml` files, and checked-in `pubspec.lock` files
- `tools/frb_internal/test/src/makefile_dart/test_dev_docker_metadata.dart`
- Existing generated example platform files under `frb_example/**`

## Phase 3: Upgrade Devcontainer First

1. Update `.devcontainer/Dockerfile`:
   - `ARG FLUTTER_VERSION`
   - Any required Rust, Rust nightly, Node, Playwright, Chrome, system package, Java, or Android tooling
     changes discovered during the release review.
2. Update tests that assert the derived dev image tag.
3. Build and smoke-test the image locally when practical:

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

4. If local build is too expensive, run the publish workflow as a dry run from the branch:

```shell
gh workflow run publish_dev_docker.yaml --ref <branch> -f publish=false
```

5. After review confidence is good and the Dockerfile change reaches the publish branch, publish the
   image:

```shell
gh workflow run publish_dev_docker.yaml --ref master
```

6. Verify the published manifest:

```shell
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:latest
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:flutter-<flutter>-rust-<rust>-nightly-<nightly>
```

Confirm both `linux/amd64` and `linux/arm64` platform images exist. BuildKit attestation manifests can
appear as `unknown/unknown`; those are not platform images.

## Phase 4: Switch CI Pins

Update `.github/workflows/ci.yaml` top-level env together:

- `FRB_MAIN_FLUTTER_VERSION`
- `FRB_MAIN_DART_VERSION`
- `FRB_MAIN_RUST_VERSION` if the Flutter or tooling bump requires newer Rust
- `FRB_RUSTFMT_NIGHTLY_VERSION` only if formatting or nightly-only rust-src behavior requires it

Then scan workflow setup steps for stale assumptions:

- `flutter-actions/setup-flutter`
- `dart-lang/setup-dart`
- Java setup for Android jobs
- Linux desktop package prerequisites
- iOS simulator names and macOS runner labels
- Windows ARM runner coverage
- Chrome/chromedriver setup for web jobs
- Any commented job that says it was waiting for a CI Flutter upgrade

## Phase 5: Regenerate and Classify Drift

Read `frb-code-generation` before running generation commands.

For a Flutter version bump, expect generated drift in at least these areas:

- `pubspec.lock` Dart SDK constraints.
- `flutter create` / `flutter integrate` scaffold output.
- Android Gradle, Kotlin, Java, NDK, and manifest files.
- iOS/macOS Xcode project files, CocoaPods files, or SwiftPM package files.
- Windows/Linux desktop scaffold files.
- Generated `frb_generated.*` files if Dart formatting, analyzer behavior, or codegen dependencies changed.

Do not hand-edit generated files as the final state. Identify the source:

- Template-driven drift belongs in `frb_codegen/assets/integration_template/**`.
- Apple scaffold drift may belong in `tools/frb_internal/assets/apple_scaffold/**`.
- Cargokit drift may belong in the upstream Cargokit repo; read `frb-cargokit` or
  `frb-cargokit-dev` before changing copied cargokit files.
- Example-only drift should come from the relevant `./frb_internal generate-*` or `precommit-*` command.

Run focused generation first when possible, then broaden:

```shell
./frb_internal precommit-generate
./frb_internal precommit-integrate
```

If multiple generated-output CI failures rotate across packages, stop package-by-package fixes and run
a clean full `./frb_internal precommit-generate` before accepting more generated diffs.

## Phase 6: Validate Locally

Read `frb-lint` and `frb-test` for exact command guidance. For this repo, Tom's environment runs FRB
tests locally, usually through the per-worktree Docker container.

Recommended minimum validation:

```shell
./frb_internal lint --fix
./frb_internal test-dart-native --package frb_example/pure_dart
./frb_internal test-dart-native --package frb_example/pure_dart_pde
./frb_internal test-flutter-native --package frb_example/flutter_via_create
./frb_internal test-flutter-web --package frb_example/gallery
```

If the change touches only CI or Docker plumbing, the dev image dry run and focused metadata tests may
be enough before opening the PR. Let CI cover the full matrix, then triage with `frb-fix-ci`.

## Phase 7: CI Triage

Read `frb-fix-ci` before deep CI debugging.

Triage in dependency order:

1. Dev Docker publish or dry-run failures.
2. Lint/setup failures caused by incompatible tool versions.
3. Generate and Generate Internal failures.
4. Integrate scaffold failures.
5. Build and platform tests.
6. Coverage, benchmark, website, and upload jobs.

When a platform starts failing after the Flutter bump, compare against the release notes before
patching symptoms. Flutter stable bumps often intentionally change generated platform projects.

## PR Notes

For the Docker/CI upgrade PR, include:

- Old and new Flutter/Dart versions.
- Whether the dev image was dry-run or published.
- The exact version tag of the dev image.
- Generated/scaffold sources that changed.
- Local validation commands and CI status.
- Any known remaining platform-specific follow-up.

Use an English PR title in the repository's existing style, for example `Upgrade Flutter CI to 3.44`.
