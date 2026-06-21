// ignore_for_file: avoid_print

import 'dart:io';

import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:path/path.dart' as path;

const kDefaultSanitizedDartReleaseName = 'Build_2026.06.19_13-45-18';
const _kSanitizedDartReleaseNameEnv = 'FRB_SANITIZED_DART_RELEASE_NAME';
const _kMainDartVersionEnv = 'FRB_MAIN_DART_VERSION';

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

  final sanitizedDart = await _getSanitizedDartBinary(config);
  await _execAndCheckWithSanitizerEnvVar(
    '$sanitizedDart run test/dart_valgrind_test_entrypoint.dart',
    const _Info(
      name: 'entrypoint',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    config.sanitizer,
    extraEnv: _dartTestSkipEnv(config),
    relativePwd: config.package,
  );
}

Future<void> _runPackageDeliberateBad(TestDartSanitizerConfig config) async {
  await _runPackageDeliberateBadRustOnly(config);
  if (config.sanitizer == Sanitizer.msan) return;

  await _buildPackageDeliberateBadNativeLibraryForDart(config);
  await _runPackageDeliberateBadWithDart(config);
}

Future<void> _buildPackageDeliberateBadNativeLibraryForDart(
  TestDartSanitizerConfig config,
) async {
  await _buildPackageNativeLibraryForDart(config);
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
      Sanitizer.ubsan => [],
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
      Sanitizer.ubsan => [],
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
      Sanitizer.tsan => [],
      Sanitizer.ubsan => [],
    },
  ];

  final sanitizedDart = await _getSanitizedDartBinary(config);
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
  Map<String, String> extraEnv = const {},
  required String relativePwd,
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
      ...sanitizer.runtimeEnv,
      ...sanitizer.threadPoolEnv,
      ...extraEnv,
      ...kEnvEnableRustBacktrace,
    },
    checkExitCode: false,
  );

  if ((output.exitCode == 0) != info.expectSucceed) {
    throw Exception(
      'Bad exitCode=${output.exitCode}, while expectSucceed=${info.expectSucceed}',
    );
  }

  if (!output.stderr.contains(info.expectStderrContains)) {
    throw Exception(
      'Bad stderr which does not contain `${info.expectStderrContains}`',
    );
  }

  print('Pass check for ${info.name}');
}

Map<String, String> _dartTestSkipEnv(TestDartSanitizerConfig config) {
  if (config.package != 'frb_example/pure_dart' &&
      config.package != 'frb_example/pure_dart_pde') {
    return {};
  }

  final skipEntryPoints = switch (config.sanitizer) {
    Sanitizer.asan || Sanitizer.msan || Sanitizer.tsan => [
      'api/dart_opaque_sync_test.dart',
      'api/exception_test.dart',
      'api/pseudo_manual/exception_twin_rust_async_test.dart',
      'api/pseudo_manual/exception_twin_rust_async_sse_test.dart',
      'api/pseudo_manual/exception_twin_sse_test.dart',
      'api/pseudo_manual/exception_twin_sync_test.dart',
      'api/pseudo_manual/exception_twin_sync_sse_test.dart',
      'api/pseudo_manual/dart_opaque_sync_twin_sse_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_rust_async_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_rust_async_sse_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_rust_async_sse_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_rust_async_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sse_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sse_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sync_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sync_sse_moi_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sync_sse_test.dart',
      'api/pseudo_manual/rust_auto_opaque_twin_sync_test.dart',
      'api/pseudo_manual/stream_twin_rust_async_test.dart',
      'api/pseudo_manual/stream_twin_rust_async_sse_test.dart',
      'api/pseudo_manual/stream_twin_sse_test.dart',
      'api/rust_auto_opaque_test.dart',
      'api/stream_test.dart',
    ],
    _ => <String>[],
  };
  if (skipEntryPoints.isEmpty) return {};

  return {'FRB_DART_TEST_SKIP_ENTRYPOINTS': skipEntryPoints.join(',')};
}

Future<String> _getSanitizedDartBinary(TestDartSanitizerConfig config) async {
  if (config.useLocalSanitizedDartBinary) {
    final path =
        '~/dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart';
    await _printSanitizedDartVersion(path);
    return path;
  }

  final releaseName = sanitizedDartReleaseName();
  final baseName = '${config.sanitizer.dartSdkBuildOutDir}_dart-sdk';
  final fileNameTarGz = '$baseName.tar.gz';

  final pathCacheRoot = path.join(
    Directory.systemTemp.path,
    'frb_sanitized_dart',
    releaseName,
  );
  await Directory(pathCacheRoot).create(recursive: true);

  final pathTarGz = path.join(pathCacheRoot, fileNameTarGz);
  final pathUnzippedDir = path.join(pathCacheRoot, baseName);
  final pathBin = path.join(
    pathUnzippedDir,
    'dart-sdk/sdk/out/${config.sanitizer.dartSdkBuildOutDir}/dart-sdk/bin/dart',
  );

  if (!await File(pathTarGz).exists()) {
    await _downloadSanitizedDartBinaryArtifact(
      releaseName: releaseName,
      fileNameTarGz: fileNameTarGz,
      pathTarGz: pathTarGz,
    );
  }

  if (!await File(pathBin).exists()) {
    await exec('mkdir -p $pathUnzippedDir');
    await exec('tar -xvzf $pathTarGz -C $pathUnzippedDir');
  }

  if (!await File(pathBin).exists()) {
    throw Exception('$pathBin still not exist');
  }

  await _printSanitizedDartVersion(pathBin);
  return pathBin;
}

String sanitizedDartReleaseName({Map<String, String>? environment}) {
  final value =
      (environment ?? Platform.environment)[_kSanitizedDartReleaseNameEnv];
  if (value == null || value.trim().isEmpty) {
    return kDefaultSanitizedDartReleaseName;
  }
  return value.trim();
}

Future<void> _printSanitizedDartVersion(String sanitizedDart) async {
  final output = await exec('$sanitizedDart --version', checkExitCode: false);
  final versionOutput = '${output.stdout}\n${output.stderr}';
  print(
    'sanitized Dart version: '
    'stdout=${output.stdout.trim()} stderr=${output.stderr.trim()}',
  );
  checkSanitizedDartVersionForTesting(
    versionOutput: versionOutput,
    environment: Platform.environment,
  );
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

void checkSanitizedDartVersionForTesting({
  required String versionOutput,
  required Map<String, String> environment,
}) {
  final expectedVersion = environment[_kMainDartVersionEnv]?.trim();
  if (expectedVersion == null || expectedVersion.isEmpty) return;

  final match = RegExp(
    r'Dart SDK version:\s*([0-9]+\.[0-9]+\.[0-9]+)',
  ).firstMatch(versionOutput);
  if (match == null) {
    throw Exception(
      'Cannot parse sanitized Dart version from output: $versionOutput',
    );
  }

  final actualVersion = match.group(1);
  if (actualVersion != expectedVersion) {
    throw Exception(
      'Sanitized Dart version $actualVersion does not match '
      '$_kMainDartVersionEnv=$expectedVersion. Build a new sanitized Dart '
      'artifact instead of lowering pubspec SDK constraints.',
    );
  }
}

Future<void> _downloadSanitizedDartBinaryArtifact({
  required String releaseName,
  required String fileNameTarGz,
  required String pathTarGz,
}) async {
  final publicUrl =
      'https://github.com/fzyzcjy/dart_lang_ci/releases/download/$releaseName/$fileNameTarGz';
  print('Download artifact from $publicUrl to $pathTarGz...');

  try {
    await Dio().download(publicUrl, pathTarGz);
    return;
  } on DioException {
    final token =
        Platform.environment['GITHUB_TOKEN'] ??
        Platform.environment['GH_TOKEN'];
    if (token == null || token.isEmpty) rethrow;

    print(
      'Public artifact download failed; retry via GitHub API asset download',
    );

    final assetId = await _findGitHubReleaseAssetId(
      releaseName: releaseName,
      fileNameTarGz: fileNameTarGz,
      token: token,
    );
    final response = await Dio().get<List<int>>(
      'https://api.github.com/repos/fzyzcjy/dart_lang_ci/releases/assets/$assetId',
      options: Options(
        responseType: ResponseType.bytes,
        headers: {
          HttpHeaders.authorizationHeader: 'Bearer $token',
          HttpHeaders.acceptHeader: 'application/octet-stream',
          HttpHeaders.userAgentHeader: 'flutter-rust-bridge-ci',
        },
      ),
    );
    await File(pathTarGz).writeAsBytes(response.data!);
  }
}

Future<int> _findGitHubReleaseAssetId({
  required String releaseName,
  required String fileNameTarGz,
  required String token,
}) async {
  final response = await Dio().get<Map<String, dynamic>>(
    'https://api.github.com/repos/fzyzcjy/dart_lang_ci/releases/tags/$releaseName',
    options: Options(
      headers: {
        HttpHeaders.authorizationHeader: 'Bearer $token',
        HttpHeaders.acceptHeader: 'application/vnd.github+json',
        HttpHeaders.userAgentHeader: 'flutter-rust-bridge-ci',
      },
    ),
  );

  final assets = response.data!['assets'] as List<dynamic>;
  final asset = assets.cast<Map<String, dynamic>>().firstWhere(
    (element) => element['name'] == fileNameTarGz,
    orElse: () => throw Exception(
      'Cannot find GitHub release asset `$fileNameTarGz` in `$releaseName`',
    ),
  );
  return asset['id'] as int;
}

const _cargoBuildExtraArgs = '-Zbuild-std --target x86_64-unknown-linux-gnu';

extension on Sanitizer {
  String? get rustflags {
    final value = rustflagValue;
    if (value == null) return null;

    return switch (this) {
      Sanitizer.msan => '-Zsanitizer=$value --cfg frb_sanitize_memory',
      _ => '-Zsanitizer=$value',
    };
  }

  String? get rustflagValue {
    return switch (this) {
      Sanitizer.asan => 'address',
      Sanitizer.msan => 'memory',
      Sanitizer.lsan => 'leak',
      Sanitizer.tsan => 'thread',
      Sanitizer.ubsan => null,
    };
  }

  String get dartSdkBuildOutDir {
    return switch (this) {
      Sanitizer.asan => 'ReleaseASANX64',
      Sanitizer.msan => 'ReleaseMSANX64',
      Sanitizer.lsan => 'ReleaseLSANX64',
      Sanitizer.tsan => 'ReleaseTSANX64',
      Sanitizer.ubsan => 'ReleaseUBSANX64',
    };
  }

  Map<String, String> get runtimeEnv {
    return switch (this) {
      Sanitizer.asan => {'ASAN_OPTIONS': _kAsanOptions},
      Sanitizer.msan => {'MSAN_OPTIONS': _kMsanOptions},
      Sanitizer.lsan => {'ASAN_OPTIONS': _kLsanOptions},
      Sanitizer.tsan => {'TSAN_OPTIONS': _kTsanOptions},
      Sanitizer.ubsan => {'UBSAN_OPTIONS': _kUbsanOptions},
    };
  }

  Map<String, String> get threadPoolEnv {
    return switch (this) {
      Sanitizer.msan || Sanitizer.tsan => {'FRB_SYNC_THREAD_POOL': '1'},
      _ => {},
    };
  }
}

const _kAsanOptions =
    'handle_segv=0:detect_leaks=1:detect_stack_use_after_return=0:'
    'disable_coredump=0:abort_on_error=1';
const _kMsanOptions =
    'handle_segv=0:detect_leaks=1:detect_stack_use_after_return=0:'
    'disable_coredump=0:abort_on_error=1:strict_memcmp=0';
const _kLsanOptions =
    'handle_segv=0:detect_leaks=1:detect_stack_use_after_return=0:'
    'disable_coredump=0:abort_on_error=1';
const _kTsanOptions =
    'handle_segv=0:disable_coredump=0:abort_on_error=1:'
    'report_thread_leaks=0';
const _kUbsanOptions =
    'handle_segv=0:disable_coredump=0:abort_on_error=1:print_stacktrace=1';
