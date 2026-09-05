@TestOn('browser')
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  for (final closeFirst in [true, false]) {
    test('stream receives every value before close, closeFirst=$closeFirst', () async {
      final receivePort = broadcastPort('stream close order');
      addTearDown(receivePort.close);
      final sender = receivePort.sendPort.nativePort as web.MessagePort;
      final received = receivePort
          .take(3)
          .map((value) => (value as JSString).toDart)
          .toList();
      final close = ['__frb_stream_close', 2, 'closed'].jsify();

      if (closeFirst) sender.postMessage(close);
      sender.postMessage('first'.toJS);
      sender.postMessage('second'.toJS);
      if (!closeFirst) sender.postMessage(close);

      expect(await received, ['first', 'second', 'closed']);
    });
  }

  test('empty stream closes without waiting for a value', () async {
    final receivePort = broadcastPort('empty stream');
    addTearDown(receivePort.close);
    final received = receivePort.first;
    final sender = receivePort.sendPort.nativePort as web.MessagePort;

    sender.postMessage(['__frb_stream_close', 0, 'closed'].jsify());

    expect((await received as JSString).toDart, 'closed');
  });

  test('named port snapshots messages sent before listening', () async {
    final receivePort = broadcastPort('buffered stream');
    addTearDown(receivePort.close);
    final sender = receivePort.sendPort.nativePort as web.MessagePort;
    final value = [7.toJS].toJS;

    sender.postMessage(value);
    value[0] = 99.toJS;

    final received = await receivePort.first as JSArray<JSNumber>;
    expect(received[0].toDartInt, 7);
    expect(serializeNativePort(sender), 'buffered stream');
  });
}
