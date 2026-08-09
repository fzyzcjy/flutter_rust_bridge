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
  @CliOption(defaultsTo: IntegrateExampleBackend.cargokit)
  final IntegrateExampleBackend integrationBackend;

  const PostReleaseConfig({
    required this.codegenInstallMode,
    required this.releaseChannel,
    required this.integrationBackend,
  });
}

Future<void> postReleaseMimicQuickstart(PostReleaseConfig config) async {
  final versionRequirement = await resolveCodegenVersionRequirement(
    config.releaseChannel,
  );

  if (versionRequirement == null) {
    print(
      'Post-release codegen install skipped: '
      'release_channel=${config.releaseChannel.name} '
      'codegen_install_mode=${config.codegenInstallMode.name} '
      'integration_backend=${config.integrationBackend.name} '
      'package=$_codegenPackageName '
      'reason=no newer unstable version',
    );
    return;
  }

  print(
    'Post-release codegen install: '
    'release_channel=${config.releaseChannel.name} '
    'codegen_install_mode=${config.codegenInstallMode.name} '
    'integration_backend=${config.integrationBackend.name} '
    'package=$_codegenPackageName '
    'version_requirement=$versionRequirement',
  );

  await quickstartStepInstall(
    config.codegenInstallMode,
    versionConstraint: versionRequirement,
  );
  await MimicQuickstartTester(
    postRelease: true,
    integrationBackend: config.integrationBackend,
  ).test();
}

enum CodegenInstallMode { cargoInstall, cargoBinstall, scoop, homebrew }

enum ReleaseChannel { stable, unstable }

typedef CratesIoMetadataFetcher =
    Future<Map<String, dynamic>> Function(String package);

const _codegenPackageName = 'flutter_rust_bridge_codegen';

Future<String?> resolveCodegenVersionRequirement(
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
      return version == null ? null : '=$version';
  }
}

String? parseLatestUnstableCodegenVersion(Map<String, dynamic> metadata) {
  final versions = [
    for (final rawVersion in metadata['versions'] as List<dynamic>? ?? [])
      if (rawVersion is Map &&
          rawVersion['yanked'] != true &&
          rawVersion['num'] is String)
        Version.parse(rawVersion['num'] as String),
  ];

  final maxStableVersion = _parseMaxStableVersion(metadata, versions);
  final newerUnstableVersions = versions
      .where((version) => version.isPreRelease && version > maxStableVersion)
      .toList();

  if (newerUnstableVersions.isEmpty) {
    return null;
  }

  newerUnstableVersions.sort();
  return newerUnstableVersions.last.toString();
}

Version _parseMaxStableVersion(
  Map<String, dynamic> metadata,
  List<Version> versions,
) {
  final crate = metadata['crate'];
  final rawMaxStableVersion = crate is Map
      ? crate['max_stable_version'] as String?
      : null;
  if (rawMaxStableVersion != null) {
    return Version.parse(rawMaxStableVersion);
  }

  final stableVersions = versions.where((version) => !version.isPreRelease);
  if (stableVersions.isEmpty) {
    throw Exception('No stable $_codegenPackageName version found');
  }
  return stableVersions.reduce((a, b) => a > b ? a : b);
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
