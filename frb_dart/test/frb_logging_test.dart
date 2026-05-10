import 'dart:async';

import 'package:flutter_rust_bridge/src/logging/frb_logging.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

void main() {
  tearDown(() async {
    await FrbDartLogging.dispose();
  });

  test('Rust log records are forwarded to Dart logging', () async {
    final controller = StreamController<_RustLogRecord>();
    final receivedRecords = <LogRecord>[];
    final previousLevel = Logger.root.level;
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    Logger.root.level = Level.ALL;

    try {
      FrbDartLogging.init<_RustLogRecord>(
        rustLogStream: controller.stream,
        setupDefaultOutput: false,
        mapRecord: (record) => FrbLogRecordData(
          level: record.level,
          message: record.message,
          target: record.target,
          modulePath: null,
          file: null,
          line: null,
        ),
      );

      controller.add(
        const _RustLogRecord(
          level: 'WARN',
          message: 'disk almost full',
          target: 'rust.storage',
        ),
      );
      await pumpEventQueue();

      expect(receivedRecords, hasLength(1));
      expect(receivedRecords.single.level, Level.WARNING);
      expect(receivedRecords.single.message, 'disk almost full');
      expect(receivedRecords.single.loggerName, 'rust.storage');
    } finally {
      Logger.root.level = previousLevel;
      await subscription.cancel();
      await controller.close();
    }
  });
}

class _RustLogRecord {
  final String level;
  final String message;
  final String target;

  const _RustLogRecord({
    required this.level,
    required this.message,
    required this.target,
  });
}
