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

ServeWebConfig _$parseServeWebConfigResult(ArgResults result) => ServeWebConfig(
      webRoot: result['web-root'] as String,
      port: int.tryParse(result['port'] as String) ??
          _$badNumberFormat(
            result['port'] as String,
            'int',
            'port',
          ),
      open: result['open'] as bool,
    );

ArgParser _$populateServeWebConfigParser(ArgParser parser) => parser
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
  )
  ..addFlag(
    'open',
    help: 'Open the webpage in a browser',
    defaultsTo: true,
  );

final _$parserForServeWebConfig = _$populateServeWebConfigParser(ArgParser());

ServeWebConfig parseServeWebConfig(List<String> args) {
  final result = _$parserForServeWebConfig.parse(args);
  return _$parseServeWebConfigResult(result);
}

abstract class _$ServeWebConfigCommand<T> extends Command<T> {
  _$ServeWebConfigCommand() {
    _$populateServeWebConfigParser(argParser);
  }

  late final _options = _$parseServeWebConfigResult(argResults!);
}
