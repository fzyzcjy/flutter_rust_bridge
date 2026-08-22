import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:path/path.dart' as path;

const _kBuildCliDependency = '  build_cli: ^2.2.5';
const _kDisabledBuildCliDependency =
    '  # Temporarily remove before https://github.com/kevmoo/build_cli/issues/168 is fixed\n'
    '  # build_cli: ^2.2.5';
const _kBuildCliPackages = ['frb_dart', 'frb_utils'];

Future<void> generateRunFrbCodegenCommandGenerateFromScratch() async {
  await wrapMaybeSetExitIfChangedRaw(true, () async {
    await _withBuildCliEnabled(() async {
      final expectedGeneratedFiles =
          await deleteTrackedGeneratedFilesForFromScratch();
      await _prepareFromScratchCodegenDependencies();
      const generateConfig = GenerateConfig(
        setExitIfChanged: false,
        coverage: false,
      );
      await generateInternalRust(generateConfig);
      await generateInternalBuildRunner(generateConfig);
      for (final package in [
        'frb_example/pure_dart',
        'frb_example/pure_dart_pde',
      ]) {
        await generateRunFrbCodegenCommandGenerate(
          GeneratePackageConfig(
            setExitIfChanged: false,
            package: package,
            coverage: false,
          ),
        );
      }
      await generateInternal(generateConfig);
      await precommitGenerate();
      verifyTrackedGeneratedFilesRestored(expectedGeneratedFiles);
    });
  });
}

Future<void> _withBuildCliEnabled(Future<void> Function() action) async {
  _setBuildCliEnabled(enabled: true);
  try {
    await action();
  } finally {
    _setBuildCliEnabled(enabled: false);
  }
}

void _setBuildCliEnabled({required bool enabled}) {
  final expected = enabled
      ? _kDisabledBuildCliDependency
      : _kBuildCliDependency;
  final replacement = enabled
      ? _kBuildCliDependency
      : _kDisabledBuildCliDependency;
  for (final package in _kBuildCliPackages) {
    final pubspec = File(path.join(exec.pwd!, package, 'pubspec.yaml'));
    final contents = pubspec.readAsStringSync();
    if (!contents.contains(expected)) {
      throw StateError(
        'Expected build_cli state not found in ${pubspec.path}.',
      );
    }
    pubspec.writeAsStringSync(contents.replaceFirst(expected, replacement));
  }
}

Future<void> _prepareFromScratchCodegenDependencies() async {
  await exec(
    'rustup toolchain install $kPinnedRustfmtNightly && '
    'rustup component add rustfmt --toolchain '
    '$kPinnedRustfmtNightly-x86_64-unknown-linux-gnu',
  );
  await exec('yarn global add all-contributors-cli');
}

Future<List<String>> deleteTrackedGeneratedFilesForFromScratch() async {
  final output = await exec('git ls-files -z', silent: true);
  final generatedFiles = _selectTrackedGeneratedFilesForFromScratch(
    output.stdout.split('\x00').where((file) => file.isNotEmpty).toList(),
  )..sort();
  if (generatedFiles.isEmpty) {
    throw StateError('No tracked generated files were selected for deletion.');
  }

  stdout.writeln('Deleting ${generatedFiles.length} tracked generated files');
  for (final relativePath in generatedFiles) {
    final file = File(path.join(exec.pwd!, relativePath));
    if (file.existsSync()) file.deleteSync();
  }
  return generatedFiles;
}

void verifyTrackedGeneratedFilesRestored(
  List<String> expectedGeneratedFiles, {
  String? repoRoot,
}) {
  final effectiveRepoRoot = repoRoot ?? exec.pwd!;
  final missingFiles = [
    for (final relativePath in expectedGeneratedFiles)
      if (!File(path.join(effectiveRepoRoot, relativePath)).existsSync())
        relativePath,
  ];
  if (missingFiles.isEmpty) return;

  throw StateError(
    'Code generation did not restore ${missingFiles.length} tracked generated files:\n'
    '${missingFiles.join('\n')}',
  );
}

List<String> _selectTrackedGeneratedFilesForFromScratch(
  List<String> trackedFiles,
) => [
  for (final file in trackedFiles)
    if (!file.startsWith('frb_codegen/assets/integration_template/') &&
        (_isDartBuilderOutput(file) ||
            _isFrbGeneratedFile(file) ||
            file.startsWith('frb_rust/src/internal_generated/') ||
            file.startsWith('frb_dart/lib/src/ffigen_generated/')))
      file,
];

bool _isFrbGeneratedFile(String file) =>
    _fileName(file).startsWith('frb_generated.');

bool _isDartBuilderOutput(String file) =>
    file.endsWith('.g.dart') || file.endsWith('.freezed.dart');

String _fileName(String file) {
  final separator = file.lastIndexOf('/');
  return separator == -1 ? file : file.substring(separator + 1);
}

List<String> selectTrackedGeneratedFilesForFromScratchForTesting(
  List<String> trackedFiles,
) => _selectTrackedGeneratedFilesForFromScratch(trackedFiles);

void verifyGeneratedFilesRestoredForTesting({
  required String repoRoot,
  required List<String> expectedGeneratedFiles,
}) => verifyTrackedGeneratedFilesRestored(
  expectedGeneratedFiles,
  repoRoot: repoRoot,
);
