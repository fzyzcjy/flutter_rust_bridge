// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'lint.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

LintConfig _$parseLintConfigResult(ArgResults result) => LintConfig(
      fix: result['fix'] as bool,
    );

ArgParser _$populateLintConfigParser(ArgParser parser) => parser
  ..addFlag(
    'fix',
  );

final _$parserForLintConfig = _$populateLintConfigParser(ArgParser());

LintConfig parseLintConfig(List<String> args) {
  final result = _$parserForLintConfig.parse(args);
  return _$parseLintConfigResult(result);
}

LintDartFormatConfig _$parseLintDartFormatConfigResult(ArgResults result) =>
    LintDartFormatConfig(
      fix: result['fix'] as bool,
      package: result['package'] as String,
    );

ArgParser _$populateLintDartFormatConfigParser(ArgParser parser) => parser
  ..addFlag(
    'fix',
    defaultsTo: true,
  )
  ..addOption(
    'package',
  );

final _$parserForLintDartFormatConfig =
    _$populateLintDartFormatConfigParser(ArgParser());

LintDartFormatConfig parseLintDartFormatConfig(List<String> args) {
  final result = _$parserForLintDartFormatConfig.parse(args);
  return _$parseLintDartFormatConfigResult(result);
}
