---
name: frb-dev-env
description: "Use when the user wants Docker-based FRB development or Tart-based iOS Simulator validation."
---

# FRB Development Environment

Use this skill before setting up, diagnosing, or running `flutter_rust_bridge` commands with a container-based development workflow.

## Choosing an Environment

Use Docker for normal FRB development checks that fit Linux containers: Rust tests, Dart tests, web tests, code generation, lint, and most `./frb_internal` commands. Docker is the default reproducible baseline when local host toolchains are missing or suspected to have drifted.

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

`tart upload` and `tart exec --sync-code` both first ensure the worktree VM is running and the host-like worktree mount exists. They then run `rsync` inside the VM from the mounted host worktree to a VM-local copy under `/Users/admin/frb-dev-env-local-copies/<worktree-hash>`. The upload excludes `.git`, `.dart_tool`, `build`, `.idea`, and `.vscode`, and uses rsync delete semantics so stale generated files from previous uploads do not survive.

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
