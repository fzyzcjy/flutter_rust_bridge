---
name: frb-dev-env
description: "Use when the user wants Docker-based FRB development or Tart-based iOS Simulator validation."
---

# FRB Development Environment

Use this skill before setting up, diagnosing, or running `flutter_rust_bridge` commands with a container-based development workflow.

## Choosing an Environment

Use Docker for normal FRB development checks that fit Linux containers: Rust tests, Dart tests, web tests, code generation, lint, Android build checks that do not need a running emulator, and most `./frb_internal` commands. Docker is the default reproducible baseline when local host toolchains are missing or suspected to have drifted.

Use a host Android Emulator for local Android runtime validation when a running Android target is required. Keep FRB, Flutter, Rust, Gradle, and Android build tooling inside Docker. The host runs only the emulator and the Android SDK command-line tools needed to manage it. Docker runs its own ADB server and connects directly to the emulator TCP endpoint so Flutter's VM service forwarding stays inside Docker. See the Android section below.

Use Tart only for checks that need macOS and Xcode, especially iOS Simulator validation. The Tart workflow is intentionally separate from Docker; it gives each worktree a reusable macOS VM cloned from a prepared base VM, then runs FRB commands inside that VM.

Do not use the Tart macOS VM as the Android emulator strategy. macOS Tart guests do not support nested virtualization, so Android emulator runtime coverage needs a host or runner with virtualization support. Android SDK compilation checks can still use Docker when they do not need a running emulator.

## Principles

- FRB runs locally in Docker by default. Do not use remote workspaces for FRB unless the user explicitly asks for a one-off remote run.
- Use the current checkout/worktree that Codex or the user already selected. Do not create a new worktree unless the user explicitly asks.
- Run FRB commands from the repository root unless the command itself intentionally changes directories.
- Prefer repository tooling such as `./frb_internal` over ad hoc direct invocations.
- Do not manually edit generated files (`frb_generated.*`, `*.freezed.dart`, `*.g.dart`) as the final fix.

## First Checks

Before running tests, lint, code generation, or setup:

```bash
git rev-parse --show-toplevel
git status --short
git submodule status --recursive
```

If submodules are uninitialized, initialize them locally:

```bash
git submodule update --init --recursive
```

## Docker

Use Docker for container-based FRB development. Each worktree should have one long-lived local container that is reused for tests, lint, code generation, and setup commands.

Use `frb_dev_env.py` next to this skill to inspect, create, start, and reuse the per-worktree container. The script derives the container name from the canonical worktree root only:

```text
frb-<worktree-root-sha256-prefix-12>
```

It mounts the canonical worktree root and git common root at their host-like absolute paths, and labels the container with:

```text
frb.dev.repo=flutter_rust_bridge
frb.dev.worktree=<canonical worktree root>
frb.dev.git-common-root=<git common root>
frb.dev.layout-version=3
```

Commands run from the host-like worktree path inside the container. There is intentionally no `/workspace` alias, because linked git worktrees and submodule gitdir references need the same path shape that they have on the host.

When creating a new per-worktree container, the helper pulls the selected Docker image before `docker run`. Existing containers are validated and reused without pulling.

Typical usage:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker info
.claude/skills/frb-dev-env/frb_dev_env.py docker create
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc './frb_internal --help'
```

### Temporary Docker Commands With Publish Credentials

Use `docker-run-rm --with-publish-credentials` only for release publishing commands that need host credentials. Unlike the normal long-lived development container, this runs a fresh temporary container with `docker run --rm`, mounts the worktree and git common root, mounts the host credential directories read-only, copies the credential files into temporary container-local homes, checks login status, runs the requested command, then discards the container.

The publish credential flag mounts these host credential sources when present:

- GitHub CLI config from `GH_CONFIG_DIR` or `~/.config/gh`
- GitHub token env vars from `GH_TOKEN` or `GITHUB_TOKEN`, forwarded without printing token values
- Cargo credentials and config from `CARGO_HOME` or `~/.cargo`
- Git config from `~/.gitconfig` and `~/.config/git`
- Dart pub credentials from `PUB_CACHE`, `~/.pub-cache`, `~/.config/dart`, or `~/Library/Application Support/dart`

Run the credential preflight before any irreversible release step:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- true
```

Run release publishing through a temporary publish-credential container:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- ./frb_internal release
```

The credential preflight fails before running release commands if host GitHub CLI auth is invalid, Cargo credentials are missing, or Dart pub credentials are missing. It then re-checks GitHub CLI auth inside the temporary container and runs `gh auth setup-git` there so HTTPS `git push` can use the mounted GitHub CLI auth.

### Cleanup

Delete a worktree's Docker container when the worktree is no longer needed, or when local Docker resources are getting crowded:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker delete
```

The delete command validates the container labels before removing it. Use `--force` only when intentionally removing a mismatched container.

Use the project `frb-docker` skill for image, devcontainer, and Dockerfile details.

## Android Host Runtime

Use this workflow for local Android runtime checks when Docker can build and run the FRB/Flutter command, but the Android target itself must be managed by the host.

### Path Selection

The supported Android runtime path is:

```text
host Android Emulator:
  host runs the emulator process
  Docker runs FRB/Flutter and its own ADB server
  Docker ADB connects to host.docker.internal:<emulator-adb-port>
```

Physical Android phones are intentionally out of scope for this helper until that path is manually validated. The host does not need FRB, Flutter, Rust, Cargo, Java, or Gradle for the emulator path.

### Host Emulator Architecture

Use this split for a host Android Emulator:

```text
host:
  Android Emulator process
  Android SDK command-line tools for SDK packages, AVD creation, emulator start/stop, and host-side inspection
  Android SDK root: ~/Library/Android/sdk unless ANDROID_HOME is set
  AVD root: ~/.android/avd

Docker container:
  FRB checkout
  Flutter, Dart, Rust, Cargo, Java, Gradle
  Android SDK/NDK/build tools needed for builds
  Docker-local ADB server connected to host.docker.internal:<emulator-adb-port>
```

Read the project `frb-android-emulator-prepare` skill before preparing a host emulator. The host emulator path needs Java, Android SDK command-line tools, emulator packages, a system image, and an AVD in the standard Android Studio host locations.

Print the resolved host Android SDK environment if needed:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android env
```

Start the emulator on the host. Pin the port when you want a stable host serial such as `emulator-5554`; the paired emulator ADB TCP endpoint is one port higher, such as `host.docker.internal:5555`:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android emulator --avd FRB_API_35 --port 5554
```

Verify Docker-local ADB can see the host emulator:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- adb devices -l
```

Run the FRB/Flutter command against the Docker-visible emulator serial:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-emulator-adb -- \
  bash -lc './frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id host.docker.internal:5555"'
```

`docker exec --android-emulator-adb` first runs `adb connect host.docker.internal:5555` inside the container, then runs the requested command with the Android SDK x86_64 runtime library path set. Override the default endpoint with `--android-emulator-adb-host`, `--android-emulator-adb-port`, `FRB_ANDROID_EMULATOR_ADB_HOST`, or `FRB_ANDROID_EMULATOR_ADB_PORT`.

### Why Emulator Does Not Use Host ADB Server

Do not use a host ADB server for host emulator full Flutter integration tests. Flutter creates dynamic VM service `adb forward` ports where the ADB server runs. If the ADB server runs on the host, those forwarded ports bind on host `127.0.0.1`, but Flutter is running inside Docker and then tries Docker-local `127.0.0.1`. This fails with DDS or VM service connection refused errors.

Use `--android-emulator-adb` so the ADB server and the forwarded VM service port both live inside Docker.

### Security And Cleanup

- Use a single Android target when possible. With multiple emulators or devices, always pass `--device-id <serial>` through `--flutter-test-args`.
- For host emulator Flutter integration tests, use `--android-emulator-adb` so `adb forward` creates VM service forwarding inside Docker.
- Do not run the Android Emulator inside Tart. Do not rely on Docker nested virtualization for Android emulator runtime checks on macOS.
- If the Docker image lacks Android SDK, NDK, or build-tools packages required by Flutter Android builds, update the FRB Docker image or mount a dedicated Android SDK into the container; do not install host Flutter or Gradle just to work around the missing container toolchain.

## Tart

Use Tart for iOS Simulator validation when Docker cannot provide the required macOS/Xcode runtime. The script uses one reusable Tart VM per worktree, named from the canonical worktree root:

```text
frb-tart-<worktree-root-sha256-prefix-12>
```

### Base VM

By default, `create` clones from a prepared local base VM named `frb-tart-base`. Override it with `FRB_TART_BASE_VM` or `--base-vm`.

Read `frb-tart-prepare` before creating or changing the base VM. The base VM should be built by the checked-in Packer template from a pinned Tart OCI artifact and treated as immutable; do not boot it and manually install tools into it.

### Mounted Checkout

For linked git worktrees, the Tart helper shares both the canonical worktree root and the git common root into the VM, then mounts them at host-like absolute paths with `mount_virtiofs`. This keeps worktree `.git` files and submodule gitdir references valid inside the VM.

Use plain `tart exec` for light commands that should see the mounted host checkout exactly, such as `git status`, `flutter --version`, `xcrun simctl list`, and simulator boot commands.

### VM Lifecycle

Inspect, create, start, and run light commands in the per-worktree VM:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart info
.claude/skills/frb-dev-env/frb_dev_env.py tart create
.claude/skills/frb-dev-env/frb_dev_env.py tart start
.claude/skills/frb-dev-env/frb_dev_env.py tart ip --wait 180
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- sw_vers
```

Delete the worktree VM when it is no longer needed:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart stop
.claude/skills/frb-dev-env/frb_dev_env.py tart delete
```

### VM-Local Copy

For heavy iOS build/test commands, prefer uploading the current worktree to a VM-local copy and running there. This avoids writing Xcode build artifacts through the virtiofs host mount:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart upload
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --flutter-test-args '--device-id <UDID>' --package <package>
```

`tart upload` and `tart exec --sync-code` both first ensure the worktree VM is running and the host-like worktree mount exists. They then run `rsync` inside the VM from the mounted host worktree to a VM-local copy under `/Users/admin/frb-dev-env-local-copies/<worktree-hash>`. The upload excludes `.git`, `.dart_tool`, `build`, `target`, `.idea`, and `.vscode`. It does not delete existing files in the VM-local copy, so build caches survive repeated runs.

Commands run with `--sync-code` start from the VM-local uploaded copy, not from the host-mounted path.

Examples:

```bash
# Show the VM-local path after uploading the current worktree.
.claude/skills/frb-dev-env/frb_dev_env.py tart upload

# Run a heavy command from the uploaded VM-local copy.
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args '--device-id <UDID>'
```

### iOS Integration Preparation

Before running iOS Simulator integration tests, prepare the VM Flutter SDK once per VM or after replacing Flutter. This applies the runtime portion of `https://github.com/flutter/flutter/pull/187643.patch` as an idempotent Flutter tool workaround for an iOS simulator VM Service discovery race where `simctl launch` can emit the Dart VM Service log before Flutter's `simctl log stream` listener is ready.

The preparation command also rebuilds `flutter_tools.snapshot`:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart prepare-ios-integration-test
```

Use the environment shown in the test command below for the integration test. `FLUTTER_GIT_URL` keeps `flutter doctor` and Flutter metadata consistent when the VM uses a local Flutter checkout. `FRB_SKIP_FLUTTER_DOCTOR=1` skips the FRB wrapper's preflight doctor because the simulator integration test itself is the validation target and doctor can hang after printing device checks in Tart.

### iOS Integration Test

Run iOS Simulator tests by starting the worktree VM, preparing the VM Flutter SDK, choosing and booting an iOS simulator inside the VM, then running the existing FRB test command from the VM-local uploaded copy:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart start --wait 300
.claude/skills/frb-dev-env/frb_dev_env.py tart prepare-ios-integration-test
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl list devices available
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl boot <UDID>
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl bootstatus <UDID> -b
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- bash -lc 'export FLUTTER_GIT_URL=file:///Users/admin/flutter; export FRB_SKIP_FLUTTER_DOCTOR=1; ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args "--device-id <UDID>"'
```

For example, when an iOS 18.1 `iPhone 16 Pro Max` simulator with UDID `826DC3E8-2073-42A9-A6DB-05C1926DC82A` is available:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- bash -lc 'export FLUTTER_GIT_URL=file:///Users/admin/flutter; export FRB_SKIP_FLUTTER_DOCTOR=1; ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args "--device-id 826DC3E8-2073-42A9-A6DB-05C1926DC82A"'
```

## VMware Windows

Use VMware Fusion + Vagrant for local Windows desktop validation when Docker, Tart, and the Android Emulator coverage are not enough. This workflow targets Apple Silicon macOS hosts running Windows 11 Arm in VMware Fusion.

Read `tools/vmware_windows/README.md` for the detailed Packer, Vagrant, VMware Fusion, proxy, storage, host-installation, daily-use, and troubleshooting instructions. The README is the source of truth for the Windows VM workflow. Read `frb-vmware-windows-prepare` before preparing or rebuilding the reusable Packer-built base box.

### Storage

Configure a per-user VM root before creating or building VMs. The helper reads
`~/.config/flutter_rust_bridge/vmware_windows.json` by default, and environment variables such as
`FRB_WINDOWS_VM_ROOT` override the config. The helper derives all heavy paths from that configured
root, including Vagrant home, boxes, per-worktree project directories, Packer cache/output, logs,
and VM bundles. Check it before starting a VM:

```bash
tools/vmware_windows/frb_vmware_windows_env.py info
```

The helper uses only the Python standard library and refuses to create directories under `/Volumes/<name>/...` unless that volume is mounted. This keeps first-run helper checks independent from `uv`/`pip` and avoids accidentally writing VM files to the internal disk when an external drive is absent.

### Network

If Windows VM downloads and Packer host-side network operations need a proxy, configure it in the
user config or with:

```bash
export FRB_WINDOWS_HOST_PROXY_URL=http://localhost:PORT
```

Leave it unset when direct network access is preferred. If guest-side provision scripts need a
proxy, set `FRB_WINDOWS_GUEST_PROXY_URL` to an address reachable from inside the Windows guest.

For real Packer builds, provide the Windows ISO SHA-256 checksum with `FRB_WINDOWS_ISO_CHECKSUM=sha256:<digest>` or an `.iso.sha256` file next to the ISO. The helper uses a placeholder checksum only for `packer-build --validate`.

### Base Box Preparation

For the initial base-image build on a developer Mac, prefer the ordinary-Terminal wrapper:

```bash
tools/vmware_windows/run_packer_build.zsh \
  --iso /path/to/frb-windows-vmware/downloads/iso/Windows11_Arm64.iso \
  --add-box
```

The wrapper applies the configured host proxy when present, runs preflight checks, regenerates the
UEFI autounattend ISO, writes logs under the configured VM root, monitors `/` and the VM root for the
100GB free-space threshold, and can import the produced Vagrant box. Use `--prepare-only` when you
want to validate ISO remastering before starting VMware.

### Daily Lifecycle

Run from the repository root:

```bash
tools/vmware_windows/frb_vmware_windows_env.py info
tools/vmware_windows/frb_vmware_windows_env.py start
tools/vmware_windows/frb_vmware_windows_env.py upload
tools/vmware_windows/frb_vmware_windows_env.py exec -- powershell -NoProfile -Command "cd C:\frb\worktree; flutter --version"
tools/vmware_windows/frb_vmware_windows_env.py stop
```

`upload` copies the host checkout into a VM-local directory before heavy builds. Do not compile from a VMware shared folder because shared folders are slower for many small files and can distort build behavior.

### Windows Flutter Test

After the VM is running and code is uploaded:

```bash
tools/vmware_windows/frb_vmware_windows_env.py test-flutter-windows \
  --package frb_example/flutter_via_create
```

The command runs inside the guest and should use the VM-local uploaded checkout. Capture logs under the external root for audit.

### Credentials And Cleanup

Do not put guest passwords in the repository. Use `FRB_WINDOWS_VM_USERNAME`, `FRB_WINDOWS_VM_PASSWORD`, or `~/.secrets/FRB_WINDOWS_VM_PASSWORD`.

Stop the per-worktree VM when it is not needed:

```bash
tools/vmware_windows/frb_vmware_windows_env.py stop
```

Delete disposable per-worktree VMs only when intentionally reclaiming space:

```bash
tools/vmware_windows/frb_vmware_windows_env.py destroy
```

## Project Skills

After applying this environment policy, also read the relevant project-level `frb-*` skills for the actual task, such as code generation, tests, lint, Docker details, CI triage, or PR preparation.
