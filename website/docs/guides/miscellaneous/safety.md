# Safety and CI

## CI

We heavily use CI to ensure everything is going well, including but not limited to memory safety.
Shortly speaking, we deploy not only `Valgrind` but also sanitizers (`ASAN`, `MSAN`, `LSAN`, `TSAN`) in the CI,
which are commonly used tools to check safety issues.

As for all things that CI checks,
please refer to [the CI configuration](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/.github/workflows)
for full details,
and here is a brief list:

* Use Valgrind to check safety
* Use Sanitizers (ASAN, MSAN, LSAN, TSAN) to check safety
* Testing (on Android, iOS, Windows, MacOS, Linux, Web)
* Run performance benchmarks
* Run `flutter_rust_bridge_codegen`
* Linters and code formatters
* Post-release tests (check the released binary)
* Test steps mentioned in quickstart
* ...

### Sanitizer checks

* The full CI matrix runs ASAN (address), MSAN (uninitialized memory), LSAN (leaks), and TSAN (thread races) on Linux x64 for `dart_minimal`, `pure_dart`, `pure_dart_pde`, and `deliberate_bad`.
* Both the Rust native library and the Dart SDK are instrumented. Rust builds use nightly with `-Zbuild-std`; debug information and disabled function merging keep allocation symbols distinguishable in reports.
* The `deliberate_bad` package checks good cases and known invalid operations against their expected exit status and diagnostic. Detection has limits: the Dart-to-Rust stack-buffer-overflow case currently expects success because ASAN does not detect it.
* The `pure_dart` and `pure_dart_pde` integration runs must exit successfully and print `FRB_DART_TEST_RESULT: success`. Suppressing a known report does not replace successful test completion.

#### Running a sanitizer check

Run from the repository root in a Linux x64 development environment with Dart, Rust nightly, the nightly `rust-src` component, and LLVM's symbolizer available:

```bash
ASAN_SYMBOLIZER_PATH=/usr/bin/llvm-symbolizer ./frb_internal test-dart-sanitizer --package frb_example--pure_dart --sanitizer asan
```

* Select `asan`, `msan`, `lsan`, or `tsan`; use `frb_example--deliberate_bad` to check that known invalid operations are detected as expected.
* The runner downloads a sanitized Dart SDK, verifies its SHA-256 checksum, and caches it by release under the system temporary directory. `FRB_SANITIZED_DART_RELEASE_NAME` overrides the release selected in [the SDK helper](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/tools/frb_internal/lib/src/misc/dart_sanitizer_tester/dart_sdk.dart).
* CI sets `FRB_MAIN_DART_VERSION` and requires the sanitized SDK to match it. Set that variable locally for the same check. Build a matching SDK artifact when upgrading Dart; do not lower package SDK constraints to accommodate an older artifact.
* `--use-local-sanitized-dart-binary` selects a locally built SDK under `~/dart-sdk/sdk/out/Release{ASAN,MSAN,LSAN,TSAN}X64/dart-sdk/bin/dart` instead of downloading one.

#### Known-report suppressions

* Suppressions must identify the affected allocation or diagnostic. Matching only a total leaked-byte count is insufficient.
* `pure_dart` ASAN/LSAN runs use exact allocation-symbol rules in [dart_lsan_cst.supp](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/tools/dart_lsan_cst.supp). The runner also rejects suppression totals above 36 allocations or 576 bytes, and unexpected or repeated rules.
* TSAN runs for `pure_dart` and `pure_dart_pde` use package-specific rules for `simple_use_async_spawn_blocking`. The runner checks any reported matches for the expected rule and exactly one suppression; thread-leak reporting remains enabled.
* When changing a suppression, inspect the symbolized report and retain the integration success checks and `deliberate_bad` detection checks. Keep the rules narrowly scoped to the known report.

See [the CI matrix](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/tools/frb_internal/lib/src/makefile_dart/ci_plan/full_jobs.dart) and [the sanitizer runner](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/tools/frb_internal/lib/src/misc/dart_sanitizer_tester/runner.dart) for the current coverage and result checks.

## Usage

This library seems to be used by (I want to say "is used by" but I need to be humble ;) ) many people
(as can be seen from [pub.dev popularity](https://pub.dev/packages/flutter_rust_bridge), blogs on the Internet, etc).
Therefore, bugs should be usually easily spotted and raised in the issue tracker.
However, during the past two years, there have not been issues about weird memory issues, hitting undefined behavior, etc.

I also personally use flutter_rust_bridge heavily in my Flutter project,
which is in production and it works quite well.
If I observe any problems, I will surely fix it in this library,
but again I have not seen any safety-related issues.

## Safety of unsafe

It is inevitable to write unsafe code, as long as we want to use Rust with another language.
The thing we can do and have done is,
make the unsafe blocks carefully limited & encapsulated,
make the code clear and well designed, use strong checkers in CI, etc.
I am happy to see that, nobody reported bugs related to this!

The vast majority of the code are written in safe Rust and (safe) Dart.
The `unsafe` code mainly happens when we need to leak a Rust Vec into a raw pointer,
and later assemble it back, in order to pass it to the Dart side.
This is widely used pattern - there are official Rust doc, and answers on StackOverflow about this also have high votes.

In terms of safety, there are two categories of codecs, and you can freely choose whichever you like.
One category uses bare minimal unsafe code, and the other maximizes performance.
Below, we only discuss the latter - since it is the harder case.

The unsafe logic is made as separated as possible with other safe logic, with as clear semantics as possible.
For example, instead of combining all logic into a single bigger `RustOpaque` Dart class,
I choose to extract a `RustArc` Dart class, which encapsulates the Rust `Arc` inside it.
Then, high-level `RustOpaque` logic will be safe as long as it uses `RustArc`'s public API,
and at the same time, `RustArc` is easy to audit because it has a clear semantics.

As another example, without extra work, `DartOpaque` is `!Send` and `!Sync`,
because the underlying objects really cannot be used in other threads.
But we `unsafe impl Send/Sync`, because we forbid users from touching or dropping it in other threads.
In V1, most logic were put inside the single `DartOpaque` class, mixing things with this unsafe part.
In V2, this unsafe logic is extracted to `GuardedBox`, and the `DartOpaque` becomes pure safe code.

These can be further seen in details in [the contributor guides](../contributing).

In addition, different parts are isolated. For example, if your project does not use opaque types,
then surely any code related to them will not be used at all.
