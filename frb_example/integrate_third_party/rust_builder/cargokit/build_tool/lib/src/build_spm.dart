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
  required Iterable<String> libraries,
  required String output,
}) {
  return [
    '-create-xcframework',
    for (final library in libraries) ...['-library', library],
    '-output',
    output,
  ];
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
    final artifacts = await provider.getArtifacts(targets);

    final crateName = environment.crateInfo.packageName;
    final workDir =
        path.join(environment.targetTempDir, 'xcframework', crateName);
    final libraries = <String>[];

    Directory(workDir).createSync(recursive: true);

    for (final entry in targetGroups.entries) {
      final inputLibraries = entry.value.map((target) {
        final staticLibraries = (artifacts[target] ?? [])
            .where((artifact) => artifact.type == AritifactType.staticlib)
            .toList();
        if (staticLibraries.length != 1) {
          throw StateError(
            'Expected one static library for $target. Swift Package Manager '
            'XCFramework builds require the Rust crate to declare '
            'crate-type = ["staticlib"].',
          );
        }
        return staticLibraries.single.path;
      }).toList();

      final output = path.join(workDir, 'lib$crateName-${entry.key}.a');
      if (inputLibraries.length == 1) {
        File(inputLibraries.single).copySync(output);
      } else {
        runCommand('lipo', [
          '-create',
          ...inputLibraries,
          '-output',
          output,
        ]);
      }
      libraries.add(output);
    }

    final outputDir = Environment.outputDir;
    final output = path.join(outputDir, '$crateName.xcframework');
    Directory(outputDir).createSync(recursive: true);
    final existingOutput = Directory(output);
    if (existingOutput.existsSync()) {
      existingOutput.deleteSync(recursive: true);
    }
    runCommand(
      'xcodebuild',
      createXcframeworkArguments(libraries: libraries, output: output),
    );
  }
}
