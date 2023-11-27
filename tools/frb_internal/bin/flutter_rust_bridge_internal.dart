import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart' as generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/entrypoint.dart' as makefile_dart;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

Future<void> main(List<String> args) async {
  // Use project root directory
  Directory.current = Directory.current.parent.parent;
 
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    // TODO mv
    ..addCommand(SimpleCommand('generate-test', generator.generate))
    ..addCommands(makefile_dart.createCommands());
  await runner.run(args);
}
