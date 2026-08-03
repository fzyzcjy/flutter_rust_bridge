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

  /// A stream bound to a controller we can feed raw wire messages into, i.e.
  /// what a receive port would deliver.
  ({Stream<int> stream, StreamController<dynamic> source}) createInjected() {
    final source = StreamController<dynamic>();
    final stream = bindDecodedStreamForTest<int>(
      codec: codec,
      source: source.stream,
      closeSource: source.close,
    );
    return (stream: stream, source: source);
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
    final harness = createInjected();
    final items = <int>[];
    final subscription = harness.stream.listen(items.add);

    // DcoCodec wire format: [action, payload]. Action 0 = success.
    harness.source.add([0, 42]);
    await Future<void>.delayed(Duration.zero);

    expect(items, [42]);
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('events arriving before the consumer listens are buffered', () async {
    final harness = createInjected();
    harness.source
      ..add([0, 1])
      ..add([0, 2])
      ..add([0, 3]);
    await Future<void>.delayed(Duration.zero);

    final items = <int>[];
    final subscription = harness.stream.listen(items.add);
    await Future<void>.delayed(Duration.zero);

    expect(items, [1, 2, 3], reason: 'buffered events replay in order');
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test(
    'close-stream message ends the stream and releases the source',
    () async {
      final harness = createInjected();
      final items = <int>[];
      Object? error;
      var done = false;
      harness.stream.listen(
        items.add,
        onError: (Object e, StackTrace _) {
          error = e;
        },
        onDone: () {
          done = true;
        },
      );

      // Action 2 = close stream, sent by Rust when the sink is dropped.
      harness.source
        ..add([0, 7])
        ..add([2]);
      await Future<void>.delayed(Duration.zero);

      expect(items, [7]);
      expect(error, isNull);
      expect(done, isTrue);
      expect(harness.source.isClosed, isTrue);
    },
  );

  test('events produced after termination are dropped', () async {
    final source = StreamController<dynamic>.broadcast();
    final stream = bindDecodedStreamForTest<int>(
      codec: codec,
      source: source.stream,
      closeSource: () {},
    );
    final items = <int>[];
    var done = false;
    stream.listen(items.add, onDone: () => done = true);

    source.add([2]);
    await Future<void>.delayed(Duration.zero);
    expect(done, isTrue);

    source.add([0, 99]);
    await Future<void>.delayed(Duration.zero);
    expect(items, isEmpty);
  });

  test('decode error is delivered and then the stream closes', () async {
    final harness = createInjected();
    Object? error;
    var done = false;
    final subscription = harness.stream.listen(
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

  test('a source error is forwarded and then the stream closes', () async {
    final harness = createInjected();
    Object? error;
    var done = false;
    harness.stream.listen(
      (_) {},
      onError: (Object e, StackTrace _) {
        error = e;
      },
      onDone: () {
        done = true;
      },
    );

    harness.source.addError(StateError('transport failed'));
    await Future<void>.delayed(Duration.zero);

    expect(error, isA<StateError>());
    expect(done, isTrue);
    expect(harness.source.isClosed, isTrue);
  });

  test('a source that is already done while binding does not throw', () async {
    var done = false;
    final stream = bindDecodedStreamForTest<int>(
      codec: codec,
      source: _SynchronouslyDoneStream(),
      closeSource: () {},
    );
    stream.listen((_) {}, onDone: () => done = true);
    await Future<void>.delayed(Duration.zero);

    expect(done, isTrue);
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
    final stream = bindDecodedStreamForTest<int>(
      codec: codec,
      source: source.stream,
      closeSource: source.close,
    );
    final subscription = stream.listen((_) {});

    subscription.pause();
    expect(pauseCount, 1);
    expect(resumeCount, 0);

    subscription.resume();
    expect(pauseCount, 1);
    expect(resumeCount, 1);

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('events produced while paused are buffered and replayed', () async {
    final harness = createInjected();
    final items = <int>[];
    final subscription = harness.stream.listen(items.add);

    subscription.pause();
    harness.source
      ..add([0, 1])
      ..add([0, 2]);
    await Future<void>.delayed(Duration.zero);
    expect(items, isEmpty, reason: 'nothing is delivered while paused');

    subscription.resume();
    await Future<void>.delayed(Duration.zero);
    expect(items, [1, 2], reason: 'buffered events replay in order');

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });
}

int _decodeInt(dynamic raw) => raw as int;

/// A source that reports done from within `listen`, before returning the
/// subscription. Ordinary receive ports never do this, but the binding must
/// not blow up on a source that does.
class _SynchronouslyDoneStream extends Stream<dynamic> {
  @override
  StreamSubscription<dynamic> listen(
    void Function(dynamic event)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    onDone?.call();
    return const Stream<dynamic>.empty().listen(null);
  }
}
