// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:io/io.dart';

part 'build.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
      'build-flutter',
      buildFlutter,
      _$populateBuildFlutterConfigParser,
      _$parseBuildFlutterConfigResult,
    ),
  ];
}

// We do not test web, since it is already tested when building the demo on website
enum BuildTarget { windows, macos, linux, androidAab, androidApk, ios, ohos }

@CliOptions()
class BuildFlutterConfig {
  final BuildTarget target;

  const BuildFlutterConfig({required this.target});
}

// ref: https://docs.flutter.dev/deployment
Future<void> buildFlutter(BuildFlutterConfig config) async {
  // TODO also consider the gallery?
  const package = 'frb_example/flutter_via_create';

  final outputDir = '${exec.pwd}target/build_flutter_output';
  Directory(outputDir).createSync(recursive: true);
  void copyArtifacts(List<String> paths) {
    for (final path in paths) {
      copyPath('${exec.pwd}$package/$path', outputDir);
    }
  }

  switch (config.target) {
    case BuildTarget.windows:
      // https://docs.flutter.dev/deployment/windows
      // https://docs.flutter.dev/platform-integration/windows/building#compiling-with-visual-studio
      await exec('flutter build windows --verbose', relativePwd: package);
      copyArtifacts(['build/windows/x64/runner/Release']);

    case BuildTarget.macos:
      // https://docs.flutter.dev/deployment/macos
      await exec('flutter build macos --verbose', relativePwd: package);
      copyArtifacts(['build/macos/Build/Products/Release']);

    case BuildTarget.linux:
      // https://docs.flutter.dev/deployment/linux
      // https://stackoverflow.com/questions/73278689/how-to-run-a-standalone-linux-app-built-with-flutter
      await exec('flutter build linux --verbose', relativePwd: package);
      copyArtifacts([
        linuxBuildBundlePathForTesting(
          machineArchitecture: currentMachineArchitectureForTesting(),
        ),
      ]);

    case BuildTarget.androidAab:
      // https://docs.flutter.dev/deployment/android
      await exec('flutter build appbundle --verbose', relativePwd: package);
      copyArtifacts(['build/app/outputs/bundle/release']);

    case BuildTarget.androidApk:
      // https://docs.flutter.dev/deployment/android
      await exec('flutter build apk --verbose', relativePwd: package);
      copyArtifacts(['build/app/outputs/apk/release']);

    case BuildTarget.ios:
      // https://docs.flutter.dev/deployment/ios
      await exec(
        'flutter build ipa --no-codesign --verbose',
        relativePwd: package,
      );
      copyArtifacts(['build/ios/archive']);

    case BuildTarget.ohos:
      await exec(
        'flutter build hap --no-codesign --verbose',
        relativePwd: package,
      );
      copyArtifacts(['build/ohos/hap']);
  }
}

String linuxBuildBundlePathForTesting({required String machineArchitecture}) =>
    'build/linux/${_linuxFlutterArchitecture(machineArchitecture)}/release/bundle';

String currentMachineArchitectureForTesting() {
  final result = Process.runSync('uname', ['-m']);
  if (result.exitCode != 0) {
    throw StateError('Failed to detect machine architecture: ${result.stderr}');
  }

  return (result.stdout as String).trim();
}

String _linuxFlutterArchitecture(String machineArchitecture) =>
    switch (machineArchitecture) {
      'x86_64' || 'amd64' => 'x64',
      'aarch64' || 'arm64' => 'arm64',
      'riscv64' => 'riscv64',
      _ => throw UnsupportedError(
        'Unsupported Linux machine architecture: $machineArchitecture',
      ),
    };
