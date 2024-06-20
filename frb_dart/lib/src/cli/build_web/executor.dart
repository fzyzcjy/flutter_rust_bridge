// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';

/// {@macro flutter_rust_bridge.cli}
class BuildWebArgs {
  /// {@macro flutter_rust_bridge.cli}
  final String output;

  /// {@macro flutter_rust_bridge.cli}
  final bool release;

  /// {@macro flutter_rust_bridge.cli}
  final bool verbose;

  /// {@macro flutter_rust_bridge.cli}
  final String rustCrateDir;

  /// {@macro flutter_rust_bridge.cli}
  final List<String> cargoBuildArgs;

  /// {@macro flutter_rust_bridge.cli}
  final List<String> wasmBindgenArgs;

  /// {@macro flutter_rust_bridge.cli}
  final String? wasmPackRustupToolchain;

  /// {@macro flutter_rust_bridge.cli}
  final String? wasmPackRustflags;

  /// {@macro flutter_rust_bridge.cli}
  final String? dartCompileJsEntrypoint;

  /// {@macro flutter_rust_bridge.cli}
  const BuildWebArgs({
    required this.output,
    required this.release,
    required this.verbose,
    required this.rustCrateDir,
    required this.cargoBuildArgs,
    required this.wasmBindgenArgs,
    required this.wasmPackRustupToolchain,
    required this.wasmPackRustflags,
    required this.dartCompileJsEntrypoint,
  });
}

extension on BuildWebArgs {
  bool get enableWasmBindgen => wasmBindgenArgs.isNotEmpty;

  String get outputWasm => '$output/pkg';

  String get outputDart => '$output/main.dart.js';
}

/// {@macro flutter_rust_bridge.cli}
Future<void> executeBuildWeb(BuildWebArgs args) async {
  await _sanityChecks(args);

  final rustCrateName =
      await _getRustCreateName(rustCrateDir: args.rustCrateDir);

  await _executeWasmPack(args, rustCrateName: rustCrateName);

  if (args.enableWasmBindgen) {
    await _executeWasmBindgen(args, rustCrateName: rustCrateName);
  }

  if (args.dartCompileJsEntrypoint != null) {
    await _executeDartCompile(args);
  }
}

final _commandWhich = Platform.isWindows ? 'where.exe' : 'which';

Future<void> _sanityChecks(BuildWebArgs args) async {
  await _ensurePackageInstalled(
    binaryName: 'wasm-pack',
    install: () async => await runCommand('cargo', ['install', 'wasm-pack']),
    hint: 'wasm-pack is required, but not found in the path.\n'
        'Please install wasm-pack by following the instructions at https://rustwasm.github.io/wasm-pack/\n'
        'or running `cargo install wasm-pack`.',
  );

  if (args.enableWasmBindgen) {
    await _ensurePackageInstalled(
      binaryName: 'wasm-bindgen',
      install: () async =>
          await runCommand('cargo', ['install', '-f', 'wasm-bindgen-cli']),
      hint:
          'wasm-bindgen flags are enabled, but wasm-bindgen could not be found in the path.\n'
          'Please install wasm-bindgen using `cargo install -f wasm-bindgen-cli`.',
    );
  }

  final crateDir = args.rustCrateDir;
  if (!await File('$crateDir/Cargo.toml').exists()) {
    bail(
      '$crateDir is not a crate directory.\n'
      'Please specify the crate directory using "--crate <CRATE>".',
    );
  }
}

Future<void> _ensurePackageInstalled({
  required String binaryName,
  required Future<void> Function() install,
  required String hint,
}) async {
  Future<bool> isBinaryInstalled() async {
    try {
      await runCommand(_commandWhich, [binaryName]);
      return true;
    } catch (_) {
      return false;
    }
  }

  if (await isBinaryInstalled()) return;

  print('Try to install `$binaryName`');
  await install();

  if (!await isBinaryInstalled()) {
    bail(hint);
  }
}

Future<String> _getRustCreateName({required String rustCrateDir}) async {
  final manifest = jsonDecode((await runCommand(
    'cargo',
    ['read-manifest'],
    pwd: rustCrateDir,
    silent: true,
  ))
      .stdout);

  final rustCrateName = (manifest['targets'] as List).firstWhere(
      (target) => (target['kind'] as List).contains('cdylib'))['name'];
  if (rustCrateName.isEmpty) bail('Crate name cannot be empty.');

  return rustCrateName;
}

Future<void> _executeWasmPack(BuildWebArgs args,
    {required String rustCrateName}) async {
  await runCommand('wasm-pack', [
    'build',
    '-t',
    'no-modules',
    '-d',
    args.outputWasm,
    '--no-typescript',
    '--out-name',
    rustCrateName,
    if (!args.release) '--dev',
    args.rustCrateDir,
    '--',
    // cargo build args
    '-Z',
    'build-std=std,panic_abort',
    ...args.cargoBuildArgs,
    // migrate to `cargoBuildArgs`
    // if (config.cliOpts.noDefaultFeatures) '--no-default-features',
    // if (config.cliOpts.features != null) '--features=${config.cliOpts.features}'
  ], env: {
    'RUSTUP_TOOLCHAIN': args.wasmPackRustupToolchain ?? 'nightly',
    'RUSTFLAGS': _computeRustflags(argsOverride: args.wasmPackRustflags),
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });
}

String _computeRustflags({required String? argsOverride}) {
  const kDefault = '-C target-feature=+atomics,+bulk-memory,+mutable-globals';
  if (argsOverride == null) return kDefault;
  if (!argsOverride.contains(kDefault)) {
    print(
        'WARN: RUSTFLAGS will be `$argsOverride`, which does not contain the default one `$kDefault`');
  }
  return argsOverride;
}

Future<void> _executeWasmBindgen(BuildWebArgs args,
    {required String rustCrateName}) async {
  await runCommand('wasm-bindgen', [
    '${args.rustCrateDir}/target/wasm32-unknown-unknown/${args.release ? 'release' : 'debug'}/$rustCrateName.wasm',
    '--out-dir',
    args.outputWasm,
    '--no-typescript',
    '--target',
    'no-modules',
    ...args.wasmBindgenArgs,
    // migrate to `wasmBindgenArgs`
    // if (config.cliOpts.weakRefs) '--weak-refs',
    // if (config.cliOpts.referenceTypes) '--reference-types',
  ]);
}

Future<void> _executeDartCompile(BuildWebArgs args) async {
  await runCommand('dart', [
    'compile',
    'js',
    '-o',
    args.outputDart,
    if (args.release) '-O2',
    if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
    if (args.verbose) '--verbose',
    args.dartCompileJsEntrypoint!,
  ]);
}
