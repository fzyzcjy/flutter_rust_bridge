// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'ohos_device_smoke.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

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
