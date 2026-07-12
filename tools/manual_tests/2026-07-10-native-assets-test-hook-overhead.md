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

2. Add the following `flutter_test` SDK dependency to `pubspec.yaml`, append an empty `[workspace]` table to `rust/Cargo.toml`, then replace `hook/build.dart` temporarily with the following equivalent wrapper.

   ```yaml
   dev_dependencies:
     flutter_test:
       sdk: flutter
   ```

   ```toml
   [workspace]
   ```

   ```bash
   cat > hook/build.dart <<'EOF'
   import 'dart:io';
   import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';

   void main(List<String> args) async {
     File('hook-invocations.log').writeAsStringSync('$pid\n', mode: FileMode.append);
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
   ```

3. Run a clean one-file control and a clean two-file comparison. Preserve each marker file and its count.

   ```bash
   for test_args in 'test/a_test.dart' 'test/a_test.dart test/b_test.dart'; do
     rm -rf .dart_tool hook-invocations.log
     flutter pub get
     flutter test $test_args | tee "flutter-test-$(echo "$test_args" | tr ' /' '__').log"
     test -f hook-invocations.log
     wc -l hook-invocations.log
     cp hook-invocations.log "hook-invocations-$(echo "$test_args" | tr ' /' '__').log"
   done
   ```

4. Run the equivalent vanilla comparison at the pinned upstream revision. Its Flutter fixture is `examples/flutter`.

   ```bash
   rm -rf /tmp/native_toolchain_rust-hook-overhead
   git clone https://github.com/GregoryConrad/native_toolchain_rust.git /tmp/native_toolchain_rust-hook-overhead
   git -C /tmp/native_toolchain_rust-hook-overhead checkout aeda048b2581317cad0051cf1e061ba6327a1c67
   cd /tmp/native_toolchain_rust-hook-overhead/examples/flutter
   sed -i "1i import 'dart:io';" hook/build.dart
   sed -i "/void main(List<String> args) async {/a\\  File('hook-invocations.log').writeAsStringSync('\$pid\\n', mode: FileMode.append);" hook/build.dart
   mkdir -p test
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('a', () {});\n" > test/a_test.dart
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('b', () {});\n" > test/b_test.dart
   for test_args in 'test/a_test.dart' 'test/a_test.dart test/b_test.dart'; do
     rm -rf .dart_tool hook-invocations.log
     flutter pub get
     flutter test $test_args | tee "flutter-test-$(echo "$test_args" | tr ' /' '__').log"
     test -f hook-invocations.log
     wc -l hook-invocations.log
     cp hook-invocations.log "hook-invocations-$(echo "$test_args" | tr ' /' '__').log"
   done
   ```

## Expected Result

All commands exit `0` and both tests pass. Compare the raw marker counts from clean one-file and two-file runs: an increase matching the added test file is evidence consistent with per-file hook execution; identical counts only show that this invocation did not add an observable hook process. Report the counts without claiming a root cause, and compare the FRB and vanilla deltas.

## Failure Criteria

The test fails or is blocked if any of the following happens:

- `flutter test` exits non-zero unexpectedly.
- `hook-invocations.log` is absent, so the build-hook invocation cannot be observed.
- The vanilla fixture cannot be prepared; mark that comparison as blocked rather than inferring a result.
- The one-file and two-file runs are not both captured from clean state.

## Results To Capture

- Full terminal logs for the FRB and vanilla runs.
- Each `hook-invocations-*.log` marker file from the clean one-file and two-file runs.
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
