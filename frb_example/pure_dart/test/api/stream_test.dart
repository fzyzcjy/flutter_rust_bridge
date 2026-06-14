// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"], "skipPde": true}

import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/stream.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
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
      throwsA(
        isA<AnyhowException>().having(
          (x) => x.message,
          'message',
          startsWith('deliberate error'),
        ),
      ),
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
        handleStreamFunction,
  ) async {
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

  test('dart call handle_stream_sink_at_1', () async {
    await testHandleStream(handleStreamSinkAt1TwinNormal);
  });

  test('dart call handle_stream_sink_at_2', () async {
    await testHandleStream(handleStreamSinkAt2TwinNormal);
  });

  test('dart call handle_stream_sink_at_3', () async {
    await testHandleStream(handleStreamSinkAt3TwinNormal);
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

  test('stream_sink_inside_vec_twin_normal_with_replay', () async {
    final sinks = [
      RustStreamSink<int>(replay: true),
      RustStreamSink<int>(replay: true),
    ];
    await streamSinkInsideVecTwinNormal(arg: sinks);
    expect(await sinks[0].stream.toList(), [100, 200]);
    expect(await sinks[1].stream.toList(), [100, 200]);
  });

  test('stream_sink_inside_struct_twin_normal', () async {
    final arg = MyStructContainingStreamSinkTwinNormal(
      a: 1000,
      b: RustStreamSink<int>(),
    );
    await streamSinkInsideStructTwinNormal(arg: arg);
    expect(await arg.b.stream.toList(), [1000]);
  });

  test('stream_sink_inside_struct_twin_normal_with_replay', () async {
    final arg = MyStructContainingStreamSinkTwinNormal(
        a: 1000, b: RustStreamSink<int>(replay: true));
    await streamSinkInsideStructTwinNormal(arg: arg);
    expect(await arg.b.stream.toList(), [1000]);
  });

  test('rust_stream_sink_stream_replay_true_late_listener', () async {
    final sink = RustStreamSink<int>(replay: true);
    await streamSinkInsideVecTwinNormal(arg: [sink]);
    final expected = [100, 200];
    expect(await sink.stream.toList(), expected);
    expect(await sink.stream.toList(), isEmpty);
  });

  test('rust_stream_sink_stream_replay_false_late_listener', () async {
    final sink = RustStreamSink<int>(replay: false);
    await streamSinkInsideVecTwinNormal(arg: [sink]);
    final values = await sink.stream.toList();
    expect(values, everyElement(anyOf(100, 200)));
    if (values.isNotEmpty) {
      expect(values.last, 200);
    }
  });

  test('rust_stream_sink_stream_replay_false_live_listener', () async {
    final sink = RustStreamSink<int>(replay: false);
    final valuesFuture = sink.stream.take(2).toList();
    await streamSinkInsideVecTwinNormal(arg: [sink]);
    expect(await valuesFuture, [100, 200]);
  });

  test('rust_stream_sink_stream_replay_true_large_data', () async {
    final values = await streamSinkEmitManyTwinNormal(count: 2000).toList();
    expect(values.length, 2000);
    expect(values.first, 0);
    expect(values.last, 1999);
  });

  test('rust_stream_sink_stream_listener_can_cancel_while_sender_active',
      () async {
    final reached = Completer<void>();
    var received = 0;
    late final StreamSubscription<int> subscription;
    final stream = streamSinkEmitRangeThenHoldTwinNormal(
      count: 5000,
      holdMillis: BigInt.from(300),
    );
    subscription = stream.listen((_) {
      received++;
      if (!reached.isCompleted && received >= 200) {
        reached.complete();
        unawaited(subscription.cancel());
      }
    });

    await reached.future.timeout(const Duration(seconds: 8));
    await Future<void>.delayed(const Duration(milliseconds: 350));
    await subscription.cancel();
    expect(received, greaterThanOrEqualTo(50));
  });

  test('rust_stream_sink_stream_done_after_sender_close', () async {
    final values = await streamSinkEmitRangeThenHoldTwinNormal(
      count: 64,
      holdMillis: BigInt.from(50),
    ).toList().timeout(const Duration(seconds: 8));
    expect(values.length, 64);
    expect(values.first, 0);
    expect(values.last, 63);
  });

  test('stored_stream_sink_no_listener_retains_history', skip: kIsWeb,
      () async {
    await clearStoredStreamSinkTwinNormal();
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    await storedStreamSinkEmitManyTwinNormal(count: 1024);
    final values = await stream.take(1024).toList();
    expect(values.length, 1024);
    expect(values.first, 0);
    expect(values.last, 1023);
  });

  test('stored_stream_sink_no_listener_and_clear', skip: kIsWeb, () async {
    await clearStoredStreamSinkTwinNormal();
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    await storedStreamSinkEmitManyTwinNormal(count: 128);
    await clearStoredStreamSinkTwinNormal();
    final values = await stream.toList();
    expect(values.length, 128);
  });

  test('stored_stream_sink_multi_listener_receive_massive_data', skip: kIsWeb,
      () async {
    await clearStoredStreamSinkTwinNormal();
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    final listener1Future = stream.take(512).toList();
    final listener2Future = stream.take(512).toList();
    await storedStreamSinkEmitManyTwinNormal(count: 512);

    final listener1Values = await listener1Future;
    final listener2Values = await listener2Future;
    expect(listener1Values.length, 512);
    expect(listener2Values.length, 512);
    expect(listener1Values.last, 511);
    expect(listener2Values.last, 511);
  });

  test('stored_stream_sink_sender_alive_listener_cancel_then_clear',
      skip: kIsWeb, () async {
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    final reached = Completer<void>();
    var received = 0;
    late final StreamSubscription<int> subscription;
    subscription = stream.listen((_) {
      received++;
      if (!reached.isCompleted && received >= 50) reached.complete();
    });

    await storedStreamSinkStartSpamTwinNormal(
      total: 2000,
      intervalMillis: BigInt.zero,
    );
    await reached.future.timeout(const Duration(seconds: 8));
    await subscription.cancel();
    expect(received, greaterThanOrEqualTo(50));

    await clearStoredStreamSinkTwinNormal();
  });

  test('stored_stream_sink_error_single_listener', skip: kIsWeb, () async {
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    final events = <String>[];
    final done = Completer<void>();
    stream.listen(
      (e) => events.add('data $e'),
      onError: (e, s) => events.add('error $e'),
      onDone: done.complete,
    );

    await storedStreamSinkEmitManyTwinNormal(count: 2);
    await storedStreamSinkEmitErrorTwinNormal(message: 'stored sink error');
    await done.future.timeout(const Duration(seconds: 2));

    expect(
      events.any(
          (e) => e.startsWith('error ') && e.contains('stored sink error')),
      isTrue,
    );
    final dataEvents = events.where((e) => e.startsWith('data '));
    expect(dataEvents.every((e) => e == 'data 0' || e == 'data 1'), isTrue);
  });

  test('stored_stream_sink_error_multi_listener', skip: kIsWeb, () async {
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    final events1 = <String>[];
    final events2 = <String>[];
    final done1 = Completer<void>();
    final done2 = Completer<void>();

    stream.listen(
      (e) => events1.add('data $e'),
      onError: (e, s) => events1.add('error $e'),
      onDone: done1.complete,
    );
    stream.listen(
      (e) => events2.add('data $e'),
      onError: (e, s) => events2.add('error $e'),
      onDone: done2.complete,
    );

    await storedStreamSinkEmitManyTwinNormal(count: 2);
    await storedStreamSinkEmitErrorTwinNormal(message: 'stored sink error');
    await Future.wait([
      done1.future.timeout(const Duration(seconds: 2)),
      done2.future.timeout(const Duration(seconds: 2)),
    ]);

    expect(events1.last, contains('stored sink error'));
    expect(events2.last, contains('stored sink error'));
  });

  test('stored_stream_sink_error_no_listener', skip: kIsWeb, () async {
    final stream = storeStreamSinkTwinNormal();
    addTearDown(clearStoredStreamSinkTwinNormal);

    Object? emitError;
    try {
      await storedStreamSinkEmitErrorTwinNormal(message: 'stored sink error');
    } catch (e) {
      emitError = e;
    }

    if (emitError != null) {
      expect(
        emitError,
        isA<AnyhowException>()
            .having((x) => x.message, 'message', contains('stored sink error')),
      );
      return;
    }

    try {
      final values = await stream
          .take(1024)
          .toList()
          .timeout(const Duration(seconds: 2), onTimeout: () => <int>[]);
      expect(values, isEmpty);
    } catch (e) {
      expect(
        e,
        isA<AnyhowException>()
            .having((x) => x.message, 'message', contains('stored sink error')),
      );
    }
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
