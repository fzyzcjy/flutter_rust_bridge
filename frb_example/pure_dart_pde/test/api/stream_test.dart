// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/stream.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcStreamSinkArgPositionTwinNormal', () async {
    // We only care about whether the codegen can understand StreamSink
    // as non-first argument in Rust, thus we do not test the return values.
    // ignore: unawaited_futures
    funcStreamSinkArgPositionTwinNormal(a: 100, b: 200);
  });

  test('call funcStreamReturnErrorTwinNormal', () async {
    await expectLater(
      () async {
        await for (final _ in await funcStreamReturnErrorTwinNormal()) {}
      },
      throwsA(isA<AnyhowException>()
          .having((x) => x.message, 'message', startsWith('deliberate error'))),
    );
  });

  // TODO implement in web
  test('call funcStreamReturnPanicTwinNormal', skip: kIsWeb, () async {
    await expectRustPanic(
      () async {
        await for (final _ in await funcStreamReturnPanicTwinNormal()) {}
      },
      'TwinNormal',
      messageOnNative: 'deliberate panic',
    );
  });

  Future<void> testHandleStream(
      Stream<LogTwinNormal> Function({required int key, required int max})
          handleStreamFunction) async {
    final max = 5;
    final key = 8;
    final stream = handleStreamFunction(key: key, max: max);
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream_x's stream: $value");
      expect(value.key, key);
      cnt++;
    }
    expect(cnt, max);
  }

  test('dart call handle_stream_sink_at_1', () {
    testHandleStream(handleStreamSinkAt1TwinNormal);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(handleStreamSinkAt2TwinNormal);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(handleStreamSinkAt3TwinNormal);
  });

  test('stream_sink_fixed_sized_primitive_array_twin_normal', () async {
    final output =
        await streamSinkFixedSizedPrimitiveArrayTwinNormal().toList();
    expect(output, [
      orderedEquals([1, 2]),
      orderedEquals([3, 4]),
    ]);
  });

  test('stream_sink_inside_vec_twin_normal', () async {
    final sinks = [RustStreamSink<int>(), RustStreamSink<int>()];
    await streamSinkInsideVecTwinNormal(arg: sinks);
    expect(await sinks[0].stream.toList(), [100, 200]);
    expect(await sinks[1].stream.toList(), [100, 200]);
  });

  test('stream_sink_inside_struct_twin_normal', () async {
    final arg = MyStructContainingStreamSinkTwinNormal(
        a: 1000, b: RustStreamSink<int>());
    await streamSinkInsideStructTwinNormal(arg: arg);
    expect(await arg.b.stream.toList(), [1000]);
  });

  test('func_stream_add_value_and_error_twin_normal', () async {
    final stream = await funcStreamAddValueAndErrorTwinNormal();
    final events = <String>[];
    final onDone = Completer<void>();
    stream.listen(
      (e) => events.add('data $e'),
      onError: (e, s) {
        print('onError $e $s');
        events.add('error $e');
      },
      onDone: () => onDone.complete(),
    );
    await onDone.future;
    expect(events, ['data 100', 'data 200', contains('deliberate error')]);
  });
}
