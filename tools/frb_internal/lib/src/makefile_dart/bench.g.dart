// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'bench.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

BenchConfig _$parseBenchConfigResult(ArgResults result) => BenchConfig(
      filter: result['filter'] as String?,
    );

ArgParser _$populateBenchConfigParser(ArgParser parser) => parser
  ..addOption(
    'filter',
  );

final _$parserForBenchConfig = _$populateBenchConfigParser(ArgParser());

BenchConfig parseBenchConfig(List<String> args) {
  final result = _$parserForBenchConfig.parse(args);
  return _$parseBenchConfigResult(result);
}
