/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html';

/// An alias to [MessagePort] on web platforms.
typedef SendPort = MessagePort;

/// An alias to [MessagePort] on web platforms.
typedef NativePortType = MessagePort;

/// Wrapper around a [MessageChannel].
class RawReceivePort {
  /// The underlying message channel.
  final channel = MessageChannel();

  set handler(Function(dynamic) handler) {
    receivePort.onMessage.listen((event) => handler(event.data));
  }

  /// Close the receive port.
  void close() => channel.port1.close();

  /// The port to be used by other workers.
  SendPort get sendPort => channel.port2;

  /// The port used to receive messages from other workers.
  SendPort get receivePort => channel.port1;
}

/// Web implementation of the `dart:isolate`'s ReceivePort.
class ReceivePort extends Stream<dynamic> {
  /// The receive port.
  final RawReceivePort port;

  /// Create a new receive port from an optional [RawReceivePort].
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

  /// The send port.
  SendPort get sendPort => port.sendPort;

  /// Close the receive port, ignoring any further messages.
  void close() => port.receivePort.close();
}

/// Implementation of `dart:isolate`'s NativePort extension.
extension NativePortExt on MessagePort {
  /// The native representation of this port.
  NativePortType get nativePort => this;
}
