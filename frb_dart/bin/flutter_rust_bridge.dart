import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/entrypoint.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge',
      'This is usually not directly called. Please use `flutter_rust_bridge_codegen` instead.') //
    ..addCommand(BuildWebCommand());
  await runner.run(args);
}
