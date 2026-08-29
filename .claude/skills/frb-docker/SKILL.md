---
name: frb-docker
description: Use for ordinary flutter_rust_bridge Docker/devcontainer setup, local image usage, and Apple Silicon containers; route image upgrades and publishing to frb-upgrade-docker.
---

# FRB Docker

Use this skill for ordinary FRB Docker/devcontainer work: local container usage, Apple Silicon behavior, and local Dockerfile validation.

Read `frb-upgrade-docker` before changing toolchain inputs or publishing `fzyzcjy/flutter_rust_bridge_dev`.

## Source of Truth

- Devcontainer config: `.devcontainer/devcontainer.json`
- Dockerfile: `.devcontainer/Dockerfile`
- Publish workflow: `.github/workflows/publish_dev_docker.yaml`
- Published image: `fzyzcjy/flutter_rust_bridge_dev`

The Dockerfile is the source of truth for tool versions. Derive image tags from its `ARG` values instead of hardcoding stale versions.

## Daily Local Use

### Devcontainer

Prefer devcontainer for normal development.

```shell
Dev Containers: Reopen in Container
```

The devcontainer builds from `.devcontainer/Dockerfile` and runs `./frb_internal pub-get-all` to prepare Dart/Flutter package dependencies; first Rust/wasm builds may still be slow because crate compilation caches are not warmed.

### Published Image

Default behavior is still to build locally from `.devcontainer/Dockerfile`. If you want to use a prebuilt image instead, switch to the full version tag derived from the current Dockerfile args rather than `latest`.

```shell
docker run --rm -it -v "$PWD:/workspace" -w /workspace fzyzcjy/flutter_rust_bridge_dev:latest bash
```

Use `latest` only for quick local checks where reproducibility does not matter.

### Manual Docker Build

Use this when not using VS Code devcontainers, or when validating local Dockerfile changes.

```shell
docker build -f .devcontainer/Dockerfile -t frb-dev .devcontainer
docker run --rm -it -v "$PWD:/workspace" -w /workspace frb-dev bash
```

Inside a fresh manual container, run:

```shell
./frb_internal pub-get-all
```

Then run normal development commands such as:

```shell
./frb_internal lint
cargo check
```

## Apple Silicon

Apple Silicon Macs use the same commands as other platforms. Do not add `--platform linux/amd64` for normal local development.

Docker should select `linux/arm64` automatically for multi-arch images. If a command only works with `--platform linux/amd64`, treat that as a regression to investigate.

## Publishing

### Local Validation

For Dockerfile changes, validate at least:

```shell
docker build -f .devcontainer/Dockerfile -t frb-dev .devcontainer
docker run --rm -v "$PWD:/workspace" -w /workspace frb-dev bash -lc './frb_internal --help'
```

For environment changes, smoke-test the installed tools:

```shell
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

For broader confidence, run:

```shell
docker run --rm -v "$PWD:/workspace" -w /workspace frb-dev bash -lc './frb_internal pub-get-all && ./frb_internal lint --fix'
docker run --rm -v "$PWD:/workspace" -w /workspace/frb_rust frb-dev bash -lc 'cargo check && cargo check --target wasm32-unknown-unknown'
```

If generated files drift during lint/codegen, do not manually edit generated files. Restore unrelated generated drift unless the task intentionally changes generation outputs.
