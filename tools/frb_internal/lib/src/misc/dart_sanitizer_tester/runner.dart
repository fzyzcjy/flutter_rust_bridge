// ignore_for_file: avoid_print

import 'dart:io';

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
    sanitizerEnvironment: sanitizerEntrypointEnvironmentForTesting(
      package: config.package,
      sanitizer: config.sanitizer,
      environment: Platform.environment,
    ),
  );
}

Map<String, String> sanitizerEntrypointEnvironmentForTesting({
  required String package,
  required Sanitizer sanitizer,
  required Map<String, String> environment,
}) {
  if (package != 'frb_example/pure_dart_pde' || sanitizer != Sanitizer.tsan) {
    return {};
  }
  final existing = environment['TSAN_OPTIONS'];
  return {
    'TSAN_OPTIONS': [
      if (existing != null && existing.isNotEmpty) existing,
      'report_thread_leaks=1',
      'print_suppressions=1',
      'suppressions=../../tools/dart_tsan_pde.supp',
    ].join(':'),
  };
}

void checkPdeThreadLeakSuppressionForTesting(String stderr) {
  const prefix = 'ThreadSanitizer: Matched ';
  if (!stderr.contains(prefix)) return;
  const expectedRule =
      'thread:frb_example_pure_dart_pde::api::async_spawn::'
      'simple_use_async_spawn_blocking::';
  final matches = RegExp(
    r'^ThreadSanitizer: Matched 1 suppressions \(pid=\d+\):\r?\n'
    '1 ${RegExp.escape(expectedRule)}\r?\n',
    multiLine: true,
  ).allMatches(stderr);
  if (matches.length != 1 || prefix.allMatches(stderr).length != 1) {
    throw Exception('Unexpected TSAN suppression count or rule');
  }
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
  return switch (config.sanitizer) {
    Sanitizer.asan => const _AllowedSanitizerFailure(
      exitCode: 1,
      normalizedReports: [],
    ),
    Sanitizer.lsan => const _AllowedSanitizerFailure(
      exitCode: 23,
      normalizedReports: [],
    ),
    Sanitizer.tsan => const _AllowedSanitizerFailure(
      exitCode: 66,
      normalizedReports: [],
    ),
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
  final List<String> normalizedReports;

  const _AllowedSanitizerFailure({
    required this.exitCode,
    required this.normalizedReports,
  });
}

Future<void> _execAndCheckWithSanitizerEnvVar(
  String cmd,
  _Info info,
  Sanitizer sanitizer, {
  required String relativePwd,
  _AllowedSanitizerFailure? allowedFailure,
  Map<String, String> sanitizerEnvironment = const {},
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
      ...sanitizerEnvironment,
    },
    checkExitCode: false,
  );

  if (sanitizerEnvironment.containsKey('TSAN_OPTIONS')) {
    checkPdeThreadLeakSuppressionForTesting(output.stderr);
  }

  final hasOnlyAllowedFailure = isOnlyAllowedSanitizerFailureForTesting(
    exitCode: output.exitCode,
    stderr: output.stderr,
    expectedExitCode: allowedFailure?.exitCode,
    expectedNormalizedReports: allowedFailure?.normalizedReports,
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
  required List<String>? expectedNormalizedReports,
}) {
  return expectedExitCode != null &&
      expectedNormalizedReports != null &&
      exitCode == expectedExitCode &&
      expectedNormalizedReports.contains(
        normalizeSanitizerReportForTesting(stderr),
      );
}

String normalizeSanitizerReportForTesting(String report) {
  return report
      .split('\n')
      .map((line) => line.trimRight())
      .map(
        (line) => line
            .replaceFirst(RegExp(r'^\d{4}-\d\d-\d\dT\S+Z '), '')
            .replaceAll(RegExp(r'==\d+=='), '==<pid>==')
            .replaceAll(RegExp(r'\(pid=\d+\)'), '(pid=<pid>)')
            .replaceAll(RegExp(r'\btid=\d+\b'), 'tid=<tid>')
            .replaceAll(RegExp(r'\bThread T\d+\b'), 'Thread T<id>')
            .replaceAll(RegExp(r'\bthread T\d+\b'), 'thread T<id>')
            .replaceAll(RegExp(r'(?<!\+)0x[0-9a-fA-F]+'), '0x<address>')
            .replaceAll(RegExp(r'\s+\(BuildId: [^)]+\)'), '')
            .replaceAll(RegExp(r'\([^)]*/dartvm\+'), '(<dartvm>+'),
      )
      .join('\n');
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
