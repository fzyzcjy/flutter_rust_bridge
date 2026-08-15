@TestOn('browser')
library;

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:test/test.dart';
import 'package:web/web.dart' as web;

void main() {
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
