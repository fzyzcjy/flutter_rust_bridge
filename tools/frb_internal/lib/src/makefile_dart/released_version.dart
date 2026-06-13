// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:toml/toml.dart';
import 'package:yaml/yaml.dart';

part 'released_version.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'get-released-version',
      getReleasedVersion,
      _$populateGetReleasedVersionConfigParser,
      _$parseGetReleasedVersionConfigResult,
    ),
  ];
}

@CliOptions()
class GetReleasedVersionConfig {
  final String? version;

  const GetReleasedVersionConfig({this.version});
}

Future<void> getReleasedVersion(GetReleasedVersionConfig config) async {
  final statuses = await fetchReleasePackageStatuses(
    targetVersion: config.version,
  );
  print(
    const JsonEncoder.withIndent(
      '  ',
    ).convert(buildReleasePackageStatusOutput(statuses)),
  );
}

class ReleasePackageStatus {
  final String registry;
  final String name;
  final String manifestVersion;
  final String? releasedVersion;

  const ReleasePackageStatus({
    required this.registry,
    required this.name,
    required this.manifestVersion,
    required this.releasedVersion,
  });

  bool get isReleased => manifestVersion == releasedVersion;

  Map<String, Object?> toJson() => {
    'registry': registry,
    'name': name,
    'manifestVersion': manifestVersion,
    'releasedVersion': releasedVersion,
    'isReleased': isReleased,
  };
}

typedef ReleaseMetadataFetcher = Future<Map<String, dynamic>> Function(Uri uri);

Future<List<ReleasePackageStatus>> fetchReleasePackageStatuses({
  String? targetVersion,
  ReleaseMetadataFetcher fetcher = _defaultReleaseMetadataFetcher,
}) async {
  final rustVersion = targetVersion ?? getWorkspaceRustVersion();
  final dartVersion = targetVersion ?? getFrbDartVersion();

  return [
    for (final package in [
      'flutter_rust_bridge_codegen',
      'flutter_rust_bridge_macros',
      'flutter_rust_bridge',
    ])
      ReleasePackageStatus(
        registry: 'crates.io',
        name: package,
        manifestVersion: rustVersion,
        releasedVersion: await fetchCratesIoReleasedVersion(
          package,
          fetcher: fetcher,
        ),
      ),
    ReleasePackageStatus(
      registry: 'pub.dev',
      name: 'flutter_rust_bridge',
      manifestVersion: dartVersion,
      releasedVersion: await fetchPubDevReleasedVersion(
        'flutter_rust_bridge',
        targetVersion: targetVersion,
        fetcher: fetcher,
      ),
    ),
  ];
}

Map<String, Object?> buildReleasePackageStatusOutput(
  List<ReleasePackageStatus> statuses,
) => {
  'allReleased': statuses.every((status) => status.isReleased),
  'packages': statuses.map((status) => status.toJson()).toList(),
};

Future<String?> fetchCratesIoReleasedVersion(
  String package, {
  ReleaseMetadataFetcher fetcher = _defaultReleaseMetadataFetcher,
}) async {
  final json = await fetcher(Uri.https('crates.io', '/api/v1/crates/$package'));
  return parseCratesIoReleasedVersion(json);
}

String? parseCratesIoReleasedVersion(Map<String, dynamic> json) =>
    (json['crate'] as Map<String, dynamic>?)?['max_version'] as String?;

Future<String?> fetchPubDevReleasedVersion(
  String package, {
  String? targetVersion,
  ReleaseMetadataFetcher fetcher = _defaultReleaseMetadataFetcher,
}) async {
  final json = await fetcher(Uri.https('pub.dev', '/api/packages/$package'));
  return parsePubDevReleasedVersion(json, targetVersion: targetVersion);
}

String? parsePubDevReleasedVersion(
  Map<String, dynamic> json, {
  String? targetVersion,
}) {
  if (targetVersion != null && pubDevVersions(json).contains(targetVersion)) {
    return targetVersion;
  }

  return (json['latest'] as Map<String, dynamic>?)?['version'] as String?;
}

Set<String> pubDevVersions(Map<String, dynamic> json) =>
    ((json['versions'] as List<dynamic>?) ?? [])
        .whereType<Map<String, dynamic>>()
        .map((version) => version['version'])
        .whereType<String>()
        .toSet();

Future<Map<String, dynamic>> _defaultReleaseMetadataFetcher(Uri uri) async {
  final response = await Dio().getUri<Map<String, dynamic>>(uri);
  final data = response.data;
  if (data == null) {
    throw Exception('No release metadata returned from $uri');
  }
  return data;
}

String getWorkspaceRustVersion() =>
    parseWorkspaceRustVersion(File('${exec.pwd}Cargo.toml').readAsStringSync());

String parseWorkspaceRustVersion(String raw) =>
    TomlDocument.parse(raw).toMap()['workspace']['package']['version']
        as String;

String getFrbDartVersion() => loadYaml(
  File('${exec.pwd}frb_dart/pubspec.yaml').readAsStringSync(),
)['version'];
