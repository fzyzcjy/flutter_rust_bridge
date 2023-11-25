import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/build_web/config.dart';
import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

/// {@macro flutter_rust_bridge.cli}
Future<void> executeBuildWeb(List<String> args) async {
  final config = parseConfig(args);

  await _sanityChecks(config);

  await _buildWebCore(config);
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

/// {@macro flutter_rust_bridge.internal}
Future<void> _buildWebCore(Config config) async {
  final crateDir = config.cliOpts.crate;

  final manifest = jsonDecode(await runCommand(
    'cargo',
    ['read-manifest'],
    pwd: crateDir,
    silent: true,
  ));

  final String crateName =
      (manifest['targets'] as List).firstWhere((target) => (target['kind'] as List).contains('cdylib'))['name'];
  if (crateName.isEmpty) bail('Crate name cannot be empty.');

  await runCommand('wasm-pack', [
    'build', '-t', 'no-modules', '-d', config.wasmOutput, '--no-typescript',
    '--out-name', crateName,
    if (!config.cliOpts.release) '--dev', crateDir,
    '--', // cargo build args
    '-Z', 'build-std=std,panic_abort',
    if (config.cliOpts.noDefaultFeatures) '--no-default-features',
    if (config.cliOpts.features != null) '--features=${config.cliOpts.features}'
  ], env: {
    'RUSTUP_TOOLCHAIN': 'nightly',
    'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });

  if (config.cliOpts.shouldRunBindgen) {
    await runCommand('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config.cliOpts.release ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      config.wasmOutput,
      '--no-typescript',
      '--target',
      'no-modules',
      if (config.cliOpts.weakRefs) '--weak-refs',
      if (config.cliOpts.referenceTypes) '--reference-types',
    ]);
  }

  if (config.cliOpts.dartInput != null) {
    final output = p.basename(config.cliOpts.dartInput!);
    await runCommand('dart', [
      'compile',
      'js',
      '-o',
      '${config.root}/$output.js',
      if (config.cliOpts.release) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      if (config.cliOpts.verbose) '--verbose',
      config.cliOpts.dartInput!,
    ]);
  } else {
    await runCommand(
      'flutter',
      ['build', 'web', if (!config.cliOpts.release) '--profile'] + config.restArgs,
    );
  }
}
