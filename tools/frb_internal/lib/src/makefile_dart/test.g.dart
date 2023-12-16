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
      coverage: result['coverage'] as bool,
    );

ArgParser _$populateTestRustConfigParser(ArgParser parser) => parser
  ..addFlag(
    'update-goldens',
  )
  ..addFlag(
    'coverage',
  );

final _$parserForTestRustConfig = _$populateTestRustConfigParser(ArgParser());

TestRustConfig parseTestRustConfig(List<String> args) {
  final result = _$parserForTestRustConfig.parse(args);
  return _$parseTestRustConfigResult(result);
}

TestDartConfig _$parseTestDartConfigResult(ArgResults result) => TestDartConfig(
      package: result['package'] as String,
      coverage: result['coverage'] as bool,
    );

ArgParser _$populateTestDartConfigParser(ArgParser parser) => parser
  ..addOption(
    'package',
  )
  ..addFlag(
    'coverage',
  );

final _$parserForTestDartConfig = _$populateTestDartConfigParser(ArgParser());

TestDartConfig parseTestDartConfig(List<String> args) {
  final result = _$parserForTestDartConfig.parse(args);
  return _$parseTestDartConfigResult(result);
}

T _$enumValueHelper<T>(Map<T, String> enumValues, String source) =>
    enumValues.entries
        .singleWhere(
          (e) => e.value == source,
          orElse: () => throw ArgumentError(
            '`$source` is not one of the supported values: '
            '${enumValues.values.join(', ')}',
          ),
        )
        .key;

TestDartSanitizerConfig _$parseTestDartSanitizerConfigResult(
        ArgResults result) =>
    TestDartSanitizerConfig(
      package: result['package'] as String,
      useLocalSanitizedDartBinary:
          result['use-local-sanitized-dart-binary'] as bool,
      sanitizer: _$enumValueHelper(
        _$SanitizerEnumMapBuildCli,
        result['sanitizer'] as String,
      ),
    );

const _$SanitizerEnumMapBuildCli = <Sanitizer, String>{
  Sanitizer.asan: 'asan',
  Sanitizer.msan: 'msan',
  Sanitizer.lsan: 'lsan',
  Sanitizer.tsan: 'tsan'
};

ArgParser _$populateTestDartSanitizerConfigParser(ArgParser parser) => parser
  ..addOption(
    'package',
  )
  ..addFlag(
    'use-local-sanitized-dart-binary',
  )
  ..addOption(
    'sanitizer',
    allowed: ['asan', 'msan', 'lsan', 'tsan'],
  );

final _$parserForTestDartSanitizerConfig =
    _$populateTestDartSanitizerConfigParser(ArgParser());

TestDartSanitizerConfig parseTestDartSanitizerConfig(List<String> args) {
  final result = _$parserForTestDartSanitizerConfig.parse(args);
  return _$parseTestDartSanitizerConfigResult(result);
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
