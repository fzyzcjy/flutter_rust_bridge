import 'dart:io';

import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'exceptions.dart';
import 'options.dart';
import 'target.dart';
import 'util.dart';

class BuildPod {
  BuildPod({required this.userOptions});

  final CargokitUserOptions userOptions;

  Future<void> build() async {
    final targets = Environment.darwinArchs.map((arch) {
      final target = Target.forDarwin(
          platformName: Environment.darwinPlatformName, darwinAarch: arch);
      if (target == null) {
        throw UnsupportedPlatformException(
          'Darwin build received unsupported platform "${Environment.darwinPlatformName}" '
          'with architecture "$arch".',
        );
      }
      return target;
    }).toList();

    final environment = BuildEnvironment.fromEnvironment(isAndroid: false);
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts(targets);

    void performLipo(String targetFile, Iterable<String> sourceFiles) {
      runCommand("lipo", [
        '-create',
        ...sourceFiles,
        '-output',
        targetFile,
      ]);
    }

    final outputDir = Environment.outputDir;

    Directory(outputDir).createSync(recursive: true);

    final staticLibs = ArtifactMaterializer.flattenForType(
      artifacts,
      type: ArtifactType.staticlib,
    );
    final dynamicLibs = ArtifactMaterializer.flattenForType(
      artifacts,
      type: ArtifactType.dylib,
    );

    final libName = environment.crateInfo.packageName;

    // If there is static lib, use it and link it with pod
    if (staticLibs.isNotEmpty) {
      final finalTargetFile = path.join(outputDir, "lib$libName.a");
      performLipo(finalTargetFile, staticLibs.map((e) => e.path));
    } else {
      // Otherwise try to replace bundle dylib with our dylib
      final bundlePaths = [
        '$libName.framework/Versions/A/$libName',
        '$libName.framework/$libName',
      ];

      for (final bundlePath in bundlePaths) {
        final targetFile = path.join(outputDir, bundlePath);
        if (File(targetFile).existsSync()) {
          performLipo(targetFile, dynamicLibs.map((e) => e.path));

          // Replace absolute id with @rpath one so that it works properly
          // when moved to Frameworks.
          runCommand("install_name_tool", [
            '-id',
            '@rpath/$bundlePath',
            targetFile,
          ]);
          return;
        }
      }
      throw ArtifactException(
        'Unable to find an existing framework binary to replace with the built dylib '
        'in "$outputDir".',
      );
    }
  }
}
