import 'dart:isolate';

export 'dart:ffi' show NativePort;
export 'dart:isolate';

/// {@macro flutter_rust_bridge.internal}
ReceivePort broadcastPort(String channelName) => ReceivePort(channelName);
