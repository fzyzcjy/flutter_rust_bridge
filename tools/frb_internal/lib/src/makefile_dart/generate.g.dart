// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

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

GeneratePackageConfig _$parseGeneratePackageConfigResult(ArgResults result) =>
    GeneratePackageConfig(
      setExitIfChanged: result['set-exit-if-changed'] as bool,
      package: result['package'] as String,
    );

ArgParser _$populateGeneratePackageConfigParser(ArgParser parser) => parser
  ..addFlag(
    'set-exit-if-changed',
  )
  ..addOption(
    'package',
  );

final _$parserForGeneratePackageConfig =
    _$populateGeneratePackageConfigParser(ArgParser());

GeneratePackageConfig parseGeneratePackageConfig(List<String> args) {
  final result = _$parserForGeneratePackageConfig.parse(args);
  return _$parseGeneratePackageConfigResult(result);
}
