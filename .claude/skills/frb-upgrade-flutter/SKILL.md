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

# 7 Preserve the OHOS composition contract

- Treat the checked-in package and the older OHOS Flutter fork as two distinct sources of truth:
  - generic Flutter scaffold files come from the target stable Flutter package;
  - overlay only explicit OHOS-owned paths from the OHOS fork;
  - include `ohos` and, when required by the package, `rust_builder/ohos` and `rust_builder/pubspec.yaml`;
  - exclude transient trees such as `node_modules`;
  - preserve intentional deletions instead of silently resurrecting stale files.
- Make preservation transactional. Stage and validate the composed result before replacing the canonical package, and
  restore the original package if create, dependency resolution, code generation, formatting, or the final swap fails.
- Test both success and failure paths. Do not add production APIs whose only purpose is exposing internals to tests.

# 8 Route CargoKit changes correctly

- Use `frb-cargokit` to determine whether a diff is an upstream source change, an FRB copy-sync change, or generated
  integration output.
- Use `frb-cargokit-dev` and a separate upstream change only when behavior belongs in the external CargoKit repository.
- Use the owning sync command for source-copy updates; do not manually patch every copied CargoKit tree.
- Keep `precommit-integrate` and CargoKit copy synchronization conceptually separate.
- Treat nested `cargokit/build_tool` packages according to their own Dart package metadata even when the parent package
  is a Flutter package.

# 9 Validate in dependency order

- Use the environment selected by `frb-dev-env`. When the Dockerfile changed, validate with the fresh local-image
  workflow from `frb-docker`.
- Read `frb-lint` and `frb-test` for exact commands.
- Validate in this order:
  1. dev-image metadata and tool versions;
  2. Dart analysis, tests, and the retained minimum-SDK lane;
  3. internal generation and second-pass cleanliness;
  4. integration generation and CargoKit synchronization;
  5. focused legacy Android and Apple platform builds affected by migrations;
  6. representative native and web examples.
- Before creating or updating the PR, run `frb-prepare-pr`.
- Before declaring it ready, run the independent review gate in `frb-pr-review`.

# 10 Triage CI and finish

- Read `frb-fix-ci` before deep CI debugging.
- Triage failures in dependency order: environment setup, generation, integration, platform builds, post-release,
  then coverage or uploads.
- Compare platform failures with target-Flutter release notes and fresh scaffold output before adding workarounds.
- Keep full CI on the top upgrade PR. Follow `frb-pr-chain-split` for filtered or deferred predecessor CI.
- After the upgrade merges, publish the dev Docker image from `master` when `.devcontainer/Dockerfile` changed, then
  verify the expected `linux/amd64` and `linux/arm64` manifests.

# 11 PR notes

- Record old and new Flutter and primary Dart versions.
- Record the retained or raised minimum Dart SDK floor and its validation lane.
- List generated files, direct Flutter migrator edits, CargoKit changes, and OHOS overlays by provenance.
- Link predecessor PRs and identify which ones intentionally use manual CI.
- Record the fresh dev-image tag, exact local validations, generation convergence result, and CI status.
- Call out any platform-specific follow-up that remains intentionally out of scope.
