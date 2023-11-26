import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/commands/generate_test_command.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    ..addCommand(GenerateTestCommand());
  await runner.run(args);
}
