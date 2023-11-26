import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_utils/src/commands/serve_web_command.dart';
import 'package:flutter_rust_bridge_utils/src/commands/test_web_command.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_utils', '') //
    ..addCommand(ServeWebCommand())
    ..addCommand(TestWebCommand());
  await runner.run(args);
}
