# Build Runner Output Scope

## Purpose

Verify that `flutter_rust_bridge_codegen generate` limits its automatic Dart `build_runner` invocation to outputs under the configured FRB `dart_output` directory. This prevents unrelated builders in a large Dart package from extending FRB code generation time.

## Source

- Context: <https://github.com/fzyzcjy/flutter_rust_bridge/issues/3299>
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-develop-feature/SKILL.md`

## When To Run

- After changing the Dart build runner command, codegen output paths, Freezed generation, or JSON serialization generation.
- Before closing a report that automatic build runner work makes FRB code generation unexpectedly slow.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized.
- Required credentials or account state: none.
- Required device or simulator state: none.
- Required container state: the per-worktree container from `.claude/skills/frb-dev-env/frb_dev_env.py docker create` is running.

## Environment

- OS: Docker development image on a supported host.
- Flutter: version bundled in the FRB Docker development image.
- Dart: version bundled in the FRB Docker development image.
- Rust: version bundled in the FRB Docker development image.
- Device or simulator: not required.
- Browser or external service: not required.

## Preparation

Run from the repository root.

```bash
git status --short
git submodule update --init --recursive
.claude/skills/frb-dev-env/frb_dev_env.py docker create
```

Confirm `git status --short` is empty before adding the temporary reproducer.

Append a complex Rust enum so FRB must invoke Freezed through build runner.

```bash
cat >> frb_example/dart_minimal/rust/src/api/minimal.rs <<'RUST'

pub enum BuildRunnerTrigger {
    Value { value: String },
}
RUST
```

Add an unrelated JSON-serializable Dart model outside the configured `lib/src/rust` FRB output directory.

```bash
cat > frb_example/dart_minimal/lib/unrelated_model.dart <<'DART'
import 'package:json_annotation/json_annotation.dart';

part 'unrelated_model.g.dart';

@JsonSerializable()
class UnrelatedModel {
  const UnrelatedModel({required this.value});

  factory UnrelatedModel.fromJson(Map<String, dynamic> json) =>
      _$UnrelatedModelFromJson(json);

  final String value;

  Map<String, dynamic> toJson() => _$UnrelatedModelToJson(this);
}
DART
```

## Test Data

- FRB input: `frb_example/dart_minimal/rust/src/api/minimal.rs`
- FRB output directory: `frb_example/dart_minimal/lib/src/rust`
- Unrelated builder input: `frb_example/dart_minimal/lib/unrelated_model.dart`
- Reset procedure: run the cleanup commands before each execution, then repeat Preparation.

## Steps

1. Record the toolchain versions.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'dart --version; rustc --version; cargo --version'
   ```

2. Run code generation and capture its full output and reported `Run Dart build_runner` duration.

   ```bash
   .claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd frb_example/dart_minimal && cargo run --manifest-path ../../frb_codegen/Cargo.toml -- generate'
   ```

3. Confirm that build runner produced the required Freezed output inside the configured FRB output directory.

   ```bash
   test -f frb_example/dart_minimal/lib/src/rust/api/minimal.freezed.dart
   ```

4. Confirm that FRB did not ask build runner to generate the unrelated output outside the configured FRB output directory.

   ```bash
   test ! -f frb_example/dart_minimal/lib/unrelated_model.g.dart
   ```

5. Record every changed or untracked path before cleanup.

   ```bash
   git status --short
   ```

## Expected Result

- Code generation exits successfully.
- `lib/src/rust/api/minimal.freezed.dart` exists, proving the required Freezed builder ran for FRB output.
- `lib/unrelated_model.g.dart` does not exist, proving unrelated package builders were excluded.
- The codegen timing report contains a finite `Run Dart build_runner` duration for comparison with earlier runs.

```text
Run Dart build_runner
```

## Failure Criteria

The test fails if any of the following happens:

- Code generation exits non-zero unexpectedly.
- `lib/src/rust/api/minimal.freezed.dart` is missing.
- `lib/unrelated_model.g.dart` exists after code generation.
- Build runner writes generated outputs outside the configured `lib/src/rust` directory for the temporary reproducer.

The test is blocked if the FRB Docker development image cannot be pulled or started.

## Results To Capture

- Full terminal log from code generation, including the `Run Dart build_runner` timing.
- Dart, Rust, and Cargo versions from the container.
- Exit status of both generated-file assertions.
- `git status --short` before cleanup.
- Final `git status --short` after cleanup.

## Troubleshooting

- If submodules are uninitialized, rerun `git submodule update --init --recursive` and record its output.
- If the container is missing, rerun `.claude/skills/frb-dev-env/frb_dev_env.py docker create` and record its output.
- If code generation cannot find Dart packages, run the repository's normal dependency setup in the Docker container and record the exact failure.
- If the required Freezed output is missing, inspect the generated `minimal.dart` for `part 'minimal.freezed.dart';` and `@freezed` before classifying the scope assertion.

## Cleanup

The test requires a clean checkout before Preparation, so restore only the temporary package and remove the three known untracked reproducer outputs.

```bash
git restore -- frb_example/dart_minimal
git clean -f -- frb_example/dart_minimal/lib/unrelated_model.dart frb_example/dart_minimal/lib/unrelated_model.g.dart frb_example/dart_minimal/lib/src/rust/api/minimal.freezed.dart
git status --short
```

The final status may contain the manual test report itself when the report is being added on the current branch; no `frb_example/dart_minimal` path may remain changed or untracked.

## Future Automation

Automate this in a focused codegen integration test that installs an unrelated builder input outside `dart_output`, runs codegen, and asserts that only FRB-owned Freezed and JSON serialization outputs are built.
