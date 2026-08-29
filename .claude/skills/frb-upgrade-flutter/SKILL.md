---
name: frb-upgrade-flutter
description: >-
  Upgrade flutter_rust_bridge to a new Flutter stable release. Use when changing Flutter/Dart versions,
  devcontainer Docker images, CI/post-release pins, generated Flutter scaffolds, or platform compatibility.
---

# 1 Start here

- Confirm the target Flutter stable release from official Flutter sources.
- Read `frb-dev-env` before running setup, generation, lint, or tests. Use its per-worktree local Docker workflow.
- Read `frb-docker` before changing `.devcontainer/**` or publishing the dev image.
- Read `frb-code-generation` before accepting generated or scaffold drift.
- Read `frb-cargokit` before changing any copied CargoKit file. Read `frb-cargokit-dev` only when the source
  change belongs in the external CargoKit repository.
- Read `frb-pr-chain-split` as soon as an independently landable prerequisite or cleanup appears.
- Read `frb-prepare-pr` and then `frb-pr-review` before treating the upgrade PR as ready.
- Read `frb-fix-ci` when CI starts failing.

# 2 Establish the version contract

- Record the target Flutter version, bundled Dart version, release date, and relevant release-note changes.
- Distinguish the primary Dart version from the minimum supported Dart SDK floor.
  - Upgrade the primary Dart pin with Flutter.
  - Do not raise package SDK constraints merely because the primary toolchain is newer.
  - When retaining an older SDK floor, run dependency resolution, analysis, and tests on that floor in CI.
- Inventory version-like values without assuming the target Dart major or minor:

```shell
rg -n "FRB_MAIN_|FLUTTER_VERSION|DART_VERSION|RUST_VERSION|setup-flutter|setup-dart|cirruslabs/flutter"
rg -n "flutter_rust_bridge_dev|stable|nightly|minimum.*version|deployment.*target" \
  .devcontainer .github tools frb_codegen frb_example
```

- Inspect at least:
  - `.devcontainer/Dockerfile`;
  - `.github/workflows/ci.yaml` and `.github/workflows/post_release.yaml`;
  - `.github/workflows/publish_dev_docker.yaml`;
  - `tools/frb_internal/test/src/makefile_dart/test_dev_docker_metadata.dart`;
  - package `pubspec.yaml` and checked-in `pubspec.lock` files;
  - `frb_codegen/assets/integration_template/**`;
  - `tools/frb_internal/assets/apple_scaffold/**`;
  - `tools/tart_macos/**`;
  - `frb_example/**`.

# 3 Choose PR boundaries before implementation

- Keep generated snapshots, their generator changes, required tests, and upgrade-specific compatibility migrations in
  the main upgrade PR.
- Split independent bug fixes, hardening, and reusable prerequisites into predecessor PRs according to
  `frb-pr-chain-split`.
- Build and maintain any predecessor chain exclusively with the official `gh stack` workflow described there.
- Put `ci-manual-dispatch` on dormant predecessors unless a predecessor specifically needs GitHub-only validation.
- Do not move incidental skill or workflow cleanup into the upgrade PR when it can stand alone against `master`.

# 4 Upgrade the development toolchain

- Update `.devcontainer/Dockerfile` inputs, including Flutter and any required Rust, Rust nightly, Node, Playwright,
  Chrome, system package, Java, or Android changes.
- Update tests that assert the derived dev image tag.
- Use `frb-dev-env` for the standard per-worktree environment. When the Dockerfile changes, use the local-image
  workflow in `frb-docker` to build and validate the unpublished image instead of reusing the stale container.
- Dry-run the publish workflow before depending on a new derived image tag.
- If Apple pins, simulators, or host tooling change, continue with the Tart workflow routed by `frb-dev-env` and
  read `frb-tart-prepare` before provisioning or validation.

# 5 Synchronize CI and post-release pins

- Update the top-level toolchain values together in `.github/workflows/ci.yaml` and
  `.github/workflows/post_release.yaml`:
  - `FRB_MAIN_FLUTTER_VERSION`;
  - `FRB_MAIN_DART_VERSION`;
  - `FRB_MAIN_RUST_VERSION` when the upgraded tooling requires it;
  - `FRB_RUSTFMT_NIGHTLY_VERSION` only when formatting or nightly `rust-src` behavior requires it.
- Keep a retained minimum Dart floor explicit and independently exercised; do not substitute the primary Dart pin for
  that compatibility lane.
- Review Java, Android SDK/NDK, Chrome/chromedriver, macOS runner, simulator, Windows ARM, and post-release install-mode
  assumptions.

# 6 Regenerate to convergence

- Read `frb-code-generation` and run the narrowest owning generator before broad generation.
- Normalize the intended source inputs, resolve dependencies, run final code generation, and format with the target
  toolchain. Do not hand formatter-unstable intermediate text to a different generation lane.
- Run the owning generation command a second time and require a clean diff. A single successful run is not convergence.
- Preserve semantic equivalence in templates. Do not move construction, scope, lifetime, or callback boundaries merely
  to make new formatter output look smaller.
- Classify every changed path by provenance:

| Provenance | Required action |
| --- | --- |
| Integration template | Change the template source and regenerate consumers |
| Apple scaffold asset | Refresh through the owning scaffold workflow |
| CargoKit copy | Follow the CargoKit ownership rules below |
| Generated example output | Keep the generator and snapshot together |
| Flutter migrator edit in a legacy example | Apply the exact current scaffold migration and record it as direct |
| Unexplained manual edit | Revert it or identify its owning source before proceeding |

- When an old example has no supported regeneration entry point, compare it with a fresh target-Flutter scaffold and
  directly apply only required tool-defined migrations. Record those files separately from automatic output.

### Step 7: Validate Locally

Read `frb-lint` and `frb-test` for exact command guidance. Tom's FRB environment runs tests locally,
usually through the per-worktree Docker container.

If the Flutter upgrade changed `.devcontainer/Dockerfile`, first ensure local validation is running
inside the fresh image built in Step 4. Seeing an old Dart or Flutter version locally means the
container is stale; recreate it before trusting any validation result.

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

Before treating the upgrade PR as ready, run the review gate in `frb-pr-review`.

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

### Step 9: Publish the Dev Docker Image After Merge

After the single upgrade PR is merged into `master`, trigger the publish workflow from `master` so the
new derived dev image tag exists for future CI and developer workflows:

```shell
gh workflow run publish_dev_docker.yaml --ref master
```

After it completes, verify both `linux/amd64` and `linux/arm64`:

```shell
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:latest
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:flutter-<flutter>-rust-<rust>-nightly-<nightly>
```

BuildKit attestation manifests can appear as `unknown/unknown`; those are not platform images.

## Non-Negotiables

- Keep `.github/workflows/ci.yaml` and `.github/workflows/post_release.yaml` toolchain env values in sync.
- Treat `.devcontainer/Dockerfile` `ARG` values as the source of truth for dev image tags.
- Dry-run the dev Docker image before depending on a new derived image tag in the PR.
- After changing `.devcontainer/Dockerfile`, build a fresh local image from that Dockerfile and use a
  container based on that fresh image for local validation. Do not keep using an older per-worktree
  container whose Flutter, Dart, Rust, Android, or browser tooling may still be stale.
- After the upgrade PR merges, trigger the dev Docker image publish workflow on `master`.
- Do not hand-edit generated files as the final state.
- Classify scaffold drift by source: integration template, Apple scaffold, Cargokit, or example output.

## PR Notes

For the single upgrade PR, include:

- Old and new Flutter/Dart versions
- Whether the dev image was dry-run or published
- Exact version tag of the dev image
- Generated/scaffold sources that changed
- Local validation commands and CI status
- Any known remaining platform-specific follow-up
