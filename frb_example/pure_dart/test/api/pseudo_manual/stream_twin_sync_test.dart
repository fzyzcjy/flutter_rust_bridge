// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/stream_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call handleStreamRealisticTwinSync', () async {
    final stream = handleStreamRealisticTwinSync(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });
}
