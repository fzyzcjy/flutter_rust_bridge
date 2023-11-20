/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html' as html;
import 'dart:html' hide MessagePort;

import 'package:flutter_rust_bridge/src/ffi/web.dart';

typedef MessagePort = PortLike;

/// An alias to [MessagePort] on web platforms.
typedef SendPort = PortLike;

typedef NativePortType = dynamic;

abstract class Channel {
  SendPort get sendPort;
  SendPort get receivePort;
  const Channel();
  factory Channel.messageChannel() = _MessageChannelWrapper;
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
  final BroadcastChannel channel;
  _BroadcastChannelWrapper(String channelName)
      : channel = BroadcastChannel(channelName);
  @override
  SendPort get sendPort => PortLike.broadcastChannel(channel);
  @override
  SendPort get receivePort => sendPort;
}

/// Wrapper around a [MessageChannel].
class RawReceivePort {
  /// The underlying message channel.
  final Channel channel;
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

ReceivePort broadcastPort(String channelName) =>
    ReceivePort(RawReceivePort(Channel.broadcastChannel(channelName)));

/// [html.MessagePort]'s interface.
abstract class PortLike extends EventTarget {
  factory PortLike.messagePort(html.MessagePort port) = _MessagePortWrapper;
  factory PortLike.broadcastChannel(BroadcastChannel channel) =
      _BroadcastPortWrapper;
  void postMessage(Object? value);
  void close();
  NativePortType get nativePort;
}

/// Delegates a subset of PortLike methods verbatim.
abstract class _DelegatedPort implements PortLike {
  @override
  void addEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      nativePort.addEventListener(type, listener, useCapture);

  @override
  void removeEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      nativePort.removeEventListener(type, listener, useCapture);

  @override
  void close() => nativePort.close();

  @override
  bool dispatchEvent(html.Event event) => nativePort.dispatchEvent(event);

  @override
  html.Events get on => nativePort.on;
}

class _MessagePortWrapper extends _DelegatedPort {
  @override
  final html.MessagePort nativePort;
  _MessagePortWrapper(this.nativePort);

  @override
  void postMessage(message, [List<Object>? transfer]) =>
      nativePort.postMessage(message, transfer);
}

class _BroadcastPortWrapper extends _DelegatedPort {
  @override
  final html.BroadcastChannel nativePort;
  _BroadcastPortWrapper(this.nativePort);

  /// This presents a limitation of BroadcastChannel,
  /// i.e. it cannot carry transferables and will unconditionally clone the items.
  @override
  void postMessage(message, [List<Object>? transfer]) {
    if (transfer != null && transfer.isNotEmpty) {
      warn("Ignoring transferables for BroadcastPort:", transfer);
    }
    nativePort.postMessage(message ?? false);
  }
}

extension on PortLike {
  static const messageEvent = EventStreamProvider<MessageEvent>('message');
  Stream<MessageEvent> get onMessage => messageEvent.forTarget(this);
}
