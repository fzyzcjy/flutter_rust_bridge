// ignore_for_file: avoid_print, implementation_imports

import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:hooks/hooks.dart';

/// New simpleBuild for Dart 3.10+ hooks.
///
/// Environment variable mapping (using NIX_ prefix to bypass semi-hermetic environment):
/// - `NIX_FRB_SIMPLE_BUILD_CARGO_NIGHTLY` → use nightly cargo
/// - `NIX_FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS` → extra cargo args (space-separated)
/// - `NIX_FRB_SIMPLE_BUILD_SKIP` → skip build
/// - `NIX_FRB_RUSTFLAGS` → RUSTFLAGS (set in cargo command)
///
/// Note: Old environment variables (FRB_*) are no longer supported because
/// new hooks run in a semi-hermetic environment.
Future<void> simpleBuild(
  List<String> args, {
  List<String> features = const [],
  String rustCrateName = 'rust',
}) async {
  await build(args, (input, output) async {
    final rustCrateDir = input.packageRoot.resolve('$rustCrateName/');

    final cargoNightly =
        Platform.environment['NIX_FRB_SIMPLE_BUILD_CARGO_NIGHTLY'] == '1';
    final cargoExtraArgs =
        Platform.environment['NIX_FRB_SIMPLE_BUILD_CARGO_EXTRA_ARGS']?.split(
          ' ',
        ) ??
        const <String>[];
    final skip = Platform.environment['NIX_FRB_SIMPLE_BUILD_SKIP'] == '1';
    final rustflags = Platform.environment['NIX_FRB_RUSTFLAGS'];

    if (skip) {
      print('frb_utils::simpleBuild SKIP BUILD (NIX_FRB_SIMPLE_BUILD_SKIP=1)');
      return;
    }

    print('Building Rust crate: ${rustCrateDir.toFilePath()}');
    print(
      'Config: cargoNightly=$cargoNightly, cargoExtraArgs=$cargoExtraArgs, rustflags=$rustflags',
    );

    final featureArgs = features.expand((f) => ['--features', f]).toList();

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
      env: {'RUSTFLAGS': ?rustflags},
    );

    output.dependencies.add(rustCrateDir);

    print('dependencies: ${output.dependencies}');
  });
}
