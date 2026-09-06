@TestOn('browser')
library;

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/_web.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  test(
    'lazy broadcast ports preserve reserved-looking user payloads',
    () async {
      final port = broadcastPort('__frb_lazy_port_protocol_test');
      final sender = port.sendPort.nativePort as web.MessagePort;
      addTearDown(port.close);
      addTearDown(() => sender.close());
      final received = port.take(2).toList();

      sender.postMessage(['__frb_stream', 0, 'user data'].jsify());
      sender.postMessage(['__frb_stream_failed'].jsify());

      final values = await received;
      expect(values, everyElement(isA<List<Object?>>()));
      expect(values, [
        ['__frb_stream', 0, 'user data'],
        ['__frb_stream_failed'],
      ]);
    },
  );

  test(
    'broadcast stream values precede an early close in sequence order',
    () async {
      final port = broadcastPort('__frb_streamsink_ordered');
      final sender = port.sendPort.nativePort as web.MessagePort;
      addTearDown(port.close);
      addTearDown(() => sender.close());
      final received = port.take(3).toList();

      sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
      sender.postMessage(['__frb_stream', 1, 'second'].jsify());
      sender.postMessage(['__frb_stream', 0, 'first'].jsify());

      final values = await received;
      expect(values, everyElement(isA<String>()));
      expect(values, ['first', 'second', 'closed']);
    },
  );

  test('rejected broadcast payload reservations do not block close', () async {
    final port = broadcastPort('__frb_streamsink_rejected');
    final sender = port.sendPort.nativePort as web.MessagePort;
    addTearDown(port.close);
    addTearDown(() => sender.close());
    final received = port.take(2).toList();

    sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
    sender.postMessage(['__frb_stream', 1, 'value'].jsify());
    sender.postMessage(['__frb_stream', 0].jsify());

    final values = await received;
    expect(values, everyElement(isA<String>()));
    expect(values, ['value', 'closed']);
  });

  test(
    'broadcast transport failure errors instead of closing across a gap',
    () async {
      final port = broadcastPort('__frb_streamsink_failed');
      final sender = port.sendPort.nativePort as web.MessagePort;
      addTearDown(port.close);
      addTearDown(() => sender.close());
      final received = port.first;

      sender.postMessage(['__frb_stream_failed'].jsify());

      await expectLater(received, throwsStateError);
    },
  );

  test(
    'ordinary message ports preserve reserved-looking user payloads',
    () async {
      final port = ReceivePort();
      final sender = port.sendPort.nativePort as web.MessagePort;
      addTearDown(port.close);
      addTearDown(() => sender.close());
      final received = port.first;

      sender.postMessage(['__frb_stream', 0, 'user data'].jsify());

      final value = await received;
      expect(value, isA<List<Object?>>());
      expect(value, ['__frb_stream', 0, 'user data']);
    },
  );
}
