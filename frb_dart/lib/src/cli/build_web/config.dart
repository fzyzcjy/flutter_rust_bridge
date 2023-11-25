import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge/src/cli/build_web/executor.dart';

/// {@template flutter_rust_bridge.cli}
/// This is mainly used for cli, not for direct function call.
/// {@endtemplate}
@CliOptions()
class Config {
  // TODO
}

/// {@macro flutter_rust_bridge.cli}
BuildWebArgs parseConfig(List<String> args) {
  return BuildWebArgs(
    wasmOutput: wasmOutput,
    release: release,
    rustCrateDir: rustCrateDir,
    wasmPackArgs: wasmPackArgs,
    enableWasmBindgen: enableWasmBindgen,
    wasmBindgenArgs: wasmBindgenArgs,
  );
}
