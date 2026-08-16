/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';

import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'options.dart';
import 'target.dart';
import 'util.dart';

List<String> createXcframeworkArguments({
  required Iterable<String> frameworks,
  required String output,
}) {
  return [
    '-create-xcframework',
    for (final framework in frameworks) ...['-framework', framework],
    '-output',
    output,
  ];
}

String createFrameworkInfoPlist(String frameworkName) {
  final bundleName = frameworkName.replaceAll('_', '-');
  return '''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>$frameworkName</string>
  <key>CFBundleIdentifier</key>
  <string>dev.cargokit.$bundleName</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>$frameworkName</string>
  <key>CFBundlePackageType</key>
  <string>FMWK</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
</dict>
</plist>
''';
}

class BuildSpm {
  BuildSpm({required this.userOptions});

  final CargokitUserOptions userOptions;

  Future<void> build() async {
    if (!Platform.isMacOS) {
      throw UnsupportedError('XCFrameworks can only be built on macOS.');
    }

    final environment = BuildEnvironment.fromEnvironment(isAndroid: false);
    final targetGroups = Target.darwinXcframeworkTargetGroups();
    final targets = targetGroups.values.expand((group) => group).toList();
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts(
      targets,
      artifactType: AritifactType.dylib,
    );

    final crateName = environment.crateInfo.packageName;
    final frameworkName = crateName.replaceAll('-', '_');
    final workDir =
        path.join(environment.targetTempDir, 'xcframework', crateName);
    final frameworks = <String>[];

    final existingWorkDir = Directory(workDir);
    if (existingWorkDir.existsSync()) {
      existingWorkDir.deleteSync(recursive: true);
    }
    Directory(workDir).createSync(recursive: true);

    for (final entry in targetGroups.entries) {
      final inputLibraries = entry.value.map((target) {
        final dynamicLibraries = (artifacts[target] ?? [])
            .where((artifact) => artifact.type == AritifactType.dylib)
            .toList();
        if (dynamicLibraries.length != 1) {
          throw StateError(
            'Expected one dynamic library for $target. Swift Package Manager '
            'XCFramework builds require the Rust crate to declare '
            'crate-type = ["cdylib"].',
          );
        }
        return dynamicLibraries.single.path;
      }).toList();

      final frameworkDir = path.join(
        workDir,
        entry.key,
        '$frameworkName.framework',
      );
      Directory(frameworkDir).createSync(recursive: true);
      final frameworkBinary = path.join(frameworkDir, frameworkName);
      runCommand('lipo', [
        '-create',
        ...inputLibraries,
        '-output',
        frameworkBinary,
      ]);
      runCommand('install_name_tool', [
        '-id',
        '@rpath/$frameworkName.framework/$frameworkName',
        frameworkBinary,
      ]);
      File(path.join(frameworkDir, 'Info.plist'))
          .writeAsStringSync(createFrameworkInfoPlist(frameworkName));
      frameworks.add(frameworkDir);
    }

    final outputDir = Environment.outputDir;
    final output = path.join(outputDir, '$frameworkName.xcframework');
    Directory(outputDir).createSync(recursive: true);
    final existingOutput = Directory(output);
    if (existingOutput.existsSync()) {
      existingOutput.deleteSync(recursive: true);
    }
    runCommand(
      'xcodebuild',
      createXcframeworkArguments(frameworks: frameworks, output: output),
    );
  }
}
