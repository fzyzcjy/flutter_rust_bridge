import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'exceptions.dart';
import 'options.dart';
import 'target.dart';

class BuildCMake {
  final CargokitUserOptions userOptions;

  BuildCMake({required this.userOptions});

  Future<void> build() async {
    final targetPlatform = Environment.targetPlatform;
    final target = Target.forFlutterName(Environment.targetPlatform);
    if (target == null) {
      throw UnsupportedPlatformException(
        'CMake build received target platform "$targetPlatform". '
        'Expected one of the known Flutter desktop target names.',
      );
    }

    final environment = BuildEnvironment.fromEnvironment(isAndroid: false);
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts([target]);

    ArtifactMaterializer.copyDynamicLibraries(
      artifacts[target]!,
      outputDir: Environment.outputDir,
    );
  }
}
