/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) {
  final name = port.getProperty<JSString?>('__frb_port_name'.toJS);
  if (name != null) {
    return name.toDart;
  }
  throw UnimplementedError(
    "serializeNativePort see unknown port=$port (type=${port.runtimeType})",
  );
}

@JS('globalThis.__frb_named_ports')
external JSObject? get _namedPorts;

@JS('globalThis.__frb_named_ports')
external set _namedPorts(JSObject value);

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
  void close() => _webReceivePort._close();

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
}

class _WebBroadcastChannel implements _WebChannel {
  final _channel = web.MessageChannel();
  final String _name;

  _WebBroadcastChannel(this._name) {
    // Note: Sender and receiver use different MessagePort endpoints,
    // because MessageChannel transfers messages to the other endpoint.
    // Both endpoints remain owned by this channel until it closes.
    final ports = _namedPorts ?? JSObject();
    _namedPorts = ports;
    _channel.port2.setProperty('__frb_port_name'.toJS, _name.toJS);
    ports.setProperty(_name.toJS, _channel.port2);
  }

  @override
  SendPort get _sendPort => SendPort._(_channel.port2);

  @override
  _WebPortLike get _receivePort =>
      _WebBroadcastPort(this);

  void _close() {
    final ports = _namedPorts;
    if (ports != null && ports.getProperty<JSAny?>(_name.toJS) == _channel.port2) {
      ports.delete(_name.toJS);
    }
    _channel.port1.close();
    _channel.port2.close();
  }
}

/// {@macro flutter_rust_bridge.same_as_native}
abstract class _WebPortLike {
  const _WebPortLike._();

  factory _WebPortLike._messagePort(web.MessagePort port) = _WebMessagePort;

  void _start();

  void _close();

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

  @override
  void _close() => _nativePort.close();
}

// A named MessageChannel exposes its receiving endpoint as a port.
class _WebBroadcastPort extends _WebPortLike {
  final _WebBroadcastChannel _channel;
  int _received = 0;
  JSArray<JSAny?>? _pendingClose;

  @override
  web.MessagePort get _nativePort => _channel._channel.port1;

  _WebBroadcastPort(this._channel) : super._();

  @override
  Stream<web.MessageEvent> get _onMessage =>
      super._onMessage.expand(_receiveMessage);

  Iterable<web.MessageEvent> _receiveMessage(web.MessageEvent event) sync* {
    final data = event.data;
    if (data.isA<JSArray<JSAny?>>() &&
        (data as JSArray<JSAny?>).length == 3 &&
        data[0] == '__frb_stream_close'.toJS) {
      _pendingClose = data;
    } else {
      _received++;
      yield event;
    }
    final pendingClose = _pendingClose;
    if (pendingClose != null &&
        (pendingClose[1] as JSNumber).toDartDouble == _received) {
      _pendingClose = null;
      yield web.MessageEvent(
        'message',
        web.MessageEventInit(data: pendingClose[2]),
      );
    }
  }

  @override
  void _start() => _nativePort.start();

  @override
  void _close() => _channel._close();
}
