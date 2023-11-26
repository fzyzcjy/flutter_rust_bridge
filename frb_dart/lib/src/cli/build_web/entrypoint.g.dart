// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entrypoint.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

Config _$parseConfigResult(ArgResults result) => Config()
  ..dartRoot = result['dart-root'] as String
  ..rustCrateDir = result['rust-crate-dir'] as String
  ..output = result['output'] as String?
  ..release = result['release'] as bool
  ..verbose = result['verbose'] as bool
  ..cargoBuildArgs = result['cargo-build-args'] as List<String>
  ..wasmBindgenArgs = result['wasm-bindgen-args'] as List<String>;

ArgParser _$populateConfigParser(ArgParser parser) => parser
  ..addOption(
    'dart-root',
    help: 'Root of dart package',
  )
  ..addOption(
    'rust-crate-dir',
    abbr: 'c',
    help: 'Directory of the crate',
    valueHelp: 'CRATE',
    defaultsTo: 'rust',
  )
  ..addOption(
    'output',
    abbr: 'o',
    help: 'Output path',
    valueHelp: 'PKG',
  )
  ..addFlag(
    'release',
    help: 'Compile in release mode',
    negatable: false,
  )
  ..addFlag(
    'verbose',
    abbr: 'v',
    help: 'Display more verbose information',
  )
  ..addMultiOption(
    'cargo-build-args',
    help: 'Arguments passed to cargo-build',
  )
  ..addMultiOption(
    'wasm-bindgen-args',
    help: 'Arguments passed to wasm-bindgen',
  );

final _$parserForConfig = _$populateConfigParser(ArgParser());

Config parseConfig(List<String> args) {
  final result = _$parserForConfig.parse(args);
  return _$parseConfigResult(result);
}

abstract class _$ConfigCommand<T> extends Command<T> {
  _$ConfigCommand() {
    _$populateConfigParser(argParser);
  }

  late final _options = _$parseConfigResult(argResults!);
}
