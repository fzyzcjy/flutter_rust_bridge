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
    SimpleCommand('bench-flamegraph-compile', benchFlamegraphCompile),
    SimpleConfigCommand(
        'bench-flamegraph-run',
        benchFlamegraphRun,
        _$populateBenchFlamegraphRunConfigParser,
        _$parseBenchFlamegraphRunConfigResult),
  ];
}

@CliOptions()
class BenchConfig {
  final String? filter;

  const BenchConfig({
    required this.filter,
  });
}

@CliOptions()
class BenchFlamegraphRunConfig {
  final String filter;
  final int loopCount;

  const BenchFlamegraphRunConfig({
    required this.filter,
    required this.loopCount,
  });
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

const _kPackage = 'frb_example/pure_dart';

Future<void> _dartBuild() async {
  await exec(
      'dart --enable-experiment=native-assets build benchmark/simple_benchmark.dart -o build/simple_benchmark/',
      relativePwd: _kPackage);
}

Future<void> benchDartNative(BenchConfig config) async {
  await runPubGetIfNotRunYet(_kPackage);
  await _dartBuild();
  await exec(
      'build/simple_benchmark/simple_benchmark.exe benchmark build/simple_benchmark/benchmark_result.json ${config.filter ?? ""}',
      relativePwd: _kPackage);
}

// ref:
// * How to symbolicate Dart things: https://github.com/dart-lang/sdk/issues/54207
// * Do profiling on MacOS with SIP enabled: https://poweruser.blog/using-dtrace-with-sip-enabled-3826a352e64b
Future<void> benchFlamegraphCompile() async {
  // Note: please manually create `dartaotruntime`
  // cp /Users/tom/fvm/default/bin/cache/dart-sdk/bin/dartaotruntime ~/temp/dartaotruntime
  // sudo codesign --remove ~/temp/dartaotruntime

  const dartSdk = '/Users/tom/fvm/default/bin/cache/dart-sdk';

  await exec(
      '$dartSdk/bin/dartaotruntime $dartSdk/bin/snapshots/gen_kernel_aot.dart.snapshot --platform=$dartSdk/lib/_internal/vm_platform_strong.dill --aot --tfa -o build/simple_benchmark.dill benchmark/simple_benchmark.dart',
      relativePwd: _kPackage);
  await exec(
      '$dartSdk/bin/utils/gen_snapshot --snapshot-kind=app-aot-assembly --assembly=build/simple_benchmark.S build/simple_benchmark.dill',
      relativePwd: _kPackage);
  await exec(
      'gcc -shared -o build/simple_benchmark.so build/simple_benchmark.S',
      relativePwd: _kPackage);

  // In order to build native Rust code
  await _dartBuild();
}

Future<void> benchFlamegraphRun(BenchFlamegraphRunConfig config) async {
  await exec(
      "sudo flamegraph -o build/my_flamegraph.svg -- ~/temp/dartaotruntime build/simple_benchmark.so loop build/whatever.out '${config.filter}' ${config.loopCount}",
      relativePwd: _kPackage);
}
