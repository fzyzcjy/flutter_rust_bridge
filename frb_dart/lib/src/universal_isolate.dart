/// A pure Dart implementation of an isolate, meant to substitute dart:isolate on Web.
library universal_isolate;

abstract class SendPort {
  void send(Object? message);

  @override
  bool operator ==(var other);

  @override
  int get hashCode;

  /// Parity with dart:ffi extension.
  int get nativePort;
}

class _JsonSendPort implements SendPort {
  @override
  final int nativePort;

  const _JsonSendPort(this.nativePort);

  @override
  int get hashCode => nativePort.hashCode;

  @override
  bool operator ==(var other) => other is _JsonSendPort && other.nativePort == nativePort;

  @override
  void send(Object? message) => pm.send(nativePort, message);
}

abstract class RawReceivePort {
  void close();
  factory RawReceivePort([Function? handler, String? debugName]) = _JsonReceivePort;
  set handler(Function? newHandler);
  SendPort get sendPort;
}

class _JsonReceivePort implements RawReceivePort {
  late int _port;
  String? debugName;
  _JsonReceivePort([Function? handler, this.debugName = '']) {
    _port = pm.makePort(handler as VoidFunction?);
  }

  @override
  void close() => pm.close(_port);

  @override
  set handler(covariant VoidFunction? f) => pm.setHandler(_port, f);

  @override
  SendPort get sendPort => _JsonSendPort(_port);
}

final pm = _PortManager();
typedef VoidFunction = void Function(Object? obj);

class _PortManager {
  final Map<int, VoidFunction?> handlers = {};
  int _idx = -9007199254740991; // Number.MIN_SAFE_INTEGER
  int makePort([VoidFunction? handler]) {
    handlers[_idx] = handler;
    return _idx++;
  }

  void setHandler(int port, VoidFunction? handler) {
    assert(port < handlers.length, "Port $port is not a valid port.");
    handlers[port] = handler;
  }

  void close(int port) {
    assert(port < handlers.length, "Port $port is not a valid port.");
    handlers[port] = null;
  }

  bool send(int port, Object? args) {
    assert(port < handlers.length, "Port $port is not a valid port.");
    handlers[port]?.call(args);
    return handlers[port] != null;
  }
}
