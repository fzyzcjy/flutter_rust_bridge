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
    'runCommand can remove selected parent environment keys',
    skip: !(Platform.isLinux || Platform.isMacOS),
    () async {
      final output = await runCommand(
        '/usr/bin/env',
        [],
        shell: false,
        silent: true,
        removedParentEnvKeys: const ['PATH'],
      );

      expect(output.stdout.split('\n'), isNot(contains(startsWith('PATH='))));
    },
  );

  test(
    'runCommand removes selected parent environment keys case-insensitively',
    skip: !(Platform.isLinux || Platform.isMacOS),
    () async {
      final output = await runCommand(
        '/usr/bin/env',
        [],
        shell: false,
        silent: true,
        removedParentEnvKeys: const ['path'],
      );

      expect(output.stdout.split('\n'), isNot(contains(startsWith('PATH='))));
    },
  );

  test(
    'runCommand kills the process tree when timeout expires',
    skip: !(Platform.isLinux || Platform.isMacOS),
    () async {
      final tempDir = await Directory.systemTemp.createTemp(
        'frb_run_command_test_',
      );
      final pidFile = File('${tempDir.path}/child.pid');

      try {
        final command =
            'sleep 30 & echo \$! > ${_shellQuote(pidFile.path)}; wait';

        await expectLater(
          () async => await runCommand(
            '/bin/sh',
            ['-c', command],
            shell: false,
            silent: true,
            timeout: const Duration(seconds: 2),
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

  test(
    'runCommand kills nested child processes when timeout expires',
    skip: !(Platform.isLinux || Platform.isMacOS),
    () async {
      final tempDir = await Directory.systemTemp.createTemp(
        'frb_run_command_nested_test_',
      );
      final childPidFile = File('${tempDir.path}/child.pid');
      final grandchildPidFile = File('${tempDir.path}/grandchild.pid');

      try {
        final nestedCommand =
            'sleep 30 & echo \$! > ${_shellQuote(grandchildPidFile.path)}; wait';
        final command =
            '/bin/sh -c ${_shellQuote(nestedCommand)} & echo \$! > ${_shellQuote(childPidFile.path)}; wait';

        await expectLater(
          () async => await runCommand(
            '/bin/sh',
            ['-c', command],
            shell: false,
            silent: true,
            timeout: const Duration(seconds: 2),
          ),
          throwsA(isA<TimeoutException>()),
        );

        final childPid = int.parse(await childPidFile.readAsString());
        final grandchildPid = int.parse(await grandchildPidFile.readAsString());
        await expectLater(_waitUntilProcessExits(childPid), completion(isTrue));
        await expectLater(
          _waitUntilProcessExits(grandchildPid),
          completion(isTrue),
        );
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
