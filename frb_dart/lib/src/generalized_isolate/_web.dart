/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:js_interop_unsafe';
import 'dart:math';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'package:meta/meta.dart';
import 'package:web/web.dart' as web;

dynamic _extractData(JSAny? data) => data.dartify();

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

final _namedPorts = <String, web.MessagePort>{};

// Indeed a BroadcastChannel, not a Broadcast "Port"
final _broadcastChannel = _createBroadcastChannel();
Future<void>? _broadcastChannelReady;

web.BroadcastChannel _createBroadcastChannel() {
  final random = Random.secure();
  final nonce = List.generate(4, (_) => random.nextInt(0x100000000)).join('_');
  return web.BroadcastChannel('__frb_broadcast_$nonce');
}

@internal
Future<void> initializeBroadcastChannel() =>
    _broadcastChannelReady ??= _initializeBroadcastChannel().onError((
      Object error,
      StackTrace stackTrace,
    ) {
      _broadcastChannelReady = null;
      Error.throwWithStackTrace(error, stackTrace);
    });

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
    final port = _namedPorts[(message[0] as JSString).toDart];
    port?.postMessage(message[1]);
  }).toJS;
  // Note: It is *wrong* to reuse the same HTML BroadcastChannel object,
  // because HTML BroadcastChannel spec says that, the event will not be fired
  // at the object which sends it. Therefore, we need two different objects.
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
    final subscription = _rawReceivePort._webReceivePort._messages
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
    _webReceivePort._messages.map(_extractData).listen(handler);
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
    _channel.port2.setProperty(
      '__frb_port_name'.toJS,
      '${_broadcastChannel.name}/$_name'.toJS,
    );
    _namedPorts[_name] = _channel.port2;
  }

  @override
  SendPort get _sendPort => SendPort._(_channel.port2);

  @override
  _WebPortLike get _receivePort => _WebBroadcastPort(this);

  void _close() {
    if (_namedPorts[_name] == _channel.port2) {
      _namedPorts.remove(_name);
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

  Stream<JSAny?> get _messages => _onMessage.map((event) => event.data);
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

class _WebBroadcastPort extends _WebPortLike {
  final _WebBroadcastChannel _channel;

  @override
  web.MessagePort get _nativePort => _channel._channel.port1;

  _WebBroadcastPort(this._channel) : super._();

  @override
  Stream<JSAny?> get _messages {
    if (!_channel._name.startsWith('__frb_streamsink_')) {
      return super._messages;
    }
    var nextSequence = 0;
    final pending = <int, JSArray<JSAny?>>{};
    return super._messages.expand((message) sync* {
      if (message != null && message.isA<JSArray>()) {
        final frame = message as JSArray<JSAny?>;
        if (frame.toDart.isNotEmpty && frame[0].isA<JSString>()) {
          final tag = (frame[0] as JSString).toDart;
          if (tag == '__frb_stream_failed') {
            if (frame.length != 1) {
              throw StateError('Invalid Web stream failure frame');
            }
            throw StateError('Web stream transport failed');
          }
          if (tag == '__frb_stream') {
            if ((frame.length != 2 && frame.length != 3) ||
                !frame[1].isA<JSNumber>()) {
              throw StateError('Invalid Web stream frame');
            }
            final sequence = (frame[1] as JSNumber).toDartDouble;
            if (!sequence.isFinite ||
                sequence != sequence.truncateToDouble() ||
                sequence < nextSequence ||
                sequence > 9007199254740991 ||
                pending.containsKey(sequence.toInt())) {
              throw StateError('Invalid Web stream sequence');
            }
            pending[sequence.toInt()] = frame;
            while (pending.containsKey(nextSequence)) {
              final ready = pending.remove(nextSequence++)!;
              if (ready.length == 3) yield ready[2];
            }
            return;
          }
        }
      }
      yield message;
    });
  }

  @override
  void _start() => _nativePort.start();

  @override
  void _close() => _channel._close();
}
