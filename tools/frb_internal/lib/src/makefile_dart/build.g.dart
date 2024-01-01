// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'build.dart';

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

BuildFlutterConfig _$parseBuildFlutterConfigResult(ArgResults result) =>
    BuildFlutterConfig(
      platform: _$enumValueHelper(
        _$BuildPlatformEnumMapBuildCli,
        result['platform'] as String,
      ),
    );

const _$BuildPlatformEnumMapBuildCli = <BuildPlatform, String>{
  BuildPlatform.windows: 'windows',
  BuildPlatform.macos: 'macos',
  BuildPlatform.linux: 'linux',
  BuildPlatform.android: 'android',
  BuildPlatform.ios: 'ios'
};

ArgParser _$populateBuildFlutterConfigParser(ArgParser parser) => parser
  ..addOption(
    'platform',
    allowed: ['windows', 'macos', 'linux', 'android', 'ios'],
  );

final _$parserForBuildFlutterConfig =
    _$populateBuildFlutterConfigParser(ArgParser());

BuildFlutterConfig parseBuildFlutterConfig(List<String> args) {
  final result = _$parserForBuildFlutterConfig.parse(args);
  return _$parseBuildFlutterConfigResult(result);
}
