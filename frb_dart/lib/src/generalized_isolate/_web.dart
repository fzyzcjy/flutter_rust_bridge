/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html' as html;
import 'dart:html' hide MessagePort;

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';

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
  final channel = MessageChannel();

  @override
  SendPort get sendPort => PortLike.messagePort(channel.port2);

  @override
  SendPort get receivePort => PortLike.messagePort(channel.port1);
}

class _BroadcastChannelWrapper implements Channel {
  final BroadcastChannel _sendChannel;
  final BroadcastChannel _receiveChannel;

  _BroadcastChannelWrapper(String channelName)
      // Note: It is *wrong* to reuse the same HTML BroadcastChannel object,
      // because HTML BroadcastChannel spec says that, the event will not be fired
      // at the object which sends it. Therefore, we need two different objects.
      : _sendChannel = BroadcastChannel(channelName),
        _receiveChannel = BroadcastChannel(channelName);

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

  static dynamic _extractData(MessageEvent event) => event.data;

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

/// [html.MessagePort]'s interface.
abstract class PortLike {
  const PortLike._();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory PortLike.messagePort(html.MessagePort port) = _MessagePortWrapper;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  factory PortLike.broadcastChannel(BroadcastChannel channel) =
      _BroadcastPortWrapper;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void postMessage(Object? value);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void close();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  html.EventTarget get nativePort;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Stream<MessageEvent> get onMessage => _kMessageEvent.forTarget(nativePort);
  static const _kMessageEvent = EventStreamProvider<MessageEvent>('message');
}

class _MessagePortWrapper extends PortLike {
  @override
  final html.MessagePort nativePort;

  _MessagePortWrapper(this.nativePort) : super._();

  @override
  void postMessage(message, [List<Object>? transfer]) =>
      nativePort.postMessage(message, transfer);

  @override
  void close() => nativePort.close();
}

class _BroadcastPortWrapper extends PortLike {
  @override
  final html.BroadcastChannel nativePort;

  _BroadcastPortWrapper(this.nativePort) : super._();

  /// This presents a limitation of BroadcastChannel,
  /// i.e. it cannot carry transferables and will unconditionally clone the items.
  @override
  void postMessage(message, [List<Object>? transfer]) {
    if (transfer != null && transfer.isNotEmpty) {
      jsConsoleWarn("Ignoring transferables for BroadcastPort:", transfer);
    }
    nativePort.postMessage(message ?? false);
  }

  @override
  void close() => nativePort.close();
}
