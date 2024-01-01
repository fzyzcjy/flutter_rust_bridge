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
      target: _$enumValueHelper(
        _$BuildTargetEnumMapBuildCli,
        result['target'] as String,
      ),
    );

const _$BuildTargetEnumMapBuildCli = <BuildTarget, String>{
  BuildTarget.windows: 'windows',
  BuildTarget.macos: 'macos',
  BuildTarget.linux: 'linux',
  BuildTarget.androidAab: 'android-aab',
  BuildTarget.androidApk: 'android-apk',
  BuildTarget.ios: 'ios'
};

ArgParser _$populateBuildFlutterConfigParser(ArgParser parser) => parser
  ..addOption(
    'target',
    allowed: ['windows', 'macos', 'linux', 'android-aab', 'android-apk', 'ios'],
  );

final _$parserForBuildFlutterConfig =
    _$populateBuildFlutterConfigParser(ArgParser());

BuildFlutterConfig parseBuildFlutterConfig(List<String> args) {
  final result = _$parserForBuildFlutterConfig.parse(args);
  return _$parseBuildFlutterConfigResult(result);
}
