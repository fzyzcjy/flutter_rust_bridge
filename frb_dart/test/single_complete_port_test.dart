@TestOn('vm')
import 'dart:async';

import 'package:flutter_rust_bridge/src/utils/single_complete_port.dart';
import 'package:test/test.dart';

void main() {
  test('singleCompletePort completes with its first message only', () async {
    final completer = Completer<int>();
    final port = singleCompletePort<int, Object?>(completer);

    port.sendPort.send(42);
    port.sendPort.send(7);

    expect(await completer.future, 42);
  });

  test(
    'singleCompletePort completes with a cast error for an invalid message',
    () async {
      final completer = Completer<int>();
      final port = singleCompletePort<int, Object?>(completer);

      port.sendPort.send('not an int');

      await expectLater(completer.future, throwsA(isA<TypeError>()));
    },
  );

  test('singleCompletePort can close before receiving a message', () async {
    final completer = Completer<int>();
    final port = singleCompletePort<int, Object?>(completer);

    port.close();
    port.sendPort.send(42);
    await Future<void>.delayed(Duration.zero);

    expect(completer.isCompleted, isFalse);
  });
}
