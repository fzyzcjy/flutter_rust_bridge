// ignore_for_file: avoid_print

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';

Future<void> run({required String package}) async {
  await runPubGetIfNotRunYet(package);

  if (package == 'frb_example/deliberate_bad') {
    await _runPackageDeliberateBad(package: package);
  } else {
    await _runEntrypoint(package: package);
  }
}

Future<void> _runEntrypoint({required String package}) async {
  final sanitizedDart = await _getSanitizedDartBinary();
  await _execAndCheckWithAsanEnvVar(
    '$sanitizedDart --enable-experiment=native-assets run test/dart_valgrind_test_entrypoint.dart',
    const _Info(
        name: 'entrypoint', expectSucceed: true, expectStderrContains: ''),
    relativePwd: package,
  );
}

Future<void> _runPackageDeliberateBad({required String package}) async {
  await _runPackageDeliberateBadRustOnly(package: package);
  await _runPackageDeliberateBadWithDart(package: package);
}

Future<void> _runPackageDeliberateBadRustOnly({required String package}) async {
  const kInfos = [
    _Info(
      name: 'RustOnly_Good',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    _Info(
      name: 'RustOnly_StackBufferOverflow',
      expectSucceed: false,
      expectStderrContains: 'ERROR: AddressSanitizer: stack-buffer-overflow',
    ),
    _Info(
      name: 'RustOnly_HeapUseAfterFree',
      expectSucceed: false,
      expectStderrContains: 'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ];

  for (final info in kInfos) {
    await _execAndCheckWithAsanEnvVar(
      'cargo +nightly run ${_CargoBuildAsanInfo.kExtraArgs.join(" ")} ${info.name}',
      info,
      relativePwd: '$package/rust',
    );
  }
}

Future<void> _runPackageDeliberateBadWithDart({required String package}) async {
  const kInfos = [
    _Info(
      name: 'DartOnly_Good',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    // NOTE ASAN does not report this as buggy...
    _Info(
      name: 'DartOnly_HeapUseAfterFree',
      expectSucceed: true,
      expectStderrContains: '',
    ),

    // NOTE It should fail, but ASAN did not realize this case...
    _Info(
      name: 'DartCallRust_StackBufferOverflow',
      expectSucceed: true,
      expectStderrContains: '',
    ),
    // ASAN successfully understand this case
    _Info(
      name: 'DartCallRust_HeapUseAfterFree',
      expectSucceed: false,
      expectStderrContains: 'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ];

  final sanitizedDart = await _getSanitizedDartBinary();
  for (final info in kInfos) {
    await _execAndCheckWithAsanEnvVar(
      '$sanitizedDart --enable-experiment=native-assets run '
      'frb_example_deliberate_bad ${info.name}',
      info,
      relativePwd: package,
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

Future<void> _execAndCheckWithAsanEnvVar(
  String cmd,
  _Info info, {
  required String relativePwd,
}) async {
  print('====== execAndCheckWithAsanEnvVar name=${info.name} ======');

  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: {
      ..._CargoBuildAsanInfo.kExtraEnv,
      'FRB_SIMPLE_BUILD_CARGO_NIGHTLY': '1',
      'FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS':
          _CargoBuildAsanInfo.kExtraArgs.join(' '),
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

Future<String> _getSanitizedDartBinary() async {
  // TODO do not hardcode
  return '/home/cs144/dart-sdk/sdk/out/ReleaseASANX64/dart-sdk/bin/dart';
}

class _CargoBuildAsanInfo {
  static const kExtraArgs = [
    '-Zbuild-std',
    '--target',
    'x86_64-unknown-linux-gnu'
  ];

  static const kExtraEnv = {
    'RUSTFLAGS': '-Zsanitizer=address',
  };
}
