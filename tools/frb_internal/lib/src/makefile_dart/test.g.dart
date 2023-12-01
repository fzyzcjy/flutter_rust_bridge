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

TestRustConfig _$parseTestRustConfigResult(ArgResults result) => TestRustConfig(
      updateGoldens: result['update-goldens'] as bool,
    );

ArgParser _$populateTestRustConfigParser(ArgParser parser) => parser
  ..addFlag(
    'update-goldens',
  );

final _$parserForTestRustConfig = _$populateTestRustConfigParser(ArgParser());

TestRustConfig parseTestRustConfig(List<String> args) {
  final result = _$parserForTestRustConfig.parse(args);
  return _$parseTestRustConfigResult(result);
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

TestFlutterConfig _$parseTestFlutterConfigResult(ArgResults result) =>
    TestFlutterConfig(
      flutterTestArgs: result['flutter-test-args'] as String?,
      package: result['package'] as String,
    );

ArgParser _$populateTestFlutterConfigParser(ArgParser parser) => parser
  ..addOption(
    'flutter-test-args',
  )
  ..addOption(
    'package',
  );

final _$parserForTestFlutterConfig =
    _$populateTestFlutterConfigParser(ArgParser());

TestFlutterConfig parseTestFlutterConfig(List<String> args) {
  final result = _$parserForTestFlutterConfig.parse(args);
  return _$parseTestFlutterConfigResult(result);
}
