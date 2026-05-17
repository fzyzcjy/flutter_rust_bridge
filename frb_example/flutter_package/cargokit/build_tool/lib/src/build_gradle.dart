/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';

import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'options.dart';
import 'target.dart';

final log = Logger('build_gradle');

class BuildGradle {
  BuildGradle({required this.userOptions});

  final CargokitUserOptions userOptions;

  Future<void> build() async {
    final targets = Environment.targetPlatforms
        .map((arch) {
          final target = Target.forFlutterName(arch);
          if (target == null) {
            throw Exception(
                "Unknown darwin target or platform: $arch, ${Environment.darwinPlatformName}");
          }
          return target;
        })
        .toSet()
        .toList();

    final environment = BuildEnvironment.fromEnvironment(isAndroid: true);
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts(targets);

    await Future.wait(targets.map((target) async {
      final libs = artifacts[target]!;
      final outputDir = path.join(Environment.outputDir, target.android!);
      await Directory(outputDir).create(recursive: true);

      for (final lib in libs) {
        if (lib.type == AritifactType.dylib) {
          await File(lib.path).copy(path.join(outputDir, lib.finalFileName));
        }
      }
    }));
  }
}
