import 'dart:async';

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

  final outputStreamController = StreamController<T>();

  late final StreamSubscription<dynamic> subscription;
  subscription = receivePort.listen(
    (message) {
      TODO;
    },
    onError: (Object e, StackTrace s) {
      TODO;
    },
    onDone: () {
      TODO;
    },
  );

  final Stream<T> stream = () async* {
    try {
      print('hi RustStreamSink async* start');
      await for (final raw in receivePort) {
        print('hi RustStreamSink recv raw=$raw');
        try {
          yield codec.decodeObject(raw);
        } on CloseStreamException {
          print('hi RustStreamSink recv see CloseStreamException');
          break;
        }
      }
    } finally {
      print('hi RustStreamSink finally');
      receivePort.close();
    }
  }();

  return _State(receivePort, outputStreamController.stream);
}
