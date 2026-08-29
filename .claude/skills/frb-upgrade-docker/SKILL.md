---
name: frb-upgrade-docker
description: Upgrade and publish the flutter_rust_bridge development Docker image, including toolchain bumps, image build fixes, candidate tags, and default-branch promotion.
---

# 1 Scope the PR

- Put every development-image upgrade in an independent PR targeting `master`. Make it the first PR in a toolchain-upgrade chain.
- Include all Docker-owned changes required by the upgrade:
  - `.devcontainer/Dockerfile` toolchain and system packages;
  - `.github/workflows/publish_dev_docker.yaml` build, test, and publication logic;
  - image metadata tests, tag derivation, and Docker-specific supporting code.
- Leave FRB runtime fixes, generated scaffolds, and example migrations to later PRs.
- Read `frb-docker` for local use and `frb-dev-env` before running repository commands in a container.

# 2 Build before merge

- Make Dockerfile and publish-workflow PRs automatically build and smoke-test native `linux/amd64` and `linux/arm64` images with `push: false`.
- Do not expose registry credentials to untrusted pull-request code.
- Derive image tags from Dockerfile `ARG` values instead of duplicating versions.
- Verify Flutter, Dart, Rust, the pinned nightly and wasm target, Node, Yarn, `wasm-pack`, Chrome, ChromeDriver, and `./frb_internal --help`.
- For a manual build-only check, run:

```shell
gh workflow run publish_dev_docker.yaml --ref <branch> -f publish=false
```

# 3 Publish safely

- If later PRs need the image before merge, let a maintainer explicitly publish only an immutable
  `candidate-pr-<number>-sha-<short_sha>` tag. Reject a conflicting existing tag; allow only an identical retry.
- Never update `latest`, canonical version tags, or stable SHA tags from pull-request code.
- Publish stable tags only from a push to `master`. A manual stable dispatch must check out the current remote
  `master` head, not an older reachable commit.
- After merge, publish from `master`:

```shell
gh workflow run publish_dev_docker.yaml --ref master
```

- Stable publication may then update:
  - `latest`;
  - `flutter-<flutter>-rust-<rust>-nightly-<nightly>`;
  - the version-plus-code and stable SHA tags.

# 4 Verify and hand off

- Inspect the candidate when used, then inspect the canonical tag and `latest` after merge:

```shell
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:<tag>
docker inspect fzyzcjy/flutter_rust_bridge_dev:latest \
  --format '{{ index .Config.Labels "org.opencontainers.image.revision" }}'
```

- Require `linux/amd64` and `linux/arm64`; ignore BuildKit attestation entries reported as `unknown/unknown`.
- Merge latest `master` into the remaining chain, switch it from any candidate to the canonical tag, and do not merge
  a consumer before that tag exists.
- Diagnose Docker Hub login, per-platform pushes, and manifest creation separately.
- If local `latest` is stale, use the exact candidate/canonical tag or build the Dockerfile locally; do not generate
  locks or scaffolds with the wrong toolchain.
