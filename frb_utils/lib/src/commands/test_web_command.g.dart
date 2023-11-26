// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'test_web_command.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

TestWebConfig _$parseTestWebConfigResult(ArgResults result) =>
    TestWebConfig()..entrypoint = result['entrypoint'] as String;

ArgParser _$populateTestWebConfigParser(ArgParser parser) => parser
  ..addOption(
    'entrypoint',
  );

final _$parserForTestWebConfig = _$populateTestWebConfigParser(ArgParser());

TestWebConfig parseTestWebConfig(List<String> args) {
  final result = _$parserForTestWebConfig.parse(args);
  return _$parseTestWebConfigResult(result);
}

abstract class _$TestWebConfigCommand<T> extends Command<T> {
  _$TestWebConfigCommand() {
    _$populateTestWebConfigParser(argParser);
  }

  late final _options = _$parseTestWebConfigResult(argResults!);
}
