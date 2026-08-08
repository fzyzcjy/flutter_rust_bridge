# Native Assets Test Hook Overhead

## Purpose

Verify whether Flutter invokes a Native Assets build hook once per `flutter test` invocation or once for every selected test file, including default test discovery, and whether an unchanged warm invocation skips the hook. The procedure records hook invocations and wall time for FRB's wrapper and a vanilla `native_toolchain_rust` fixture.

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

Run all command blocks in the same shell in the per-worktree FRB Docker container. Before starting, set `EVIDENCE_ROOT` to a new, run-specific absolute path outside the repository, such as `/frb-manual-test-evidence/issue-3296-20260710T120000Z`. Use disposable fixtures under `frb_example`; do not commit their marker code, generated test files, or `.dart_tool` output.

## Test Data

- Input files: two independent lightweight test files and a temporary hook marker that appends its timestamp and process ID to `hook-invocations.log`.
- Native Assets hook-cache cold cases: one explicitly selected file, two explicitly selected files, and default discovery with no file arguments. Rust compilation caches are not reset between cases, so wall times must not be compared across fixtures as a Rust build benchmark.
- Warm case: immediately repeat the one-file command without deleting `.dart_tool`, `build`, or `hook-invocations.log`.
- Reset procedure before each cold case: remove the fixture's `.dart_tool`, `build`, and `hook-invocations.log` files.

## Steps

1. Create a disposable FRB fixture and record tool versions.

   ```bash
   set -euo pipefail
   repo_root="$(git rev-parse --show-toplevel)"
   evidence_root_input="${EVIDENCE_ROOT:?Set EVIDENCE_ROOT to a new absolute run-specific path}"
   case "$evidence_root_input" in /*) ;; *) echo 'EVIDENCE_ROOT must be absolute' >&2; exit 1;; esac
   evidence_root="$(realpath -m -- "$evidence_root_input")"
   case "$evidence_root" in "$repo_root"|"$repo_root"/*) echo 'EVIDENCE_ROOT must be outside the repository' >&2; exit 1;; esac
   test ! -e "$evidence_root"
   mkdir -p "$evidence_root/frb" "$evidence_root/vanilla"
   cd "$repo_root"
   rm -rf frb_example/native_assets_hook_overhead_fixture
   cp -a frb_example/flutter_package_native_assets frb_example/native_assets_hook_overhead_fixture
   rm -rf frb_example/native_assets_hook_overhead_fixture/rust/target
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
     File('hook-invocations.log').writeAsStringSync(
       '${DateTime.now().toIso8601String()} $pid\n',
       mode: FileMode.append,
     );
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

3. Run the three cold FRB cases, then repeat the one-file command without clearing the cache. Preserve each terminal log, marker file, marker count, and wall time.

   ```bash
   set -euo pipefail
   evidence_dir="$evidence_root/frb"
   TIMEFORMAT='real %3R s user %3U s sys %3S s'

   run_cold_case() {
     case_name="$1"
     shift
     rm -rf .dart_tool build hook-invocations.log
     flutter pub get
     { time flutter test "$@"; } 2>&1 | tee "$evidence_dir/$case_name.log"
     test -f hook-invocations.log
     cp hook-invocations.log "$evidence_dir/$case_name-hook-invocations.log"
     wc -l hook-invocations.log | tee "$evidence_dir/$case_name-hook-count.log"
   }

   run_cold_case single-file test/a_test.dart
   before_count="$(wc -l < hook-invocations.log)"
   { time flutter test test/a_test.dart; } 2>&1 | tee "$evidence_dir/warm-repeat.log"
   after_count="$(wc -l < hook-invocations.log)"
   printf 'before=%s after=%s delta=%s\n' \
     "$before_count" "$after_count" "$((after_count - before_count))" \
     | tee "$evidence_dir/warm-repeat-hook-count.log"
   cp hook-invocations.log "$evidence_dir/warm-repeat-hook-invocations.log"

   run_cold_case explicit-two-files test/a_test.dart test/b_test.dart
   run_cold_case default-discovery
   ```

4. Run the equivalent vanilla comparison at the pinned upstream revision. Its Flutter fixture is `examples/flutter`; default discovery also includes the existing `widget_test.dart`.

   ```bash
   cd "$repo_root"
   rm -rf frb_example/native_toolchain_rust_hook_overhead_fixture
   git clone https://github.com/GregoryConrad/native_toolchain_rust.git \
     frb_example/native_toolchain_rust_hook_overhead_fixture
   git -C frb_example/native_toolchain_rust_hook_overhead_fixture \
     checkout aeda048b2581317cad0051cf1e061ba6327a1c67
   cd frb_example/native_toolchain_rust_hook_overhead_fixture/examples/flutter
   cat > hook/build.dart <<'EOF'
   import 'dart:io';

   import 'package:hooks/hooks.dart';
   import 'package:native_toolchain_rust/native_toolchain_rust.dart';

   void main(List<String> args) async {
     File('hook-invocations.log').writeAsStringSync(
       '${DateTime.now().toIso8601String()} $pid\n',
       mode: FileMode.append,
     );
     await build(args, (input, output) async {
       await const RustBuilder(assetName: 'src/ffi.g.dart').run(
         input: input,
         output: output,
       );
     });
   }
   EOF
   mkdir -p test
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('a', () {});\n" > test/a_test.dart
   printf "import 'package:flutter_test/flutter_test.dart'; void main() => test('b', () {});\n" > test/b_test.dart
   set -euo pipefail
   evidence_dir="$evidence_root/vanilla"
   TIMEFORMAT='real %3R s user %3U s sys %3S s'

   run_cold_case() {
     case_name="$1"
     shift
     rm -rf .dart_tool build hook-invocations.log
     flutter pub get
     { time flutter test "$@"; } 2>&1 | tee "$evidence_dir/$case_name.log"
     test -f hook-invocations.log
     cp hook-invocations.log "$evidence_dir/$case_name-hook-invocations.log"
     wc -l hook-invocations.log | tee "$evidence_dir/$case_name-hook-count.log"
   }

   run_cold_case single-file test/a_test.dart
   before_count="$(wc -l < hook-invocations.log)"
   { time flutter test test/a_test.dart; } 2>&1 | tee "$evidence_dir/warm-repeat.log"
   after_count="$(wc -l < hook-invocations.log)"
   printf 'before=%s after=%s delta=%s\n' \
     "$before_count" "$after_count" "$((after_count - before_count))" \
     | tee "$evidence_dir/warm-repeat-hook-count.log"
   cp hook-invocations.log "$evidence_dir/warm-repeat-hook-invocations.log"

   run_cold_case explicit-two-files test/a_test.dart test/b_test.dart
   run_cold_case default-discovery
   ```

## Expected Result

All commands exit `0` and all selected tests pass. Compare the raw marker counts from the three cold cases: a default-discovery or explicit-two-files count that grows with the number of suites is evidence consistent with per-file hook execution; identical counts show that the invocation did not add observable hook processes for additional suites. For the warm case, a zero marker delta shows that the hook runner reused its cached result, while a nonzero delta shows that it invoked the hook again. Report counts and wall times without inferring a root cause from timing alone, and compare FRB with vanilla.

## Failure Criteria

The test fails or is blocked if any of the following happens:

- `flutter test` exits non-zero unexpectedly.
- `hook-invocations.log` is absent, so the build-hook invocation cannot be observed.
- The vanilla fixture cannot be prepared; mark that comparison as blocked rather than inferring a result.
- Any cold case is not captured from clean state.
- The warm case clears `.dart_tool`, `build`, or `hook-invocations.log`, making its cache result invalid.

## Results To Capture

- Full terminal logs for the FRB and vanilla runs.
- Each marker file and count from the three cold cases and warm repeat.
- Wall time for every `flutter test` command.
- Flutter, Dart, and Rust version output.
- The exact vanilla upstream commit.
- Copy the complete `EVIDENCE_ROOT` directory from the container to a new durable artifact directory before the final evidence cleanup, then verify the copied file list and marker counts.

## Troubleshooting

- If Flutter dependencies are missing, run `flutter pub get` and record the exact error before retrying.
- If the fixture has stale Native Assets output, delete its `.dart_tool` directory and retry once.
- If cloning the public upstream fixture fails, record the failure and mark the vanilla comparison blocked.

## Cleanup

```bash
cd "$repo_root"
rm -rf \
  frb_example/native_assets_hook_overhead_fixture \
  frb_example/native_toolchain_rust_hook_overhead_fixture
```

No repository files, simulators, or external account state should remain changed. After copying the evidence to durable storage and verifying the copy, remove the container-local staging directory:

```bash
rm -rf "$evidence_root"
```

## Future Automation

Once Flutter exposes hook-runner invocation accounting in a stable test harness, replace this report with an automated integration test that asserts the selected test-file count does not multiply hook work.
