import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'bench.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('bench-dart-native', benchDartNative,
        _$populateBenchConfigParser, _$parseBenchConfigResult),
  ];
}

@CliOptions()
class BenchConfig {
  final String? filter;

  const BenchConfig({
    required this.filter,
  });
}

Future<void> benchDartNative(BenchConfig config) async {
  const package = 'frb_example/pure_dart';
  await runPubGetIfNotRunYet(package);
  await exec(
      'dart --enable-experiment=native-assets build benchmark/simple_benchmark.dart -o build/simple_benchmark/',
      relativePwd: package);
  await exec(
      'build/simple_benchmark/simple_benchmark.exe benchmark build/simple_benchmark/benchmark_result.json ${config.filter ?? ""}',
      relativePwd: package);
}
