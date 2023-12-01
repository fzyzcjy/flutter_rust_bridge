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
  throw Exception('TODO');
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
    await _execAndCheck(
      'cargo +nightly run ${_CargoBuildAsanInfo.kExtraArgs.join(" ")} $name',
      extraEnv: _CargoBuildAsanInfo.kExtraEnv,
      relativePwd: 'rust',
      expectSucceed: expectSucceed,
      expectStderrContains: expectStderrContains,
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

Future<void> _execAndCheck(
  String cmd,
  _Info info, {
  required String relativePwd,
  Map<String, String>? extraEnv,
}) async {
  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: extraEnv,
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
