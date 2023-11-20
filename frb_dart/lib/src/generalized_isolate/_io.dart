import 'dart:isolate';
export 'dart:isolate';

ReceivePort broadcastPort(String channelName) => ReceivePort(channelName);
