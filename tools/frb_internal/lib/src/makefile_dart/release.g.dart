// GENERATED CODE - DO NOT MODIFY BY HAND

// ignore_for_file: prefer_const_constructors

part of 'release.dart';

// **************************************************************************
// CliGenerator
// **************************************************************************

ReleaseConfig _$parseReleaseConfigResult(ArgResults result) =>
    ReleaseConfig(newVersion: result['new-version'] as String);

ArgParser _$populateReleaseConfigParser(ArgParser parser) => parser
  ..addOption(
    'new-version',
    help: 'The exact version to release, for example 2.13.0-beta.1.',
  );

final _$parserForReleaseConfig = _$populateReleaseConfigParser(ArgParser());

ReleaseConfig parseReleaseConfig(List<String> args) {
  final result = _$parserForReleaseConfig.parse(args);
  return _$parseReleaseConfigResult(result);
}
