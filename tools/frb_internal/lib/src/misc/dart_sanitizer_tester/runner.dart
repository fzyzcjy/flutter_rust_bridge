// ignore_for_file: avoid_print

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/dart_sdk.dart';
import 'package:flutter_rust_bridge_internal/src/misc/dart_sanitizer_tester/sanitizer.dart';
import 'package:path/path.dart' as path;

// for rust san also ref
// * https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
// * https://github.com/japaric/rust-san
Future<void> run(TestDartSanitizerConfig config) async {
  await _printSanitizerToolchainInfo(config);
  await runPubGet(config.package, kDartModeOfPackage[config.package]!);

  // Otherwise it seems the sanitized dart binary does not compile native assets
  await exec(
    'dart run test/empty_entrypoint.dart',
    relativePwd: config.package,
  );

  if (config.package == 'frb_example/deliberate_bad') {
    await _runPackageDeliberateBad(config);
  } else {
    await _runEntrypoint(config);
  }
}

Future<void> _runEntrypoint(TestDartSanitizerConfig config) async {
  await _buildPackageNativeLibraryForDart(config);

  final sanitizedDart = await getSanitizedDartBinary(config);
  await _execAndCheckWithSanitizerEnvVar(
    '$sanitizedDart --enable-vm-service=0 --disable-service-auth-codes '
    'run test/dart_valgrind_test_entrypoint.dart',
    _Info(
      name: 'entrypoint',
      expectSucceed: true,
      expectStderrContains: '',
      expectStdoutContains: _expectedTestResultMarker(config.package),
    ),
    config.sanitizer,
    relativePwd: config.package,
    allowedFailure: _allowedSanitizerFailure(config),
  );
}

String _expectedTestResultMarker(String package) {
  return switch (package) {
    'frb_example/pure_dart' ||
    'frb_example/pure_dart_pde' => 'FRB_DART_TEST_RESULT: success',
    _ => '',
  };
}

_AllowedSanitizerFailure? _allowedSanitizerFailure(
  TestDartSanitizerConfig config,
) {
  final summaries = _allowedLeakSummaries(config);
  if (summaries.isEmpty) return null;

  return switch (config.sanitizer) {
    Sanitizer.asan => _AllowedSanitizerFailure(
      exitCode: 1,
      reportHeader: 'ERROR: LeakSanitizer: detected memory leaks',
      summaries: summaries,
      trailingLinesAfterSummary: const [],
    ),
    Sanitizer.lsan => _AllowedSanitizerFailure(
      exitCode: 23,
      reportHeader: 'ERROR: LeakSanitizer: detected memory leaks',
      summaries: summaries,
      trailingLinesAfterSummary: const [],
    ),
    Sanitizer.tsan => _AllowedSanitizerFailure(
      exitCode: 66,
      reportHeader: 'WARNING: ThreadSanitizer: thread leak',
      summaries: summaries,
      trailingLinesAfterSummary: const [
        '==================',
        'ThreadSanitizer: reported 1 warnings',
      ],
    ),
    _ => null,
  };
}

List<String> _allowedLeakSummaries(TestDartSanitizerConfig config) {
  return allowedLeakSummariesForTesting(
    sanitizer: config.sanitizer,
    package: config.package,
  );
}

List<String> allowedLeakSummariesForTesting({
  required Sanitizer sanitizer,
  required String package,
}) {
  final primary = allowedLeakSummaryForTesting(
    sanitizer: sanitizer,
    package: package,
  );
  if (primary == null) return const [];

  if (sanitizer == Sanitizer.lsan && package == 'frb_example/pure_dart') {
    return [
      primary,
      'SUMMARY: LeakSanitizer: 1048 byte(s) leaked in 95 allocation(s).',
    ];
  }

  return [primary];
}

String? allowedLeakSummaryForTesting({
  required Sanitizer sanitizer,
  required String package,
}) {
  final sanitizerName = switch (sanitizer) {
    Sanitizer.asan => 'AddressSanitizer',
    Sanitizer.lsan => 'LeakSanitizer',
    Sanitizer.tsan => 'ThreadSanitizer',
    _ => null,
  };
  if (sanitizerName == null) return null;

  if (sanitizer == Sanitizer.tsan) {
    return switch (package) {
      'frb_example/pure_dart' || 'frb_example/pure_dart_pde' =>
        'SUMMARY: ThreadSanitizer: thread leak ??:? in pthread_create',
      _ => null,
    };
  }

  return switch (package) {
    'frb_example/pure_dart' =>
      'SUMMARY: $sanitizerName: 1056 byte(s) leaked in 96 allocation(s).',
    'frb_example/pure_dart_pde' =>
      'SUMMARY: $sanitizerName: 240 byte(s) leaked in 30 allocation(s).',
    _ => null,
  };
}

Future<void> _runPackageDeliberateBad(TestDartSanitizerConfig config) async {
  await _runPackageDeliberateBadRustOnly(config);
  await _buildPackageNativeLibraryForDart(config);
  await _runPackageDeliberateBadWithDart(config);
}

Future<void> _buildPackageNativeLibraryForDart(
  TestDartSanitizerConfig config,
) async {
  final crateName = path.basename(config.package);
  final libraryName = 'libfrb_example_$crateName.so';
  final featureArgs = _cargoFeatureArgs(config);
  await _execAndCheckWithSanitizerEnvVar(
    'cargo +nightly build --release $_cargoBuildExtraArgs$featureArgs'
    ' && mkdir -p target/release'
    ' && cp target/x86_64-unknown-linux-gnu/release/$libraryName'
    ' target/release/$libraryName',
    const _Info(
      name: 'BuildNativeLibraryForDart',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    config.sanitizer,
    relativePwd: '${config.package}/rust',
  );
}

String _cargoFeatureArgs(TestDartSanitizerConfig config) {
  return switch (config.package) {
    'frb_example/pure_dart' ||
    'frb_example/pure_dart_pde' => ' --features internal_feature_for_testing',
    _ => '',
  };
}

Future<void> _runPackageDeliberateBadRustOnly(
  TestDartSanitizerConfig config,
) async {
  final kInfos = [
    const _Info(
      name: 'RustOnly_Good',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    ...switch (config.sanitizer) {
      Sanitizer.asan => [
        const _Info(
          name: 'RustOnly_StackBufferOverflow',
          expectSucceed: false,
          expectStderrContains:
              'ERROR: AddressSanitizer: stack-buffer-overflow',
        ),
        const _Info(
          name: 'RustOnly_HeapUseAfterFree',
          expectSucceed: false,
          expectStderrContains: 'ERROR: AddressSanitizer: heap-use-after-free',
        ),
      ],
      Sanitizer.msan => [
        const _Info(
          name: 'RustOnly_UseOfUninitializedValue',
          expectSucceed: false,
          expectStderrContains:
              'WARNING: MemorySanitizer: use-of-uninitialized-value',
        ),
      ],
      Sanitizer.lsan => [
        const _Info(
          name: 'RustOnly_MemoryLeak',
          expectSucceed: false,
          expectStderrContains: 'ERROR: LeakSanitizer: detected memory leaks',
        ),
      ],
      Sanitizer.tsan => [
        const _Info(
          name: 'RustOnly_DataRace',
          expectSucceed: false,
          expectStderrContains: 'WARNING: ThreadSanitizer: data race',
        ),
      ],
    },
  ];

  for (final info in kInfos) {
    await _execAndCheckWithSanitizerEnvVar(
      'cargo +nightly run $_cargoBuildExtraArgs ${info.name}',
      info,
      config.sanitizer,
      relativePwd: '${config.package}/rust',
    );
  }
}

Future<void> _runPackageDeliberateBadWithDart(
  TestDartSanitizerConfig config,
) async {
  final kDartOnlyInfos = [
    const _Info(
      name: 'DartOnly_Good',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    ...switch (config.sanitizer) {
      Sanitizer.asan => [
        const _Info(
          name: 'DartOnly_HeapUseAfterFree',
          expectSucceed: false,
          expectStderrContains: 'ERROR: AddressSanitizer: heap-use-after-free',
        ),
      ],
      Sanitizer.msan => [
        // Pure dart almost cannot have this problem
      ],
      Sanitizer.lsan => [
        const _Info(
          name: 'DartOnly_MemoryLeak',
          expectSucceed: false,
          expectStderrContains: 'ERROR: LeakSanitizer: detected memory leaks',
        ),
      ],
      Sanitizer.tsan => [
        // Pure-dart almost cannot have data race
      ],
    },
  ];

  final kDartCallRustInfos = [
    ...switch (config.sanitizer) {
      Sanitizer.asan => [
        // NOTE It should fail, but ASAN did not realize this case...
        const _Info(
          name: 'DartCallRust_StackBufferOverflow',
          expectSucceed: true,
          expectStderrContains: '',
        ),
        // ASAN successfully understand this case
        const _Info(
          name: 'DartCallRust_HeapUseAfterFree',
          expectSucceed: false,
          expectStderrContains: 'ERROR: AddressSanitizer: heap-use-after-free',
        ),
      ],
      Sanitizer.msan => [
        const _Info(
          name: 'DartCallRust_UseOfUninitializedValue',
          expectSucceed: false,
          expectStderrContains:
              'WARNING: MemorySanitizer: use-of-uninitialized-value',
        ),
      ],
      Sanitizer.lsan => [
        const _Info(
          name: 'DartCallRust_MemoryLeak',
          expectSucceed: false,
          expectStderrContains: 'ERROR: LeakSanitizer: detected memory leaks',
        ),
      ],
      Sanitizer.tsan => [
        const _Info(
          name: 'DartCallRust_DataRace',
          expectSucceed: false,
          expectStderrContains: 'WARNING: ThreadSanitizer: data race',
        ),
      ],
    },
  ];

  final sanitizedDart = await getSanitizedDartBinary(config);
  for (final info in kDartOnlyInfos + kDartCallRustInfos) {
    await _execAndCheckWithSanitizerEnvVar(
      '$sanitizedDart run '
      'frb_example_deliberate_bad ${info.name}',
      info,
      config.sanitizer,
      relativePwd: config.package,
    );
  }
}

class _Info {
  final String name;
  final bool expectSucceed;
  final String expectStderrContains;
  final String expectStdoutContains;

  const _Info({
    required this.name,
    required this.expectSucceed,
    required this.expectStderrContains,
    this.expectStdoutContains = '',
  });
}

class _AllowedSanitizerFailure {
  final int exitCode;
  final String reportHeader;
  final List<String> summaries;
  final List<String> trailingLinesAfterSummary;

  const _AllowedSanitizerFailure({
    required this.exitCode,
    required this.reportHeader,
    required this.summaries,
    required this.trailingLinesAfterSummary,
  });
}

Future<void> _execAndCheckWithSanitizerEnvVar(
  String cmd,
  _Info info,
  Sanitizer sanitizer, {
  required String relativePwd,
  _AllowedSanitizerFailure? allowedFailure,
}) async {
  print('====== execAndCheckWithSanitizerEnvVar name=${info.name} ======');

  final rustflags = sanitizer.rustflags;
  final rustSanitizerEnv = rustflags == null
      ? <String, String>{}
      : {
          'NIX_FRB_RUSTFLAGS': rustflags,
          'RUSTFLAGS': rustflags,
          'NIX_FRB_SIMPLE_BUILD_CARGO_NIGHTLY': '1',
          'NIX_FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS': _cargoBuildExtraArgs,
        };

  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: {
      ...rustSanitizerEnv,
      // because we unconventionally specified the `--target` in cargo build
      'FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR': 'rust/target/release/',
      ...kEnvEnableRustBacktrace,
    },
    checkExitCode: false,
  );

  final hasOnlyAllowedFailure = isOnlyAllowedSanitizerFailureForTesting(
    exitCode: output.exitCode,
    stderr: output.stderr,
    expectedExitCode: allowedFailure?.exitCode,
    expectedReportHeader: allowedFailure?.reportHeader,
    expectedSummaries: allowedFailure?.summaries,
    expectedTrailingLinesAfterSummary:
        allowedFailure?.trailingLinesAfterSummary,
  );
  if ((output.exitCode == 0) != info.expectSucceed && !hasOnlyAllowedFailure) {
    throw Exception(
      'Bad exitCode=${output.exitCode}, while expectSucceed=${info.expectSucceed}',
    );
  }

  if (!output.stderr.contains(info.expectStderrContains)) {
    throw Exception(
      'Bad stderr which does not contain `${info.expectStderrContains}`',
    );
  }
  if (!output.stdout.contains(info.expectStdoutContains)) {
    throw Exception(
      'Bad stdout which does not contain `${info.expectStdoutContains}`',
    );
  }

  print('Pass check for ${info.name}');
}

bool isOnlyAllowedSanitizerFailureForTesting({
  required int exitCode,
  required String stderr,
  required int? expectedExitCode,
  required String? expectedReportHeader,
  required List<String>? expectedSummaries,
  required List<String>? expectedTrailingLinesAfterSummary,
}) {
  final lines = stderr.split('\n').map((line) => line.trimRight()).toList();
  final sanitizerSummaries = lines
      .where((line) => line.startsWith('SUMMARY: '))
      .toList();
  final diagnosticHeaders = lines.where(_isFatalDiagnosticHeader).toList();
  final nonEmptyLines = lines.where((line) => line.isNotEmpty).toList();

  return expectedExitCode != null &&
      expectedReportHeader != null &&
      expectedSummaries != null &&
      expectedTrailingLinesAfterSummary != null &&
      exitCode == expectedExitCode &&
      sanitizerSummaries.length == 1 &&
      expectedSummaries.contains(sanitizerSummaries.single) &&
      diagnosticHeaders.length == 1 &&
      diagnosticHeaders.single.contains(expectedReportHeader) &&
      nonEmptyLines.length >= expectedTrailingLinesAfterSummary.length + 1 &&
      _iterableEquals(
        nonEmptyLines.skip(
          nonEmptyLines.length - expectedTrailingLinesAfterSummary.length - 1,
        ),
        [sanitizerSummaries.single, ...expectedTrailingLinesAfterSummary],
      );
}

bool _isFatalDiagnosticHeader(String line) {
  final trimmed = line.trimLeft();
  return (trimmed.startsWith('==') && trimmed.contains('==ERROR:')) ||
      (trimmed.startsWith('WARNING: ') && trimmed.contains('Sanitizer:')) ||
      trimmed.startsWith('FATAL:') ||
      trimmed.startsWith('Fatal error') ||
      trimmed.startsWith('runtime error:') ||
      trimmed.startsWith('Unhandled exception:') ||
      trimmed.contains('Sanitizer:DEADLYSIGNAL');
}

bool _iterableEquals(Iterable<String> left, Iterable<String> right) {
  final leftIterator = left.iterator;
  final rightIterator = right.iterator;
  while (leftIterator.moveNext()) {
    if (!rightIterator.moveNext() ||
        leftIterator.current != rightIterator.current) {
      return false;
    }
  }
  return !rightIterator.moveNext();
}

Future<void> _printSanitizerToolchainInfo(
  TestDartSanitizerConfig config,
) async {
  final releaseName = config.useLocalSanitizedDartBinary
      ? '<local-sanitized-dart>'
      : sanitizedDartReleaseName();
  print(
    'sanitizer config: sanitizer=${config.sanitizer.name} '
    'package=${config.package} sanitizedDartReleaseName=$releaseName',
  );

  final rustcOutput = await exec(
    'rustc +nightly --version',
    checkExitCode: false,
  );
  print(
    'Rust nightly version: '
    'stdout=${rustcOutput.stdout.trim()} stderr=${rustcOutput.stderr.trim()}',
  );
}

const _cargoBuildExtraArgs = '-Zbuild-std --target x86_64-unknown-linux-gnu';
