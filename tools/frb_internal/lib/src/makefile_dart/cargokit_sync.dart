import 'dart:io';

import 'package:path/path.dart' as path;

// Mirrors Cargokit's handling in frb_codegen's Rust integrator:
// skip VCS/docs/test files, add the copied-file prelude, and keep scripts executable.
//
// Do not list `flutter_via_create`, `flutter_via_integrate`, or `flutter_package`
// here; the integrate generation CI already recreates those examples from templates.
const _kCargokitCopyMappings = [
  _CargokitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/gallery/rust_builder/cargokit',
  ),
  _CargokitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/integrate_third_party/rust_builder/cargokit',
  ),
  _CargokitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/rust_ui_counter/ui/rust_builder/cargokit',
  ),
  _CargokitCopyMapping(
    source: 'frb_codegen/assets/integration_template/app/rust_builder/cargokit',
    target: 'frb_example/rust_ui_todo_list/ui/rust_builder/cargokit',
  ),
];

const _kSkippedCargokitNames = {'.git', '.github', 'docs', 'test'};

const _kExecutableRelativePaths = [
  'build_pod.sh',
  'run_build_tool.sh',
  'run_build_tool.cmd',
];

const _kCargokitPrelude = [
  'This is copied from Cargokit (which is the official way to use it currently)',
  'Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin',
];

Future<void> syncCargokitCopies() async {
  final repoRoot = _findRepoRoot();

  for (final mapping in _kCargokitCopyMappings) {
    final source = Directory(path.join(repoRoot.path, mapping.source));
    final target = Directory(path.join(repoRoot.path, mapping.target));

    if (!source.existsSync()) {
      throw Exception(
        'Cargokit source `${mapping.source}` does not exist. '
        'Run `git submodule update --init --recursive` first.',
      );
    }

    if (target.existsSync()) {
      target.deleteSync(recursive: true);
    }
    target.createSync(recursive: true);

    await _copyCargokitDirectory(source: source, target: target);
    await _makeScriptsExecutable(target);

    stdout.writeln('Synced ${mapping.source} -> ${mapping.target}');
  }
}

Future<void> _copyCargokitDirectory({
  required Directory source,
  required Directory target,
}) async {
  for (final entity in source.listSync()) {
    final name = path.basename(entity.path);
    if (_kSkippedCargokitNames.contains(name)) continue;

    final targetPath = path.join(target.path, name);
    if (entity is Directory) {
      final targetDirectory = Directory(targetPath)
        ..createSync(recursive: true);
      await _copyCargokitDirectory(source: entity, target: targetDirectory);
    } else if (entity is File) {
      await _copyCargokitFile(source: entity, targetPath: targetPath);
    }
  }
}

Future<void> _copyCargokitFile({
  required File source,
  required String targetPath,
}) async {
  final target = File(targetPath);
  target.parent.createSync(recursive: true);

  final prelude = _computeCargokitPrelude(targetPath);
  if (prelude.isEmpty) {
    await source.copy(targetPath);
  } else {
    await target.writeAsBytes(prelude);
    await target.writeAsBytes(
      await source.readAsBytes(),
      mode: FileMode.append,
    );
  }
}

Directory _findRepoRoot() {
  var directory = Directory.current.absolute;
  while (true) {
    if (File(path.join(directory.path, 'frb_internal')).existsSync() &&
        Directory(
          path.join(directory.path, 'tools', 'frb_internal'),
        ).existsSync()) {
      return directory;
    }

    final parent = directory.parent;
    if (parent.path == directory.path) {
      throw Exception('Could not find flutter_rust_bridge repository root.');
    }
    directory = parent;
  }
}

List<int> _computeCargokitPrelude(String targetPath) {
  final basename = path.basename(targetPath);
  if (basename == '.gitignore') return const [];

  final commentLeading = switch (path.extension(targetPath)) {
    '.dart' || '.md' || '.gradle' || '' => '///',
    '.yaml' || '.toml' => '#',
    '.lock' || '.cmake' || '.sh' || '.ps1' || '.cmd' => null,
    final extension => throw Exception(
      'Unexpected Cargokit file extension `$extension` for `$targetPath`.',
    ),
  };

  if (commentLeading == null) return const [];

  return '${_kCargokitPrelude.map((line) => '$commentLeading $line').join('\n')}\n\n'
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

class _CargokitCopyMapping {
  final String source;
  final String target;

  const _CargokitCopyMapping({required this.source, required this.target});
}
