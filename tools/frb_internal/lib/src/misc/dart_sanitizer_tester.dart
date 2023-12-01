import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';

Future<void> run({required String package}) async {
  if (package == 'frb_example/deliberate_bad') {
    await _runPackageDeliberateBad(package: package);
  } else {
    await _runEntrypoint(package: package);
  }
}

Future<void> _runEntrypoint({required String package}) async {
  throw Exception('TODO');
}

Future<void> _runPackageDeliberateBad({required String package}) async {
  await _runPackageDeliberateBadRustOnly(package: package);
  await _runPackageDeliberateBadDartOnly(package: package);
  await _runPackageDeliberateBadDartCallRust(package: package);
}

Future<void> _runPackageDeliberateBadRustOnly({required String package}) async {
  const kInfos = [
    _Info(
      name: 'RustOnly_Good',
      expectSucceed: true,
      expectOutputContains: '',
    ),
    _Info(
      name: 'RustOnly_StackBufferOverflow',
      expectSucceed: false,
      expectOutputContains: 'ERROR: AddressSanitizer: stack-buffer-overflow',
    ),
    _Info(
      name: 'RustOnly_HeapUseAfterFree',
      expectSucceed: false,
      expectOutputContains: 'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ];

  for (final info in kInfos) {
    await _execAndCheckWithAsanEnvVar(
      'cargo +nightly run ${_CargoBuildAsanInfo.kExtraArgs.join(" ")} ${info.name}',
      info,
      extraEnv: _CargoBuildAsanInfo.kExtraEnv,
      relativePwd: '$package/rust',
    );
  }
}

Future<void> _runPackageDeliberateBadDartOnly({required String package}) async {
  const kInfos = [
    _Info(
      name: 'DartOnly_Good',
      expectSucceed: true,
      expectOutputContains: '',
    ),
    // NOTE ASAN does not report this as buggy...
    _Info(
      name: 'DartOnly_HeapUseAfterFree',
      expectSucceed: true,
      expectOutputContains: '',
    ),
  ];

  for (final info in kInfos) {
    await _execAndCheckWithAsanEnvVar(
      '$sanitizedDart --enable-experiment=native-assets run '
      'frb_example_deliberate_bad ${info.name}',
      info,
      relativePwd: package,
    );
  }
}

Future<void> _runPackageDeliberateBadDartCallRust(
    {required String package}) async {
  const kInfos = [
    // NOTE It should fail, but ASAN did not realize this case...
    _Info(
      name: 'DartCallRust_StackBufferOverflow',
      expectSucceed: true,
      expectOutputContains: '',
    ),
    // ASAN successfully understand this case
    _Info(
      name: 'DartCallRust_HeapUseAfterFree',
      expectSucceed: false,
      expectOutputContains: 'ERROR: AddressSanitizer: heap-use-after-free',
    ),
  ];

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
  final String expectOutputContains;

  const _Info({
    required this.name,
    required this.expectSucceed,
    required this.expectOutputContains,
  });
}

Future<void> _execAndCheckWithAsanEnvVar(
  String cmd,
  _Info info, {
  required String relativePwd,
  Map<String, String>? extraEnv,
}) async {
  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: {
      ...?extraEnv,
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

  if (!output.stdout.contains(info.expectOutputContains)) {
    throw Exception(
        'Bad stdout which does not contain `${info.expectOutputContains}`');
  }
}

// TODO do not hardcode
const sanitizedDart =
    '/home/cs144/dart-sdk/sdk/out/ReleaseASANX64/dart-sdk/bin/dart';

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
