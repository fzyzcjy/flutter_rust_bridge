import 'dart:isolate';
export 'dart:isolate';

ReceivePort broadcastPort(String channelName) {
  return ReceivePort(channelName);
}
