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

  test(
    'runCommand kills the process tree when timeout expires',
    skip: !(Platform.isLinux || Platform.isMacOS),
    () async {
      final tempDir = await Directory.systemTemp.createTemp(
        'frb_run_command_test_',
      );
      final script = File('${tempDir.path}/hang.dart');
      final pidFile = File('${tempDir.path}/child.pid');

      try {
        await script.writeAsString(
          [
            'Future<void> main() async {',
            '  await Future<void>.delayed(const Duration(seconds: 30));',
            '}',
          ].join('\n'),
        );
        final command =
            '${_shellQuote(Platform.resolvedExecutable)} ${_shellQuote(script.path)} '
            '& echo \$! > ${_shellQuote(pidFile.path)}; wait';

        await expectLater(
          () async => await runCommand(
            '/bin/sh',
            ['-c', command],
            shell: false,
            silent: true,
            timeout: const Duration(milliseconds: 100),
          ),
          throwsA(isA<TimeoutException>()),
        );

        final childPid = int.parse(await pidFile.readAsString());
        await expectLater(_waitUntilProcessExits(childPid), completion(isTrue));
      } finally {
        await tempDir.delete(recursive: true);
      }
    },
  );
}

Future<bool> _processExists(int pid) async {
  final psResult = await Process.run('ps', ['-o', 'stat=', '-p', '$pid']);
  if (psResult.exitCode != 0) return false;
  final state = (psResult.stdout as String).trim();
  if (state.startsWith('Z')) return false;

  final killResult = await Process.run('kill', ['-0', '$pid']);
  return killResult.exitCode == 0;
}

Future<bool> _waitUntilProcessExits(int pid) async {
  for (var i = 0; i < 20; ++i) {
    if (!await _processExists(pid)) return true;
    await Future<void>.delayed(const Duration(milliseconds: 50));
  }
  return false;
}

String _shellQuote(String value) => "'${value.replaceAll("'", r"'\''")}'";
