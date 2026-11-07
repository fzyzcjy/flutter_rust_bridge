@TestOn('vm && !windows')
library;

import 'dart:convert';
import 'dart:io';

import 'package:test/test.dart';

void main() {
  test(
    'code generation rebuilds existing outputs excluded by its filters',
    () async {
      final repoRoot = Directory.current.parent.parent.path;
      final cargoTargetDir =
          Platform.environment['CARGO_TARGET_DIR'] ?? 'target';
      final targetDir = cargoTargetDir.startsWith('/')
          ? cargoTargetDir
          : '$repoRoot/$cargoTargetDir';
      final binaryDir = '$targetDir/debug';
      final temp = await Directory.systemTemp.createTemp('frb_build_runner_');
      addTearDown(() => temp.delete(recursive: true));

      final fixture = Directory('test/fixtures/build_runner_preservation');
      for (final entry in fixture.listSync(recursive: true)) {
        final relative = entry.path.substring(fixture.path.length + 1);
        final destination =
            '${temp.path}/${relative.replaceFirst(RegExp(r'\.template$'), '')}';
        if (entry is Directory) {
          Directory(destination).createSync(recursive: true);
        } else if (entry is File) {
          File(destination).parent.createSync(recursive: true);
          entry.copySync(destination);
        }
      }
      final pubspec = File('${temp.path}/pubspec.yaml');
      pubspec.writeAsStringSync(
        pubspec.readAsStringSync().replaceFirst(
          '"../../../../../frb_dart"',
          jsonEncode('$repoRoot/frb_dart'),
        ),
      );

      final environment = {
        'PATH': '$binaryDir:${Platform.environment['PATH']}',
        'FRB_SIMPLE_BUILD_SKIP': '1',
      };
      Future<String> run(
        String executable,
        List<String> arguments, {
        String? workingDirectory,
      }) async {
        final result = await Process.run(
          executable,
          arguments,
          workingDirectory: workingDirectory ?? temp.path,
          environment: environment,
        );
        expect(
          result.exitCode,
          0,
          reason: '$executable $arguments\n${result.stdout}\n${result.stderr}',
        );
        return '${result.stdout}\n${result.stderr}';
      }

      await run('cargo', [
        'build',
        '--manifest-path',
        '$repoRoot/frb_codegen/Cargo.toml',
        '--package',
        'flutter_rust_bridge_codegen',
        '--bin',
        'flutter_rust_bridge_codegen',
      ], workingDirectory: repoRoot);
      await run('dart', ['pub', 'get']);

      // This setup runs the workaround posted in issue #3478.
      await run('bash', [
        '${temp.path}/bin/frb-generate',
      ], workingDirectory: repoRoot);
      final unrelated = File('${temp.path}/lib/unrelated_model.g.dart');
      final custom = File('${temp.path}/test/other.custom.dart');
      final binary = File('${temp.path}/test/other.bin');
      final freezed = File('${temp.path}/lib/src/rust/api.freezed.dart');
      expect(unrelated.existsSync(), isTrue);
      expect(freezed.existsSync(), isTrue);
      final original = unrelated.readAsStringSync();
      final originalCustom = custom.readAsStringSync();
      final originalBinary = binary.readAsBytesSync();
      expect(originalBinary, [0, 255, 10]);
      binary.deleteSync();
      expect(originalCustom, 'const customValue = 42;\n');

      final config = File('${temp.path}/flutter_rust_bridge.yaml');
      config.writeAsStringSync(
        config.readAsStringSync().replaceFirst(
          'build_runner: false',
          'build_runner: true',
        ),
      );
      File('${temp.path}/test/.test_config.dart')
          .writeAsStringSync('// config');
      final buildCache = Directory('${temp.path}/.dart_tool/build');
      buildCache.deleteSync(recursive: true);
      freezed.deleteSync();

      final coldOutput = await run('flutter_rust_bridge_codegen', ['generate']);
      expect(
        unrelated.existsSync(),
        isTrue,
        reason: 'FRB generation deleted an unrelated builder output',
      );
      expect(unrelated.readAsStringSync(), original);
      expect(custom.readAsStringSync(), originalCustom);
      expect(binary.existsSync(), isFalse);
      expect(freezed.existsSync(), isTrue, reason: coldOutput);
      await run('dart', ['run', 'bin/check.dart']);

      // A warm graph must also leave the unrelated output available.
      await run('flutter_rust_bridge_codegen', ['generate']);
      expect(unrelated.readAsStringSync(), original);
      expect(custom.readAsStringSync(), originalCustom);
      expect(binary.existsSync(), isFalse);

      // A custom Dart output must not pull in the unrelated JSON builder.
      unrelated.deleteSync();
      buildCache.deleteSync(recursive: true);
      freezed.deleteSync();
      final filteredOutput = await run('flutter_rust_bridge_codegen', [
        'generate',
      ]);
      expect(unrelated.existsSync(), isFalse, reason: filteredOutput);
      expect(custom.readAsStringSync(), originalCustom);
      expect(binary.existsSync(), isFalse);
      expect(freezed.existsSync(), isTrue, reason: filteredOutput);

      // Without existing unrelated outputs, keep the filtered build.
      custom.deleteSync();
      buildCache.deleteSync(recursive: true);
      freezed.deleteSync();
      final frbOnlyOutput = await run('flutter_rust_bridge_codegen', [
        'generate',
      ]);
      expect(unrelated.existsSync(), isFalse, reason: frbOnlyOutput);
      expect(custom.existsSync(), isFalse, reason: frbOnlyOutput);
      expect(binary.existsSync(), isFalse, reason: frbOnlyOutput);
      expect(freezed.existsSync(), isTrue, reason: frbOnlyOutput);
    },
    timeout: const Timeout(Duration(minutes: 10)),
  );
}
