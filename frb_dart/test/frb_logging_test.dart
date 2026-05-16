import 'dart:async';

import 'package:flutter_rust_bridge/src/logging/frb_logging.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

void main() {
  tearDown(() async {
    kFrbDartLogging.dispose();
  });

  test('dispose without init is a no-op', () {
    kFrbDartLogging.dispose();
  });

  test('dispose invokes Rust logger cleanup callback', () async {
    final controller = StreamController<_RustLogRecord>();
    var didDisposeRustLogger = false;

    try {
      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: controller.stream,
        setupDefaultOutput: false,
        disposeRustLogger: () => didDisposeRustLogger = true,
        mapRecord: (record) => FrbLogRecordData(
          level: record.level,
          message: record.message,
          target: record.target,
          modulePath: null,
          file: null,
          line: null,
        ),
      );

      kFrbDartLogging.dispose();

      expect(didDisposeRustLogger, isTrue);
    } finally {
      await controller.close();
    }
  });

  test('init rejects duplicate initialization until disposed', () async {
    final firstController = StreamController<_RustLogRecord>();
    final secondController = StreamController<_RustLogRecord>();
    final thirdController = StreamController<_RustLogRecord>();

    try {
      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: firstController.stream,
        setupDefaultOutput: false,
        mapRecord: _mapRustLogRecord,
      );

      expect(
        () => kFrbDartLogging.init<_RustLogRecord>(
          rustLogStream: secondController.stream,
          setupDefaultOutput: false,
          mapRecord: _mapRustLogRecord,
        ),
        throwsA(isA<StateError>()),
      );

      kFrbDartLogging.dispose();

      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: thirdController.stream,
        setupDefaultOutput: false,
        mapRecord: _mapRustLogRecord,
      );
    } finally {
      kFrbDartLogging.dispose();
      await firstController.close();
      await secondController.close();
      await thirdController.close();
    }
  });

  test('Rust log records are forwarded to Dart logging', () async {
    final controller = StreamController<_RustLogRecord>();
    final receivedRecords = <LogRecord>[];
    final previousLevel = Logger.root.level;
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    Logger.root.level = Level.ALL;

    try {
      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: controller.stream,
        setupDefaultOutput: false,
        mapRecord: _mapRustLogRecord,
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

  test('Rust log stream errors are forwarded to Dart logging', () async {
    final controller = StreamController<_RustLogRecord>();
    final receivedRecords = <LogRecord>[];
    final previousLevel = Logger.root.level;
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    Logger.root.level = Level.ALL;

    try {
      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: controller.stream,
        setupDefaultOutput: false,
        mapRecord: _mapRustLogRecord,
      );

      final error = StateError('stream failed');
      controller.addError(error, StackTrace.current);
      await pumpEventQueue();

      expect(receivedRecords, hasLength(1));
      expect(receivedRecords.single.level, Level.SEVERE);
      expect(receivedRecords.single.message, 'Error in Rust log stream');
      expect(receivedRecords.single.error, same(error));
      expect(receivedRecords.single.loggerName, 'flutter_rust_bridge.logging');
    } finally {
      Logger.root.level = previousLevel;
      await subscription.cancel();
      await controller.close();
    }
  });

  test('Rust log levels are mapped to idiomatic Dart logging levels', () async {
    final controller = StreamController<_RustLogRecord>();
    final receivedRecords = <LogRecord>[];
    final previousLevel = Logger.root.level;
    final subscription = Logger.root.onRecord.listen(receivedRecords.add);
    Logger.root.level = Level.ALL;

    try {
      kFrbDartLogging.init<_RustLogRecord>(
        rustLogStream: controller.stream,
        setupDefaultOutput: false,
        mapRecord: _mapRustLogRecord,
      );

      controller.add(
        const _RustLogRecord(
          level: 'TRACE',
          message: 'trace message',
          target: 'rust.trace',
        ),
      );
      controller.add(
        const _RustLogRecord(
          level: 'DEBUG',
          message: 'debug message',
          target: 'rust.debug',
        ),
      );
      await pumpEventQueue();

      expect(receivedRecords.map((record) => record.level), [
        Level.FINER,
        Level.FINE,
      ]);
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

FrbLogRecordData _mapRustLogRecord(_RustLogRecord record) => FrbLogRecordData(
  level: record.level,
  message: record.message,
  target: record.target,
  modulePath: null,
  file: null,
  line: null,
);
