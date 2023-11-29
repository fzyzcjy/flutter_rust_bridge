import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('bench-dart-native', benchDartNative),
  ];
}

Future<void> benchDartNative() async {
  TODO;
}
