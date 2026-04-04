import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/precommit_autofix.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  group('PrecommitAutofixService', () {
    test('clean repo exits successfully and reports no patch', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {},
      );

      expect(result.hasPatch, isFalse);
      expect(File(outputPath).existsSync(), isFalse);
      expect(result.summary.status, PrecommitAutofixStatus.clean);
      expect(result.summary.message, 'No changes were produced.');
      expect(result.summary.artifactName, 'precommit-autofix-diff');
      expect(
        result.summary.toJsonString(),
        contains('"status":"clean"'),
      );
    });

    test('modified tracked file emits non-empty diff', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {
          File(path.join(repo.path, 'tracked.txt')).writeAsStringSync(
            'hello world\n',
          );
        },
      );

      final patchText = File(outputPath).readAsStringSync();

      expect(result.hasPatch, isTrue);
      expect(patchText, isNotEmpty);
      expect(patchText, contains('tracked.txt'));
      expect(result.summary.status, PrecommitAutofixStatus.patched);
      expect(result.summary.applyCommand, 'git apply precommit.diff');
    });

    test('newly created file diff contains the file', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {
          File(path.join(repo.path, 'generated', 'new_file.txt'))
              .createSync(recursive: true);
          File(path.join(repo.path, 'generated', 'new_file.txt'))
              .writeAsStringSync('new content\n');
        },
      );

      final patchText = File(outputPath).readAsStringSync();

      expect(result.hasPatch, isTrue);
      expect(patchText, contains('generated/new_file.txt'));
      expect(patchText, contains('new file mode'));
    });

    test('deleted file diff records deletion', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {
          File(path.join(repo.path, 'tracked.txt')).deleteSync();
        },
      );

      final patchText = File(outputPath).readAsStringSync();

      expect(result.hasPatch, isTrue);
      expect(patchText, contains('deleted file mode'));
      expect(patchText, contains('tracked.txt'));
    });

    test('excluded integrate drift does not emit a patch', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final excludedPath = path.join(
        repo.path,
        'frb_example',
        'flutter_via_create',
        'macos',
        'Flutter',
        'Flutter-Debug.xcconfig',
      );
      File(excludedPath).createSync(recursive: true);

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {
          File(excludedPath).writeAsStringSync(
            '#include "ephemeral/Flutter-Generated.xcconfig"\n',
          );
        },
      );

      expect(result.hasPatch, isFalse);
      expect(File(outputPath).existsSync(), isFalse);
      expect(result.summary.status, PrecommitAutofixStatus.clean);
    });

    test('excluded integrate drift is omitted while real changes remain', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      final excludedPath = path.join(
        repo.path,
        'frb_example',
        'flutter_via_create',
        'macos',
        'Flutter',
        'Flutter-Debug.xcconfig',
      );
      File(excludedPath).createSync(recursive: true);

      final outputPath = path.join(repo.path, 'artifacts', 'precommit.diff');
      final result = await _runService(
        repo: repo,
        outputPath: outputPath,
        precommitRunner: (_) async {
          File(
            path.join(repo.path, 'tracked.txt'),
          ).writeAsStringSync('hello world\n');
          File(excludedPath).writeAsStringSync(
            '#include "ephemeral/Flutter-Generated.xcconfig"\n',
          );
        },
      );

      final patchText = File(outputPath).readAsStringSync();

      expect(result.hasPatch, isTrue);
      expect(patchText, contains('tracked.txt'));
      expect(patchText, isNot(contains('Flutter-Debug.xcconfig')));
    });

    test('dirty-before-run repo fails with actionable error', () async {
      final repo = await _createCommittedRepo(
        fileName: 'tracked.txt',
        fileContents: 'hello\n',
      );
      addTearDown(() => repo.delete(recursive: true));

      File(path.join(repo.path, 'tracked.txt')).writeAsStringSync(
        'dirty before run\n',
      );

      var precommitInvoked = false;

      await expectLater(
        _runService(
          repo: repo,
          outputPath: path.join(repo.path, 'artifacts', 'precommit.diff'),
          precommitRunner: (_) async {
            precommitInvoked = true;
          },
        ),
        throwsA(
          isA<StateError>().having(
            (error) => error.message,
            'message',
            contains('Repository must be clean before running precommit-autofix.'),
          ),
        ),
      );

      expect(precommitInvoked, isFalse);
    });
  });

  test('parsePrecommitAutofixMode accepts fast and slow', () {
    expect(parsePrecommitAutofixMode('fast'), PrecommitAutofixMode.fast);
    expect(parsePrecommitAutofixMode('slow'), PrecommitAutofixMode.slow);
  });

  test('hasPrecommitAutofixPatch detects blank and non-blank diff text', () {
    expect(hasPrecommitAutofixPatch(''), isFalse);
    expect(hasPrecommitAutofixPatch('diff --git a/a b/a\n'), isTrue);
  });

  test('renderPrecommitAutofixSummary includes artifact and apply command', () {
    final summary = renderPrecommitAutofixSummary(
      status: PrecommitAutofixStatus.patched,
      outputPath: '/tmp/precommit-autofix.diff',
    );

    expect(summary, contains('"artifact_name":"precommit-autofix-diff"'));
    expect(summary, contains('"apply_command":"git apply precommit-autofix.diff"'));
    expect(summary, contains('"status":"patched"'));
  });
}

Future<Directory> _createCommittedRepo({
  required String fileName,
  required String fileContents,
}) async {
  final repo = Directory.systemTemp.createTempSync('frb-precommit-autofix-');

  await runCommand('git', ['init'], pwd: repo.path);
  await runCommand(
    'git',
    ['config', 'user.email', 'test@example.com'],
    pwd: repo.path,
  );
  await runCommand(
    'git',
    ['config', 'user.name', 'Test User'],
    pwd: repo.path,
  );

  final file = File(path.join(repo.path, fileName));
  file.parent.createSync(recursive: true);
  file.writeAsStringSync(fileContents);

  await runCommand('git', ['add', '.'], pwd: repo.path);
  await runCommand('git', ['commit', '-m', 'initial'], pwd: repo.path);

  return repo;
}

Future<PrecommitAutofixRunResult> _runService({
  required Directory repo,
  required String outputPath,
  required PrecommitAutofixRunner precommitRunner,
}) async {
  return PrecommitAutofixService(
    commandRunner: SimpleExecutor(pwd: repo.path).call,
    precommitRunner: precommitRunner,
    repoRootPath: repo.path,
  ).run(
    mode: PrecommitAutofixMode.slow,
    outputPath: outputPath,
  );
}
