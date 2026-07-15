import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge/src/codec/dco.dart';
import 'package:test/test.dart';

void main() {
  RustStreamSink<int> createSink() {
    final sink = RustStreamSink<int>();
    sink.setupAndSerialize(
      codec: DcoCodec<int, Exception>(
        decodeSuccessData: (raw) => raw as int,
        decodeErrorData: null,
      ),
    );
    return sink;
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
      final sink = createSink();
      final subscription = sink.stream.listen((_) {});

      await subscription.cancel().timeout(const Duration(seconds: 1));
    },
  );

  test(
    'cancelling an idle subscription completes even after it is suspended',
    () async {
      final sink = createSink();
      final subscription = sink.stream.listen((_) {});
      // Give the underlying port subscription time to settle so it is genuinely
      // waiting for the next (never arriving) message, which is the scenario that
      // used to deadlock cancel().
      await Future<void>.delayed(const Duration(milliseconds: 10));

      await subscription.cancel().timeout(const Duration(seconds: 1));
    },
  );

  test('cancelling twice is safe and completes', () async {
    final sink = createSink();
    final subscription = sink.stream.listen((_) {});

    await subscription.cancel().timeout(const Duration(seconds: 1));
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('pausing then cancelling an idle subscription completes', () async {
    final sink = createSink();
    final subscription = sink.stream.listen((_) {});
    subscription.pause();
    await Future<void>.delayed(const Duration(milliseconds: 10));

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('data sent to the port arrives through the stream', () async {
    // dart:isolate.SendPort.send() is VM-only; on web (JS) int and double
    // share identity, so this detects the platform without importing dart:io.
    if (identical(0, 0.0)) return;

    final sink = createSink();
    final items = <int>[];
    final subscription = sink.stream.listen(items.add);

    // DcoCodec wire format: [action, payload]. Action 0 = success.
    (sink.debugSendPort as dynamic).send([0, 42]);
    await Future<void>.delayed(const Duration(milliseconds: 50));

    expect(items, [42]);
    await subscription.cancel().timeout(const Duration(seconds: 1));
  });

  test('pause then resume then cancel completes', () async {
    final sink = createSink();
    final subscription = sink.stream.listen((_) {});
    subscription.pause();
    await Future<void>.delayed(const Duration(milliseconds: 10));
    subscription.resume();
    await Future<void>.delayed(const Duration(milliseconds: 10));

    await subscription.cancel().timeout(const Duration(seconds: 1));
  });
}
