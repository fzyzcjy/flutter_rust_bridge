// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/args.dart' show ArgParser, ArgResults;
import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart'
    hide ArgParser, ArgResults;
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';
import 'package:pub_semver/pub_semver.dart';

part 'release.g.dart';

List<Command<void>> createCommands() {
  return [
    _releaseConfigCommand('release', release),
    _releaseConfigCommand('release-update-version', releaseUpdateVersion),
    _releaseConfigCommand('release-update-code', releaseUpdateCode),
    SimpleCommand('release-update-scoop', releaseUpdateScoop),
    _releaseConfigCommand('release-update-git', releaseUpdateGit),
    _releaseConfigCommand('release-update-github', releaseUpdateGithub),
    SimpleCommand('release-publish-all', releasePublishAll),
  ];
}

SimpleConfigCommand<ReleaseConfig> _releaseConfigCommand(
  String name,
  Future<void> Function(ReleaseConfig config) executor,
) => SimpleConfigCommand(
  name,
  executor,
  _$populateReleaseConfigParser,
  parseRequiredReleaseConfigResult,
  description: 'Release command that requires --new-version.',
);

@CliOptions()
class ReleaseConfig {
  @CliOption(help: 'The exact version to release, for example 2.13.0-beta.1.')
  final String newVersion;

  const ReleaseConfig({required this.newVersion});
}

ReleaseConfig parseRequiredReleaseConfigResult(ArgResults result) {
  final newVersion = result['new-version'] as String?;
  if (newVersion == null || newVersion.isEmpty) {
    throw ArgumentError('Missing required option --new-version.');
  }
  return ReleaseConfig(newVersion: newVersion);
}

ReleaseConfig parseRequiredReleaseConfigForTesting(List<String> args) {
  final parser = _$populateReleaseConfigParser(ArgParser());
  return parseRequiredReleaseConfigResult(parser.parse(args));
}

class VersionInfo {
  final String oldVersion;
  final String newVersion;

  const VersionInfo({required this.oldVersion, required this.newVersion});

  @override
  String toString() =>
      '_VersionInfo{oldVersion: $oldVersion, newVersion: $newVersion}';
}

Future<void> release(ReleaseConfig config) async {
  final versionInfo = computeVersionInfo(config);
  print('Version info: $versionInfo');
  await releaseUpdateVersionWithInfo(versionInfo);
  await releaseUpdateCodeWithInfo(versionInfo);
  await releaseUpdateScoop();
  await releaseUpdateGitWithInfo(versionInfo);
  await releaseUpdateGithubWithInfo(versionInfo);
  await releasePublishAll();
}

Future<void> releaseUpdateVersion(ReleaseConfig config) async {
  await releaseUpdateVersionWithInfo(computeVersionInfo(config));
}

Future<void> releaseUpdateVersionWithInfo(VersionInfo versionInfo) async {
  simpleReplaceFile(
    '${exec.pwd}Cargo.toml',
    '\nversion = "${versionInfo.oldVersion}"\n',
    '\nversion = "${versionInfo.newVersion}"\n',
  );
  simpleReplaceFile(
    '${exec.pwd}Cargo.toml',
    ', version = "=${versionInfo.oldVersion}" }\n',
    ', version = "=${versionInfo.newVersion}" }\n',
    expectReplaceCount: 3,
  );
  simpleReplaceFile(
    '${exec.pwd}frb_dart/pubspec.yaml',
    '\nversion: ${versionInfo.oldVersion}\n',
    '\nversion: ${versionInfo.newVersion}\n',
  );
}

Future<void> releaseUpdateCode(ReleaseConfig config) async {
  await releaseUpdateCodeWithInfo(computeVersionInfo(config));
}

Future<void> releaseUpdateCodeWithInfo(VersionInfo versionInfo) async {
  _updateVersionInText(versionInfo);
  await pubGetAll();
  await miscNormalizePubspec();
  await cargoFetchAll();
}

class FrbDartCodeVersionInfo {
  static final kPath = '${exec.pwd}frb_dart/lib/src/misc/version.dart';

  static String createCode(String version) =>
      "const kFlutterRustBridgeRuntimeVersion = '$version';";
}

void _updateVersionInText(VersionInfo versionInfo) {
  simpleReplaceFile(
    FrbDartCodeVersionInfo.kPath,
    FrbDartCodeVersionInfo.createCode(versionInfo.oldVersion),
    FrbDartCodeVersionInfo.createCode(versionInfo.newVersion),
  );

  for (final package in ['flutter_rust_bridge', 'flutter_rust_bridge_macros']) {
    simpleReplaceFile(
      '${exec.pwd}frb_codegen/assets/integration_template/shared/REPLACE_ME_RUST_CRATE_DIR/Cargo.lock.template',
      '[[package]]\nname = "$package"\nversion = "${versionInfo.oldVersion}"',
      '[[package]]\nname = "$package"\nversion = "${versionInfo.newVersion}"',
    );
  }

  for (final relativePattern in [
    'frb_codegen/assets/integration_template/**',
    ...kDartExamplePackages.expand(
      (x) => ['$x/lib/**', '$x/rust/src/**', '$x/../src/**'],
    ),
  ]) {
    for (final file in Glob('${exec.pwd}$relativePattern').listSync()) {
      if (file is File) {
        simpleReplaceFile(
          file.path,
          '@generated by `flutter_rust_bridge`@ ${versionInfo.oldVersion}',
          '@generated by `flutter_rust_bridge`@ ${versionInfo.newVersion}',
          expectReplaceCount: null,
        );
        simpleReplaceFile(
          file.path,
          "codegenVersion => '${versionInfo.oldVersion}'",
          "codegenVersion => '${versionInfo.newVersion}'",
          expectReplaceCount: null,
        );
        simpleReplaceFile(
          file.path,
          'FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "${versionInfo.oldVersion}";',
          'FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "${versionInfo.newVersion}";',
          expectReplaceCount: null,
        );
      }
    }
  }
}

Future<void> releaseUpdateScoop() async {
  await exec(
    'cd frb_codegen && ./contrib/scoop.json.sh > ./contrib/flutter_rust_bridge_codegen.json',
  );
}

Future<void> releaseUpdateGit(ReleaseConfig config) async {
  await releaseUpdateGitWithInfo(computeVersionInfo(config));
}

Future<void> releaseUpdateGitWithInfo(VersionInfo versionInfo) async {
  await exec('git add --all');
  await exec('git status && git diff --staged | grep ""');
  await exec(
    'git commit -m "bump from ${versionInfo.oldVersion} to ${versionInfo.newVersion}"',
  );
  await exec('git push');
}

Future<void> releaseUpdateGithub(ReleaseConfig config) async {
  await releaseUpdateGithubWithInfo(computeVersionInfo(config));
}

Future<void> releaseUpdateGithubWithInfo(VersionInfo versionInfo) async {
  File('${exec.pwd}temp.txt').writeAsStringSync(_extractChangelog(versionInfo));
  await exec(
    'gh release create v${versionInfo.newVersion} '
    '--notes-file temp.txt '
    '--draft '
    '--title v${versionInfo.newVersion}',
  );
  print(
    'A *DRAFT* release has been created. Please go to the webpage and really release if you find it correct.',
  );
  await exec('open https://github.com/fzyzcjy/flutter_rust_bridge/releases');
}

Future<void> releasePublishAll() async {
  await exec('cd frb_codegen && cargo publish');
  await exec('cd frb_macros && cargo publish');
  await exec('cd frb_rust && cargo publish');
  await exec(
    'cd frb_dart && flutter pub publish --force --server=https://pub.dartlang.org',
  );
}

VersionInfo computeVersionInfo(ReleaseConfig config) {
  final versionInfo = extractVersionInfoFromChangelog(config.newVersion);
  validateNextReleaseVersion(
    oldVersion: versionInfo.oldVersion,
    newVersion: versionInfo.newVersion,
  );
  return versionInfo;
}

VersionInfo extractVersionInfoFromChangelog(String newVersion) {
  final versions = _extractChangelogVersions();
  final newVersionIndex = versions.indexWhere((item) => item.$2 == newVersion);
  if (newVersionIndex < 0 || newVersionIndex + 1 >= versions.length) {
    throw Exception(
      'CHANGELOG.md must contain $newVersion above the previous version',
    );
  }
  return VersionInfo(
    oldVersion: versions[newVersionIndex + 1].$2,
    newVersion: newVersion,
  );
}

void validateNextReleaseVersion({
  required String oldVersion,
  required String newVersion,
}) {
  _ReleaseVersionValidator.validate(
    oldVersion: oldVersion,
    newVersion: newVersion,
  );
}

class _ReleaseVersionValidator {
  static const _stages = ['alpha', 'beta', 'rc'];

  static void validate({
    required String oldVersion,
    required String newVersion,
  }) {
    final oldParsed = _parseVersion(oldVersion);
    final newParsed = _parseVersion(newVersion);
    if (newParsed.compareTo(oldParsed) <= 0) {
      throw ArgumentError.value(
        newVersion,
        'newVersion',
        'Must be greater than current version $oldVersion.',
      );
    }

    if (oldParsed.isPreRelease) {
      _validatePrereleaseSuccessor(
        oldVersion: oldVersion,
        oldParsed: oldParsed,
        newVersion: newVersion,
        newParsed: newParsed,
      );
    } else {
      _validateStableSuccessor(
        oldVersion: oldVersion,
        oldParsed: oldParsed,
        newVersion: newVersion,
        newParsed: newParsed,
      );
    }
  }

  static Version _parseVersion(String raw) {
    try {
      return Version.parse(raw);
    } on FormatException {
      throw ArgumentError.value(
        raw,
        'version',
        'Must be a SemVer version such as 2.13.0 or 2.13.0-beta.1.',
      );
    }
  }

  static void _validateStableSuccessor({
    required String oldVersion,
    required Version oldParsed,
    required String newVersion,
    required Version newParsed,
  }) {
    final allowedCores = {
      _core(oldParsed.nextPatch),
      _core(oldParsed.nextMinor),
      _core(oldParsed.nextMajor),
    };
    if (!allowedCores.contains(_core(newParsed))) {
      throw ArgumentError.value(
        newVersion,
        'newVersion',
        'Must be the next patch, minor, or major version after $oldVersion.',
      );
    }
    if (newParsed.isPreRelease) {
      _validateFirstPrerelease(newParsed.preRelease, newVersion);
    }
  }

  static void _validatePrereleaseSuccessor({
    required String oldVersion,
    required Version oldParsed,
    required String newVersion,
    required Version newParsed,
  }) {
    if (_core(newParsed) != _core(oldParsed)) {
      throw ArgumentError.value(
        newVersion,
        'newVersion',
        'Must finish the ${_core(oldParsed)} prerelease series before changing '
            'the version core.',
      );
    }
    if (!newParsed.isPreRelease) {
      return;
    }

    final oldPrerelease = _ParsedPrerelease.parse(
      oldParsed.preRelease,
      oldVersion,
    );
    final newPrerelease = _ParsedPrerelease.parse(
      newParsed.preRelease,
      newVersion,
    );
    if (newPrerelease.stage == oldPrerelease.stage) {
      if (newPrerelease.number != oldPrerelease.number + 1) {
        throw ArgumentError.value(
          newVersion,
          'newVersion',
          'Must increment ${oldPrerelease.stage}.'
              '${oldPrerelease.number} by exactly one.',
        );
      }
      return;
    }

    if (newPrerelease.stageIndex != oldPrerelease.stageIndex + 1 ||
        newPrerelease.number != 1) {
      throw ArgumentError.value(
        newVersion,
        'newVersion',
        'Must move from ${oldPrerelease.stage}.${oldPrerelease.number} to the '
            'next prerelease stage ending in .1.',
      );
    }
  }

  static void _validateFirstPrerelease(
    List<Object> prerelease,
    String rawVersion,
  ) {
    final parsed = _ParsedPrerelease.parse(prerelease, rawVersion);
    if (parsed.number != 1) {
      throw ArgumentError.value(
        rawVersion,
        'newVersion',
        'First prerelease for a version core must end in .1.',
      );
    }
  }

  static String _core(Version version) =>
      '${version.major}.${version.minor}.${version.patch}';
}

class _ParsedPrerelease {
  final String stage;
  final int number;

  const _ParsedPrerelease({required this.stage, required this.number});

  static _ParsedPrerelease parse(List<Object> prerelease, String rawVersion) {
    if (prerelease.length != 2 ||
        prerelease[0] is! String ||
        !_ReleaseVersionValidator._stages.contains(prerelease[0]) ||
        prerelease[1] is! int ||
        (prerelease[1] as int) == 0) {
      throw ArgumentError.value(
        rawVersion,
        'newVersion',
        'Prerelease versions must use alpha.N, beta.N, or rc.N.',
      );
    }
    return _ParsedPrerelease(
      stage: prerelease[0] as String,
      number: prerelease[1] as int,
    );
  }

  int get stageIndex => _ReleaseVersionValidator._stages.indexOf(stage);
}

String _extractChangelog(VersionInfo versionInfo) {
  final versions = _extractChangelogVersions();
  final newVersion = versions.firstWhereOrNull(
    (item) => item.$2 == versionInfo.newVersion,
  );
  final oldVersion = versions.firstWhereOrNull(
    (item) => item.$2 == versionInfo.oldVersion,
  );
  if (newVersion == null ||
      oldVersion == null ||
      newVersion.$1 >= oldVersion.$1) {
    throw Exception(
      'CHANGELOG.md must contain ${versionInfo.newVersion} above '
      '${versionInfo.oldVersion}',
    );
  }
  return _readChangelogLines()
      .sublist(newVersion.$1 + 1, oldVersion.$1)
      .join("\n")
      .trim();
}

List<(int, String)> _extractChangelogVersions() {
  final lines = _readChangelogLines();
  return lines
      .mapIndexed((index, line) {
        final version = RegExp(r'^## (\d.+)$').firstMatch(line)?.group(1);
        return version == null ? null : (index, version);
      })
      .nonNulls
      .toList();
}

List<String> _readChangelogLines() {
  final lines = File('${exec.pwd}CHANGELOG.md').readAsStringSync().split('\n');
  return lines;
}

String simpleReplaceSection(
  String raw, {
  required String prelude,
  required String postlude,
  required String inside,
}) {
  return raw.replaceAll(
    RegExp('$prelude(.|\n)*?$postlude', multiLine: true),
    '$prelude\n$inside\n$postlude',
  );
}

void simpleReplaceFile(
  String path,
  Pattern from,
  String replace, {
  int? expectReplaceCount = 1,
}) {
  simpleActFile(
    path,
    (x) => simpleReplaceString(
      x,
      from,
      replace,
      expectReplaceCount: expectReplaceCount,
    ),
  );
}

String simpleReplaceString(
  String text,
  Pattern from,
  String replace, {
  int? expectReplaceCount = 1,
}) {
  var actualReplaceCount = 0;
  final ans = text.replaceAllMapped(from, (match) {
    ++actualReplaceCount;
    return replace;
  });
  if (expectReplaceCount != null && expectReplaceCount != actualReplaceCount) {
    throw Exception(
      'expectReplaceCount=$expectReplaceCount '
      'actualReplaceCount=$actualReplaceCount '
      'from=$from replace=$replace',
    );
  }
  return ans;
}

void simpleActFile(String path, String Function(String) replacer) {
  final file = File(path);
  file.writeAsStringSync(replacer(file.readAsStringSync()));
}
