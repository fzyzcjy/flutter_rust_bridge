@TestOn('vm')
import 'dart:async';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:test/test.dart';

void main() {
  test('runCommand', skip: !(Platform.isLinux || Platform.isMacOS), () async {
    expect((await runCommand('ls', [])).exitCode, 0);

    await expectLater(
      () async => await runCommand('ls', ['/deliberately-not-exist-path']),
      throwsA(isA<ProcessException>()),
    );
  });

  test('runCommand kills the process when timeout expires', () async {
    final tempDir = await Directory.systemTemp.createTemp(
      'frb_run_command_test_',
    );
    final script = File('${tempDir.path}/hang.dart');

    try {
      await script.writeAsString(
        'Future<void> main() async { await Future<void>.delayed(const Duration(seconds: 30)); }\n',
      );

      await expectLater(
        () async => await runCommand(
          Platform.resolvedExecutable,
          [script.path],
          silent: true,
          timeout: const Duration(milliseconds: 10),
        ),
        throwsA(isA<TimeoutException>()),
      );
    } finally {
      await tempDir.delete(recursive: true);
    }
  });
}
