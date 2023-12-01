// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import 'src/benchmark_classes.dart';
import 'src/benchmark_utils.dart';

Future<void> main(List<String> args) async {
  await RustLib.init();

  final [pathOutput, partialName, ...] = args;
  final filterRegex = RegExp(args.length >= 3 ? args[2] : '.*');

  final emitter = JsonEmitter(namer: (x) => 'PureDart_${x}_$partialName');
  final benchmarks = createBenchmarks(emitter: emitter);

  for (final benchmark in benchmarks) {
    if (!filterRegex.hasMatch(benchmark.name)) {
      print('Skip ${benchmark.name} since not match filter $filterRegex');
      continue;
    }
    await benchmark.reportMaybeAsync();
  }

  final output = jsonEncode(emitter.items);
  print('Write reports to $pathOutput with output=$output');
  File(pathOutput).writeAsStringSync(output);

  // to avoid hang forever
  exit(0);
}
