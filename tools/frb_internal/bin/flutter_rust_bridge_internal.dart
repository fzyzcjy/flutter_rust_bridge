import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/bench.dart'
    as bench;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/build.dart'
    as test;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart'
    as generate;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/lint.dart'
    as lint;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart'
    as misc;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/post_release.dart'
    as post_release;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart'
    as release;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart'
    as build;
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

Future<void> main(List<String> args) async {
  final runner = CommandRunner<void>('flutter_rust_bridge_internal', '') //
    ..addCommands(generate.createCommands())
    ..addCommands(lint.createCommands())
    ..addCommands(test.createCommands())
    ..addCommands(bench.createCommands())
    ..addCommands(misc.createCommands())
    ..addCommands(release.createCommands())
    ..addCommands(post_release.createCommands())
    ..addCommands(build.createCommands());
  await runner.run(args);
}
