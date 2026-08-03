import 'dart:async';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:meta/meta.dart';

/// The Rust `StreamSink<T>` on the Dart side.
class RustStreamSink<T> {
  _State<T>? _state;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  String setupAndSerialize({required BaseCodec<T, dynamic, dynamic> codec}) {
    _state ??= _setup(codec);
    return serializeNativePort(_state!.receivePort.sendPort.nativePort);
  }

  /// The Dart stream for the Rust sink
  Stream<T> get stream {
    final state = _state;
    if (state == null) {
      throw StateError(
        'RustStreamSink.stream is not ready yet. Pass this RustStreamSink to a '
        'generated flutter_rust_bridge API before accessing stream. Listening '
        'before setup is not supported yet.',
      );
    }
    return state.stream;
  }
}

class _State<T> {
  final ReceivePort receivePort;
  final Stream<T> stream;

  const _State(this.receivePort, this.stream);
}

_State<T> _setup<T>(BaseCodec<T, dynamic, dynamic> codec) {
  final portName = ExecuteStreamPortGenerator.create('RustStreamSink');
  final receivePort = broadcastPort(portName);
  return _State(
    receivePort,
    _bindDecodedStream(codec, receivePort, closeSource: receivePort.close),
  );
}

/// Test-only seam exposing [_bindDecodedStream] over an injectable raw event
/// [source] instead of a platform receive port.
///
/// Kept as a top-level symbol (not a member of [RustStreamSink]) so that
/// entrypoints exporting `show RustStreamSink` do not leak it to consumers.
/// Tests reach it via `package:flutter_rust_bridge/src/...`.
@visibleForTesting
Stream<T> bindDecodedStreamForTest<T>({
  required BaseCodec<T, dynamic, dynamic> codec,
  required Stream<dynamic> source,
  required void Function() closeSource,
}) => _bindDecodedStream(codec, source, closeSource: closeSource);

/// Listen to [source] directly instead of wrapping it in an `async*` generator
/// that does `await for`. A generator suspended in `await for` cannot be
/// interrupted by cancelling its subscription, so if the producer stays idle
/// (never sends another message and never closes the stream) then
/// `StreamSubscription.cancel()` would hang forever. Closing a receive port
/// only wakes such a generator on native (where `ReceivePort.close()` delivers
/// a done event) but not on web (where closing a `BroadcastChannel` delivers
/// nothing), so `await for` is fundamentally unsafe here. A plain subscription
/// can always be cancelled immediately and identically on every platform.
Stream<T> _bindDecodedStream<T>(
  BaseCodec<T, dynamic, dynamic> codec,
  Stream<dynamic> source, {
  required void Function() closeSource,
}) {
  final controller = StreamController<T>(sync: true);

  // Nullable rather than `late`: a source is allowed to report done or error
  // before `listen` returns, so `terminate` can run while the subscription
  // does not exist yet. The `if (terminated)` check below closes that window.
  StreamSubscription<dynamic>? sourceSubscription;
  var terminated = false;

  void terminate() {
    if (terminated) return;
    terminated = true;
    closeSource();
    sourceSubscription?.cancel();
    controller.close();
  }

  sourceSubscription = source.listen(
    (raw) {
      final T decoded;
      try {
        decoded = codec.decodeObject(raw);
      } on CloseStreamException {
        terminate();
        return;
      } catch (error, stackTrace) {
        // Preserve the previous `async*` behaviour: a decoded error/panic ends
        // the stream after the error event is delivered.
        controller.addError(error, stackTrace);
        terminate();
        return;
      }
      controller.add(decoded);
    },
    onError: (Object error, StackTrace stackTrace) {
      // Receive ports never surface stream-level errors, but an arbitrary
      // source may, and silently dropping them would hide failures.
      controller.addError(error, stackTrace);
      terminate();
    },
    onDone: terminate,
  );
  if (terminated) sourceSubscription.cancel();

  controller
    ..onPause = () {
      if (!terminated) sourceSubscription!.pause();
    }
    ..onResume = () {
      if (!terminated) sourceSubscription!.resume();
    }
    ..onCancel = () {
      terminated = true;
      closeSource();
      return sourceSubscription!.cancel();
    };

  return controller.stream;
}
