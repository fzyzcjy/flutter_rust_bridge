/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html';

typedef SendPort = MessagePort;

typedef NativePortType = MessagePort;

/// Wrapper around a [MessageChannel].
class RawReceivePort {
  final channel = MessageChannel();

  set handler(Function(dynamic) handler) {
    receivePort.onMessage.listen((event) => handler(event.data));
  }

  void close() => channel.port1.close();
  SendPort get sendPort => channel.port2;
  SendPort get receivePort => channel.port1;
}

class ReceivePort extends Stream<dynamic> {
  final RawReceivePort port;
  ReceivePort([RawReceivePort? port]) : port = port ?? RawReceivePort();

  @override
  StreamSubscription listen(
    void Function(dynamic event)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    return port.receivePort.onMessage.listen(
      onData,
      onError: onError,
      onDone: onDone,
      cancelOnError: cancelOnError,
    );
  }

  SendPort get sendPort => port.sendPort;
  void close() => port.receivePort.close();
}

extension NativePortExt on MessagePort {
  NativePortType get nativePort => this;
}
