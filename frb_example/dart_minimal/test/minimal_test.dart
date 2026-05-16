import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

Future<void> main() async {
  final previousLevel = Logger.root.level;
  final receivedRecords = <LogRecord>[];
  final subscription = Logger.root.onRecord.listen(receivedRecords.add);
  Logger.root.level = Level.ALL;

  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  tearDownAll(() async {
    Logger.root.level = previousLevel;
    await subscription.cancel();
  });

  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });

  test('rust logs are forwarded to Dart logging', () async {
    await emitLogMessage();
    await pumpEventQueue();

    expect(
      receivedRecords,
      contains(
        isA<LogRecord>()
            .having((record) => record.level, 'level', Level.WARNING)
            .having(
              (record) => record.message,
              'message',
              'hello from rust logging bridge',
            ),
      ),
    );
  });
  print('Action: Configure tests (end)');
}
