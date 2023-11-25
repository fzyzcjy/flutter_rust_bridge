// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'config.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

Config _$parseConfigResult(ArgResults result) => Config()
  ..dartRoot = result['dart-root'] as String
  ..rustCrateDir = result['rust-crate-dir'] as String
  ..wasmOutput = result['wasm-output'] as String?
  ..release = result['release'] as bool
  ..wasmPackArgs = result['wasm-pack-args'] as List<String>
  ..wasmBindgenArgs = result['wasm-bindgen-args'] as List<String>
  ..help = result['help'] as bool;

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
    defaultsTo: 'native',
  )
  ..addOption(
    'wasm-output',
    abbr: 'w',
    help: 'WASM output path',
    valueHelp: 'PKG',
  )
  ..addFlag(
    'release',
    help: 'Compile in release mode',
    negatable: false,
  )
  ..addMultiOption(
    'wasm-pack-args',
    help: 'Arguments passed to wasm-pack',
  )
  ..addMultiOption(
    'wasm-bindgen-args',
    help: 'Arguments passed to wasm-bindgen',
  )
  ..addFlag(
    'help',
    abbr: 'h',
    help: 'Print this help message',
    negatable: false,
  );

final _$parserForConfig = _$populateConfigParser(ArgParser());

Config parseConfig(List<String> args) {
  final result = _$parserForConfig.parse(args);
  return _$parseConfigResult(result);
}
