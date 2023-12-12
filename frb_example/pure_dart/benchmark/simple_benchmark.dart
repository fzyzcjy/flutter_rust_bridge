// ignore_for_file: invalid_use_of_internal_member, invalid_use_of_protected_member

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import 'src/benchmark_classes.dart';
import 'src/benchmark_utils.dart';

Future<void> main(List<String> args) async {
  await RustLib.init();

  final [modeStr, pathOutput, partialName, ...] = args;
  final mode = _Mode.values.byName(modeStr);

  final filterStr = args.get(3) ?? '.*';
  final filterRegex = RegExp(filterStr);

  final emitter = JsonEmitter(namer: (x) => 'PureDart_${x}_$partialName');
  final allBenchmarks = createBenchmarks(emitter: emitter);
  final interestBenchmarks = [
    for (final b in allBenchmarks)
      if (filterRegex.hasMatch(b.name)) b
  ];

  for (final benchmark in interestBenchmarks) {
    switch (mode) {
      case _Mode.benchmark:
        await benchmark.reportMaybeAsync();

      case _Mode.loop:
        final loopCount = int.parse(args[4]);
        final stopwatch = Stopwatch()..start();
        print('Mode=loop START loopCount=$loopCount');
        await benchmark.loop(loopCount);
        print('Mode=loop END totalTime(us)=${stopwatch.elapsedMicroseconds}');
    }
  }

  final output = jsonEncode(emitter.items);
  print('Write reports to $pathOutput with output=$output');
  File(pathOutput).writeAsStringSync(output);
 
  // To avoid dead code elimination
  print(dummyValue);

  // to avoid hang forever
  exit(0);
}

extension<T> on List<T> {
  T? get(int index) => index >= 0 && index < length ? this[index] : null;
}

enum _Mode { benchmark, loop }
