/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.name;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef MessagePort = PortLike;

/// An alias to [MessagePort] on web platforms.
typedef SendPort = PortLike;

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class Channel {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  SendPort get sendPort;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  SendPort get receivePort;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const Channel();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory Channel.messageChannel() = _MessageChannelWrapper;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory Channel.broadcastChannel(String channelName) =
      _BroadcastChannelWrapper;
}

class _MessageChannelWrapper implements Channel {
  final channel = web.MessageChannel();

  @override
  SendPort get sendPort => PortLike.messagePort(channel.port2);

  @override
  SendPort get receivePort => PortLike.messagePort(channel.port1);
}

class _BroadcastChannelWrapper implements Channel {
  final web.BroadcastChannel _sendChannel;
  final web.BroadcastChannel _receiveChannel;

  _BroadcastChannelWrapper(String channelName)
      // Note: It is *wrong* to reuse the same HTML BroadcastChannel object,
      // because HTML BroadcastChannel spec says that, the event will not be fired
      // at the object which sends it. Therefore, we need two different objects.
      : _sendChannel = web.BroadcastChannel(channelName),
        _receiveChannel = web.BroadcastChannel(channelName);

  @override
  SendPort get sendPort => PortLike.broadcastChannel(_sendChannel);

  @override
  SendPort get receivePort => PortLike.broadcastChannel(_receiveChannel);
}

/// Wrapper around a [MessageChannel].
class RawReceivePort {
  /// The underlying message channel.
  final Channel channel;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  RawReceivePort([Channel? channel])
      : channel = channel ?? Channel.messageChannel();

  set handler(Function(dynamic) handler) {
    receivePort.onMessage.listen((event) => handler(event.data));
  }

  /// Close the receive port.
  void close() => channel.receivePort.close();

  /// The port to be used by other workers.
  SendPort get sendPort => channel.sendPort;

  /// The port used to receive messages from other workers.
  SendPort get receivePort => channel.receivePort;
}

/// Web implementation of the `dart:isolate`'s ReceivePort.
class ReceivePort extends Stream<dynamic> {
  /// The receive port.
  final RawReceivePort port;

  static dynamic _extractData(web.MessageEvent event) => event.data;

  /// Create a new receive port from an optional [RawReceivePort].
  ReceivePort([RawReceivePort? port]) : port = port ?? RawReceivePort();

  @override
  StreamSubscription listen(
    void Function(dynamic event)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    return port.receivePort.onMessage.map(_extractData).listen(
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

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) =>
    ReceivePort(RawReceivePort(Channel.broadcastChannel(channelName)));

/// [web.MessagePort]'s interface.
abstract class PortLike extends web.EventTarget {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory PortLike.messagePort(web.MessagePort port) = _MessagePortWrapper;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory PortLike.broadcastChannel(web.BroadcastChannel channel) =
      _BroadcastPortWrapper;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void postMessage(Object? value);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void close();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get nativePort;
}

/// Delegates a subset of PortLike methods verbatim.
abstract class _DelegatedPort implements PortLike {
  @override
  void addEventListener(String type, web.EventListener? listener,
          [bool? useCapture]) =>
      nativePort.addEventListener(type, listener, useCapture);

  @override
  void removeEventListener(String type, web.EventListener? listener,
          [bool? useCapture]) =>
      nativePort.removeEventListener(type, listener, useCapture);

  @override
  void close() => nativePort.close();

  @override
  bool dispatchEvent(web.Event event) => nativePort.dispatchEvent(event);

  @override
  web.Events get on => nativePort.on;
}

class _MessagePortWrapper extends _DelegatedPort {
  @override
  final web.MessagePort nativePort;

  _MessagePortWrapper(this.nativePort);

  @override
  void postMessage(message, [List<Object>? transfer]) =>
      nativePort.postMessage(message, transfer);
}

class _BroadcastPortWrapper extends _DelegatedPort {
  @override
  final web.BroadcastChannel nativePort;

  _BroadcastPortWrapper(this.nativePort);

  /// This presents a limitation of BroadcastChannel,
  /// i.e. it cannot carry transferables and will unconditionally clone the items.
  @override
  void postMessage(message, [List<Object>? transfer]) {
    if (transfer != null && transfer.isNotEmpty) {
      jsConsoleWarn("Ignoring transferables for BroadcastPort:", transfer);
    }
    nativePort.postMessage(message ?? false);
  }
}

extension on PortLike {
  static const messageEvent =
      web.EventStreamProvider<web.MessageEvent>('message');

  Stream<web.MessageEvent> get onMessage => messageEvent.forTarget(this);
}
