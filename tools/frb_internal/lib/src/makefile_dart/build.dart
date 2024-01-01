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
    SimpleConfigCommand('build-flutter', buildFlutter,
        _$populateBuildFlutterConfigParser, _$parseBuildFlutterConfigResult),
  ];
}

enum BuildTarget {
  windows,
  macos,
  linux,
  androidAab,
  androidApk,
  ios,
}

@CliOptions()
class BuildFlutterConfig {
  final BuildTarget target;

  const BuildFlutterConfig({
    required this.target,
  });
}

// ref: https://docs.flutter.dev/deployment
Future<void> buildFlutter(BuildFlutterConfig config) async {
  const package = 'frb_example/flutter_via_create';
  final outputDir = '${exec.pwd}target/build_flutter_output';
  Directory(outputDir).createSync(recursive: true);

  switch (config.platform) {
    case BuildTarget.windows:
      TODO;

    case BuildTarget.macos:
      // https://docs.flutter.dev/deployment/macos
      await exec('flutter build macos --verbose', relativePwd: package);
      copyPathSync(TODO, outputDir);

    case BuildTarget.linux:
      TODO;

    case BuildTarget.android:
      // https://docs.flutter.dev/deployment/android
      await exec('flutter build apk --verbose', relativePwd: package);
      copyPathSync('${exec.pwd}/build/app/outputs/apk/release', outputDir);

    case BuildTarget.ios:
      // https://docs.flutter.dev/deployment/ios
      await exec('flutter build ipa --verbose', relativePwd: package);
      copyPathSync('${exec.pwd}/build/ios/archive', outputDir);
      copyPathSync('${exec.pwd}/build/ios/ipa', outputDir);
  }
}
