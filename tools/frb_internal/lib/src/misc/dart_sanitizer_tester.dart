import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';

Future<void> run({required String name}) async {
  if (name.startsWith('RustOnly_')) {
    await _execAndCheck(
      'cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu $name',
      extraEnv: {'RUSTFLAGS': '-Zsanitizer=address'},
    );
  } else {
    throw UnimplementedError('TODO');
  }
}

class _ExpectInfo {
  final bool expectSucceed;
  final String expectOutputContains;

  const _ExpectInfo({
    required this.expectSucceed,
    required this.expectOutputContains,
  });
}

Future<void> _execAndCheck(
  String cmd,
  _ExpectInfo info, {
  Map<String, String>? extraEnv,
}) async {
  final output = await exec(
    cmd,
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
