import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/entrypoint.dart' as makefile_dart;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    // TODO mv
    ..addCommand(SimpleCommand(name: 'generate-test', executor: () => generator.generate()))
    ..addCommands(makefile_dart.createCommands());
  await runner.run(args);
}
