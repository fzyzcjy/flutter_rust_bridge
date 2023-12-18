// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'generate.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

GenerateConfig _$parseGenerateConfigResult(ArgResults result) => GenerateConfig(
      setExitIfChanged: result['set-exit-if-changed'] as bool,
      coverage: result['coverage'] as bool,
    );

ArgParser _$populateGenerateConfigParser(ArgParser parser) => parser
  ..addFlag(
    'set-exit-if-changed',
  )
  ..addFlag(
    'coverage',
  );

final _$parserForGenerateConfig = _$populateGenerateConfigParser(ArgParser());

GenerateConfig parseGenerateConfig(List<String> args) {
  final result = _$parserForGenerateConfig.parse(args);
  return _$parseGenerateConfigResult(result);
}

GeneratePackageConfig _$parseGeneratePackageConfigResult(ArgResults result) =>
    GeneratePackageConfig(
      setExitIfChanged: result['set-exit-if-changed'] as bool,
      package: convertConfigPackage(result['package'] as String),
      coverage: result['coverage'] as bool,
    );

ArgParser _$populateGeneratePackageConfigParser(ArgParser parser) => parser
  ..addFlag(
    'set-exit-if-changed',
  )
  ..addFlag(
    'coverage',
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

GenerateWebsiteConfig _$parseGenerateWebsiteConfigResult(ArgResults result) =>
    GenerateWebsiteConfig(
      coverage: result['coverage'] as bool,
    );

ArgParser _$populateGenerateWebsiteConfigParser(ArgParser parser) => parser
  ..addFlag(
    'coverage',
  );

final _$parserForGenerateWebsiteConfig =
    _$populateGenerateWebsiteConfigParser(ArgParser());

GenerateWebsiteConfig parseGenerateWebsiteConfig(List<String> args) {
  final result = _$parserForGenerateWebsiteConfig.parse(args);
  return _$parseGenerateWebsiteConfigResult(result);
}
