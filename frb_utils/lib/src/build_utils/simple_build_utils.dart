// ignore_for_file: avoid_print

import 'dart:io';

import 'package:native_assets_cli/native_assets_cli.dart';

/// Utilities that can be used in `build.dart`.
/// Do not export this function for public use yet, since Dart's `build.dart` support
/// is still experimental.
// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
void simpleBuild(List<String> args) async {
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  final rustCrateDir = buildConfig.packageRoot.resolve('rust');
  _executeProcess('cargo', ['build', '--release'], workingDirectory: rustCrateDir.toFilePath());

  final dependencies = {
    rustCrateDir,
    buildConfig.packageRoot.resolve('build.rs'),
  };
  print('dependencies: $dependencies');
  buildOutput.dependencies.dependencies.addAll(dependencies);

  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}

Future<void> _executeProcess(String executable, List<String> arguments, {String? workingDirectory}) async {
  print('executeProcess: $executable $arguments $workingDirectory');
  final process = await Process.start(executable, arguments, workingDirectory: workingDirectory);
  process.stdout.listen((e) => print(String.fromCharCodes(e)));
  process.stderr.listen((e) => print('[STDERR] ${String.fromCharCodes(e)}'));
  final exitCode = await process.exitCode;
  if (exitCode != 0) throw Exception('Process execution failed (exitCode=$exitCode)');
}
