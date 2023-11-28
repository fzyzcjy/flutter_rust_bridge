// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

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
