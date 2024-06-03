import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';
import 'package:path/path.dart' as path;

part 'entrypoint.g.dart';

/// {@macro flutter_rust_bridge.cli}
class BuildWebCommand extends _$ConfigCommand<void> {
  @override
  String get name => 'build-web';

  @override
  String get description => 'Build for web platform';

  @override
  Future<void> run() async {
    await executeBuildWeb(_parseConfigToArgs(_options));
  }
}

/// {@template flutter_rust_bridge.cli}
/// This is mainly used for cli, not for direct function call.
/// {@endtemplate}
@CliOptions(createCommand: true)
class Config {
  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Root folder of dart package')
  late String? dartRoot;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'c',
    help: 'Directory of the rust package',
    defaultsTo: 'rust',
  )
  late String rustRoot;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'o', help: 'Output path', valueHelp: 'PKG')
  late String? output;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Compile in release mode', negatable: false)
  late bool release;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'v', help: 'Display more verbose information')
  late bool verbose;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Arguments passed to cargo-build')
  late List<String> cargoBuildArgs;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Arguments passed to wasm-bindgen')
  late List<String> wasmBindgenArgs;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
      help:
          'Override RUSTUP_TOOLCHAIN environment variable when running wasm-pack')
  late String? wasmPackRustupToolchain;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
      help: 'Override RUSTFLAGS environment variable when running wasm-pack')
  late String? wasmPackRustflags;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
      help:
          'If specified, compile Dart into JavaScript and use this option as entrypoint')
  late String? dartCompileJsEntrypoint;

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

BuildWebArgs _parseConfigToArgs(Config config) {
  return BuildWebArgs(
    output: config.output ?? _fallbackOutput(dartRoot: config.dartRoot),
    release: config.release,
    verbose: config.verbose,
    rustCrateDir: config.rustRoot,
    cargoBuildArgs: config.cargoBuildArgs,
    wasmBindgenArgs: config.wasmBindgenArgs,
    wasmPackRustupToolchain: config.wasmPackRustupToolchain,
    wasmPackRustflags: config.wasmPackRustflags,
    dartCompileJsEntrypoint: config.dartCompileJsEntrypoint,
  );
}

String _fallbackOutput({required String? dartRoot}) =>
    path.join(dartRoot!, 'web');
