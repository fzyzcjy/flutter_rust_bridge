---
name: frb-tart-prepare
description: Use when preparing or explaining the FRB Tart base VM for iOS Simulator validation, including local OCI registry mirroring, direct OCI clone, and base VM hygiene.
---

# FRB Tart Base VM Preparation

Use this skill when preparing the reusable Tart base VM used by `frb-dev-env tart ...`.

## Model

The reusable base VM is a clean local VM cloned directly from a pinned OCI artifact:

```text
ghcr.io/cirruslabs/macos-sonoma-xcode:16.1
  -> optional local OCI registry mirror
  -> tart clone ... frb-tart-base
  -> tart clone frb-tart-base frb-tart-<worktree-hash>
```

`frb-tart-base` is the base image. Do not use it for development, do not run tests in it, and do not manually install tools into it. Treat it as immutable after creation. Per-worktree VMs are cloned from it via APFS copy-on-write and can be dirtied, stopped, deleted, and recreated.

## Why Not Mutate the Base VM

Do not boot `frb-tart-base` and manually install Flutter, Android SDK packages, CocoaPods updates, Rust targets, or other dependencies into it.

Manual changes make the base hard to reproduce and hard to audit. If the base needs different tools, choose or build a new pinned OCI image, document the exact source, and recreate `frb-tart-base` from that artifact. The base VM should answer the question "which OCI artifact did this come from?", not "what did someone install by hand last week?".

## Source Image

Use a pinned macOS/Xcode OCI reference. The current known-good source is:

```text
ghcr.io/cirruslabs/macos-sonoma-xcode:16.1
```

Avoid `latest` tags. Avoid broad multi-Xcode runner images unless there is a specific reason; they are larger and less reproducible.

## Fast Path When Network Is Good

If network access to GHCR is reliable enough, create the base VM directly:

```bash
tart clone ghcr.io/cirruslabs/macos-sonoma-xcode:16.1 frb-tart-base
```

Use this only when the direct download path is acceptable. `tart clone` uses macOS networking, so shell proxy environment variables may not control it the way they control CLI tools like `curl` or `crane`.

## Controlled Local Registry Path

Use this path when direct `tart clone ghcr.io/...` is slow, unreliable, or hard to route through the desired proxy. The idea is:

1. Run a local OCI registry on `127.0.0.1:5000`.
2. Use `crane copy` with explicit proxy environment variables to mirror the remote Tart OCI artifact into the local registry.
3. Run `tart clone --insecure` from the local registry to create `frb-tart-base`.

### Start Local Registry

Use a large disk for registry storage:

```bash
mkdir -p /Volumes/MyExternal/temp/frb-mobile-registry

docker run -d --name frb-mobile-registry \
  -p 127.0.0.1:5000:5000 \
  -v /Volumes/MyExternal/temp/frb-mobile-registry:/var/lib/registry \
  registry:2
```

Verify it:

```bash
NO_PROXY=127.0.0.1,localhost,::1 no_proxy=127.0.0.1,localhost,::1 \
  curl -fsS http://127.0.0.1:5000/v2/
```

Use `127.0.0.1`, not `localhost`, to avoid IPv6 or host service surprises.

### Mirror OCI Artifact

Mirror with `crane`. If a proxy is needed, set it explicitly for this command:

```bash
PROXY_URL=http://localhost:7897

HTTP_PROXY=$PROXY_URL HTTPS_PROXY=$PROXY_URL \
http_proxy=$PROXY_URL https_proxy=$PROXY_URL \
ALL_PROXY=$PROXY_URL all_proxy=$PROXY_URL \
NO_PROXY=localhost,127.0.0.1,::1 no_proxy=localhost,127.0.0.1,::1 \
  crane --insecure copy --jobs 4 \
  ghcr.io/cirruslabs/macos-sonoma-xcode:16.1 \
  127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1
```

For long mirrors, run it in `screen` and write logs to disk:

```bash
PROXY_URL=http://localhost:7897

screen -dmS frb-mobile-crane-copy zsh -lc "HTTP_PROXY=$PROXY_URL HTTPS_PROXY=$PROXY_URL http_proxy=$PROXY_URL https_proxy=$PROXY_URL ALL_PROXY=$PROXY_URL all_proxy=$PROXY_URL NO_PROXY=localhost,127.0.0.1,::1 no_proxy=localhost,127.0.0.1,::1 crane --insecure copy --jobs 4 ghcr.io/cirruslabs/macos-sonoma-xcode:16.1 127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1 >> /Volumes/MyExternal/temp/frb-mobile-crane-copy.log 2>&1"
```

Check that `crane` only connects to the intended local proxy, if any, and local registry:

```bash
PROXY_PORT=7897

lsof -nP -p <crane-pid> | rg "TCP|127\\.0\\.0\\.1|:${PROXY_PORT}|:5000|ghcr|github"
```

Expected network shape:

- `127.0.0.1:<ephemeral> -> 127.0.0.1:<selected-proxy-port>` for remote GHCR fetches through the selected proxy.
- `127.0.0.1:<ephemeral> -> 127.0.0.1:5000` for local registry writes.
- No other proxy ports unless they were intentionally selected.

### Verify Local Registry Digest

Use an empty Docker config if the host credential helper interferes:

```bash
mkdir -p /private/tmp/frb-empty-docker-config

DOCKER_CONFIG=/private/tmp/frb-empty-docker-config \
NO_PROXY=localhost,127.0.0.1,::1 no_proxy=localhost,127.0.0.1,::1 \
  crane --insecure digest 127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1
```

Known digest for the current source image:

```text
sha256:f181b76eede9acd7a6db76438a4cf73e76550c3f9e8104aed16347122e132184
```

## Create Base VM

Create `frb-tart-base` from the local registry:

```bash
tart clone --insecure \
  127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1 \
  frb-tart-base
```

Verify:

```bash
tart list
tart get frb-tart-base --format json
```

Do not boot and mutate `frb-tart-base` during verification. If you need to inspect runtime state, clone a disposable probe VM from it:

```bash
tart clone frb-tart-base frb-tart-probe
tart run --no-graphics frb-tart-probe
tart ip frb-tart-probe --wait 180
tart exec frb-tart-probe sw_vers
tart stop frb-tart-probe
tart delete frb-tart-probe
```

## Cost Notes

Local VM-to-VM cloning is cheap because Tart uses APFS copy-on-write for local clones. `du` may count shared blocks and look large, so prefer `df` available space when measuring real disk pressure.

Even though local clone is cheap, keep a conservative free-space threshold before creating worktree VMs. `frb-dev-env tart create` requires at least 150GB free by default because base image setup, caches, build products, and simulator state can grow quickly.
