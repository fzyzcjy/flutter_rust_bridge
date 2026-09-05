@TestOn('browser')
library;

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/_web.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  test('broadcast stream values precede an early close in sequence order', () async {
    final port = broadcastPort('ordered stream');
    final sender = port.sendPort.nativePort as web.BroadcastChannel;
    addTearDown(port.close);
    addTearDown(() => sender.close());
    final received = port.take(3).map((value) => (value as JSString).toDart).toList();

    sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
    sender.postMessage(['__frb_stream', 1, 'second'].jsify());
    sender.postMessage(['__frb_stream', 0, 'first'].jsify());

    expect(await received, ['first', 'second', 'closed']);
  });

  test('rejected broadcast payload reservations do not block close', () async {
    final port = broadcastPort('rejected stream payload');
    final sender = port.sendPort.nativePort as web.BroadcastChannel;
    addTearDown(port.close);
    addTearDown(() => sender.close());
    final received = port.take(2).map((value) => (value as JSString).toDart).toList();

    sender.postMessage(['__frb_stream', 2, 'closed'].jsify());
    sender.postMessage(['__frb_stream', 1, 'value'].jsify());
    sender.postMessage(['__frb_stream', 0].jsify());

    expect(await received, ['value', 'closed']);
  });

  test('broadcast transport failure errors instead of closing across a gap', () async {
    final port = broadcastPort('failed stream');
    final sender = port.sendPort.nativePort as web.BroadcastChannel;
    addTearDown(port.close);
    addTearDown(() => sender.close());
    final received = port.first;

    sender.postMessage(['__frb_stream_failed'].jsify());

    await expectLater(received, throwsStateError);
  });

  test('ordinary message ports preserve reserved-looking user payloads', () async {
    final port = ReceivePort();
    final sender = port.sendPort.nativePort as web.MessagePort;
    addTearDown(port.close);
    addTearDown(() => sender.close());
    final received = port.first;

    sender.postMessage(['__frb_stream', 0, 'user data'].jsify());

    expect((await received as JSArray).dartify(), ['__frb_stream', 0, 'user data']);
  });
}
