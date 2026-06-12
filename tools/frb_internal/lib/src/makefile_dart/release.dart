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
import 'package:yaml/yaml.dart';

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

String getFrbDartVersion() => loadYaml(
  File('${exec.pwd}frb_dart/pubspec.yaml').readAsStringSync(),
)['version'];

VersionInfo computeVersionInfo(ReleaseConfig config) {
  final oldVersion = getFrbDartVersion();
  validateNextReleaseVersion(
    oldVersion: oldVersion,
    newVersion: config.newVersion,
  );
  return VersionInfo(oldVersion: oldVersion, newVersion: config.newVersion);
}

void validateNextReleaseVersion({
  required String oldVersion,
  required String newVersion,
}) {
  final oldParsed = ReleaseVersion.parse(oldVersion);
  final newParsed = ReleaseVersion.parse(newVersion);
  if (newParsed.compareTo(oldParsed) <= 0) {
    throw ArgumentError.value(
      newVersion,
      'newVersion',
      'Must be greater than current version $oldVersion.',
    );
  }

  if (oldParsed.hasPrerelease) {
    _validatePrereleaseSuccessor(oldParsed, newParsed);
  } else {
    _validateStableSuccessor(oldParsed, newParsed);
  }
}

void _validateStableSuccessor(
  ReleaseVersion oldVersion,
  ReleaseVersion newVersion,
) {
  final allowedCores = {
    oldVersion.nextPatchCore,
    oldVersion.nextMinorCore,
    oldVersion.nextMajorCore,
  };
  if (!allowedCores.contains(newVersion.core)) {
    throw ArgumentError.value(
      newVersion.raw,
      'newVersion',
      'Must be the next patch, minor, or major version after ${oldVersion.raw}.',
    );
  }
  if (newVersion.hasPrerelease) {
    _validateFirstPrerelease(newVersion.prerelease, newVersion.raw);
  }
}

void _validatePrereleaseSuccessor(
  ReleaseVersion oldVersion,
  ReleaseVersion newVersion,
) {
  if (newVersion.core != oldVersion.core) {
    throw ArgumentError.value(
      newVersion.raw,
      'newVersion',
      'Must finish the ${oldVersion.core} prerelease series before changing '
          'the version core.',
    );
  }
  if (!newVersion.hasPrerelease) {
    return;
  }

  final oldPrerelease = ParsedPrerelease.parse(
    oldVersion.prerelease,
    oldVersion.raw,
  );
  final newPrerelease = ParsedPrerelease.parse(
    newVersion.prerelease,
    newVersion.raw,
  );
  if (newPrerelease.stage == oldPrerelease.stage) {
    if (newPrerelease.number != oldPrerelease.number + 1) {
      throw ArgumentError.value(
        newVersion.raw,
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
      newVersion.raw,
      'newVersion',
      'Must move from ${oldPrerelease.stage}.${oldPrerelease.number} to the '
          'next prerelease stage ending in .1.',
    );
  }
}

void _validateFirstPrerelease(List<String> prerelease, String rawVersion) {
  final parsed = ParsedPrerelease.parse(prerelease, rawVersion);
  if (parsed.number != 1) {
    throw ArgumentError.value(
      rawVersion,
      'newVersion',
      'First prerelease for a version core must end in .1.',
    );
  }
}

class ReleaseVersion implements Comparable<ReleaseVersion> {
  static final _regex = RegExp(
    r'^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)'
    r'(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$',
  );

  final String raw;
  final int major;
  final int minor;
  final int patch;
  final List<String> prerelease;

  const ReleaseVersion({
    required this.raw,
    required this.major,
    required this.minor,
    required this.patch,
    required this.prerelease,
  });

  static ReleaseVersion parse(String raw) {
    final match = _regex.firstMatch(raw);
    if (match == null) {
      throw ArgumentError.value(
        raw,
        'version',
        'Must be a SemVer version such as 2.13.0 or 2.13.0-beta.1.',
      );
    }
    final prereleaseText = match.group(4);
    return ReleaseVersion(
      raw: raw,
      major: int.parse(match.group(1)!),
      minor: int.parse(match.group(2)!),
      patch: int.parse(match.group(3)!),
      prerelease: prereleaseText == null ? [] : prereleaseText.split('.'),
    );
  }

  String get core => '$major.$minor.$patch';

  String get nextPatchCore => '$major.$minor.${patch + 1}';

  String get nextMinorCore => '$major.${minor + 1}.0';

  String get nextMajorCore => '${major + 1}.0.0';

  bool get hasPrerelease => prerelease.isNotEmpty;

  @override
  int compareTo(ReleaseVersion other) {
    for (final pair in [
      (major, other.major),
      (minor, other.minor),
      (patch, other.patch),
    ]) {
      final compared = pair.$1.compareTo(pair.$2);
      if (compared != 0) return compared;
    }

    if (!hasPrerelease && !other.hasPrerelease) return 0;
    if (!hasPrerelease) return 1;
    if (!other.hasPrerelease) return -1;
    return _comparePrerelease(prerelease, other.prerelease);
  }
}

int _comparePrerelease(List<String> left, List<String> right) {
  for (var i = 0; i < left.length && i < right.length; ++i) {
    final compared = _comparePrereleaseIdentifier(left[i], right[i]);
    if (compared != 0) return compared;
  }
  return left.length.compareTo(right.length);
}

int _comparePrereleaseIdentifier(String left, String right) {
  final leftNumeric = _isNumericIdentifier(left);
  final rightNumeric = _isNumericIdentifier(right);
  if (leftNumeric && rightNumeric) {
    return int.parse(left).compareTo(int.parse(right));
  }
  if (leftNumeric) return -1;
  if (rightNumeric) return 1;
  return left.compareTo(right);
}

class ParsedPrerelease {
  static const _stages = ['alpha', 'beta', 'rc'];

  final String stage;
  final int number;

  const ParsedPrerelease({required this.stage, required this.number});

  static ParsedPrerelease parse(List<String> prerelease, String rawVersion) {
    if (prerelease.length != 2 ||
        !_stages.contains(prerelease[0]) ||
        !_isNumericIdentifier(prerelease[1]) ||
        int.parse(prerelease[1]) == 0) {
      throw ArgumentError.value(
        rawVersion,
        'newVersion',
        'Prerelease versions must use alpha.N, beta.N, or rc.N.',
      );
    }
    return ParsedPrerelease(
      stage: prerelease[0],
      number: int.parse(prerelease[1]),
    );
  }

  int get stageIndex => _stages.indexOf(stage);
}

bool _isNumericIdentifier(String value) {
  return RegExp(r'^(0|[1-9]\d*)$').hasMatch(value);
}

String _extractChangelog(VersionInfo versionInfo) {
  final lines = File('${exec.pwd}CHANGELOG.md').readAsStringSync().split('\n');
  final versions = lines
      .mapIndexed((index, line) {
        final version = RegExp(r'^## (\d.+)$').firstMatch(line)?.group(1);
        return version == null ? null : (index, version);
      })
      .nonNulls
      .toList();

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
  return lines.sublist(newVersion.$1 + 1, oldVersion.$1).join("\n").trim();
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
