import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'exceptions.dart';
import 'options.dart';
import 'target.dart';

final log = Logger('build_gradle');

class BuildGradle {
  BuildGradle({required this.userOptions});

  final CargokitUserOptions userOptions;

  Future<void> build() async {
    final targets = Environment.targetPlatforms.map((arch) {
      final target = Target.forFlutterName(arch);
      if (target == null) {
        throw UnsupportedPlatformException(
          'Android build received unsupported Flutter target "$arch". '
          'Expected one of the known Android target platform names.',
        );
      }
      return target;
    }).toList();

    final environment = BuildEnvironment.fromEnvironment(isAndroid: true);
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts(targets);

    for (final target in targets) {
      final outputDir = path.join(Environment.outputDir, target.android!);
      ArtifactMaterializer.copyDynamicLibraries(
        artifacts[target]!,
        outputDir: outputDir,
      );
    }
  }
}
