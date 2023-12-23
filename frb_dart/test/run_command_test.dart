@TestOn('vm')
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
}
