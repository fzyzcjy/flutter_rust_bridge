import 'dart:isolate';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export 'dart:ffi' show NativePort;
export 'dart:isolate';

/// {@macro flutter_rust_bridge.internal}
const bool orderedReceivePortDrainsAfterCancel = false;

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName, {bool ordered = false}) =>
    ReceivePort(channelName);

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.toString();
