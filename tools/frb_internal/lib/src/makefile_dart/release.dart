// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('release', release),
  ];
}

class _VersionInfo {
  final String oldVersion;
  final String newVersion;

  const _VersionInfo({required this.oldVersion, required this.newVersion});
}

Future<void> release() async {
  final versionInfo = _computeVersionInfo();
  await _releaseUpdateVersion(versionInfo);
  await _releaseUpdateScoop();
  await _releaseUpdateGit(versionInfo);
  await _releaseUpdateGitHub(versionInfo);
  await _releasePublishAll();
}

_VersionInfo _computeVersionInfo() {
  return TODO;
}

Future<void> _releaseUpdateVersion(_VersionInfo versionInfo) async {
  TODO;
}

Future<void> _releaseUpdateScoop() async {
  await exec(
      'cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json');
}

Future<void> _releaseUpdateGit(_VersionInfo versionInfo) async {
  TODO;
}

Future<void> _releaseUpdateGitHub(_VersionInfo versionInfo) async {
  TODO;
}

Future<void> _releasePublishAll() async {
  await exec('cd frb_codegen && cargo publish');
  await exec('cd frb_rust && cargo publish');
  await exec('cd frb_macros && cargo publish');
  await exec(
      'cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org');
}
