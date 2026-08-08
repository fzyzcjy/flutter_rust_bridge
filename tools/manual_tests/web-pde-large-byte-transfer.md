# Web PDE Large Byte Transfer

## Purpose

Compare the default pure-Dart-extension (PDE) Web codec path with the full-dependency Web codec path for a large `Uint8List` passed to a Rust `Vec<u8>`. The test records the generated codec selection and end-to-end browser latency for the same 64 MiB payload.

## Source

- Context: [GitHub issue #3326](https://github.com/fzyzcjy/flutter_rust_bridge/issues/3326)
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`, `website/docs/guides/miscellaneous/codec.md`

## When To Run

Run this test when changing PDE dispatch, SSE, CST/DCO Web bindings, `Uint8List` or primitive-list conversion, Wasm bindings, or the `full_dep` configuration behavior.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout containing `frb_example/dart_minimal/tool/web_large_byte_transfer.dart` and initialized dependencies.
- Required credentials or account state: Docker can pull or use the configured FRB development image.
- Required device or simulator state: none.

## Environment

- OS: Docker-capable macOS or Linux host.
- Flutter: record `flutter --version` inside the FRB Docker container.
- Dart: record `dart --version` inside the FRB Docker container.
- Rust: record `rustc --version` inside the FRB Docker container.
- Device or simulator: none.
- Browser or external service: record `google-chrome --version` inside the FRB Docker container.

## Preparation

Run from the repository root:

```bash
git submodule update --init --recursive
.claude/skills/frb-dev-env/frb_dev_env.py docker create
```

Confirm `frb_example/dart_minimal/flutter_rust_bridge.yaml` keeps the default PDE configuration:

```text
#full_dep: true
```

Generate the PDE bindings:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- generate'
```

## Test Data

- Input fixture: `frb_example/dart_minimal/tool/web_large_byte_transfer.dart` allocates a 64 MiB `Uint8List`.
- Rust fixture: `process_large_bytes` accepts `Vec<u8>` and returns the observed length as `i32`.
- Reset procedure before each run: regenerate bindings after changing `full_dep`, and ensure no previous `test-web` process is listening on port 8080.

## Steps

1. Confirm the default generated API uses PDE/SSE for `processLargeBytes`.

   ```bash
   rg -n 'processLargeBytes|sse_encode_list_prim_u_8_loose|codec: SseCodec' frb_example/dart_minimal/lib/src/rust/frb_generated.dart
   ```

2. Run the default PDE/SSE Web measurement with a finite timeout.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && timeout 180 dart run flutter_rust_bridge_utils test-web --entrypoint "$PWD/tool/web_large_byte_transfer.dart"'
   ```

3. Uncomment `full_dep: true` in `frb_example/dart_minimal/flutter_rust_bridge.yaml`, then regenerate bindings.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- generate'
   ```

4. Confirm the full-dependency generated API uses CST/DCO for `processLargeBytes`.

   ```bash
   rg -n 'processLargeBytes|cst_encode_list_prim_u_8_loose|codec: DcoCodec' frb_example/dart_minimal/lib/src/rust/frb_generated.dart
   ```

5. Run the full-dependency CST/DCO Web measurement with the same payload and timeout.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && timeout 180 dart run flutter_rust_bridge_utils test-web --entrypoint "$PWD/tool/web_large_byte_transfer.dart"'
   ```

6. Restore `#full_dep: true`, regenerate PDE bindings, and inspect the checkout.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- generate'
   git status --short
   ```

## Expected Result

Both browser runs must validate all 67,108,864 bytes and exit successfully. Each run prints one result line:

```text
WEB_LARGE_BYTE_TRANSFER bytes=67108864 elapsed_ms=<milliseconds>
```

On the issue reproduction baseline, the default PDE/SSE result is expected to be at least ten times slower than the full-dependency CST/DCO result. The generated code must show SSE for the PDE run and CST/DCO for the full-dependency control.

## Failure Criteria

The test fails if any of the following happens:

- Either browser run returns a byte length other than 67,108,864.
- Either run times out, crashes, or exits non-zero after the environment is prepared.
- Generated codec selection does not match the active `full_dep` setting.
- The PDE/SSE baseline is not measurably slower than the CST/DCO control on a reproduction branch.

Mark the run blocked if Docker or Chrome cannot start, required Wasm dependencies are unavailable, or port 8080 is occupied by a process not owned by the executor.

## Results To Capture

- Complete PDE codegen and browser logs.
- Complete full-dependency codegen and browser logs.
- Both `WEB_LARGE_BYTE_TRANSFER` result lines and their latency ratio.
- Generated-code excerpts proving SSE versus CST/DCO selection.
- Host OS, Docker image digest, Dart, Flutter, Rust, and Chrome versions.
- Final `git status --short` output after restoring PDE generation.

## Troubleshooting

- Run the command from `frb_example/dart_minimal` and pass an absolute entrypoint using `$PWD`; otherwise Dart may resolve the wrong package config.
- If port 8080 is occupied after an interrupted run, identify the exact `test-web` and Chrome parent PIDs inside this worktree's container before terminating only those processes.
- If the fixture always throws a length mismatch, confirm `process_large_bytes` returns `i32`, not `usize`, so Dart compares the result with an `int`.
- If 64 MiB exceeds the browser or runner limit, record the failure and repeat both paths with the same smaller power-of-two size.

## Cleanup

Restore the default PDE configuration and regenerated outputs, then confirm no manual browser process remains:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- ps -eo pid,ppid,etime,args
git status --short
```

Keep the per-worktree Docker container for subsequent FRB development, or delete it through the dev-environment helper when the worktree is retired.

## Future Automation

The correctness portion can become a Web regression test in `frb_example/pure_dart_pde`. Keep the large-payload timing comparison manual because browser allocation limits, shared CI load, and release-versus-dev compilation can make absolute latency thresholds unstable.
