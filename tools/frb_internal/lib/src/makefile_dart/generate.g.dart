// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'generate.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

GenerateConfig _$parseGenerateConfigResult(ArgResults result) => GenerateConfig(
      setExitIfChanged: result['set-exit-if-changed'] as bool,
    );

ArgParser _$populateGenerateConfigParser(ArgParser parser) => parser
  ..addFlag(
    'set-exit-if-changed',
  );

final _$parserForGenerateConfig = _$populateGenerateConfigParser(ArgParser());

GenerateConfig parseGenerateConfig(List<String> args) {
  final result = _$parserForGenerateConfig.parse(args);
  return _$parseGenerateConfigResult(result);
}
