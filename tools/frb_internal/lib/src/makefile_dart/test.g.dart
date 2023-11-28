// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'test.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

TestConfig _$parseTestConfigResult(ArgResults result) => TestConfig();

ArgParser _$populateTestConfigParser(ArgParser parser) => parser;

final _$parserForTestConfig = _$populateTestConfigParser(ArgParser());

TestConfig parseTestConfig(List<String> args) {
  final result = _$parserForTestConfig.parse(args);
  return _$parseTestConfigResult(result);
}

TestDartConfig _$parseTestDartConfigResult(ArgResults result) => TestDartConfig(
      package: result['package'] as String,
    );

ArgParser _$populateTestDartConfigParser(ArgParser parser) => parser
  ..addOption(
    'package',
  );

final _$parserForTestDartConfig = _$populateTestDartConfigParser(ArgParser());

TestDartConfig parseTestDartConfig(List<String> args) {
  final result = _$parserForTestDartConfig.parse(args);
  return _$parseTestDartConfigResult(result);
}
