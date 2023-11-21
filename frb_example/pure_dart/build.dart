import 'dart:io';

import 'package:native_assets_cli/native_assets_cli.dart';

// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
void main(List<String> args) async {
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

  // Write the output according to the native assets protocol so that Dart or
  // Flutter can find the native assets produced by this script.
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
