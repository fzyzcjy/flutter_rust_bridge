// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'build.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

T _$enumValueHelper<T>(Map<T, String> enumValues, String source) => enumValues
    .entries
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
      package: convertConfigPackage(result['package'] as String),
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
  BuildTarget.ios: 'ios',
  BuildTarget.ohos: 'ohos',
};

ArgParser _$populateBuildFlutterConfigParser(ArgParser parser) => parser
  ..addOption('package', defaultsTo: 'frb_example/flutter_via_create')
  ..addOption(
    'target',
    allowed: [
      'windows',
      'macos',
      'linux',
      'android-aab',
      'android-apk',
      'ios',
      'ohos',
    ],
  );

final _$parserForBuildFlutterConfig = _$populateBuildFlutterConfigParser(
  ArgParser(),
);

BuildFlutterConfig parseBuildFlutterConfig(List<String> args) {
  final result = _$parserForBuildFlutterConfig.parse(args);
  return _$parseBuildFlutterConfigResult(result);
}

T _$badNumberFormat<T extends num>(
  String source,
  String type,
  String argName,
) => throw FormatException(
  'Cannot parse "$source" into `$type` for option "$argName".',
);

OhosDeviceSmokeConfig _$parseOhosDeviceSmokeConfigResult(ArgResults result) =>
    OhosDeviceSmokeConfig(
      hap: result['hap'] as String,
      bundle: result['bundle'] as String,
      ability: result['ability'] as String,
      expectedLog: result['expected-log'] as String,
      timeoutSeconds:
          int.tryParse(result['timeout-seconds'] as String) ??
          _$badNumberFormat(
            result['timeout-seconds'] as String,
            'int',
            'timeout-seconds',
          ),
      deviceId: result['device-id'] as String?,
      artifactDir: result['artifact-dir'] as String,
    );

ArgParser _$populateOhosDeviceSmokeConfigParser(ArgParser parser) => parser
  ..addOption('hap')
  ..addOption('bundle')
  ..addOption('ability', defaultsTo: 'EntryAbility')
  ..addOption('expected-log', defaultsTo: 'FRB_OHOS_SMOKE_RESULT=PASS')
  ..addOption('timeout-seconds', defaultsTo: '30')
  ..addOption('device-id')
  ..addOption('artifact-dir', defaultsTo: 'target/ohos_device_smoke');

final _$parserForOhosDeviceSmokeConfig = _$populateOhosDeviceSmokeConfigParser(
  ArgParser(),
);

OhosDeviceSmokeConfig parseOhosDeviceSmokeConfig(List<String> args) {
  final result = _$parserForOhosDeviceSmokeConfig.parse(args);
  return _$parseOhosDeviceSmokeConfigResult(result);
}
