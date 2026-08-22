@TestOn('vm')
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';

void main() {
  test(
    'broadcastPort receives messages and serializes its native port',
    () async {
      final receivePort = broadcastPort('frb generalized isolate test');
      addTearDown(receivePort.close);

      final received = receivePort.first;
      receivePort.sendPort.send('message');

      expect(await received, 'message');
      expect(
        serializeNativePort(receivePort.sendPort.nativePort),
        receivePort.sendPort.nativePort.toString(),
      );
    },
  );
}
