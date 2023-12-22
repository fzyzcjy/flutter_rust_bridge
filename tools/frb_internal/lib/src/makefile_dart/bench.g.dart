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

T _$badNumberFormat<T extends num>(
  String source,
  String type,
  String argName,
) =>
    throw FormatException(
      'Cannot parse "$source" into `$type` for option "$argName".',
    );

BenchFlamegraphRunConfig _$parseBenchFlamegraphRunConfigResult(
        ArgResults result) =>
    BenchFlamegraphRunConfig(
      filter: result['filter'] as String,
      loopCount: int.tryParse(result['loop-count'] as String) ??
          _$badNumberFormat(
            result['loop-count'] as String,
            'int',
            'loop-count',
          ),
    );

ArgParser _$populateBenchFlamegraphRunConfigParser(ArgParser parser) => parser
  ..addOption(
    'filter',
  )
  ..addOption(
    'loop-count',
  );

final _$parserForBenchFlamegraphRunConfig =
    _$populateBenchFlamegraphRunConfigParser(ArgParser());

BenchFlamegraphRunConfig parseBenchFlamegraphRunConfig(List<String> args) {
  final result = _$parserForBenchFlamegraphRunConfig.parse(args);
  return _$parseBenchFlamegraphRunConfigResult(result);
}
