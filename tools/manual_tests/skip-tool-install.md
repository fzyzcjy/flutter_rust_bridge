# Skip Tool Install

## Purpose

Verify that `--skip-tool-install` stops `flutter_rust_bridge_codegen` from running `cargo install` for missing tools: `generate` must fail with an install hint instead of installing `cargo-expand`, and `build-web` must fail with an install hint instead of installing `wasm-pack`.

## Source

- Context: [#3468](https://github.com/fzyzcjy/flutter_rust_bridge/issues/3468) (opt out of automatic tool installation), [#3476](https://github.com/fzyzcjy/flutter_rust_bridge/pull/3476)
- Related docs or skills: `website/docs/quickstart.md`, `frb_codegen/src/library/commands/cargo_expand/real.rs`, `frb_dart/lib/src/cli/build_web/executor.dart`

## When To Run

Run this after changing tool detection or auto-install logic for `cargo-expand`, `wasm-pack`, or `wasm-bindgen-cli`, after changing how `build-web` forwards arguments from the Rust CLI to the Dart CLI, or before a release that touches these paths. Automated tests cover flag parsing, but not the behavior with the tools actually missing.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized.
- Required credentials or account state: none.
- Required device or simulator state: none.
- The executor may temporarily move `cargo-expand`, `wasm-pack`, and `wasm-bindgen` out of `PATH`. Run every step in the same repository root, and always run Cleanup afterwards.

## Environment

- OS: macOS or Linux with a POSIX shell. On Windows, use WSL or adapt the `mv` and `command -v` commands.
- Flutter: not required.
- Dart: record `dart --version`.
- Rust: record `rustc --version` and `cargo --version`.
- Device or simulator: not required.
- Browser or external service: not required. No network access is needed after Preparation.

## Preparation

```bash
git submodule update --init --recursive
cargo build --manifest-path frb_codegen/Cargo.toml
(cd frb_example/dart_minimal && dart pub get)
```

Run all commands from the repository root.

## Test Data

- Input files, API examples, account fixtures, or generated assets: `frb_example/dart_minimal`, unchanged.
- Reset procedure before each run: run Cleanup, then confirm `git status --short` prints nothing.

## Steps

1. Record which tools are installed, then move them into a backup folder so they cannot be found. The backup folder is under the git-ignored `target/` directory.

   ```bash
   mkdir -p target/manual_test_skip_tool_install
   for tool in cargo-expand wasm-pack wasm-bindgen; do
     tool_path="$(command -v "$tool" || true)"
     if [ -n "$tool_path" ]; then
       echo "$tool_path" >> target/manual_test_skip_tool_install/paths.txt
       mv "$tool_path" target/manual_test_skip_tool_install/
     fi
   done
   hash -r
   command -v cargo-expand wasm-pack wasm-bindgen || echo "all tools hidden"
   ```

   Expected intermediate state: the last command prints `all tools hidden`. `hash -r` clears the shell's cached command locations so `command -v` sees the moved tools.

2. Run `generate` with the flag.

   ```bash
   (cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- generate --skip-tool-install); echo "exit=$?"
   ```

3. Confirm that `generate` did not install `cargo-expand`.

   ```bash
   hash -r
   command -v cargo-expand || echo "cargo-expand still missing"
   ```

4. Run `build-web` with the flag.

   ```bash
   (cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- build-web --skip-tool-install); echo "exit=$?"
   ```

5. Confirm that `build-web` did not install `wasm-pack`.

   ```bash
   hash -r
   command -v wasm-pack || echo "wasm-pack still missing"
   ```

## Expected Result

The test passes when all of the following are true:

- Step 2 prints `exit=` with a non-zero value and contains this error:

  ```text
  Error: cargo-expand is required but not installed. Install it with `cargo install cargo-expand`, or run without `--skip-tool-install` to let flutter_rust_bridge install it automatically.
  ```

- Step 2 does not contain `Cargo expand is not installed. Automatically install and re-run.`
- Step 3 prints `cargo-expand still missing`.
- Step 4 prints `exit=` with a non-zero value and contains:

  ```text
  wasm-pack is required, but not found in the path.
  ```

- Step 4 does not contain ``Try to install `wasm-pack` ``.
- Step 5 prints `wasm-pack still missing`.

## Failure Criteria

The test fails if any of the following happens:

- Step 2 or step 4 exits with status `0`.
- Step 2 or step 4 prints an auto-install message (`Automatically install and re-run`, ``Try to install `wasm-pack` ``) or runs `cargo install`.
- Step 3 or step 5 prints a path, meaning a tool was installed.
- Step 2 or step 4 fails with a different error, such as a compile error or missing `pubspec.yaml`. Report this as blocked, not as a product failure, and record the full log.

## Results To Capture

- Full terminal log of steps 1 to 5, including every `exit=` line.
- The contents of `target/manual_test_skip_tool_install/paths.txt`, which lists the tools that were hidden.
- Output of `dart --version`, `rustc --version`, and `cargo --version`.
- Final `git status --short` output after Cleanup.

## Troubleshooting

- If step 1 prints a tool path instead of `all tools hidden`, the tool exists in more than one `PATH` directory. Rerun step 1 until `all tools hidden` is printed; each rerun appends the next location to `paths.txt`.
- If step 2 fails with `Could not find Cargo.toml` or similar, check that the command runs inside `frb_example/dart_minimal`.
- If step 4 fails because `dart run` cannot find `flutter_rust_bridge`, rerun `(cd frb_example/dart_minimal && dart pub get)`.
- If steps fail to compile on macOS with linker errors such as `unknown architecture`, the Xcode and Command Line Tools SDK versions may not match; record the error and mark the run as blocked.

## Cleanup

Restore every hidden tool to its original location, then remove the backup folder:

```bash
if [ -f target/manual_test_skip_tool_install/paths.txt ]; then
  while read -r tool_path; do
    mv "target/manual_test_skip_tool_install/$(basename "$tool_path")" "$tool_path"
  done < target/manual_test_skip_tool_install/paths.txt
fi
rm -rf target/manual_test_skip_tool_install
hash -r
command -v cargo-expand wasm-pack wasm-bindgen
git status --short
```

`command -v` should print the original paths of every tool that was installed before step 1. `git status --short` should print nothing.

## Future Automation

This could become a CI job that runs steps 2 to 5 in a container or runner where `cargo-expand` and `wasm-pack` are not installed. It stays manual for now because the existing CI jobs install these tools, and hiding tools on a shared runner would affect other jobs.
