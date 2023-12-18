// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('release', release),
    SimpleCommand('release-update-version', releaseUpdateVersion),
    SimpleCommand('release-update-scoop', releaseUpdateScoop),
    SimpleCommand('release-update-git', releaseUpdateGit),
    SimpleCommand('release-update-github', releaseUpdateGithub),
    SimpleCommand('release-publish-all', releasePublishAll),
  ];
}

class _VersionInfo {
  final String oldVersion;
  final String newVersion;

  const _VersionInfo({required this.oldVersion, required this.newVersion});

  @override
  String toString() =>
      '_VersionInfo{oldVersion: $oldVersion, newVersion: $newVersion}';
}

Future<void> release() async {
  print('Version info: ${_computeVersionInfo()}');
  await releaseUpdateVersion();
  await releaseUpdateScoop();
  await releaseUpdateGit();
  await releaseUpdateGithub();
  await releasePublishAll();
}

Future<void> releaseUpdateVersion() async {
  final versionInfo = _computeVersionInfo();

  _simpleReplaceFile(
    'Cargo.toml',
    '\nversion = "${versionInfo.oldVersion}"\n',
    '\nversion = "${versionInfo.newVersion}"\n',
  );
  _simpleReplaceFile(
    'frb_dart/pubspec.yaml',
    '\nversion: ${versionInfo.oldVersion}\n',
    '\nversion: ${versionInfo.newVersion}\n',
  );
}

Future<void> releaseUpdateScoop() async {
  await exec(
      'cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json');
}

Future<void> releaseUpdateGit() async {
  final versionInfo = _computeVersionInfo();
  throw UnimplementedError();
}

Future<void> releaseUpdateGithub() async {
  final versionInfo = _computeVersionInfo();
  throw UnimplementedError();
}

Future<void> releasePublishAll() async {
  await exec('cd frb_codegen && cargo publish');
  await exec('cd frb_rust && cargo publish');
  await exec('cd frb_macros && cargo publish');
  await exec(
      'cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org');
}

_VersionInfo _computeVersionInfo() {
  final lines = File('${exec.pwd}CHANGELOG.md').readAsStringSync().split('\n');
  final versions = lines
      .map((line) => RegExp(r'^## (\d.+)$').firstMatch(line)?.group(1))
      .whereNotNull()
      .toList();
  return _VersionInfo(
    newVersion: versions[0],
    oldVersion: versions[1],
  );
}

void _simpleReplaceFile(String relativePath, String from, String replace) {
  _simpleActFile(relativePath, (x) => x.replaceAll(from, replace));
}

void _simpleActFile(String relativePath, String Function(String) replacer) {
  final file = File('${exec.pwd}$relativePath');
  file.writeAsStringSync(replacer(file.readAsStringSync()));
}
