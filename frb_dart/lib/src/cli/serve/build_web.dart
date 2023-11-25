import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge/src/cli/serve/config.dart';
import 'package:path/path.dart' as p;

/// {@macro flutter_rust_bridge.internal}
Future<void> buildWeb(
  Opts config, {
  required String wasmOutput,
  required String root,
  required List<String> args,
}) async {
  final crateDir = config.crate;
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
    'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
    '--out-name', crateName,
    if (!config.release) '--dev', crateDir,
    '--', // cargo build args
    '-Z', 'build-std=std,panic_abort',
    if (config.noDefaultFeatures) '--no-default-features',
    if (config.features != null) '--features=${config.features}'
  ], env: {
    'RUSTUP_TOOLCHAIN': 'nightly',
    'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });
  if (config.shouldRunBindgen) {
    await runCommand('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config.release ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      wasmOutput,
      '--no-typescript',
      '--target',
      'no-modules',
      if (config.weakRefs) '--weak-refs',
      if (config.referenceTypes) '--reference-types',
    ]);
  }
  if (config.dartInput != null) {
    final output = p.basename(config.dartInput!);
    await runCommand('dart', [
      'compile',
      'js',
      '-o',
      '$root/$output.js',
      if (config.release) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      if (config.verbose) '--verbose',
      config.dartInput!,
    ]);
  } else {
    await runCommand(
      'flutter',
      ['build', 'web', if (!config.release) '--profile'] + Opts.rest(args),
    );
  }
}
