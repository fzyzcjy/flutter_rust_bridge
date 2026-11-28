@TestOn('vm && !windows')
library;

import 'dart:convert';
import 'dart:io';

import 'package:test/test.dart';

void main() {
  test('code generation keeps the outputs of other builders', () async {
    final fixture = await _Fixture.prepare();
    addTearDown(fixture.dispose);

    await fixture.checkWarmBuild();
    await fixture.checkColdBuild();
    await fixture.checkFilteredBuild();
  }, timeout: const Timeout(Duration(minutes: 15)));
}

/// Runs code generation against a package that mixes FRB outputs with the
/// outputs of other builders.
class _Fixture {
  final Directory _temp;
  final String _repoRoot;
  final Map<String, String> _environment;
  final File _unrelated;
  final File _custom;
  final File _binary;
  final File _plain;
  final File _freezed;
  final File _config;
  late final String _originalUnrelated;
  late final String _originalCustom;
  late final List<int> _originalBinary;
  late final String _originalPlain;

  _Fixture._(this._temp, this._repoRoot)
    : _environment = {
        'PATH': '${_binaryDir(_repoRoot)}:${Platform.environment['PATH']}',
        'FRB_SIMPLE_BUILD_SKIP': '1',
      },
      _unrelated = File('${_temp.path}/lib/unrelated_model.g.dart'),
      _custom = File('${_temp.path}/test/other.custom.dart'),
      _binary = File('${_temp.path}/test/other.bin'),
      _plain = File('${_temp.path}/test/other.dart'),
      _freezed = File('${_temp.path}/lib/src/rust/api.freezed.dart'),
      _config = File('${_temp.path}/flutter_rust_bridge.yaml');

  /// Copies the fixture into a temporary package and generates once.
  static Future<_Fixture> prepare() async {
    final repoRoot = Directory.current.parent.parent.path;
    final temp = await Directory.systemTemp.createTemp('frb_build_runner_');
    _copyFixture(temp);
    _rewriteFrbdartPath(temp: temp, repoRoot: repoRoot);

    final fixture = _Fixture._(temp, repoRoot);
    await fixture._buildCodegen();
    await fixture._pubGet();
    await fixture._checkInitialBuild();
    return fixture;
  }

  void dispose() => _temp.deleteSync(recursive: true);

  /// Keeps the outputs of the other builders when the asset graph is warm.
  Future<void> checkWarmBuild() async {
    final output = await _generate();
    _expectUnrelatedOutputsKept(output);
    expect(_freezed.existsSync(), isTrue, reason: output);
  }

  /// Keeps the outputs of the other builders when the asset graph is cold.
  Future<void> checkColdBuild() async {
    Directory('${_temp.path}/.dart_tool/build').deleteSync(recursive: true);
    _freezed.deleteSync();
    final output = await _generate();
    _expectUnrelatedOutputsKept(output);
    expect(_freezed.existsSync(), isTrue, reason: output);
  }

  /// Scopes the build to the FRB outputs once the flag is enabled.
  Future<void> checkFilteredBuild() async {
    _config.writeAsStringSync(
      _config.readAsStringSync().replaceFirst(
        'build_runner: true',
        'build_runner: true\nenable_build_runner_filter: true',
      ),
    );
    _unrelated.deleteSync();
    _freezed.deleteSync();
    final output = await _generate();
    expect(_unrelated.existsSync(), isFalse, reason: output);
    expect(_freezed.existsSync(), isTrue, reason: output);
    expect(_custom.readAsStringSync(), _originalCustom);
    expect(_binary.readAsBytesSync(), _originalBinary);
    expect(_plain.readAsStringSync(), _originalPlain);
  }

  Future<void> _buildCodegen() => _run('cargo', [
    'build',
    '--manifest-path',
    '$_repoRoot/frb_codegen/Cargo.toml',
    '--package',
    'flutter_rust_bridge_codegen',
    '--bin',
    'flutter_rust_bridge_codegen',
  ], workingDirectory: _repoRoot);

  Future<void> _pubGet() async {
    await _run('dart', ['pub', 'get']);
  }

  Future<void> _checkInitialBuild() async {
    final output = await _generate();
    for (final file in [_unrelated, _custom, _binary, _plain]) {
      expect(
        file.existsSync(),
        isTrue,
        reason:
            'FRB generation did not generate an unrelated builder output '
            '$file\n$output',
      );
    }
    expect(_freezed.existsSync(), isTrue, reason: output);

    _originalUnrelated = _unrelated.readAsStringSync();
    _originalCustom = _custom.readAsStringSync();
    _originalBinary = _binary.readAsBytesSync();
    _originalPlain = _plain.readAsStringSync();
    expect(_originalCustom, 'const customValue = 42;\n');
    expect(_originalBinary, [0, 255, 10]);
    expect(_originalPlain, 'const plainValue = 7;\n');
    await _run('dart', ['run', 'bin/check.dart']);
  }

  void _expectUnrelatedOutputsKept(String reason) {
    for (final file in [_unrelated, _custom, _binary, _plain]) {
      expect(
        file.existsSync(),
        isTrue,
        reason:
            'FRB generation deleted an unrelated builder output $file\n$reason',
      );
    }
    expect(_unrelated.readAsStringSync(), _originalUnrelated);
    expect(_custom.readAsStringSync(), _originalCustom);
    expect(_binary.readAsBytesSync(), _originalBinary);
    expect(_plain.readAsStringSync(), _originalPlain);
  }

  Future<String> _generate() =>
      _run('flutter_rust_bridge_codegen', ['generate']);

  Future<String> _run(
    String executable,
    List<String> arguments, {
    String? workingDirectory,
  }) async {
    final result = await Process.run(
      executable,
      arguments,
      workingDirectory: workingDirectory ?? _temp.path,
      environment: _environment,
    );
    expect(
      result.exitCode,
      0,
      reason: '$executable $arguments\n${result.stdout}\n${result.stderr}',
    );
    return '${result.stdout}\n${result.stderr}';
  }
}

String _binaryDir(String repoRoot) {
  final cargoTargetDir = Platform.environment['CARGO_TARGET_DIR'] ?? 'target';
  final targetDir = cargoTargetDir.startsWith('/')
      ? cargoTargetDir
      : '$repoRoot/$cargoTargetDir';
  return '$targetDir/debug';
}

void _copyFixture(Directory temp) {
  final fixture = Directory(
    'integration_test/fixtures/build_runner_preservation',
  );
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
}

void _rewriteFrbdartPath({required Directory temp, required String repoRoot}) {
  final pubspec = File('${temp.path}/pubspec.yaml');
  pubspec.writeAsStringSync(
    pubspec.readAsStringSync().replaceFirst(
      '"../../../../../frb_dart"',
      jsonEncode('$repoRoot/frb_dart'),
    ),
  );
}
