# FRB Tart Base VM Preparation

Use this skill when preparing the reusable Tart base VM used by `frb-dev-env tart ...`.

## Model

The reusable base VM is a clean local VM built by Packer from a pinned Tart OCI artifact:

```text
ghcr.io/cirruslabs/macos-sonoma-xcode:16.1
  -> optional local OCI registry mirror
  -> packer build ... frb-tart-base-candidate
  -> smoke test candidate
  -> promote candidate to frb-tart-base
  -> tart clone frb-tart-base frb-tart-<worktree-hash>
```

`frb-tart-base` is the prepared base image. Do not use it for development, do not run tests in it, and do not manually install tools into it. Treat it as immutable after creation. Per-worktree VMs are cloned from it via APFS copy-on-write and can be dirtied, stopped, deleted, and recreated.

## Why Not Mutate the Base VM

Do not boot `frb-tart-base` and manually install Flutter, Android SDK packages, CocoaPods updates, Rust targets, or other dependencies into it.

Manual changes make the base hard to reproduce and hard to audit. If the base needs different tools, update the Packer template or provision script next to this skill, document the exact source, rebuild a candidate, smoke test it, and then promote it to `frb-tart-base`. The base VM should answer both questions: "which OCI artifact did this come from?" and "which provision script changed it?"

## Source Image

Use a pinned macOS/Xcode OCI reference. The current known-good source is:

```text
ghcr.io/cirruslabs/macos-sonoma-xcode:16.1
```

Avoid `latest` tags. Avoid broad multi-Xcode runner images unless there is a specific reason; they are larger and less reproducible.

## Fast Path When Network Is Good

If network access to GHCR is reliable enough, Packer can clone the raw source image directly:

```bash
cd tools/tart_macos/packer
packer init .
packer build \
  -var 'source_vm=ghcr.io/cirruslabs/macos-sonoma-xcode:16.1' \
  -var 'target_vm=frb-tart-base-candidate' \
  -var 'flutter_version=3.44.0' \
  -var 'allow_insecure=false' \
  .
```

Use this only when the direct download path is acceptable. Packer delegates the source VM clone to Tart, so shell proxy environment variables may not control it the way they control CLI tools like `curl` or `crane`.

## Controlled Local Registry Path

Use this path when direct `tart clone ghcr.io/...` is slow, unreliable, or hard to route through the desired proxy. The idea is:

1. Run a local OCI registry on `127.0.0.1:5000`.
2. Use `crane copy` with explicit proxy environment variables to mirror the remote Tart OCI artifact into the local registry.
3. Run Packer with `allow_insecure=true` and `source_vm=127.0.0.1:5000/...` to create `frb-tart-base-candidate`.
4. Smoke test the candidate.
5. Promote it to `frb-tart-base`.

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

## Build Base VM With Packer

Build a candidate base VM from the local registry:

```bash
cd tools/tart_macos/packer
packer init .
packer build \
  -var 'source_vm=127.0.0.1:5000/cirruslabs/macos-sonoma-xcode:16.1' \
  -var 'target_vm=frb-tart-base-candidate' \
  -var 'flutter_version=3.44.0' \
  -var 'allow_insecure=true' \
  .
```

The Packer template clones the raw source VM, boots it as `admin/admin`, runs `scripts/provision-frb-tart-base.sh`, and leaves a stopped local VM named by `target_vm`. The current provision step initializes Xcode, checks out the requested Flutter SDK version, installs CocoaPods with Homebrew, installs Rust matching the devcontainer, adds the `aarch64-apple-ios-sim` and `x86_64-apple-ios` Rust targets for both the pinned toolchain and the `stable-aarch64-apple-darwin` toolchain used by CargoKit, and pre-caches Flutter iOS and macOS artifacts. It also verifies the image contains Xcode and simulator tooling.

Keep `flutter_version` in sync with `FRB_MAIN_FLUTTER_VERSION` in `.github/workflows/ci.yaml` and the Flutter version in `.devcontainer/Dockerfile`. Do not rely on the source Tart OCI image's preinstalled Flutter version, because Flutter template drift can change generated scaffold files.

Verify the candidate:

```bash
tart list
tart get frb-tart-base-candidate --format json
```

Do not boot and mutate `frb-tart-base` during verification. If you need to inspect runtime state, clone a disposable probe VM from the candidate:

```bash
tart clone frb-tart-base-candidate frb-tart-probe
tart run --no-graphics frb-tart-probe
tart ip frb-tart-probe --wait 180
tart exec frb-tart-probe /bin/zsh -lc 'sw_vers && xcodebuild -version && flutter --version && flutter --version --machine | python3 -c '"'"'import json,sys; assert json.load(sys.stdin)["frameworkVersion"] == "3.44.0"'"'"' && pod --version && rustc --version && cargo --version && rustup target list --installed | grep aarch64-apple-ios-sim && rustup target list --installed | grep x86_64-apple-ios && rustup target list --toolchain stable-aarch64-apple-darwin --installed | grep aarch64-apple-ios-sim && rustup target list --toolchain stable-aarch64-apple-darwin --installed | grep x86_64-apple-ios'
tart stop frb-tart-probe
tart delete frb-tart-probe
```

After the candidate passes smoke, promote it by moving the old base out of the way and renaming the candidate:

```bash
stamp=$(date +%Y%m%d-%H%M%S)
tart rename frb-tart-base "frb-tart-base-before-packer-$stamp"
tart rename frb-tart-base-candidate frb-tart-base
```

If there is no existing `frb-tart-base`, only the second rename is needed.

## Cost Notes

Local VM-to-VM cloning is cheap because Tart uses APFS copy-on-write for local clones. `du` may count shared blocks and look large, so prefer `df` available space when measuring real disk pressure.

Even though local clone is cheap, keep a conservative free-space threshold before creating worktree VMs. `frb-dev-env tart create` requires at least 150GB free by default because base image setup, caches, build products, and simulator state can grow quickly.
