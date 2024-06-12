/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html' as html;
import 'dart:html' hide MessagePort;

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.name;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef MessagePort = _WebPortLike;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef SendPort = _WebPortLike;

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
    return _rawReceivePort._receivePort._onMessage.map(_extractData).listen(
          onData,
          onError: onError,
          onDone: onDone,
          cancelOnError: cancelOnError,
        );
  }

  static dynamic _extractData(MessageEvent event) => event.data;

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => _rawReceivePort.sendPort;

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => _rawReceivePort.close();
}

/// {@macro flutter_rust_bridge.same_as_native}
class RawReceivePort {
  final _WebChannel _channel;

  /// {@macro flutter_rust_bridge.same_as_native}
  factory RawReceivePort() => RawReceivePort._raw();

  RawReceivePort._raw([_WebChannel? channel])
      : _channel = channel ?? _WebChannel.messageChannel();

  /// {@macro flutter_rust_bridge.same_as_native}
  set handler(Function(dynamic) handler) {
    _receivePort._onMessage.listen((event) => handler(event.data));
  }

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => _channel.receivePort._close();

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => _channel.sendPort;

  SendPort get _receivePort => _channel.receivePort;
}

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) => ReceivePort._raw(
    RawReceivePort._raw(_WebChannel.broadcastChannel(channelName)));

abstract class _WebChannel {
  SendPort get sendPort;

  SendPort get receivePort;

  const _WebChannel();

  factory _WebChannel.messageChannel() = _WebMessageChannelWrapper;

  factory _WebChannel.broadcastChannel(String channelName) =
      _WebBroadcastChannelWrapper;
}

class _WebMessageChannelWrapper implements _WebChannel {
  final channel = MessageChannel();

  @override
  SendPort get sendPort => _WebPortLike._messagePort(channel.port2);

  @override
  SendPort get receivePort => _WebPortLike._messagePort(channel.port1);
}

class _WebBroadcastChannelWrapper implements _WebChannel {
  final BroadcastChannel _sendChannel;
  final BroadcastChannel _receiveChannel;

  _WebBroadcastChannelWrapper(String channelName)
      // Note: It is *wrong* to reuse the same HTML BroadcastChannel object,
      // because HTML BroadcastChannel spec says that, the event will not be fired
      // at the object which sends it. Therefore, we need two different objects.
      : _sendChannel = BroadcastChannel(channelName),
        _receiveChannel = BroadcastChannel(channelName);

  @override
  SendPort get sendPort => _WebPortLike._broadcastChannel(_sendChannel);

  @override
  SendPort get receivePort => _WebPortLike._broadcastChannel(_receiveChannel);
}

/// {@macro flutter_rust_bridge.same_as_native}
abstract class _WebPortLike {
  const _WebPortLike._();

  factory _WebPortLike._messagePort(html.MessagePort port) =
      _MessagePortWrapper;

  factory _WebPortLike._broadcastChannel(BroadcastChannel channel) =
      _BroadcastPortWrapper;

  void _close();

  /// {@macro flutter_rust_bridge.same_as_native}
  html.EventTarget get nativePort;

  Stream<MessageEvent> get _onMessage => _kMessageEvent.forTarget(nativePort);
  static const _kMessageEvent = EventStreamProvider<MessageEvent>('message');
}

class _MessagePortWrapper extends _WebPortLike {
  @override
  final html.MessagePort nativePort;

  _MessagePortWrapper(this.nativePort) : super._();

  @override
  void _close() => nativePort.close();
}

class _BroadcastPortWrapper extends _WebPortLike {
  @override
  final html.BroadcastChannel nativePort;

  _BroadcastPortWrapper(this.nativePort) : super._();

  @override
  void _close() => nativePort.close();
}
