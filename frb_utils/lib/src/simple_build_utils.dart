// ignore_for_file: avoid_print

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:native_assets_cli/native_assets_cli.dart';

/// Utilities that can be used in `build.dart`.
/// Do not export this function for public use yet, since Dart's `build.dart` support
/// is still experimental.
// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
void simpleBuild(List<String> args) async {
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  final rustCrateDir = buildConfig.packageRoot.resolve('rust');
  await runCommand('cargo', ['build', '--release'],
      pwd: rustCrateDir.toFilePath(), silent: false);

  final dependencies = {
    rustCrateDir,
    buildConfig.packageRoot.resolve('build.rs'),
  };
  print('dependencies: $dependencies');
  buildOutput.dependencies.dependencies.addAll(dependencies);

  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
