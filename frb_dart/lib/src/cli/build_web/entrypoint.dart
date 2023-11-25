import 'package:flutter_rust_bridge/src/cli/build_web/config.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';

/// {@macro flutter_rust_bridge.cli}
Future<void> run(List<String> args) async => await executeBuildWeb(parseConfig(args));
