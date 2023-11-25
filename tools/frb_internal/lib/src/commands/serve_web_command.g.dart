// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serve_web_command.dart';

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

Config _$parseConfigResult(ArgResults result) => Config()
  ..webRoot = result['web-root'] as String
  ..port = int.tryParse(result['port'] as String) ??
      _$badNumberFormat(
        result['port'] as String,
        'int',
        'port',
      );

ArgParser _$populateConfigParser(ArgParser parser) => parser
  ..addOption(
    'web-root',
    abbr: 'r',
    help: 'Root of the directory to be served',
    valueHelp: 'ROOT',
  )
  ..addOption(
    'port',
    abbr: 'p',
    help: 'HTTP port to listen to',
    valueHelp: 'PORT',
    defaultsTo: '8080',
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
