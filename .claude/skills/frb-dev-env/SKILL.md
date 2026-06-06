---
name: frb-dev-env
description: "Use when the user wants Docker-based FRB development or Tart-based iOS Simulator validation."
---

# FRB Development Environment

Use this skill before setting up, diagnosing, or running `flutter_rust_bridge` commands with a container-based development workflow.

## Choosing an Environment

Use Docker for normal FRB development checks that fit Linux containers: Rust tests, Dart tests, web tests, code generation, lint, Android build checks that do not need a running emulator, and most `./frb_internal` commands. Docker is the default reproducible baseline when local host toolchains are missing or suspected to have drifted.

Use a host Android Emulator plus a host ADB server for local Android runtime validation when a running Android device is required. Keep FRB, Flutter, Rust, Gradle, and Android build tooling inside Docker; the host should only provide the emulator runtime and the ADB server that manages it. See the Android section below.

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

Typical usage:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker info
.claude/skills/frb-dev-env/frb_dev_env.py docker create
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc './frb_internal --help'
```

### Cleanup

Delete a worktree's Docker container when the worktree is no longer needed, or when local Docker resources are getting crowded:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker delete
```

The delete command validates the container labels before removing it. Use `--force` only when intentionally removing a mismatched container.

Use the project `frb-docker` skill for image, devcontainer, and Dockerfile details.

## Android Host Emulator

Use this workflow for local Android runtime checks when Docker can build and run the FRB/Flutter command, but the Android Emulator itself must run on the host for hardware virtualization support. The recommended split is:

```text
host:
  Android Emulator
  ADB server bound to a host TCP port

Docker container:
  FRB checkout
  Flutter, Dart, Rust, Cargo, Java, Gradle
  Android SDK/NDK/build tools needed for builds
  ADB client connected to the host ADB server
```

The host does not need FRB, Flutter, Rust, or Gradle for this workflow. It only needs enough Android SDK components to create and boot the emulator, plus `platform-tools/adb` to run the host ADB server.

Prefer an isolated host Android root instead of the default global Android locations:

```bash
export FRB_ANDROID_ROOT="$HOME/main/artifacts/flutter_rust_bridge/android-local"
export ANDROID_HOME="$FRB_ANDROID_ROOT/sdk"
export ANDROID_AVD_HOME="$FRB_ANDROID_ROOT/avd"
export ANDROID_USER_HOME="$FRB_ANDROID_ROOT/user-home"
export PATH="$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools:$PATH"
```

Start the emulator on the host. Pin the port when you want a stable serial such as `emulator-5554`:

```bash
emulator -avd FRB_API_35 -port 5554
```

Start an ADB server on the host that Docker can reach. Keep this as a foreground process so it is easy to stop after validation:

```bash
adb kill-server
adb -a -L tcp:0.0.0.0:5037 server nodaemon
```

From inside the FRB Docker container, verify that the Docker-side ADB client can see the host-managed emulator:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-host-adb -- adb devices
```

For FRB commands, point ADB clients at the host ADB server and pass the emulator serial explicitly:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-host-adb -- \
  bash -lc './frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id emulator-5554"'
```

`docker exec --android-host-adb` injects `ADB_SERVER_SOCKET=tcp:host.docker.internal:5037` plus `ANDROID_ADB_SERVER_ADDRESS` and `ANDROID_ADB_SERVER_PORT` into the command environment. Override the default host and port with `--android-adb-server-host`, `--android-adb-server-port`, `FRB_ANDROID_ADB_SERVER_HOST`, or `FRB_ANDROID_ADB_SERVER_PORT`.

If Flutter does not honor the injected ADB server environment in the current SDK/tooling combination, use an `adb` wrapper earlier in `PATH` inside the container that forwards to the host server:

```bash
exec /path/to/real/adb -H host.docker.internal -P 5037 "$@"
```

Security and reliability notes:

- Exposing ADB on `0.0.0.0:5037` should only be used on trusted local networks or with local firewall restrictions. Stop the foreground ADB server when the Android check is done.
- Use a single emulator when possible. With multiple emulators or devices, always pass `--device-id <serial>` through `--flutter-test-args`.
- Do not run the Android Emulator inside Tart. Do not rely on Docker nested virtualization for Android emulator runtime checks on macOS.
- If the Docker image lacks Android SDK, NDK, or build-tools packages required by Flutter Android builds, update the FRB Docker image or mount a dedicated Android SDK into the container; do not install host Flutter or Gradle just to work around the missing container toolchain.

## Tart

Use Tart for iOS Simulator validation when Docker cannot provide the required macOS/Xcode runtime. The script uses one reusable Tart VM per worktree, named from the canonical worktree root:

```text
frb-tart-<worktree-root-sha256-prefix-12>
```

By default, `create` clones from a prepared local base VM named `frb-tart-base`. Override it with `FRB_TART_BASE_VM` or `--base-vm`.

Read `frb-tart-prepare` before creating or changing the base VM. The base VM should be built by the checked-in Packer template from a pinned Tart OCI artifact and treated as immutable; do not boot it and manually install tools into it.

For linked git worktrees, the Tart helper shares both the canonical worktree root and the git common root into the VM, then mounts them at host-like absolute paths with `mount_virtiofs`. This keeps worktree `.git` files and submodule gitdir references valid inside the VM.

Typical usage:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart info
.claude/skills/frb-dev-env/frb_dev_env.py tart create
.claude/skills/frb-dev-env/frb_dev_env.py tart start
.claude/skills/frb-dev-env/frb_dev_env.py tart ip --wait 180
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- sw_vers
```

For heavy iOS build/test commands, prefer uploading the current worktree to a VM-local copy and running there. This avoids writing Xcode build artifacts through the virtiofs host mount:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart upload
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --flutter-test-args '--device-id <UDID>' --package <package>
```

`tart upload` and `tart exec --sync-code` both first ensure the worktree VM is running and the host-like worktree mount exists. They then run `rsync` inside the VM from the mounted host worktree to a VM-local copy under `/Users/admin/frb-dev-env-local-copies/<worktree-hash>`. The upload excludes `.git`, `.dart_tool`, `build`, `target`, `.idea`, and `.vscode`. It does not delete existing files in the VM-local copy, so build caches survive repeated runs.

Use plain `tart exec` for light commands that should see the mounted host checkout exactly, such as `git status`, `flutter --version`, `xcrun simctl list`, and simulator boot commands. Use `tart exec --sync-code` for heavy build/test commands that create many artifacts, especially iOS Flutter/Xcode tests. Commands run with `--sync-code` start from the VM-local uploaded copy, not from the host-mounted path.

Examples:

```bash
# Show the VM-local path after uploading the current worktree.
.claude/skills/frb-dev-env/frb_dev_env.py tart upload

# Run a light command from the mounted host worktree.
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- git status --short

# Run a heavy command from the uploaded VM-local copy.
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args '--device-id <UDID>'
```

Run iOS Simulator tests by starting the worktree VM, choosing and booting an iOS simulator inside it, then running the existing FRB test command from the VM-local uploaded copy:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart start --wait 300
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl list devices available
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl boot <UDID>
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- xcrun simctl bootstatus <UDID> -b
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args '--device-id <UDID>'
```

For example, when an iOS 18.1 `iPhone 16 Pro Max` simulator with UDID `826DC3E8-2073-42A9-A6DB-05C1926DC82A` is available:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- ./frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args '--device-id 826DC3E8-2073-42A9-A6DB-05C1926DC82A'
```

Delete the worktree VM when it is no longer needed:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py tart stop
.claude/skills/frb-dev-env/frb_dev_env.py tart delete
```

## Project Skills

After applying this environment policy, also read the relevant project-level `frb-*` skills for the actual task, such as code generation, tests, lint, Docker details, CI triage, or PR preparation.
