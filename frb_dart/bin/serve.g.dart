// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serve.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

T _$badNumberFormat<T extends num>(
  String source,
  String type,
  String argName,
) =>
    throw FormatException(
      'Cannot parse "$source" into `$type` for option "$argName".',
    );

Opts _$parseOptsResult(ArgResults result) => Opts()
  ..port = int.tryParse(result['port'] as String) ??
      _$badNumberFormat(
        result['port'] as String,
        'int',
        'port',
      )
  ..root = result['root'] as String?
  ..crate = result['crate'] as String
  ..dartInput = result['dart-input'] as String?
  ..wasmOutput = result['wasm-output'] as String?
  ..verbose = result['verbose'] as bool
  ..relaxCoep = result['relax-coep'] as bool
  ..relaxCoepWasParsed = result.wasParsed('relax-coep')
  ..open = result['open'] as bool
  ..runTests = result['run-tests'] as bool
  ..release = result['release'] as bool
  ..weakRefs = result['weak-refs'] as bool
  ..referenceTypes = result['reference-types'] as bool
  ..help = result['help'] as bool
  ..build = result['build'] as bool;

ArgParser _$populateOptsParser(ArgParser parser) => parser
  ..addOption(
    'port',
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: '8080',
  )
  ..addOption(
    'root',
    abbr: 'r',
    help: 'Root of the Flutter/Dart output',
    valueHelp: 'ROOT',
  )
  ..addOption(
    'crate',
    abbr: 'c',
    help: 'Directory of the crate',
    valueHelp: 'CRATE',
    defaultsTo: 'native',
  )
  ..addOption(
    'dart-input',
    abbr: 'd',
    help:
        'Run "dart compile" with the specified input instead of "flutter build"',
    valueHelp: 'ENTRY',
  )
  ..addOption(
    'wasm-output',
    abbr: 'w',
    help: 'WASM output path',
    valueHelp: 'PKG',
  )
  ..addFlag(
    'verbose',
    abbr: 'v',
    help: 'Display more verbose information',
  )
  ..addFlag(
    'relax-coep',
    help: 'Set COEP to credentialless\nDefaults to true for Flutter',
  )
  ..addFlag(
    'open',
    help: 'Open the webpage in a browser',
    defaultsTo: true,
  )
  ..addFlag(
    'run-tests',
    help: 'Run tests in headless Chromium',
    negatable: false,
  )
  ..addFlag(
    'release',
    help: 'Compile in release mode',
    negatable: false,
  )
  ..addFlag(
    'weak-refs',
    help: 'Enable the weak references proposal\nRequires wasm-bindgen in path',
  )
  ..addFlag(
    'reference-types',
    help: 'Enable the reference types proposal\nRequires wasm-bindgen in path',
  )
  ..addFlag(
    'help',
    abbr: 'h',
    help: 'Print this help message',
    negatable: false,
  )
  ..addFlag(
    'build',
    help: 'Whether to build the library.',
    defaultsTo: true,
  );

final _$parserForOpts = _$populateOptsParser(ArgParser());

Opts parseOpts(List<String> args) {
  final result = _$parserForOpts.parse(args);
  return _$parseOptsResult(result);
}
