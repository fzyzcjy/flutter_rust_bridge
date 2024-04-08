import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';

/// The Rust `StreamSink<T>` on the Dart side.
class RustStreamSink<T> {
  final ReceivePort _receivePort;
  final Stream<T> _stream;

  /// Construct an instance
  factory RustStreamSink({
    required BaseCodec<T, dynamic, dynamic> codec,
    String? debugName,
  }) {
    final portName = ExecuteStreamPortGenerator.create(debugName ?? 'unnamed');
    final receivePort = broadcastPort(portName);

    final Stream<T> stream = () async* {
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

    return RustStreamSink._(receivePort, stream);
  }

  RustStreamSink._(this._receivePort, this._stream);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  String serialize() {
    // TODO serialize in web
    return _receivePort.sendPort.nativePort;
  }

  /// The Dart stream for the Rust sink
  Stream<T> get stream => _stream;
}
