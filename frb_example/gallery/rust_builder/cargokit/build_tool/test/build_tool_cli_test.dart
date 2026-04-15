import 'dart:io';

import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  group('build_tool CLI', () {
    test('gen-key prints a private and public key', () async {
      final result = await _runCli(['run', 'bin/build_tool.dart', 'gen-key']);

      expect(result.exitCode, 0, reason: _describeResult(result));
      expect(result.stdout, contains('Private Key: '));
      expect(result.stdout, contains('Public Key: '));
    });

    test('verify-binaries succeeds for crates without precompiled config',
        () async {
      final tempDir = Directory.systemTemp.createTempSync('cli_manifest_');
      addTearDown(() => tempDir.deleteSync(recursive: true));

      File(path.join(tempDir.path, 'Cargo.toml')).writeAsStringSync('''
[package]
name = "demo"
version = "0.1.0"
edition = "2021"
''');

      final result = await _runCli([
        'run',
        'bin/build_tool.dart',
        'verify-binaries',
        '--manifest-dir=${tempDir.path}',
      ]);

      expect(result.exitCode, 0, reason: _describeResult(result));
      expect(result.stdout,
          contains('Crate does not support precompiled binaries.'));
    });

    test('build-cmake reports missing environment variables clearly', () async {
      final result = await _runCli(
        ['run', 'bin/build_tool.dart', 'build-cmake'],
      );

      expect(result.exitCode, 1, reason: _describeResult(result));
      expect(
        result.stderr,
        contains(
            'Missing required environment variable "CARGOKIT_ROOT_PROJECT_DIR"'),
      );
    });
  });
}

Future<ProcessResult> _runCli(List<String> args) {
  final dartExecutable = _dartExecutable();
  return Process.run(
    '/bin/zsh',
    [
      '-lc',
      [
        _shellQuote(dartExecutable),
        ...args.map(_shellQuote),
      ].join(' '),
    ],
    workingDirectory: Directory.current.path,
  );
}

String _shellQuote(String value) {
  return "'${value.replaceAll("'", "'\"'\"'")}'";
}

String _describeResult(ProcessResult result) {
  return 'stdout:\n${result.stdout}\n\nstderr:\n${result.stderr}';
}

String _dartExecutable() {
  final flutterRoot = Platform.environment['FLUTTER_ROOT'];
  if (flutterRoot != null) {
    return path.join(flutterRoot, 'bin', 'dart');
  }

  if (path.basename(Platform.resolvedExecutable) == 'flutter_tester') {
    final flutterRootFromTester = path.dirname(
      path.dirname(
        path.dirname(
          path.dirname(
            path.dirname(
              path.dirname(Platform.resolvedExecutable),
            ),
          ),
        ),
      ),
    );
    return path.join(flutterRootFromTester, 'bin', 'dart');
  }

  return Platform.resolvedExecutable;
}
