// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:path/path.dart' as path;
import 'package:yaml/yaml.dart';

// for rust san also ref
// * https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
// * https://github.com/japaric/rust-san
Future<void> run(TestDartSanitizerConfig config) async {
  await _lowerSdkMinVersion(package: config.package);

  await runPubGet(config.package, kDartModeOfPackage[config.package]!);

  if (config.package == 'frb_example/deliberate_bad') {
    await _runPackageDeliberateBad(config);
  } else {
    await _runEntrypoint(config);
  }
}

Future<void> _lowerSdkMinVersion({required String package}) async {
  await _modifySdkMinVersion(path: '${exec.pwd}$package/pubspec.yaml');
  await _modifySdkMinVersion(path: '${exec.pwd}frb_dart/pubspec.yaml');
}

Future<void> _modifySdkMinVersion({required String path}) async {
  print('modifySdkMinVersion(path=$path)');

  final file = File(path);

  final contentRaw = loadYaml(file.readAsStringSync());
  final content = jsonDecode(jsonEncode(contentRaw));

  // Lower the version since the custom compiled Dart SDK is done before,
  // and we do not really need new sdk version when on native platform.
  content['environment']['sdk'] = '>=3.2.0';
  file.writeAsStringSync(jsonEncode(content));

  print('modifySdkMinVersion path=$path content=${file.readAsStringSync()}');
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
            expectStderrContains:
                'ERROR: AddressSanitizer: heap-use-after-free',
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
      ...kEnvEnableRustBacktrace,
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

  const releaseName = 'Build_2023.12.01_09-42-01';
  final baseName = '${config.sanitizer.dartSdkBuildOutDir}_dart-sdk';
  final fileNameTarGz = '$baseName.tar.gz';

  final pathTarGz = path.join(Directory.systemTemp.path, fileNameTarGz);
  final pathUnzippedDir = path.join(Directory.systemTemp.path, baseName);
  final pathBin = path.join(pathUnzippedDir,
      'dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart');

  if (!await File(pathTarGz).exists()) {
    final url =
        'https://github.com/fzyzcjy/dart_lang_ci/releases/download/$releaseName/$fileNameTarGz';
    print('Download artifact from $url to $pathTarGz...');
    await Dio().download(url, pathTarGz);
  }

  if (!await File(pathBin).exists()) {
    await exec('mkdir $pathUnzippedDir');
    await exec('tar -xvzf $pathTarGz -C $pathUnzippedDir');
  }

  if (!await File(pathBin).exists()) {
    throw Exception('$pathBin still not exist');
  }

  return pathBin;
}

const _cargoBuildExtraArgs = '-Zbuild-std --target x86_64-unknown-linux-gnu';

extension on Sanitizer {
  String get rustflagValue {
    return switch (this) {
      Sanitizer.asan => 'address',
      Sanitizer.msan => 'memory',
      Sanitizer.lsan => 'leak',
      Sanitizer.tsan => 'thread',
    };
  }

  String get dartSdkBuildOutDir {
    return switch (this) {
      Sanitizer.asan => 'ReleaseASANX64',
      Sanitizer.msan => 'ReleaseMSANX64',
      Sanitizer.lsan => 'ReleaseLSANX64',
      Sanitizer.tsan => 'ReleaseTSANX64',
    };
  }
}
