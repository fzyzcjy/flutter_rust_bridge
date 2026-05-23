---
name: frb-upgrade-flutter
description: >-
  Upgrade flutter_rust_bridge to a new Flutter stable release. Use when changing Flutter/Dart versions,
  devcontainer Docker images, CI/post-release pins, generated Flutter scaffolds, or platform compatibility.
---

# FRB Upgrade Flutter

Use this as the entry workflow for Flutter stable bumps in `flutter_rust_bridge`.

## Start Here

1. Confirm the target Flutter stable release from official Flutter sources.
2. Read `references/workflow.md` before editing files.
3. Also read:
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

## Preferred PR Shape

Prefer small PRs in this order:

1. Workflow skill/docs update, if the process needs clarification.
2. Dev Docker image upgrade and publish.
3. CI and post-release version pin update.
4. Generated/scaffold drift.
5. Real compatibility fixes.

## Quick Inventory

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
