// ignore_for_file: avoid_print

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('TODO', postReleaseWhat),
  ];
}

Future<void> postReleaseWhat() async {
  // TODO
}
