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

  /// {@macro flutter_rust_bridge.internal}
  @visibleForTesting
  dynamic get debugSendPort => _state?.receivePort.sendPort;

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

  // Listen to the port directly instead of wrapping it in an `async*` generator
  // that does `await for (receivePort)`. A generator suspended in `await for`
  // cannot be interrupted by cancelling its subscription, so if Rust stays idle
  // (never sends another message and never closes the stream) then
  // `StreamSubscription.cancel()` would hang forever. Closing the port only
  // wakes such a generator on native (where `ReceivePort.close()` delivers a
  // done event) but not on web (where closing a `BroadcastChannel` delivers
  // nothing), so `await for` is fundamentally unsafe here. A plain subscription
  // can always be cancelled immediately and identically on every platform.
  final controller = StreamController<T>(sync: true);
  late final StreamSubscription<dynamic> portSubscription;

  var terminated = false;
  void terminate() {
    if (terminated) return;
    terminated = true;
    receivePort.close();
    portSubscription.cancel();
    controller.close();
  }

  portSubscription = receivePort.listen(
    (raw) {
      final T decoded;
      try {
        decoded = codec.decodeObject(raw);
      } on CloseStreamException {
        terminate();
        return;
        // coverage:ignore-start
      } catch (error, stackTrace) {
        controller.addError(error, stackTrace);
        terminate();
        return;
      }
      // coverage:ignore-end
      controller.add(decoded);
    },
    // coverage:ignore-start
    onError: (Object error, StackTrace stackTrace) {
      controller.addError(error, stackTrace);
      terminate();
    },
    // coverage:ignore-end
    onDone: terminate,
  );

  controller
    ..onPause = () {
      if (!terminated) portSubscription.pause();
    }
    ..onResume = () {
      if (!terminated) portSubscription.resume();
    }
    ..onCancel = () {
      terminated = true;
      receivePort.close();
      return portSubscription.cancel();
    };

  return _State(receivePort, controller.stream);
}
