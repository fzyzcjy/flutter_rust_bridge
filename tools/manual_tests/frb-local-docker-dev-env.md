# Local Docker Development Environment Coverage

## Purpose

Verify that the per-worktree FRB Docker development container can run the normal headless local validation surface: Rust tests, Dart native tests, Dart web tests, and the internal development entrypoint. This test also records that Docker is not the Android emulator, iOS Simulator, or macOS GUI runtime strategy.

## Source

- Context: Maintenance check for the local FRB Docker development workflow.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-docker/SKILL.md`, `.claude/skills/frb-test/SKILL.md`

## When To Run

Run this after changing the FRB Docker image, devcontainer setup, per-worktree Docker helper, Rust/Dart/web test tooling, or Flutter/Rust/Dart versions. It is also useful before relying on a new local machine or worktree for headless FRB development.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Docker must be able to pull or use the configured FRB development image. Docker Hub credentials are required only if the local setup needs authenticated pulls.
- Required device or simulator state: none. This test does not require Android devices, Android emulators, iOS simulators, or desktop GUI sessions.

## Environment

- OS: host OS capable of running Docker and repository shell scripts.
- Flutter: record `flutter --version` inside the Docker container.
- Dart: record `dart --version` inside the Docker container.
- Rust: record `rustc --version` and `cargo --version` inside the Docker container.
- Device or simulator: not required.
- Browser or external service: record the Chrome or Chromium version used by the container for headless web tests.

## Preparation

Run preparation commands from the repository root of the checkout being tested.

```bash
git rev-parse --show-toplevel
git status --short
git submodule update --init --recursive
docker version
.claude/skills/frb-dev-env/frb_dev_env.py docker info
.claude/skills/frb-dev-env/frb_dev_env.py docker create
```

Confirm the container can run commands at the host-like worktree path.

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'pwd && ./frb_internal --help'
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in packages `frb_rust`, `frb_dart`, and `frb_example/pure_dart`.
- Reset procedure before each run: return to a clean checkout or record intentional local changes in the execution record.

## Steps

1. Record container tool versions and browser availability.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc '
   set -euo pipefail
   flutter --version
   dart --version
   rustc --version
   cargo --version
   "${CHROME_BIN:-google-chrome}" --version || chromium --version || chromium-browser --version
   '
   ```

2. Run a focused Rust package test in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-rust-package --package frb_rust
   ```

3. Run a focused Dart native test in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-dart-native --package frb_dart
   ```

4. Run a focused Dart web test in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-dart-web --package frb_example/pure_dart
   ```

5. Confirm the checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

## Expected Result

The Docker environment coverage test passes when every command exits successfully and the terminal log shows that the commands ran inside the per-worktree container. The web step must run through a headless browser without requiring a visible GUI session.

```text
./frb_internal test-rust-package --package frb_rust
./frb_internal test-dart-native --package frb_dart
./frb_internal test-dart-web --package frb_example/pure_dart
```

## Failure Criteria

The test fails if any of the following happens:

- Docker is unavailable, the per-worktree container cannot be created, or the container labels do not match the current worktree.
- Any required tool version command fails inside the container.
- The Rust, Dart native, or Dart web test command exits non-zero unexpectedly.
- The Dart web test requires a visible GUI session instead of running in a headless browser.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if the Docker image cannot be pulled because of network or registry access.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host OS, Docker version, container image, and container name from `docker info`.
- Flutter, Dart, Rust, Cargo, and browser versions inside the container.
- Final `git status --short` output.

## Troubleshooting

- If submodules are uninitialized, rerun `git submodule update --init --recursive` and record the output.
- If the container is missing or stopped, rerun `.claude/skills/frb-dev-env/frb_dev_env.py docker create` and record the helper output.
- If Chrome or Chromium cannot start, record the browser command, browser version, and whether the web test log mentions sandbox flags.
- If dependency downloads fail, record the failed URL or package source without adding secrets.
- If a generated file changes, record the exact `git status --short` output and inspect whether the tested command intentionally regenerated it.

## Cleanup

The per-worktree Docker container may be kept for future local FRB development. Delete it only when the worktree no longer needs the cached environment.

```bash
git status --short
.claude/skills/frb-dev-env/frb_dev_env.py docker info
```

If cleanup requires deleting the container, run:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker delete
```

## Future Automation

Most commands in this report are already suitable for automation. Keep this manual report as a local environment acceptance check for machines, worktrees, and Docker helper changes that are outside the normal CI runner lifecycle.
