// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';

part 'bench.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('bench-dart-native', benchDartNative,
        _$populateBenchConfigParser, _$parseBenchConfigResult),
    SimpleCommand('bench-merge', benchMerge),
  ];
}

@CliOptions()
class BenchConfig {
  final String partialName;
  final String? filter;

  const BenchConfig({
    required this.partialName,
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
      'build/simple_benchmark/simple_benchmark.exe benchmark build/simple_benchmark/benchmark_result.json ${config.partialName} ${config.filter ?? ""}',
      relativePwd: package);
}

Future<void> benchMerge() async {
  final pattern = '${exec.pwd}downloaded-artifacts/**/*.json';
  print('glob pattern: $pattern');

  final inputFiles = [
    for (final file in Glob(pattern).listSync())
      if (file is File) file as File
  ];
  print('benchMerge inputFiles=$inputFiles');
  if (inputFiles.isEmpty) throw Exception('No input files, are you sure?');

  final outputContent = jsonEncode([
    for (final file in inputFiles)
      ...(jsonDecode(file.readAsStringSync()) as List<dynamic>)
  ]);

  final pathOutput = '${exec.pwd}merged_benchmark.json';
  File(pathOutput).writeAsStringSync(outputContent);

  await exec('cat $pathOutput');
}
