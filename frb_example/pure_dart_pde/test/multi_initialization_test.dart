// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/frb_logging.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/simple_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('Can initialize twice and refresh logging sink (mimic Dart hot restart)',
      () async {
    final previousLevel = Logger.root.level;
    final receivedRecords = <LogRecord>[];
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    Logger.root.level = Level.ALL;

    addTearDown(() async {
      Logger.root.level = previousLevel;
      await subscription.cancel();
    });

    // Step 1: Initialize once and confirm ordinary calls work before restart.
    await RustLib.init();

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

    // Step 2: Reset Dart-side singleton state to mimic hot restart.
    // ignore: invalid_use_of_internal_member
    RustLib.instance.resetState();

    // Step 3: Initialize again and verify both calls and logging use the new state.
    await RustLib.init();

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

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
}
