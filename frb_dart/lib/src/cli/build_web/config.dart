import 'package:build_cli_annotations/build_cli_annotations.dart';

/// {@template flutter_rust_bridge.cli}
/// This is mainly used for cli, not for direct function call.
/// {@endtemplate}
@CliOptions()
class Opts {
  // TODO
}

/// {@macro flutter_rust_bridge.internal}
class Config {
  /// {@macro flutter_rust_bridge.internal}
  final Opts cliOpts;

  /// {@macro flutter_rust_bridge.internal}
  const Config({
    required this.cliOpts,
  });
}

/// {@macro flutter_rust_bridge.internal}
Opts parseConfig(List<String> args) {
  return TODO;
}
