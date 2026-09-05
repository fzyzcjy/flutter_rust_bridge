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
  if (port.isA<web.BroadcastChannel>()) {
    return (port as web.BroadcastChannel).name;
  }
  throw UnimplementedError(
    "serializeNativePort see unknown port=$port (type=${port.runtimeType})",
  );
}

@JS('globalThis.__frb_named_ports')
external JSObject? get _namedPorts;

@JS('globalThis.__frb_named_ports')
external set _namedPorts(JSObject value);

final _broadcastChannel = web.BroadcastChannel(
  '__frb_broadcast_${_randomUUID()}',
);
final _broadcastChannelReady = _initializeBroadcastChannel();

@JS('globalThis.crypto.randomUUID')
external String _randomUUID();

Future<void> initializeBroadcastChannel() => _broadcastChannelReady;

Future<void> _initializeBroadcastChannel() async {
  final ready = Completer<void>();
  _broadcastChannel.onmessage = ((web.MessageEvent event) {
    final data = event.data;
    if (data == '__frb_ready'.toJS) {
      if (!ready.isCompleted) ready.complete();
      return;
    }
    if (!data.isA<JSArray<JSAny?>>()) return;
    final message = data as JSArray<JSAny?>;
    if (message.length != 2 || !message[0].isA<JSString>()) return;
    final port = _namedPorts?.getProperty<web.MessagePort?>(message[0] as JSString);
    port?.postMessage(message[1]);
  }).toJS;
  final sender = web.BroadcastChannel(_broadcastChannel.name);
  final timer = Timer.periodic(const Duration(milliseconds: 10), (_) {
    sender.postMessage('__frb_ready'.toJS);
  });
  try {
    sender.postMessage('__frb_ready'.toJS);
    await ready.future.timeout(const Duration(seconds: 10));
  } finally {
    timer.cancel();
    sender.close();
  }
}

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) =>
    ReceivePort._raw(RawReceivePort._raw(_WebChannel(channelName)));

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
    final subscription = _rawReceivePort._webChannel._messages.listen(
      onData,
      onError: onError,
      onDone: onDone,
      cancelOnError: cancelOnError,
    );
    _rawReceivePort._webChannel._start();
    return subscription;
  }

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
    : _webChannel = channel ?? _WebChannel();

  /// {@macro flutter_rust_bridge.same_as_native}
  set handler(Function(dynamic) handler) {
    _webChannel._messages.listen(handler);
    _webChannel._start();
  }

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => _webChannel._close();

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => _webChannel._sendPort;
}

/// {@macro flutter_rust_bridge.same_as_native}
class SendPort {
  /// {@macro flutter_rust_bridge.same_as_native}
  final web.EventTarget nativePort;

  const SendPort._(this.nativePort);
}

/// {@macro flutter_rust_bridge.same_as_native}
// A named MessageChannel exposes its receiving endpoint as a port.
class _WebChannel {
  final _channel = web.MessageChannel();
  final String? _name;

  _WebChannel([String? name]) : _name = name {
    if (name == null) return;
    // Note: Sender and receiver use different MessagePort endpoints,
    // because MessageChannel transfers messages to the other endpoint.
    // Both endpoints remain owned by this channel until it closes.
    final ports = _namedPorts ?? JSObject();
    _namedPorts = ports;
    _channel.port2.setProperty(
      '__frb_port_name'.toJS,
      '${_broadcastChannel.name}/$name'.toJS,
    );
    ports.setProperty(name.toJS, _channel.port2);
  }

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get _sendPort => SendPort._(_channel.port2);

  Stream<JSAny?> get _messages {
    final messages = _kMessageEvent
        .forTarget(_channel.port1)
        .map((event) => event.data);
    if (_name == null) return messages;
    var received = 0;
    JSArray<JSAny?>? pendingClose;
    return messages.expand((data) sync* {
      if (data.isA<JSArray<JSAny?>>() &&
          (data as JSArray<JSAny?>).length == 3 &&
          data[0] == '__frb_stream_close'.toJS) {
        pendingClose = data;
      } else {
        received++;
        yield data;
      }
      final close = pendingClose;
      if (close != null && (close[1] as JSNumber).toDartDouble == received) {
        pendingClose = null;
        yield close[2];
      }
    });
  }

  void _start() => _channel.port1.start();

  void _close() {
    final name = _name;
    if (name != null) {
      final ports = _namedPorts;
      if (ports != null &&
          ports.getProperty<JSAny?>(name.toJS) == _channel.port2) {
        ports.delete(name.toJS);
      }
      _channel.port2.close();
    }
    _channel.port1.close();
  }

  static const _kMessageEvent = web.EventStreamProvider<web.MessageEvent>(
    'message',
  );
}
