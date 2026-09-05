@TestOn('browser')
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/_web.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  setUpAll(initializeBroadcastChannel);

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
        final close = ['__frb_stream', 2, 'closed'].jsify();

        if (closeFirst) sender.postMessage(close);
        sender.postMessage(['__frb_stream', 0, 'first'].jsify());
        sender.postMessage(['__frb_stream', 1, 'second'].jsify());
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

    sender.postMessage(['__frb_stream', 0, 'closed'].jsify());

    expect((await received as JSString).toDart, 'closed');
  });

  test('reversed stream values are delivered in sending order', () async {
    final port = broadcastPort('reversed values');
    addTearDown(port.close);
    final received = port
        .take(3)
        .map((value) => (value as JSString).toDart)
        .toList();
    final sender = port.sendPort.nativePort as web.MessagePort;
    sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
    sender.postMessage(['__frb_stream', 1, 'second'].jsify());
    sender.postMessage(['__frb_stream', 0, 'first'].jsify());
    expect(await received, ['first', 'second', 'closed']);
  });

  test('a rejected send does not block later values or close', () async {
    final port = broadcastPort('rejected value');
    addTearDown(port.close);
    final received = port
        .take(2)
        .map((value) => (value as JSString).toDart)
        .toList();
    final sender = port.sendPort.nativePort as web.MessagePort;
    sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
    sender.postMessage(['__frb_stream', 1, 'value'].jsify());
    sender.postMessage(['__frb_stream', 0].jsify());
    expect(await received, ['value', 'closed']);
  });

  test(
    'transport failure reports an error instead of waiting for a gap',
    () async {
      final port = broadcastPort('transport failure');
      addTearDown(port.close);
      final received = port.first;
      final sender = port.sendPort.nativePort as web.MessagePort;
      sender.postMessage(['__frb_stream', 1, 'value'].jsify());
      sender.postMessage(['__frb_stream_failed'].jsify());
      await expectLater(received, throwsStateError);
    },
  );

  test(
    'each listener independently waits for all values before close',
    () async {
      final receivePort = broadcastPort('multiple listeners');
      addTearDown(receivePort.close);
      final first = receivePort
          .take(3)
          .map((value) => (value as JSString).toDart)
          .toList();
      final second = receivePort
          .take(3)
          .map((value) => (value as JSString).toDart)
          .toList();
      final sender = receivePort.sendPort.nativePort as web.MessagePort;

      sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
      sender.postMessage(['__frb_stream', 0, 'first'].jsify());
      sender.postMessage(['__frb_stream', 1, 'second'].jsify());

      expect(await first, ['first', 'second', 'closed']);
      expect(await second, ['first', 'second', 'closed']);
    },
  );

  test('ordinary ports deliver close-shaped user data unchanged', () async {
    final receivePort = ReceivePort();
    addTearDown(receivePort.close);
    final sender = receivePort.sendPort.nativePort as web.MessagePort;
    addTearDown(() => sender.close());
    final received = receivePort.first;

    sender.postMessage(['__frb_stream', 0, 'user data'].jsify());

    expect((await received as JSArray).dartify(), [
      '__frb_stream',
      0,
      'user data',
    ]);
  });

  test('closing an old named port preserves its replacement', () async {
    final oldPort = broadcastPort('replacement');
    final replacement = broadcastPort('replacement');
    addTearDown(oldPort.close);
    addTearDown(replacement.close);
    final received = replacement.first;

    oldPort.close();
    final name = serializeNativePort(replacement.sendPort.nativePort);
    final sender = web.BroadcastChannel(name.split('/').first);
    addTearDown(() => sender.close());
    sender.postMessage(['replacement', 'new port'].jsify());

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
    expect(serializeNativePort(sender), endsWith('/buffered stream'));
  });

  test('workers send by name without an FRB worker bootstrap', () async {
    final url = web.URL.createObjectURL(
      web.Blob(
        [
          '''
      onmessage = ({data: names}) => {
        for (const name of names) {
          const separator = name.indexOf('/');
          const channel = new BroadcastChannel(name.slice(0, separator));
          const port = name.slice(separator + 1);
          channel.postMessage([port, ['__frb_stream', 0, 'first']]);
          channel.postMessage([port, ['__frb_stream', 1, 'second']]);
          channel.postMessage([port, ['__frb_stream', 2, 'closed']]);
          channel.close();
        }
      };
      '''
              .toJS,
        ].toJS,
      ),
    );
    addTearDown(() => web.URL.revokeObjectURL(url));
    final worker = web.Worker(url.toJS);
    addTearDown(() => worker.terminate());
    for (var batch = 0; batch < 100; batch++) {
      final ports = List.generate(
        8,
        (index) => broadcastPort('worker/$batch/$index'),
      );
      final received = ports
          .map(
            (port) => port
                .take(3)
                .map((value) => (value as JSString).toDart)
                .toList(),
          )
          .toList();
      try {
        worker.postMessage(
          ports
              .map((port) => serializeNativePort(port.sendPort.nativePort).toJS)
              .toList()
              .toJS,
        );
        for (final values in received) {
          expect(await values, ['first', 'second', 'closed']);
        }
      } finally {
        for (final port in ports) {
          port.close();
        }
      }
    }
  });
}
