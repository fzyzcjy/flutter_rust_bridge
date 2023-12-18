import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/bench.dart'
    as misc;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart'
    as generate;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart'
    as lint;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart'
    as bench;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart'
    as test;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    ..addCommands(generate.createCommands())
    ..addCommands(lint.createCommands())
    ..addCommands(test.createCommands())
    ..addCommands(bench.createCommands())
    ..addCommands(misc.createCommands());
  await runner.run(args);
}
