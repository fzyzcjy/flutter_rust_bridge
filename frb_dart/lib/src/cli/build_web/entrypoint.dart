import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/config.dart';
import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

/// {@macro flutter_rust_bridge.cli}
Future<void> executeBuildWeb(List<String> args) async {
  final config = parseConfig(args);

  await _sanityChecks(config);

  TODO;
}

Future<void> _sanityChecks(Config config) async {
  final which = Platform.isWindows ? 'where.exe' : 'which';

  await runCommand(which, ['wasm-pack']).catchError((_) {
    bail(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
  });

  if (config.cliOpts.shouldRunBindgen) {
    await runCommand(which, ['wasm-bindgen']).catchError((_) {
      bail(
        'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
        'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
      );
    });
  }

  final crateDir = config.cliOpts.crate;
  if (!await File('$crateDir/Cargo.toml').exists()) {
    bail(
      '$crateDir is not a crate directory.\n'
      'Please specify the crate directory using "--crate <CRATE>".',
    );
  }
}
