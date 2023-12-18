// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'misc.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

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

PrecommitConfig _$parsePrecommitConfigResult(ArgResults result) =>
    PrecommitConfig(
      mode: _$enumValueHelper(
        _$PrecommitModeEnumMapBuildCli,
        result['mode'] as String,
      ),
    );

const _$PrecommitModeEnumMapBuildCli = <PrecommitMode, String>{
  PrecommitMode.fast: 'fast',
  PrecommitMode.slow: 'slow'
};

ArgParser _$populatePrecommitConfigParser(ArgParser parser) => parser
  ..addOption(
    'mode',
    allowed: ['fast', 'slow'],
  );

final _$parserForPrecommitConfig = _$populatePrecommitConfigParser(ArgParser());

PrecommitConfig parsePrecommitConfig(List<String> args) {
  final result = _$parserForPrecommitConfig.parse(args);
  return _$parsePrecommitConfigResult(result);
}
