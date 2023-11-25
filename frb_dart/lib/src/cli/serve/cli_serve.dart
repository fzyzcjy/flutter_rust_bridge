// ignore_for_file: avoid_print

import 'package:flutter_rust_bridge/src/cli/serve/build_web.dart';
import 'package:flutter_rust_bridge/src/cli/serve/config.dart';
import 'package:flutter_rust_bridge/src/cli/serve/run_server.dart';

/// {@macro flutter_rust_bridge.internal}
void runCliServe(List<String> args) async {
  final config = parseConfig(args);

  if (config.cliOpts.build) {
    TODO;
  }

  await runServer(config);
}
