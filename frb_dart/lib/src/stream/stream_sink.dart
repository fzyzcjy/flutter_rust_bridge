import 'dart:async';

import 'package:async/async.dart';
import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';

/// The Rust `StreamSink<T>` on the Dart side.
class RustStreamSink<T> {
  _State<T>? _state;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  String setupAndSerialize({required BaseCodec<T, dynamic, dynamic> codec}) {
    _state ??= _setup(codec);
    return serializeNativePort(_state!.receivePort.sendPort.nativePort);
  }

  /// The Dart stream for the Rust sink
  Stream<T> get stream => _state!.stream;
}

class _State<T> {
  final ReceivePort receivePort;
  final Stream<T> stream;

  const _State(this.receivePort, this.stream);
}

_State<T> _setup<T>(BaseCodec<T, dynamic, dynamic> codec) {
  final portName = ExecuteStreamPortGenerator.create('RustStreamSink');
  final receivePort = broadcastPort(portName);

  final Stream<T> rawStream = () async* {
    try {
      await for (final raw in receivePort) {
        try {
          yield codec.decodeObject(raw);
        } on CloseStreamException {
          break;
        }
      }
    } finally {
      receivePort.close();
    }
  }();

  final stream = rawStream.listenAndBuffer();

  return _State(receivePort, stream);
}
