// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:dio/dio.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:pub_semver/pub_semver.dart';

part 'post_release.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'post-release-mimic-quickstart',
      postReleaseMimicQuickstart,
      _$populatePostReleaseConfigParser,
      _$parsePostReleaseConfigResult,
    ),
  ];
}

@CliOptions()
class PostReleaseConfig {
  final CodegenInstallMode codegenInstallMode;
  final ReleaseChannel releaseChannel;

  const PostReleaseConfig({
    required this.codegenInstallMode,
    required this.releaseChannel,
  });
}

Future<void> postReleaseMimicQuickstart(PostReleaseConfig config) async {
  final versionConstraint = await resolveCodegenVersionRequirement(
    config.releaseChannel,
  );

  await quickstartStepInstall(
    config.codegenInstallMode,
    versionConstraint: versionConstraint,
  );
  await const MimicQuickstartTester(postRelease: true).test();
}

enum CodegenInstallMode { cargoInstall, cargoBinstall, scoop, homebrew }

enum ReleaseChannel { stable, unstable }

typedef CratesIoMetadataFetcher =
    Future<Map<String, dynamic>> Function(String package);

const _codegenPackageName = 'flutter_rust_bridge_codegen';

Future<String> resolveCodegenVersionRequirement(
  ReleaseChannel channel, {
  CratesIoMetadataFetcher fetcher = fetchCratesIoMetadata,
}) async {
  switch (channel) {
    case ReleaseChannel.stable:
      return '^2.0.0';
    case ReleaseChannel.unstable:
      final version = parseLatestUnstableCodegenVersion(
        await fetcher(_codegenPackageName),
      );
      return '=$version';
  }
}

String parseLatestUnstableCodegenVersion(Map<String, dynamic> metadata) {
  final versions = [
    for (final rawVersion in metadata['versions'] as List<dynamic>? ?? [])
      if (rawVersion is Map &&
          rawVersion['yanked'] != true &&
          rawVersion['num'] is String)
        Version.parse(rawVersion['num'] as String),
  ].where((version) => version.isPreRelease).toList();

  if (versions.isEmpty) {
    throw Exception('No unstable $_codegenPackageName version found');
  }

  versions.sort();
  return versions.last.toString();
}

Future<Map<String, dynamic>> fetchCratesIoMetadata(String package) async {
  final response = await Dio().getUri<Map<String, dynamic>>(
    Uri.https('crates.io', '/api/v1/crates/$package'),
  );
  final data = response.data;
  if (data == null) {
    throw Exception('No crates.io metadata returned for $package');
  }
  return data;
}

Future<void> quickstartStepInstall(
  CodegenInstallMode mode, {
  required String versionConstraint,
}) async {
  switch (mode) {
    case CodegenInstallMode.cargoInstall:
      await exec(
        "cargo install 'flutter_rust_bridge_codegen@$versionConstraint'",
      );
    case CodegenInstallMode.cargoBinstall:
      await exec(
        "cargo binstall -y 'flutter_rust_bridge_codegen@$versionConstraint'",
      );
    case CodegenInstallMode.scoop:
      await exec(
        'scoop bucket add frb https://github.com/Desdaemon/scoop-repo',
      );
      await exec('scoop install flutter_rust_bridge_codegen');
    case CodegenInstallMode.homebrew:
      await exec('brew install desdaemon/repo/flutter_rust_bridge_codegen');
  }
}
