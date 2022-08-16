/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';
import 'dart:html' as html;
import 'dart:html' hide MessagePort;

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

// TODO: Find a way to make this less verbose
class _MessagePortWrapper implements PortLike, html.MessagePort {
  final html.MessagePort port;
  _MessagePortWrapper(this.port);

  @override
  get nativePort => port;
  @override
  void addEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      port.addEventListener(type, listener, useCapture);
  @override
  bool dispatchEvent(html.Event event) => port.dispatchEvent(event);
  @override
  void removeEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      port.removeEventListener(type, listener, useCapture);
  @override
  void postMessage(message, [List<Object>? transfer]) =>
      port.postMessage(message, transfer);
  @override
  void close() => port.close();
  @override
  html.Events get on => port.on;
  @override
  Stream<html.MessageEvent> get onMessage => port.onMessage;
}

class _BroadcastPortWrapper implements PortLike, html.BroadcastChannel {
  final html.BroadcastChannel channel;
  _BroadcastPortWrapper(this.channel);

  @override
  get nativePort => channel;
  @override
  void addEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      channel.addEventListener(type, listener, useCapture);
  @override
  bool dispatchEvent(html.Event event) => channel.dispatchEvent(event);
  @override
  void removeEventListener(String type, html.EventListener? listener,
          [bool? useCapture]) =>
      channel.removeEventListener(type, listener, useCapture);
  @override
  void postMessage(message, [List<Object>? transfer]) =>
      channel.postMessage(message ?? false);
  @override
  void close() => channel.close();
  @override
  html.Events get on => channel.on;
  @override
  Stream<html.MessageEvent> get onMessage => channel.onMessage;
  @override
  String? get name => channel.name;
}

extension on PortLike {
  static const messageEvent = EventStreamProvider<MessageEvent>('message');
  Stream<MessageEvent> get onMessage => messageEvent.forTarget(this);
}
