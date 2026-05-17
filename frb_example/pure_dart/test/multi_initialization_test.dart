import 'package:frb_example_pure_dart/src/rust/api/frb_logging.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/simple_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('Can initialize twice and refresh logging sink (mimic Dart hot restart)',
      () async {
    final previousLevel = Logger.root.level;
    final receivedRecords = <LogRecord>[];
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    final uniqueSuffix = DateTime.now().microsecondsSinceEpoch;
    final firstMessage = 'first rust logging bridge message $uniqueSuffix';
    final secondMessage = 'second rust logging bridge message $uniqueSuffix';
    var didInitialize = false;
    Logger.root.level = Level.ALL;

    addTearDown(() async {
      if (didInitialize) {
        RustLib.dispose();
      }
      Logger.root.level = previousLevel;
      await subscription.cancel();
    });

    // Step 1: Initialize once and confirm ordinary calls work before restart.
    await RustLib.init();
    didInitialize = true;

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

    await emitLogMessage(message: firstMessage);
    await pumpEventQueue();

    expect(_countLogMessage(receivedRecords, firstMessage), 1);
    expect(_countLogMessage(receivedRecords, secondMessage), 0);

    // Step 2: Dispose Dart-side state to mimic hot restart teardown.
    RustLib.dispose();

    // Step 3: Initialize again and verify both calls and logging use the new state.
    await RustLib.init();

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

    await emitLogMessage(message: secondMessage);
    await pumpEventQueue();

    // Step 4: Exercise the platform console fallback path without asserting
    // platform-specific output capture.
    await printToConsoleSmokeTest();

    expect(_countLogMessage(receivedRecords, firstMessage), 1);
    expect(_countLogMessage(receivedRecords, secondMessage), 1);
    expect(
        _hasLogRecord(receivedRecords, secondMessage, Level.WARNING), isTrue);
  });
}

int _countLogMessage(List<LogRecord> records, String message) =>
    records.where((record) => record.message == message).length;

bool _hasLogRecord(List<LogRecord> records, String message, Level level) =>
    records.any((record) => record.message == message && record.level == level);
