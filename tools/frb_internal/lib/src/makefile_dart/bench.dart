import 'package:args/command_runner.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

List<Command<void>> createCommands() {
  return [
    SimpleCommand('bench-dart-native', benchDartNative),
  ];
}

Future<void> benchDartNative() async {
  const package = 'frb_example/pure_dart';
  await runPubGetIfNotRunYet(package);
  await exec(
      'dart --enable-experiment=native-assets build benchmark/simple_benchmark.dart -o build/simple_benchmark/',
      relativePwd: package);
  await exec(
      'build/simple_benchmark/simple_benchmark.exe build/simple_benchmark/benchmark_result.json',
      relativePwd: package);
}
