import 'dart:isolate';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

export 'dart:ffi' show NativePort;
export 'dart:isolate';

@internal
Future<void> initializeBroadcastChannel() async {}

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) => ReceivePort(channelName);

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.toString();
