import 'dart:async';

import 'package:flutter_rust_bridge/src/codec/dco.dart';
import 'package:flutter_rust_bridge/src/stream/stream_sink.dart';
import 'package:test/test.dart';

void main() {
  const codec = DcoCodec<int, Exception>(
    decodeSuccessData: _decodeInt,
    decodeErrorData: null,
  );

  RustStreamSink<int> createPortSink() {
    final sink = RustStreamSink<int>();
    sink.setupAndSerialize(codec: codec);
    return sink;
  }

  ({RustStreamSink<int> sink, StreamController<dynamic> source})
  createInjectedSink() {
    final source = StreamController<dynamic>();
    final sink = rustStreamSinkWithRawSourceForTest<int>(
      codec: codec,
      source: source.stream,
      closeSource: source.close,
    );
    return (sink: sink, source: source);
  }

  test('RustStreamSink stream before setup throws actionable StateError', () {
    expect(
      () => RustStreamSink<int>().stream,
      throwsA(
        isA<StateError>().having(
          (error) => error.message,
          'message',
          allOf(
            contains('RustStreamSink.stream is not ready yet'),
            contains('generated flutter_rust_bridge API'),
          ),
        ),
      ),
    );
  });

  test(
    'cancelling Dart subscription does not wait for Rust stream close',
    () async {
      final sink = createPortSink();
      final subscription = sink.stream.listen((_) {});

      await subscription.cancel().timeout(const Duration(seconds: 1));
    },
  );

  test(
    'cancelling an idle subscription completes even after it is suspended',
    () async {
      final sink = createPortSink();
      final subscription = sink.stream.listen((_) {});
      // Give the underlying port subscription time to settle so it is genuinely
      // waiting for the next (never arriving) message, which is the scenario that
      // used to deadlock cancel().
      await Future<void>.delayed(const Duration(milliseconds: 10));

      await subscription.cancel().timeout(const Duration(seconds: 1));
    },
  );

  test('cancelling twice is safe and completes', () async {
    final sink = createPortSink();
    final subscription = sink.stream.listen((_) {});

    await subscription.cancel().timeout(const Duration(seconds: 1));
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('pausing then cancelling an idle subscription completes', () async {
    final sink = createPortSink();
    final subscription = sink.stream.listen((_) {});
    subscription.pause();
    await Future<void>.delayed(const Duration(milliseconds: 10));

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('data sent through the raw source arrives through the stream', () async {
    final harness = createInjectedSink();
    final items = <int>[];
    final subscription = harness.sink.stream.listen(items.add);

    // DcoCodec wire format: [action, payload]. Action 0 = success.
    harness.source.add([0, 42]);
    await Future<void>.delayed(Duration.zero);

    expect(items, [42]);
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('decode error is delivered and then the stream closes', () async {
    final harness = createInjectedSink();
    Object? error;
    var done = false;
    final subscription = harness.sink.stream.listen(
      (_) {},
      onError: (Object e, StackTrace _) {
        error = e;
      },
      onDone: () {
        done = true;
      },
    );

    // Action 0 = success, but payload is not an int → TypeError in decoder.
    harness.source.add([0, 'not-an-int']);
    await Future<void>.delayed(Duration.zero);

    expect(error, isA<TypeError>());
    expect(done, isTrue);
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('pause and resume propagate to the upstream source', () async {
    var pauseCount = 0;
    var resumeCount = 0;
    final source = StreamController<dynamic>(
      onPause: () {
        pauseCount++;
      },
      onResume: () {
        resumeCount++;
      },
    );
    final sink = rustStreamSinkWithRawSourceForTest<int>(
      codec: codec,
      source: source.stream,
      closeSource: source.close,
    );
    final subscription = sink.stream.listen((_) {});

    subscription.pause();
    expect(pauseCount, 1);
    expect(resumeCount, 0);

    subscription.resume();
    expect(pauseCount, 1);
    expect(resumeCount, 1);

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });
}

int _decodeInt(dynamic raw) => raw as int;
