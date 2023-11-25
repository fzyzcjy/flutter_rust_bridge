import 'package:flutter_rust_bridge/src/cli/build_web/config.dart';

/// {@macro flutter_rust_bridge.cli}
Future<void> executeBuildWeb(List<String> args) async {
  final config = parseConfig(args);
}
