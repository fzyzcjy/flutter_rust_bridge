// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'released_version.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

GetReleasedVersionConfig _$parseGetReleasedVersionConfigResult(
  ArgResults result,
) => GetReleasedVersionConfig(version: result['version'] as String?);

ArgParser _$populateGetReleasedVersionConfigParser(ArgParser parser) =>
    parser..addOption('version');

final _$parserForGetReleasedVersionConfig =
    _$populateGetReleasedVersionConfigParser(ArgParser());

GetReleasedVersionConfig parseGetReleasedVersionConfig(List<String> args) {
  final result = _$parserForGetReleasedVersionConfig.parse(args);
  return _$parseGetReleasedVersionConfigResult(result);
}
