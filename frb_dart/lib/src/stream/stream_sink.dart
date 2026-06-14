import 'dart:async';

import 'package:async/async.dart';
import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';

/// The Rust `StreamSink<T>` on the Dart side.
class RustStreamSink<T> {
  _State<T>? _state;
  final StreamController<T> _controller = StreamController<T>.broadcast();
  late final Stream<T> _stream;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  /// when [replay] is false, subsequent stream listeners do not receive the previous data.
  /// When [replay] is true, the first subsequent stream listeners can receive all the data sent before (in the case of infinite streams or large data streams, it can cause memory overflow!). ）
  RustStreamSink({bool replay = true}) {
    if (replay) {
      _stream = _controller.stream.listenAndBuffer().asBroadcastStream();
    } else {
      _stream = _controller.stream.asBroadcastStream();
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  String setupAndSerialize({required BaseCodec<T, dynamic, dynamic> codec}) {
    _state ??= _setup(codec, _controller);
    return serializeNativePort(_state!.receivePort.sendPort.nativePort);
  }

  /// The Dart stream for the Rust sink
  Stream<T> get stream => _stream;

  /// cancel stream in case memory inifinty increased.
  Future<void> cancel() async {
    final subscription = _stream.listen(null);
    await subscription.cancel();
  }
}

class _State<T> {
  final ReceivePort receivePort;

  const _State(this.receivePort);
}

_State<T> _setup<T>(
    BaseCodec<T, dynamic, dynamic> codec, StreamController<T> controller) {
  final portName = ExecuteStreamPortGenerator.create('RustStreamSink');
  final receivePort = broadcastPort(portName);

  unawaited(() async {
    try {
      await for (final raw in receivePort) {
        try {
          controller.add(codec.decodeObject(raw));
        } on CloseStreamException {
          break;
        } catch (e, s) {
          controller.addError(e, s);
          break;
        }
      }
    } finally {
      await controller.close();
    }
  }());

  return _State(receivePort);
}
