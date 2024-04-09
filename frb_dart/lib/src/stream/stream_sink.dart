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

  late final StreamSubscription<dynamic> receivePortSubscription;
  late final StreamController<T> outputStreamController;

  void cleanup() {
    receivePortSubscription.cancel();
    outputStreamController.close();
    receivePort.close();
  }

  outputStreamController = StreamController<T>(
    onCancel: () => cleanup(),
  );
  receivePortSubscription = receivePort.listen(
    (message) {
      try {
        outputStreamController.add(codec.decodeObject(message));
      } on CloseStreamException {
        cleanup();
      }
    },
    onError: (Object e, StackTrace s) => outputStreamController.addError(e, s),
  );

  return _State(receivePort, outputStreamController.stream);
}
