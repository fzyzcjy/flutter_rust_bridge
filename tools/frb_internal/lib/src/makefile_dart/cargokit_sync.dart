import 'dart:io';

import 'package:path/path.dart' as path;

const _kCargoKitCopyMappings = [
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/flutter_via_create/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/flutter_via_integrate/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/gallery/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/integrate_third_party/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/rust_ui_counter/ui/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/rust_ui_todo_list/ui/rust_builder/cargokit',
  ),
  _CargoKitCopyMapping(
    source: 'frb_codegen/assets/integration_template/plugin/cargokit',
    target: 'frb_example/flutter_package/cargokit',
  ),
];

const _kSkippedCargoKitNames = {'.git', '.github', 'docs', 'test'};

const _kExecutableRelativePaths = [
  'build_pod.sh',
  'run_build_tool.sh',
  'run_build_tool.cmd',
];

const _kCargoKitPrelude = [
  'This is copied from Cargokit (which is the official way to use it currently)',
  'Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin',
];

Future<void> syncCargoKitCopies() async {
  final repoRoot = Directory.current.parent.parent;

  for (final mapping in _kCargoKitCopyMappings) {
    final source = Directory(path.join(repoRoot.path, mapping.source));
    final target = Directory(path.join(repoRoot.path, mapping.target));

    if (!source.existsSync()) {
      throw Exception(
        'CargoKit source `${mapping.source}` does not exist. '
        'Run `git submodule update --init --recursive` first.',
      );
    }

    if (target.existsSync()) {
      target.deleteSync(recursive: true);
    }
    target.createSync(recursive: true);

    await _copyCargoKitDirectory(source: source, target: target);
    await _makeScriptsExecutable(target);

    stdout.writeln('Synced ${mapping.source} -> ${mapping.target}');
  }
}

Future<void> _copyCargoKitDirectory({
  required Directory source,
  required Directory target,
}) async {
  for (final entity in source.listSync()) {
    final name = path.basename(entity.path);
    if (_kSkippedCargoKitNames.contains(name)) continue;

    final targetPath = path.join(target.path, name);
    if (entity is Directory) {
      final targetDirectory = Directory(targetPath)
        ..createSync(recursive: true);
      await _copyCargoKitDirectory(source: entity, target: targetDirectory);
    } else if (entity is File) {
      await _copyCargoKitFile(source: entity, targetPath: targetPath);
    }
  }
}

Future<void> _copyCargoKitFile({
  required File source,
  required String targetPath,
}) async {
  final target = File(targetPath);
  target.parent.createSync(recursive: true);

  final bytes = await source.readAsBytes();
  final prelude = _computeCargoKitPrelude(targetPath);
  await target.writeAsBytes([...prelude, ...bytes]);
}

List<int> _computeCargoKitPrelude(String targetPath) {
  final basename = path.basename(targetPath);
  if (basename == '.gitignore') return const [];

  final commentLeading = switch (path.extension(targetPath)) {
    '.dart' || '.md' || '.gradle' || '' => '///',
    '.yaml' || '.toml' => '#',
    '.lock' || '.cmake' || '.sh' || '.ps1' || '.cmd' => null,
    final extension => throw Exception(
      'Unexpected CargoKit file extension `$extension` for `$targetPath`.',
    ),
  };

  if (commentLeading == null) return const [];

  return '${_kCargoKitPrelude.map((line) => '$commentLeading $line').join('\n')}\n\n'
      .codeUnits;
}

Future<void> _makeScriptsExecutable(Directory target) async {
  if (Platform.isWindows) return;

  for (final relativePath in _kExecutableRelativePaths) {
    final file = File(path.join(target.path, relativePath));
    if (!file.existsSync()) continue;
    final result = await Process.run('chmod', ['+x', file.path]);
    if (result.exitCode != 0) {
      throw Exception('Failed to chmod +x `${file.path}`: ${result.stderr}');
    }
  }
}

class _CargoKitCopyMapping {
  final String source;
  final String target;

  const _CargoKitCopyMapping({required this.source, required this.target});
}
