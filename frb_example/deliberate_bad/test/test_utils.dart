import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:test/test.dart';

final exec = SimpleExecutor(env: {'CARGO_TERM_COLOR': 'always'});

Future<void> execAndCheck(
  String cmd, {
  String? relativePwd,
  Map<String, String>? extraEnv,
  required bool expectSucceed,
  required String expectStderrContains,
}) async {
  final output = await exec(
    cmd,
    relativePwd: relativePwd,
    extraEnv: extraEnv,
    checkExitCode: false,
  );

  expect(output.exitCode, expectSucceed ? 0 : isNot(0));
  expect(output.stderr, contains(expectStderrContains));
}

// TODO do not hardcode
const sanitizedDart =
    '/home/cs144/dart-sdk/sdk/out/ReleaseASANX64/dart-sdk/bin/dart';
