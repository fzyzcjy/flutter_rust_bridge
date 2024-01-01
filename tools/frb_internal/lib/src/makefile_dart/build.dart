// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'build.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('build-flutter', buildFlutter,
        _$populateBuildFlutterConfigParser, _$parseBuildFlutterConfigResult),
  ];
}

enum BuildPlatform { windows, macos, linux, android, ios }

@CliOptions()
class BuildFlutterConfig {
  final BuildPlatform platform;

  const BuildFlutterConfig({
    required this.platform,
  });
}

Future<void> buildFlutter(BuildFlutterConfig config) async {
  TODO;
}
