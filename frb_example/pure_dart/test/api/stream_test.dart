// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/stream.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call funcStreamRealisticTwinNormal', () async {
    final stream = funcStreamRealisticTwinNormal(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      debugPrint("output from func_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });

  test('dart call funcStreamSinkArgPositionTwinNormal', () async {
    // We only care about whether the codegen can understand StreamSink
    // as non-first argument in Rust, thus we do not test the return values.
    funcStreamSinkArgPositionTwinNormal(a: 100, b: 200);
  });

  test('call funcStreamReturnErrorTwinNormal', () async {
    await expectLater(
        () async => funcStreamReturnErrorTwinNormal(),
        throwsA(isA<AnyhowException>()
            .having((x) => x.message, 'message', 'deliberate error')));
  });

  test('call funcStreamReturnPanicTwinNormal', () async {
    await expectLater(
        () async => funcStreamReturnPanicTwinNormal(),
        throwsA(isA<PanicException>()
            .having((x) => x.message, 'message', 'deliberate panic')));
  });
}
