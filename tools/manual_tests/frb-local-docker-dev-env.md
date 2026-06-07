# Local Docker Development Environment Coverage

## Purpose

Verify that the per-worktree FRB Docker development container can run the normal headless local validation surface: Rust tests, Dart native tests, Dart web tests, Flutter web tests, Linux Flutter native tests, Linux Flutter release builds, and the internal development entrypoint. This test also records that Docker is not the Android emulator, iOS Simulator, or macOS GUI runtime strategy.

## Source

- Context: Maintenance check for the local FRB Docker development workflow.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `.claude/skills/frb-docker/SKILL.md`, `.claude/skills/frb-test/SKILL.md`

## When To Run

Run this after changing the FRB Docker image, devcontainer setup, per-worktree Docker helper, Rust/Dart/web/Linux Flutter test tooling, Linux Flutter build tooling, or Flutter/Rust/Dart versions. It is also useful before relying on a new local machine or worktree for headless FRB development.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized. Intentional local changes are allowed only if the execution record lists them.
- Required credentials or account state: Docker must be able to pull or use the configured FRB development image. Docker Hub credentials are required only if the local setup needs authenticated pulls.
- Required device or simulator state: none. This test does not require Android devices, Android emulators, iOS simulators, or visible desktop GUI sessions.
- Required browser driver state: Flutter web drive coverage requires a compatible `chromedriver` on `PATH` and a headless WebDriver setup for the container architecture. The Docker image must provide this for both amd64 and arm64 local containers.
- Required Rust web build state: Flutter web drive coverage requires the Docker image to provide `rust-src` and `llvm-tools-preview` on the pinned nightly Rust toolchain because `build-web --dart-coverage` builds the standard library with `-Z build-std` under `cargo llvm-cov`.
- Required Linux desktop test state: Linux Flutter native coverage requires the container to provide Flutter Linux desktop build dependencies and a headless display runner such as `xvfb-run`.

## Environment

- OS: host OS capable of running Docker and repository shell scripts.
- Flutter: record `flutter --version` inside the Docker container.
- Dart: record `dart --version` inside the Docker container.
- Rust: record `rustc --version` and `cargo --version` inside the Docker container.
- Device or simulator: the Linux Flutter native test uses Flutter's `linux` device inside the container.
- Browser or external service: record the Chrome or Chromium version and ChromeDriver version used by the container for headless web tests.

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

Verify the Docker image already contains the pinned nightly Rust toolchain components needed by the Flutter web coverage step.

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc '
set -euo pipefail
nightly_version="$(awk -F= '"'"'/^ARG RUST_NIGHTLY_VERSION=/{print $2; exit}'"'"' .devcontainer/Dockerfile)"
nightly_toolchain="nightly-${nightly_version}"
rustup toolchain list | grep "${nightly_toolchain}"
rustup component list --toolchain "${nightly_toolchain}" --installed | grep rust-src
rustup component list --toolchain "${nightly_toolchain}" --installed | grep llvm-tools
'
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: checked-in packages `frb_rust`, `frb_dart`, `frb_example/pure_dart`, and `frb_example/flutter_via_create`.
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
   uname -a
   cmake --version
   ninja --version
   pkg-config --version
   xvfb-run --help >/dev/null
   "${CHROME_BIN:-google-chrome}" --version || chromium --version || chromium-browser --version
   chromedriver --version
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

4. Run a focused Dart web JavaScript test in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-dart-web --package frb_example/pure_dart
   ```

5. Run the Dart web dart2wasm browser test command in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-dart-web --package frb_dart --wasm
   ```

6. Start the headless WebDriver service used by Flutter web drive commands.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc '
   set -euo pipefail
   export DISPLAY=:99
   if ! curl -fsS http://127.0.0.1:4444/status >/tmp/frb-local-webdriver-status.log 2>&1; then
     chromedriver --port=4444 --verbose >/tmp/frb-local-chromedriver.log 2>&1 &
   fi
   if ! xdpyinfo -display :99 >/tmp/frb-local-xdpyinfo.log 2>&1; then
     Xvfb -ac :99 -screen 0 1280x1024x24 >/tmp/frb-local-xvfb.log 2>&1 &
   fi
   sleep 15
   curl -fsS http://127.0.0.1:4444/status
   '
   ```

7. Run the Flutter web JavaScript coverage command in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-flutter-web --package frb_example/flutter_via_create --coverage
   ```

8. Run the Flutter web dart2wasm coverage command in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal test-flutter-web --package frb_example/flutter_via_create --coverage --wasm
   ```

9. Run a focused Linux Flutter native integration test in Docker through a headless display.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- xvfb-run -a ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id linux'
   ```

10. Smoke-test the Linux Flutter release build command in Docker.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ./frb_internal build-flutter --target linux
   ```

11. Confirm the checkout did not gain unexpected generated or cache files.

   ```bash
   git status --short
   ```

## Expected Result

The Docker environment coverage test passes when every command exits successfully and the terminal log shows that the commands ran inside the per-worktree container. The web step must run through a headless browser without requiring a visible GUI session. The Linux Flutter native step must run against Flutter's `linux` device through a headless display, and the Linux build step must copy release artifacts into `target/build_flutter_output`.

```text
./frb_internal test-rust-package --package frb_rust
./frb_internal test-dart-native --package frb_dart
./frb_internal test-dart-web --package frb_example/pure_dart
./frb_internal test-dart-web --package frb_dart --wasm
./frb_internal test-flutter-web --package frb_example/flutter_via_create --coverage
./frb_internal test-flutter-web --package frb_example/flutter_via_create --coverage --wasm
xvfb-run -a ./frb_internal test-flutter-native --package frb_example/flutter_via_create --flutter-test-args '--device-id linux'
./frb_internal build-flutter --target linux
```

## Failure Criteria

The test fails if any of the following happens:

- Docker is unavailable, the per-worktree container cannot be created, or the container labels do not match the current worktree.
- Any required tool version command fails inside the container.
- The Rust, Dart native, Dart web, Flutter web, or Linux Flutter native test command exits non-zero because Docker, browser startup, Linux desktop startup, package setup, toolchain availability, or local environment plumbing failed.
- The Dart or Flutter web test requires a visible GUI session instead of running in a headless browser.
- The dart2wasm web commands fail before launching Chrome or before reaching the actual test body. A deterministic test assertion failure or timeout after Chrome starts is product/test behavior to investigate separately, not a Docker environment failure; record the exact failing test and command.
- The Flutter web drive command fails with `Unable to start a WebDriver session` because no compatible `chromedriver` is available.
- The Linux Flutter native test cannot find the `linux` device, cannot start under `xvfb-run`, or exits non-zero unexpectedly.
- The Linux Flutter build command exits non-zero unexpectedly or does not produce release artifacts under `target/build_flutter_output`.
- `git status --short` shows unexpected local changes after the run.

The test is blocked, not failed, if the Docker image cannot be pulled because of network or registry access.

## Results To Capture

- Full terminal log for all preparation and test commands.
- Host OS, Docker version, container image, and container name from `docker info`.
- Flutter, Dart, Rust, Cargo, Linux kernel, CMake, Ninja, pkg-config, browser, and ChromeDriver versions inside the container.
- Linux Flutter native test success lines and Linux build output artifact paths.
- Final `git status --short` output.

## Troubleshooting

- If submodules are uninitialized, rerun `git submodule update --init --recursive` and record the output.
- If the container is missing or stopped, rerun `.claude/skills/frb-dev-env/frb_dev_env.py docker create` and record the helper output.
- If Chrome, Chromium, or ChromeDriver cannot start, record the command, version, container architecture, and whether the web test log mentions sandbox flags. For `dart test -p chrome`, verify that `CHROME_EXECUTABLE` points to the Docker no-sandbox wrapper when running inside the FRB Docker container.
- If `flutter drive` reaches `dart2wasm` successfully and then fails to start a WebDriver session, record the container architecture, whether `chromedriver --version` succeeds, and whether the run is using `web-server` or `chrome`.
- If Flutter web coverage fails only on the dart2wasm command, record whether the failure happened during FRB codegen, `flutter drive --wasm`, coverage formatting, or artifact generation.
- If the Flutter web coverage step fails with a missing `Cargo.lock` under the nightly standard library source, verify the container is using the current Docker image and that the image has `rust-src` installed for the pinned nightly toolchain.
- If the Flutter web coverage step fails with `can't find crate for profiler_builtins`, verify the container is using the current Docker image and that the image has `llvm-tools-preview` installed for the pinned nightly toolchain.
- If the Linux Flutter native test cannot start, record `flutter devices`, `xvfb-run --help`, and whether the log mentions GTK, display, OpenGL, or missing Linux desktop dependencies.
- If the Linux Flutter build fails, record `flutter doctor -v`, `cmake --version`, `ninja --version`, and `pkg-config --version` from inside the container.
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
