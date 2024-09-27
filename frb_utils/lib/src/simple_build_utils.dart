// ignore_for_file: avoid_print, implementation_imports

import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:native_assets_cli/native_assets_cli.dart';

/// Utilities that can be used in `build.dart`.
/// Do not export this function for public use yet, since Dart's `build.dart` support
/// is still experimental.
// ref: https://github.com/dart-lang/native/blob/main/pkgs/native_assets_cli/example/native_add_library/build.dart
void simpleBuild(List<String> args, {List<String> features = const []}) async {
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  final rustCrateDir = buildConfig.packageRoot.resolve('rust');

  final cargoNightly =
      Platform.environment['FRB_SIMPLE_BUILD_CARGO_NIGHTLY'] == '1';
  final cargoExtraArgs =
      Platform.environment['FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS']?.split(' ') ??
          const <String>[];
  final skip = Platform.environment['FRB_SIMPLE_BUILD_SKIP'] == '1';
  final rustflags = Platform.environment['RUSTFLAGS'];

  if (skip) {
    print(
        'frb_utils::simpleBuild SKIP BUILD since environment variable requires this');
  } else {
    final featureArgs = features.expand((x) => ['--features', x]).toList();
    await runCommand(
      'cargo',
      [
        if (cargoNightly) '+nightly',
        'build',
        '--release',
        ...cargoExtraArgs,
        ...featureArgs,
      ],
      pwd: rustCrateDir.toFilePath(),
      printCommandInStderr: true,
      env: {
        // Though runCommand auto pass environment variable to commands,
        // we do this to explicitly show this important flag
        if (rustflags != null) 'RUSTFLAGS': rustflags,
      },
    );
  }

  final dependencies = {
    rustCrateDir,
    buildConfig.packageRoot.resolve('build.rs'),
  };
  print('dependencies: $dependencies');
  buildOutput.dependencies.dependencies.addAll(dependencies);

  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
