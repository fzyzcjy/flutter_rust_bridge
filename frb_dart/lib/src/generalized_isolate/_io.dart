import 'dart:isolate';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export 'dart:ffi' show NativePort;
export 'dart:isolate';

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName, {bool keepIsolateAlive = true}) {
  if (keepIsolateAlive) {
    return ReceivePort(channelName);
  }

  final rawReceivePort = RawReceivePort(null, channelName);
  rawReceivePort.keepIsolateAlive = false;
  return ReceivePort.fromRawReceivePort(rawReceivePort);
}

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.toString();
