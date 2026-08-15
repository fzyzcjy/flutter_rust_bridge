@TestOn('browser')
library;

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/codec/dco.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/stream/stream_sink.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
  /// Buffers BroadcastChannel messages sent before Dart subscribes.
  test('broadcast port buffers messages before listen', () async {
    final channelName =
        'frb-broadcast-buffer-${DateTime.now().microsecondsSinceEpoch}';
    final port = broadcastPort(channelName);
    final sender = web.BroadcastChannel(channelName);
    addTearDown(() {
      port.close();
      sender.close();
    });

    sender.postMessage(42.toJS);
    await Future<void>.delayed(Duration.zero);

    expect(await port.first, 42);
  });

  /// Closes both Dart BroadcastChannel objects owned by a receive port.
  test('closing ordered port closes its serialized channel', () {
    final channelName =
        'frb-ordered-close-${DateTime.now().microsecondsSinceEpoch}';
    final port = broadcastPort(channelName, ordered: true);
    final serializedChannel = port.sendPort.nativePort as web.BroadcastChannel;

    port.close();

    expect(
      () => serializedChannel.postMessage(null),
      throwsA(isA<web.DOMException>()),
    );
  });

  /// Responds to a Rust worker's BroadcastChannel readiness probe.
  test('broadcast port acknowledges a readiness probe', () async {
    final channelName =
        'frb-readiness-${DateTime.now().microsecondsSinceEpoch}';
    final port = broadcastPort(channelName);
    final probe = web.BroadcastChannel(
      '${channelName}__flutter_rust_bridge_ready',
    );
    addTearDown(() {
      port.close();
      probe.close();
    });

    final acknowledged = const web.EventStreamProvider<web.MessageEvent>(
      'message',
    ).forTarget(probe).first;
    probe.postMessage(null);

    await acknowledged.timeout(const Duration(seconds: 1));
  });

  /// Completes outer cancellation while draining the ordered source to ACK.
  test('RustStreamSink cancel completes before ordered release', () async {
    const codec = DcoCodec<int, Exception>(
      decodeSuccessData: _decodeInt,
      decodeErrorData: null,
    );
    final sink = RustStreamSink<int>();
    final channelName = sink.setupAndSerialize(codec: codec);
    final sender = web.BroadcastChannel(channelName);
    final release = web.BroadcastChannel(
      '${channelName}__flutter_rust_bridge_release',
    );
    addTearDown(() {
      sender.close();
      release.close();
    });

    final events = <int>[];
    final subscription = sink.stream.listen(events.add);
    await subscription.cancel().timeout(const Duration(seconds: 1));

    final acknowledged = const web.EventStreamProvider<web.MessageEvent>(
      'message',
    ).forTarget(release).first;
    sender
      ..postMessage(
        <Object?>[
          0,
          0,
          <Object?>[0, 42],
        ].jsify(),
      )
      ..postMessage(
        <Object?>[
          0,
          1,
          <Object?>[],
          <Object?>[2],
          true,
        ].jsify(),
      );

    await acknowledged.timeout(const Duration(seconds: 1));
    expect(events, isEmpty);
  });

  /// Drains an ordered port through its release marker after consumer cancel.
  test('ordered port acknowledges release after consumer cancel', () async {
    final channelName =
        'frb-ordered-cancel-${DateTime.now().microsecondsSinceEpoch}';
    final port = broadcastPort(channelName, ordered: true);
    final sender = web.BroadcastChannel(channelName);
    final release = web.BroadcastChannel(
      '${channelName}__flutter_rust_bridge_release',
    );
    addTearDown(() {
      port.close();
      sender.close();
      release.close();
    });

    final events = <dynamic>[];
    final subscription = port.listen(events.add);
    await subscription.cancel();

    final acknowledged = const web.EventStreamProvider<web.MessageEvent>(
      'message',
    ).forTarget(release).first;
    sender
      ..postMessage(<Object?>[0, 0, 42].jsify())
      ..postMessage(<Object?>[0, 1, <Object?>[], 'close', true].jsify());

    await acknowledged.timeout(const Duration(seconds: 1));
    expect(events, isEmpty);
  });
}

int _decodeInt(dynamic raw) => raw as int;
