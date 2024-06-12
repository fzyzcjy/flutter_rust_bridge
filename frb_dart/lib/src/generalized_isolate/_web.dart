/// Shims for dart:isolate on the web.
library html_isolate;

import 'dart:async';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.toString();

/// {@template flutter_rust_bridge.same_as_native}
/// Web implementation of the one with same name in native.
/// {@endtemplate}
class ReceivePort extends StreamView<dynamic> {
  /// {@macro flutter_rust_bridge.same_as_native}
  ReceivePort() : super(throw UnimplementedError());

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => throw UnimplementedError();
}

/// {@macro flutter_rust_bridge.same_as_native}
class RawReceivePort {
  /// {@macro flutter_rust_bridge.same_as_native}
  set handler(void Function(dynamic) handler) {
    throw UnimplementedError();
  }

  /// {@macro flutter_rust_bridge.same_as_native}
  void close() => throw UnimplementedError();

  /// {@macro flutter_rust_bridge.same_as_native}
  SendPort get sendPort => throw UnimplementedError();
}

/// {@macro flutter_rust_bridge.same_as_native}
class SendPort {
  /// {@macro flutter_rust_bridge.same_as_native}
  int get nativePort => _nativePort;
  final int _nativePort;

  SendPort._(this._nativePort);
}

// TODO
// class _PortNameGenerator {
//   static int _nextPort = 0;
//
//   static String create() => '__frb_port_${_nextPort++}';
// }
