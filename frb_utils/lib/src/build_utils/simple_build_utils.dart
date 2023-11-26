// ignore_for_file: avoid_print

import 'dart:io';

/// Utilities that can be used in `build.dart`.
/// Do not export this function for public use (at least yet), since Dart's `build.dart` support
/// is still experimental.
// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
Future<Set<Uri>> simpleBuild({
  required Uri packageRoot,
}) async {
  final rustCrateDir = packageRoot.resolve('rust');
  _executeProcess('cargo', ['build', '--release'], workingDirectory: rustCrateDir.toFilePath());

  final dependencies = {
    rustCrateDir,
    packageRoot.resolve('build.rs'),
  };
  print('dependencies: $dependencies');
  return dependencies;
}

Future<void> _executeProcess(String executable, List<String> arguments, {String? workingDirectory}) async {
  print('executeProcess: $executable $arguments $workingDirectory');
  final process = await Process.start(executable, arguments, workingDirectory: workingDirectory);
  process.stdout.listen((e) => print(String.fromCharCodes(e)));
  process.stderr.listen((e) => print('[STDERR] ${String.fromCharCodes(e)}'));
  final exitCode = await process.exitCode;
  if (exitCode != 0) throw Exception('Process execution failed (exitCode=$exitCode)');
}
