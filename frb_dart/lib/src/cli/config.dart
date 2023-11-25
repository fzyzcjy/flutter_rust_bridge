import 'package:build_cli_annotations/build_cli_annotations.dart';

part 'config.g.dart';

/// {@template flutter_rust_bridge.cli}
/// This is mainly used for cli, not for direct function call.
/// {@endtemplate}
@CliOptions()
class Opts {
  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: 8080,
  )
  late int port;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'r',
    help: 'Root of the Flutter/Dart output',
    valueHelp: 'ROOT',
  )
  late String? root;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'c',
    help: 'Directory of the crate',
    valueHelp: 'CRATE',
    defaultsTo: 'native',
  )
  late String crate;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    abbr: 'd',
    help: 'Run "dart compile" with the specified input instead of "flutter build"',
    valueHelp: 'ENTRY',
  )
  late String? dartInput;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'w', help: 'WASM output path', valueHelp: 'PKG')
  late String? wasmOutput;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'v', help: 'Display more verbose information')
  late bool verbose;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    help: 'Set COEP to credentialless\n'
        'Defaults to true for Flutter',
  )
  late bool relaxCoep;

  /// {@macro flutter_rust_bridge.cli}
  late bool relaxCoepWasParsed;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Open the webpage in a browser', defaultsTo: true)
  late bool open;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Run tests in headless Chromium', negatable: false)
  late bool runTests;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Compile in release mode', negatable: false)
  late bool release;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    help: 'Enable the weak references proposal\n'
        'Requires wasm-bindgen in path',
  )
  late bool weakRefs;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    help: 'Enable the reference types proposal\n'
        'Requires wasm-bindgen in path',
  )
  late bool referenceTypes;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(abbr: 'h', help: 'Print this help message', negatable: false)
  late bool help;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(help: 'Whether to build the library.', defaultsTo: true)
  late bool build;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    help: 'A comma-separated list of features to pass to `cargo build`.',
  )
  late String? features;

  /// {@macro flutter_rust_bridge.cli}
  @CliOption(
    help: 'Whether to disable all features, useful with --features',
    negatable: false,
  )
  late bool noDefaultFeatures;

  /// {@macro flutter_rust_bridge.cli}
  static List<String> rest(List<String> args) => _$parserForOpts.parse(args).rest;
}
