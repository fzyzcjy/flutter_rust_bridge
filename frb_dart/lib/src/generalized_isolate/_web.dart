/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) {
  if (port.isA<web.BroadcastChannel>()) {
    return (port as web.BroadcastChannel).name;
  }
  throw UnimplementedError(
    "serializeNativePort see unknown port=$port (type=${port.runtimeType})",
  );
}

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) => ReceivePort._raw(
  RawReceivePort._raw(_WebChannel.broadcastChannel(channelName)),
);

/// {@template flutter_rust_bridge.same_as_native}
/// Web implementation of the one with same name in native.
/// {@endtemplate}
class ReceivePort extends Stream<dynamic> {
  final RawReceivePort _rawReceivePort;

  /// {@macro flutter_rust_bridge.same_as_native}
  factory ReceivePort() => ReceivePort._raw();

  ReceivePort._raw([RawReceivePort? rawReceivePort])
    : _rawReceivePort = rawReceivePort ?? RawReceivePort();

  @override
  StreamSubscription listen(
    void Function(dynamic event)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    final subscription = _rawReceivePort._webReceivePort._onMessage
        .map(_extractData)
        .listen(
          onData,
          onError: onError,
          onDone: onDone,
          cancelOnError: cancelOnError,
        );
    _rawReceivePort._webReceivePort._start();
    return subscription;
  }

  static dynamic _extractData(web.MessageEvent event) => event.data;

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => _rawReceivePort.sendPort;

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => _rawReceivePort.close();
}

/// {@macro flutter_rust_bridge.same_as_native}
class RawReceivePort {
  final _WebChannel _webChannel;

  /// {@macro flutter_rust_bridge.same_as_native}
  factory RawReceivePort() => RawReceivePort._raw();

  RawReceivePort._raw([_WebChannel? channel])
    : _webChannel = channel ?? _WebChannel.messageChannel();

  /// {@macro flutter_rust_bridge.same_as_native}
  set handler(Function(dynamic) handler) {
    _webReceivePort._onMessage.listen((event) => handler(event.data));
    _webReceivePort._start();
  }

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => _webChannel._close();

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => _webChannel._sendPort;

  _WebPortLike get _webReceivePort => _webChannel._receivePort;
}

/// {@macro flutter_rust_bridge.same_as_native}
class SendPort {
  /// {@macro flutter_rust_bridge.same_as_native}
  final web.EventTarget nativePort;

  const SendPort._(this.nativePort);
}

abstract class _WebChannel {
  SendPort get _sendPort;

  _WebPortLike get _receivePort;

  void _close();

  factory _WebChannel.messageChannel() = _WebMessageChannel;

  factory _WebChannel.broadcastChannel(String channelName) =
      _WebBroadcastChannel;
}

class _WebMessageChannel implements _WebChannel {
  final _channel = web.MessageChannel();

  @override
  SendPort get _sendPort => SendPort._(_channel.port2);

  @override
  _WebPortLike get _receivePort => _WebPortLike._messagePort(_channel.port1);

  @override
  void _close() => _channel.port1.close();
}

class _WebBroadcastChannel implements _WebChannel {
  final web.BroadcastChannel _sendChannel;
  final web.BroadcastChannel _receiveChannel;
  final web.BroadcastChannel _readyChannel;
  late final Timer _readyTimer;

  _WebBroadcastChannel(String channelName)
    // Note: It is *wrong* to reuse the same HTML BroadcastChannel object,
    // because HTML BroadcastChannel spec says that, the event will not be fired
    // at the object which sends it. Therefore, we need two different objects.
    : _sendChannel = web.BroadcastChannel(channelName),
      _receiveChannel = web.BroadcastChannel(channelName),
      _readyChannel = web.BroadcastChannel('${channelName}__frb_ready') {
    _readyTimer = Timer.periodic(
      const Duration(milliseconds: 10),
      (_) => _readyChannel.postMessage(null),
    );
  }

  @override
  SendPort get _sendPort => SendPort._(_sendChannel);

  @override
  _WebPortLike get _receivePort =>
      _WebPortLike._broadcastChannel(_receiveChannel);

  @override
  void _close() {
    _readyTimer.cancel();
    _readyChannel.postMessage(null);
    _sendChannel.close();
    _receiveChannel.close();
    Timer(const Duration(milliseconds: 100), () => _readyChannel.close());
  }
}

/// {@macro flutter_rust_bridge.same_as_native}
abstract class _WebPortLike {
  const _WebPortLike._();

  factory _WebPortLike._messagePort(web.MessagePort port) = _WebMessagePort;

  factory _WebPortLike._broadcastChannel(web.BroadcastChannel channel) =
      _WebBroadcastPort;

  void _start();

  /// {@macro flutter_rust_bridge.same_as_native}
  web.EventTarget get _nativePort;

  Stream<web.MessageEvent> get _onMessage =>
      _kMessageEvent.forTarget(_nativePort);
  static const _kMessageEvent = web.EventStreamProvider<web.MessageEvent>(
    'message',
  );
}

class _WebMessagePort extends _WebPortLike {
  @override
  final web.MessagePort _nativePort;

  _WebMessagePort(this._nativePort) : super._();

  @override
  void _start() => _nativePort.start();

}

// Indeed a BroadcastChannel, not a Broadcast "Port"
class _WebBroadcastPort extends _WebPortLike {
  @override
  final web.BroadcastChannel _nativePort;

  _WebBroadcastPort(this._nativePort) : super._();

  @override
  void _start() {}

}
