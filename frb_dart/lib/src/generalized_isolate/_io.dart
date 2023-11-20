import 'dart:isolate';

export 'dart:ffi' show NativePort;
export 'dart:isolate';

ReceivePort broadcastPort(String channelName) => ReceivePort(channelName);
