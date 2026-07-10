## Purpose

Verify whether Flutter invokes a Native Assets build hook once per `flutter test` invocation or once for every selected test file. The procedure records hook invocations for FRB's wrapper and a vanilla `native_toolchain_rust` fixture.

## Source

- Context: GitHub issue #3296, <https://github.com/fzyzcjy/flutter_rust_bridge/issues/3296>
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`

## When To Run

Run before diagnosing or closing a Native Assets test-startup regression, and after upgrading Flutter, `hooks`, or `native_toolchain_rust`.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized.
- Required credentials or account state: network access to clone the public vanilla fixture.
- Required device or simulator state: not required.

## Environment

- OS: Linux Docker container supplied by `frb_dev_env.py`.
- Flutter: record `flutter --version`.
- Dart: record `dart --version`.
- Rust: record `rustc --version`.
- Device or simulator: not required.
- Browser or external service: GitHub checkout of `GregoryConrad/native_toolchain_rust`.

## Preparation

Run all commands in the per-worktree FRB Docker container. Use a disposable fixture under `frb_example/native_assets_hook_overhead_fixture`; do not commit its marker code, generated test files, or `.dart_tool` output.

## Test Data

- Input files: two independent lightweight test files and a temporary hook marker that appends its process id to `hook-invocations.log`.
- Reset procedure before each run: remove the fixture's `.dart_tool`, `build`, and `hook-invocations.log` files.

## Steps

1. Create a disposable FRB fixture and record tool versions.

   ```bash
   rm -rf frb_example/native_assets_hook_overhead_fixture
   cp -a frb_example/flutter_package_native_assets frb_example/native_assets_hook_overhead_fixture
   cd frb_example/native_assets_hook_overhead_fixture
   flutter --version
   dart --version
   rustc --version
   ```

2. Add `flutter_test` from the Flutter SDK to `dev_dependencies` in `pubspec.yaml`; add an empty `[workspace]` table to `rust/Cargo.toml` so the temporary crate is isolated from the repository Cargo workspace. Then replace `hook/build.dart` temporarily with the following equivalent wrapper, create two test files, and run them in one `flutter test` invocation.

   ```bash
   cat > hook/build.dart <<'EOF'
   import 'dart:io';
   import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

   void main(List<String> args) async {
     File('hook-invocations.log').writeAsStringSync('\${pid}\n', mode: FileMode.append);
     await build(args, (input, output) async {
       await const FlutterRustBridgeNativeAssetsBuilder(cratePath: 'rust').run(
         input: input,
         output: output,
       );
     });
   }
   EOF
   mkdir test
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('a', () {});\n" > test/a_test.dart
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('b', () {});\n" > test/b_test.dart
   flutter test test/a_test.dart test/b_test.dart | tee flutter-test.log
   sort -u hook-invocations.log | tee hook-invocations-unique.log
   ```

3. Repeat the same marker-and-two-tests procedure in a vanilla `native_toolchain_rust` Flutter example at its recorded Git revision. Preserve the full command log and invocation marker file.

## Expected Result

`flutter test` exits `0`, both test names pass, and the marker has a count that can be compared with the number of selected files. Two or more distinct marker entries for the two-file run demonstrate per-file hook execution; one entry demonstrates per-invocation execution.

## Failure Criteria

The test fails or is blocked if any of the following happens:

- `flutter test` exits non-zero unexpectedly.
- `hook-invocations.log` is absent, so the build-hook invocation cannot be observed.
- The vanilla fixture cannot be prepared; mark that comparison as blocked rather than inferring a result.

## Results To Capture

- Full terminal logs for the FRB and vanilla runs.
- `hook-invocations.log` and `hook-invocations-unique.log` from each fixture.
- Flutter, Dart, and Rust version output.
- The exact vanilla upstream commit.

## Troubleshooting

- If Flutter dependencies are missing, run `flutter pub get` and record the exact error before retrying.
- If the fixture has stale Native Assets output, delete its `.dart_tool` directory and retry once.
- If cloning the public upstream fixture fails, record the failure and mark the vanilla comparison blocked.

## Cleanup

```bash
   rm -rf frb_example/native_assets_hook_overhead_fixture /tmp/native_toolchain_rust-hook-overhead
```

No repository files, simulators, or external account state should remain changed.

## Future Automation

Once Flutter exposes hook-runner invocation accounting in a stable test harness, replace this report with an automated integration test that asserts the selected test-file count does not multiply hook work.
