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
enum BuildTarget { windows, macos, linux, androidAab, androidApk, ios }

@CliOptions()
class BuildFlutterConfig {
  final BuildTarget target;

  const BuildFlutterConfig({required this.target});
}

// ref: https://docs.flutter.dev/deployment
Future<void> buildFlutter(BuildFlutterConfig config) async {
  const packages = ['frb_example/flutter_via_create', 'frb_example/gallery'];

  final outputDir = '${exec.pwd}target/build_flutter_output';
  Directory(outputDir).createSync(recursive: true);
  void copyArtifacts(String package, List<String> paths) {
    final packageOutputDir = '$outputDir/${package.replaceAll('/', '_')}';
    Directory(packageOutputDir).createSync(recursive: true);
    for (final path in paths) {
      copyPath('${exec.pwd}$package/$path', packageOutputDir);
    }
  }

  for (final package in packages) {
    switch (config.target) {
      case BuildTarget.windows:
        // https://docs.flutter.dev/deployment/windows
        // https://docs.flutter.dev/platform-integration/windows/building#compiling-with-visual-studio
        await exec('flutter build windows --verbose', relativePwd: package);
        copyArtifacts(package, ['build/windows/x64/runner/Release']);

      case BuildTarget.macos:
        // https://docs.flutter.dev/deployment/macos
        await exec('flutter build macos --verbose', relativePwd: package);
        copyArtifacts(package, ['build/macos/Build/Products/Release']);

      case BuildTarget.linux:
        // https://docs.flutter.dev/deployment/linux
        // https://stackoverflow.com/questions/73278689/how-to-run-a-standalone-linux-app-built-with-flutter
        await exec('flutter build linux --verbose', relativePwd: package);
        copyArtifacts(package, ['build/linux/x64/release/bundle']);

      case BuildTarget.androidAab:
        // https://docs.flutter.dev/deployment/android
        await exec('flutter build appbundle --verbose', relativePwd: package);
        copyArtifacts(package, ['build/app/outputs/bundle/release']);

      case BuildTarget.androidApk:
        // https://docs.flutter.dev/deployment/android
        await exec('flutter build apk --verbose', relativePwd: package);
        copyArtifacts(package, ['build/app/outputs/apk/release']);

      case BuildTarget.ios:
        // https://docs.flutter.dev/deployment/ios
        await exec(
          'flutter build ipa --no-codesign --verbose',
          relativePwd: package,
        );
        copyArtifacts(package, ['build/ios/archive']);
    }
  }
}
