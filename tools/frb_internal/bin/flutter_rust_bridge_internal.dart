import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart' as lint;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart' as misc;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    ..addCommands(misc.createCommands())
    ..addCommands(lint.createCommands());
  await runner.run(args);
}
