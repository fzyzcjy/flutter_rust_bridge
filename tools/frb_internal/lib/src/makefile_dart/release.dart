// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
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
}

Future<void> release() async {
  await releaseUpdateVersion();
  await releaseUpdateScoop();
  await releaseUpdateGit();
  await releaseUpdateGithub();
  await releasePublishAll();
}

Future<void> releaseUpdateVersion() async {
  final versionInfo = _computeVersionInfo();
  TODO;
}

Future<void> releaseUpdateScoop() async {
  await exec(
      'cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json');
}

Future<void> releaseUpdateGit() async {
  final versionInfo = _computeVersionInfo();
  TODO;
}

Future<void> releaseUpdateGithub() async {
  final versionInfo = _computeVersionInfo();
  TODO;
}

Future<void> releasePublishAll() async {
  await exec('cd frb_codegen && cargo publish');
  await exec('cd frb_rust && cargo publish');
  await exec('cd frb_macros && cargo publish');
  await exec(
      'cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org');
}

_VersionInfo _computeVersionInfo() {
  return TODO;
}
