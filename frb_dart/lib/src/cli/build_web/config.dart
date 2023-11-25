// ignore_for_file: avoid_print

import 'dart:io';

import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:path/path.dart' as p;

part 'config.g.dart';

/// {@template flutter_rust_bridge.cli}
/// This is mainly used for cli, not for direct function call.
/// {@endtemplate}
@CliOptions()
class Config {
  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Root of dart package')
  late String dartRoot;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'c',
    help: 'Directory of the crate',
    valueHelp: 'CRATE',
    defaultsTo: 'native',
  )
  late String rustCrateDir;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'w', help: 'WASM output path', valueHelp: 'PKG')
  late String? wasmOutput;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Compile in release mode', negatable: false)
  late bool release;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Arguments passed to wasm-pack')
  late List<String> wasmPackArgs;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Arguments passed to wasm-bindgen')
  late List<String> wasmBindgenArgs;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'h', help: 'Print this help message', negatable: false)
  late bool help;

// migrate to `wasmPackArgs`
// /// {@macro flutter_rust_bridge.cli}
// @CliOption(
//   help: 'A comma-separated list of features to pass to `cargo build`.',
// )
// late String? features;
//
// /// {@macro flutter_rust_bridge.cli}
// @CliOption(
//   help: 'Whether to disable all features, useful with --features',
//   negatable: false,
// )
// late bool noDefaultFeatures;

// migrate to `wasmPackArgs`
// /// {@macro flutter_rust_bridge.cli}
// @CliOption(
//   help: 'Enable the weak references proposal\n'
//       'Requires wasm-bindgen in path',
// )
// late bool weakRefs;
//
// /// {@macro flutter_rust_bridge.cli}
// @CliOption(
//   help: 'Enable the reference types proposal\n'
//       'Requires wasm-bindgen in path',
// )
// late bool referenceTypes;
}

/// {@macro flutter_rust_bridge.cli}
BuildWebArgs parseConfigToArgs(List<String> args) {
  final Config config = parseConfig(args);

  if (config.help) {
    _printHelpAndExit();
  }

  return BuildWebArgs(
    wasmOutput: config.wasmOutput ?? _fallbackWasmOutput(dartRoot: config.dartRoot),
    release: config.release,
    rustCrateDir: config.rustCrateDir,
    wasmPackArgs: config.wasmPackArgs,
    enableWasmBindgen: config.wasmBindgenArgs.isNotEmpty,
    wasmBindgenArgs: config.wasmBindgenArgs,
  );
}

String _fallbackWasmOutput({required String dartRoot}) => p.join(dartRoot, 'web/pkg');

Never _printHelpAndExit() {
  print("""
USAGE:
\t[OPTIONS]

OPTIONS:""");
  print(_$parserForConfig.usage);

  exit(0);
}
