import 'package:build_tool/src/environment.dart';
import 'package:build_tool/src/exceptions.dart';
import 'package:test/test.dart';

void main() {
  tearDown(() {
    Environment.testEnvironment = null;
  });

  test('reads environment values from test override', () {
    Environment.testEnvironment = {
      'CARGOKIT_CONFIGURATION': 'Debug',
      'CARGOKIT_TARGET_TEMP_DIR': '/tmp/target',
      'CARGOKIT_OUTPUT_DIR': '/tmp/output',
      'CARGOKIT_MANIFEST_DIR': '/tmp/manifest',
      'CARGOKIT_ROOT_PROJECT_DIR': '/tmp/root',
      'CARGOKIT_TARGET_PLATFORM': 'linux-x64',
      'CARGOKIT_TARGET_PLATFORMS': 'android-arm64,android-x64',
      'CARGOKIT_MIN_SDK_VERSION': '21',
      'CARGOKIT_NDK_VERSION': '26.1.10909125',
      'CARGOKIT_SDK_DIR': '/tmp/sdk',
      'CARGOKIT_JAVA_HOME': '/tmp/java',
      'CARGOKIT_DARWIN_PLATFORM_NAME': 'iphonesimulator',
      'CARGOKIT_DARWIN_ARCHS': 'arm64 x86_64',
    };

    expect(Environment.configuration, 'debug');
    expect(Environment.targetTempDir, '/tmp/target');
    expect(Environment.outputDir, '/tmp/output');
    expect(Environment.manifestDir, '/tmp/manifest');
    expect(Environment.rootProjectDir, '/tmp/root');
    expect(Environment.targetPlatform, 'linux-x64');
    expect(Environment.targetPlatforms, ['android-arm64', 'android-x64']);
    expect(Environment.minSdkVersion, '21');
    expect(Environment.ndkVersion, '26.1.10909125');
    expect(Environment.sdkPath, '/tmp/sdk');
    expect(Environment.javaHome, '/tmp/java');
    expect(Environment.darwinPlatformName, 'iphonesimulator');
    expect(Environment.darwinArchs, ['arm64', 'x86_64']);
  });

  test('throws typed error for missing variables', () {
    Environment.testEnvironment = {
      'CARGOKIT_CONFIGURATION': 'release',
    };

    expect(
      () => Environment.targetTempDir,
      throwsA(
        isA<EnvironmentVariableException>().having(
          (e) => e.toString(),
          'message',
          contains('CARGOKIT_TARGET_TEMP_DIR'),
        ),
      ),
    );
  });
}
