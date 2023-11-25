// ignore_for_file: avoid_print

import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge/src/cli/serve/build_web.dart';
import 'package:flutter_rust_bridge/src/cli/serve/config.dart';
import 'package:flutter_rust_bridge/src/cli/serve/run_server.dart';

final _kWhich = Platform.isWindows ? 'where.exe' : 'which';

/// {@macro flutter_rust_bridge.internal}
void runCliServe(List<String> args) async {
  final config = parseConfig();

  await _sanityChecks(config);

  if (config.build) {
    await buildWeb(
      config,
      wasmOutput: wasmOutput,
      root: root,
      args: args,
    );
  }

  await runServer(config, root: root);
}

Future<void> _sanityChecks(Opts config) async {
  await runCommand(_kWhich, ['wasm-pack']).catchError((_) {
    bail(
      'wasm-pack is required, but not found in the path.\n'
      'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
      'or running `cargo install wasm-pack`.',
    );
  });

  if (config.shouldRunBindgen) {
    await runCommand(_kWhich, ['wasm-bindgen']).catchError((_) {
      bail(
        'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
        'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
      );
    });
  }

  final crateDir = config.crate;
  if (!await File('$crateDir/Cargo.toml').exists()) {
    bail(
      '$crateDir is not a crate directory.\n'
      'Please specify the crate directory using "--crate <CRATE>".',
    );
  }
}
