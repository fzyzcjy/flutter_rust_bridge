/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port;

/// Web implementation of the `dart:isolate`'s ReceivePort.
class ReceivePort extends Stream<dynamic> {
  /// The receive port.
  final RawReceivePort _rawReceivePort;

  /// Create a new receive port from an optional [RawReceivePort].
  ReceivePort() : _rawReceivePort = RawReceivePort();

  @override
  StreamSubscription listen(
    void Function(dynamic event)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    return _rawReceivePort._onMessage.map(_extractData).listen(
          onData,
          onError: onError,
          onDone: onDone,
          cancelOnError: cancelOnError,
        );
  }

  static dynamic _extractData(MessageEvent event) => event.data;

  /// The send port.
  SendPort get sendPort => _rawReceivePort.sendPort;

  /// Close the receive port, ignoring any further messages.
  void close() => _rawReceivePort.close();
}

/// Web implementation of the `dart:isolate`'s RawReceivePort.
class RawReceivePort {
  // Note: It is *wrong* to reuse the same HTML BroadcastChannel object
  // for both sending and receiving, because HTML BroadcastChannel spec says
  // that, the event will not be fired at the object which sends it.
  /// The underlying message channel.
  final BroadcastChannel _receiveChannel;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  RawReceivePort()
      : _receiveChannel = html.BroadcastChannel(_PortNameGenerator.create());

  set handler(Function(dynamic) handler) {
    _kMessageEvent
        .forTarget(_receiveChannel)
        .listen((event) => handler(event.data));
  }

  /// Close the receive port.
  void close() => _receiveChannel.close();

  /// The port to be used by other workers.
  SendPort get sendPort => SendPort._(_receiveChannel.name!);

  Stream<MessageEvent> get _onMessage =>
      _kMessageEvent.forTarget(_receiveChannel);
}

const _kMessageEvent = EventStreamProvider<MessageEvent>('message');

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SendPort {
  final String _broadcastChannelName;

  SendPort._(this._broadcastChannelName);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  String get nativePort => _broadcastChannelName;
}

class _PortNameGenerator {
  static int _nextPort = 0;

  static String create() => '__frb_port_${_nextPort++}';
}
