---
name: frb-upgrade-docker
description: Upgrade and publish the flutter_rust_bridge development Docker image.
---

# FRB Upgrade Docker

### Publish Workflow

Publish the dev image from the workflow:

```shell
gh workflow run publish_dev_docker.yaml --ref master
```

Manual dispatch defaults to `publish=true`.

To verify the workflow without publishing:

```shell
gh workflow run publish_dev_docker.yaml --ref master -f publish=false
```

The workflow builds and smoke-tests:

- `linux/amd64` on `ubuntu-latest`
- `linux/arm64` on `ubuntu-24.04-arm`

When publishing, it pushes per-platform images and then creates multi-arch tags:

- `latest`
- `flutter-<flutter_version>-rust-<rust_version>-nightly-<rust_nightly_version>`
- `flutter-<flutter_version>-rust-<rust_version>-nightly-<rust_nightly_version>-code-<short_sha>`
- `sha-<short_sha>`

After publishing, inspect the manifest:

```shell
docker buildx imagetools inspect fzyzcjy/flutter_rust_bridge_dev:latest
```

It should include `linux/amd64` and `linux/arm64`. BuildKit attestation manifests may appear as `unknown/unknown`; those are not platform images.

Inspect the image revision label when checking exactly which source commit was published:

```shell
docker inspect fzyzcjy/flutter_rust_bridge_dev:latest \
  --format '{{ index .Config.Labels "org.opencontainers.image.revision" }}'
```

## CI Workflow Guidance

Avoid building the full Rust/Flutter arm64 image under QEMU on an amd64 runner; it is much slower than native arm64 and can make the workflow impractical. Use native arch runners for heavy image builds.

If a dry-run passes but publishing fails, inspect whether Docker Hub login, per-platform push, or manifest creation failed; those are separate phases in the workflow.
