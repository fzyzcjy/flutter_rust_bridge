// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream_misc_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/stream_misc_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcStreamRealisticTwinSse', () async {
    final stream = funcStreamRealisticTwinSse(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      print("output from func_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });

  test('streamSinkDartAsyncTwinSse', () async {
    final stream = await streamSinkDartAsyncTwinSse();
    expect(await stream.toList(), [100]);
  });
}
