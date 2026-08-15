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
  final receivePort = broadcastPort(portName, ordered: true);
  return _State(
    receivePort,
    _bindDecodedStream(
      codec.decodeObject,
      receivePort,
      closeSource: receivePort.close,
      sourceDrainsAfterCancel: orderedReceivePortDrainsAfterCancel,
    ),
  );
}

@visibleForTesting
/// Binds and decodes a source stream for internal tests.
Stream<T> bindDecodedStreamForTest<T>({
  required T Function(dynamic) decodeObject,
  required Stream<dynamic> source,
  required void Function() closeSource,
}) => _bindDecodedStream(decodeObject, source, closeSource: closeSource);

Stream<T> _bindDecodedStream<T>(
  T Function(dynamic) decodeObject,
  Stream<dynamic> source, {
  required void Function() closeSource,
  bool sourceDrainsAfterCancel = false,
}) {
  final controller = StreamController<T>(sync: true);

  StreamSubscription<dynamic>? sourceSubscription;
  var terminated = false;

  void terminate() {
    if (terminated) return;
    terminated = true;
    if (!sourceDrainsAfterCancel) closeSource();
    sourceSubscription?.cancel();
    controller.close();
  }

  sourceSubscription = source.listen(
    (raw) {
      final T decoded;
      try {
        decoded = decodeObject(raw);
      } on CloseStreamException {
        terminate();
        return;
      } catch (error, stackTrace) {
        controller.addError(error, stackTrace);
        terminate();
        return;
      }
      controller.add(decoded);
    },
    onError: (Object error, StackTrace stackTrace) {
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
      if (!sourceDrainsAfterCancel) closeSource();
      return sourceSubscription!.cancel();
    };

  return controller.stream;
}
