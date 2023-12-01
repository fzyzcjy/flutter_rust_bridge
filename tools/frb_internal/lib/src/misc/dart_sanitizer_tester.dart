// ignore_for_file: avoid_print

import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:path/path.dart' as path;

// for rust san also ref
// * https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
// * https://github.com/japaric/rust-san
Future<void> run(TestDartSanitizerConfig config) async {
  await runPubGetIfNotRunYet(config.package);

  if (config.package == 'frb_example/deliberate_bad') {
    await _runPackageDeliberateBad(config);
  } else {
    await _runEntrypoint(config);
  }
}

Future<void> _runEntrypoint(TestDartSanitizerConfig config) async {
  final sanitizedDart = await _getSanitizedDartBinary(config);
  await _execAndCheckWithSanitizerEnvVar(
    '$sanitizedDart --enable-experiment=native-assets run test/dart_valgrind_test_entrypoint.dart',
    const _Info(
        name: 'entrypoint', expectSucceed: true, expectStderrContains: ''),
    config.sanitizer,
    relativePwd: config.package,
  );
}

Future<void> _runPackageDeliberateBad(TestDartSanitizerConfig config) async {
  await _runPackageDeliberateBadRustOnly(config);
  await _runPackageDeliberateBadWithDart(config);
}

Future<void> _runPackageDeliberateBadRustOnly(
    TestDartSanitizerConfig config) async {
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
            expectStderrContains:
                'ERROR: AddressSanitizer: heap-use-after-free',
          ),
        ],
      Sanitizer.lsan => [
          const _Info(
            name: 'RustOnly_MemoryLeak',
            expectSucceed: false,
            expectStderrContains: 'ERROR: LeakSanitizer: detected memory leaks',
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
    TestDartSanitizerConfig config) async {
  final kDartOnlyInfos = [
    const _Info(
      name: 'DartOnly_Good',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    ...switch (config.sanitizer) {
      Sanitizer.asan => [
          // NOTE ASAN does not report this as buggy...
          const _Info(
            name: 'DartOnly_HeapUseAfterFree',
            expectSucceed: true,
            expectStderrContains: '',
          ),
        ],
      Sanitizer.lsan => [],
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
            expectStderrContains:
                'ERROR: AddressSanitizer: heap-use-after-free',
          ),
        ],
      Sanitizer.lsan => [
          const _Info(
            name: 'DartCallRust_MemoryLeak',
            expectSucceed: false,
            expectStderrContains: 'ERROR: LeakSanitizer: detected memory leaks',
          ),
        ],
    },
  ];

  final sanitizedDart = await _getSanitizedDartBinary(config);
  for (final info in kDartOnlyInfos + kDartCallRustInfos) {
    await _execAndCheckWithSanitizerEnvVar(
      '$sanitizedDart --enable-experiment=native-assets run '
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

  const _Info({
    required this.name,
    required this.expectSucceed,
    required this.expectStderrContains,
  });
}

Future<void> _execAndCheckWithSanitizerEnvVar(
  String cmd,
  _Info info,
  Sanitizer sanitizer, {
  required String relativePwd,
}) async {
  print('====== execAndCheckWithSanitizerEnvVar name=${info.name} ======');

  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: {
      'RUSTFLAGS': '-Zsanitizer=${sanitizer.rustflagValue}',
      'FRB_SIMPLE_BUILD_CARGO_NIGHTLY': '1',
      'FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS': _cargoBuildExtraArgs,
      // because we unconventionally specified the `--target` in cargo build
      'FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR':
          'rust/target/x86_64-unknown-linux-gnu/release/',
    },
    checkExitCode: false,
  );

  if ((output.exitCode == 0) != info.expectSucceed) {
    throw Exception(
        'Bad exitCode=${output.exitCode}, while expectSucceed=${info.expectSucceed}');
  }

  if (!output.stderr.contains(info.expectStderrContains)) {
    throw Exception(
        'Bad stderr which does not contain `${info.expectStderrContains}`');
  }

  print('Pass check for ${info.name}');
}

Future<String> _getSanitizedDartBinary(TestDartSanitizerConfig config) async {
  if (config.useLocalSanitizedDartBinary) {
    return '~/dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart';
  }

  const url =
      'https://github.com/fzyzcjy/dart_lang_ci/releases/download/Build_2023.12.01_06-51-09/dart';
  final pathBin = path.join(
      Directory.systemTemp.path, 'dart_${config.sanitizer.dartSdkBuildOutDir}');
  if (await File(pathBin).exists()) {
    print('Skip downloading artifat since $pathBin already exists');
  } else {
    print('Download artifact from $url to $pathBin...');
    await Dio().download(url, pathBin);
  }
  await exec('chmod +x $pathBin');
  return pathBin;
}

const _cargoBuildExtraArgs = '-Zbuild-std --target x86_64-unknown-linux-gnu';

extension on Sanitizer {
  String get rustflagValue {
    return switch (this) {
      Sanitizer.asan => 'address',
      Sanitizer.lsan => 'leak',
    };
  }

  String get dartSdkBuildOutDir {
    return switch (this) {
      Sanitizer.asan => 'ReleaseASANX64',
      Sanitizer.lsan => 'ReleaseLSANX64',
    };
  }
}
