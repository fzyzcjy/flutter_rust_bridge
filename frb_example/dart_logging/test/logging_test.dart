import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:frb_example_dart_logging/Logger.dart';
import 'package:frb_example_dart_logging/main.dart';
import 'package:frb_example_dart_logging/src/rust/api/logging_example.dart';
import 'package:frb_example_dart_logging/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');

  print('Action: Setup logging tests');

  var logOutput = "no output received";

  init_logger();
  LOGGER.setLogFunction(
      (LogRecord record) => logOutput = "${record.level}! - ${record.message}");

  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  test('log message from dart to stdout', () async {
    final msg = "test log from dart";
    LOGGER.info(msg);
    expect(logOutput, "INFO! - ${msg}");
  });
  print('Action: Configure tests (end)');
}
