# Sanitizer and Valgrind CI Maintenance

## Purpose

Keep the Dart sanitizer and Valgrind CI lanes reproducible when the project Dart version, sanitizer artifacts, or CI matrix changes.

Run this checklist after changing `.github/workflows/ci.yaml`, `tools/frb_internal/lib/src/misc/dart_sanitizer_tester.dart`, `tools/frb_internal/lib/src/makefile_dart/ci_plan/full_jobs.dart`, the main Dart version, or the sanitizer artifact release workflow in `fzyzcjy/dart_lang_ci`.

## Coverage Levels

Required PR CI coverage:

- ASAN normal examples: `frb_example--dart_minimal`, `frb_example--pure_dart`, and `frb_example--pure_dart_pde`.
- LSAN normal examples: `frb_example--dart_minimal`, `frb_example--pure_dart`, and `frb_example--pure_dart_pde`.
- UBSAN normal examples: `frb_example--dart_minimal`, `frb_example--pure_dart`, and `frb_example--pure_dart_pde`.
- ASAN deliberate-bad sentinel: `frb_example--deliberate_bad`.
- MSAN deliberate-bad sentinel: `frb_example--deliberate_bad`.
- LSAN deliberate-bad sentinel: `frb_example--deliberate_bad`.
- TSAN deliberate-bad sentinel: `frb_example--deliberate_bad`.
- Valgrind normal examples: `frb_example--dart_minimal`, `frb_example--pure_dart`, and `frb_example--pure_dart_pde`.

Experimental or manual coverage:

- TSAN normal examples are not required while Dart VM/runtime reports ThreadSanitizer noise on otherwise normal examples. The required TSAN coverage is the Rust-only deliberate-bad sentinel.
- UBSAN has normal-example required coverage only. It does not have a Rust-side deliberate-bad sentinel while Rust nightly rejects `-Zsanitizer=undefined`; a Dart VM UBSAN artifact alone is not enough to prove FRB's Rust-side undefined behavior cases.
- MSAN normal examples are not required while Dart/Rust initialization produces the known `MemcmpInterceptorCommon` false positive.

## Artifact Refresh

The FRB sanitizer jobs consume Dart SDK artifacts from `fzyzcjy/dart_lang_ci` releases. The default release tag lives in `kDefaultSanitizedDartReleaseName`, and CI can override it with `FRB_SANITIZED_DART_RELEASE_NAME`.

Refresh steps:

```bash
gh workflow run release.yaml --repo fzyzcjy/dart_lang_ci --ref master -f dart_ref=3.11.0
```

After the run finishes, confirm that the release contains these required assets:

```text
ReleaseASANX64_dart-sdk.tar.gz
ReleaseASANX64_dart-sdk.tar.gz.sha256
ReleaseMSANX64_dart-sdk.tar.gz
ReleaseMSANX64_dart-sdk.tar.gz.sha256
ReleaseLSANX64_dart-sdk.tar.gz
ReleaseLSANX64_dart-sdk.tar.gz.sha256
ReleaseTSANX64_dart-sdk.tar.gz
ReleaseTSANX64_dart-sdk.tar.gz.sha256
ReleaseUBSANX64_dart-sdk.tar.gz
ReleaseUBSANX64_dart-sdk.tar.gz.sha256
```

Do not lower package `sdk:` constraints to keep old sanitizer artifacts working. Build a new artifact release for the Dart version used by FRB CI.

## Focused CI Commands

Run the full sanitizer matrix:

```bash
gh workflow run ci.yaml --repo fzyzcjy/flutter_rust_bridge --ref <branch> -f ci_filter=test_dart_sanitizer
```

Run one sanitizer entry:

```bash
gh workflow run ci.yaml --repo fzyzcjy/flutter_rust_bridge --ref <branch> -f 'ci_filter=test_dart_sanitizer[sanitizer=asan,package=frb_example--pure_dart]'
```

Run the full Valgrind matrix:

```bash
gh workflow run ci.yaml --repo fzyzcjy/flutter_rust_bridge --ref <branch> -f ci_filter=test_dart_valgrind
```

Run one Valgrind entry:

```bash
gh workflow run ci.yaml --repo fzyzcjy/flutter_rust_bridge --ref <branch> -f 'ci_filter=test_dart_valgrind[package=frb_example--pure_dart]'
```

## Local Checks

Run planner and sanitizer runner unit tests in the FRB Docker environment:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec -- bash -lc 'cd tools/frb_internal && dart test test/makefile_dart_test.dart test/ci_plan_test.dart'
```

Local Apple Silicon Docker may execute x64 sanitizer Dart binaries through qemu. Treat qemu sanitizer allocator failures as local environment limitations and use GitHub native x64 runners for final sanitizer runtime validation.

## Valgrind Notes

Valgrind uses the normal Dart SDK from `FRB_MAIN_DART_VERSION`, not the sanitized Dart artifacts. It remains useful even when ASAN/MSAN/LSAN are enabled because it exercises a different runtime and parser path.

`test-dart-valgrind` compiles a Dart CLI bundle and runs it under Memcheck with:

```text
--leak-check=full --show-leak-kinds=definite --errors-for-leak-kinds=definite --error-exitcode=1 --num-callers=1
```

The parser requires the generated entrypoint to print the shared all-tests-passed marker. If the marker is missing, treat the Dart test itself as failed even when Valgrind did not report leaks.
