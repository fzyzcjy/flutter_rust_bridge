@TestOn('browser')
import 'dart:js_interop';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

@JS('globalThis.__frb_named_ports')
external JSObject get _namedPorts;

void main() {
  for (final closeFirst in [true, false]) {
    test(
      'stream receives every value before close, closeFirst=$closeFirst',
      () async {
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
      },
    );
  }

  test('empty stream closes without waiting for a value', () async {
    final receivePort = broadcastPort('empty stream');
    addTearDown(receivePort.close);
    final received = receivePort.first;
    final sender = receivePort.sendPort.nativePort as web.MessagePort;

    sender.postMessage(['__frb_stream_close', 0, 'closed'].jsify());

    expect((await received as JSString).toDart, 'closed');
  });

  test('each listener independently waits for all values before close', () async {
    final receivePort = broadcastPort('multiple listeners');
    addTearDown(receivePort.close);
    final first = receivePort.take(3).map((value) => (value as JSString).toDart).toList();
    final second = receivePort.take(3).map((value) => (value as JSString).toDart).toList();
    final sender = receivePort.sendPort.nativePort as web.MessagePort;

    sender.postMessage(['__frb_stream_close', 2, 'closed'].jsify());
    sender.postMessage('first'.toJS);
    sender.postMessage('second'.toJS);

    expect(await first, ['first', 'second', 'closed']);
    expect(await second, ['first', 'second', 'closed']);
  });

  test('ordinary ports deliver close-shaped user data unchanged', () async {
    final receivePort = ReceivePort();
    addTearDown(receivePort.close);
    final sender = receivePort.sendPort.nativePort as web.MessagePort;
    addTearDown(sender.close);
    final received = receivePort.first;

    sender.postMessage(['__frb_stream_close', 0, 'user data'].jsify());

    expect((await received as JSArray).dartify(), ['__frb_stream_close', 0, 'user data']);
  });

  test('closing an old named port preserves its replacement', () async {
    final oldPort = broadcastPort('replacement');
    final replacement = broadcastPort('replacement');
    addTearDown(oldPort.close);
    addTearDown(replacement.close);
    final received = replacement.first;

    oldPort.close();
    final sender = _namedPorts.getProperty<web.MessagePort>('replacement'.toJS);
    sender.postMessage('new port'.toJS);

    expect((await received as JSString).toDart, 'new port');
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
