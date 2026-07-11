// ignore_for_file: implementation_imports, avoid_print, deprecated_member_use, deprecated_member_use_from_same_package

import 'dart:async';
import 'dart:js_interop';

import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS('close')
external void _jsClose();

@JS('sendTestResult')
external JSPromise<JSAny?> _jsSendTestResult(JSBoolean result);

Future<void> dartWebTestEntrypoint(FutureOr<void> Function() testMain) async {
  final result = await directRunTests(
    testMain,
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      PrintSink(),
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );

  // print('hack: sleep forever...');
  // await Future.delayed(Duration(seconds: 10000000));

  await _sendResult(result: result);

  _jsClose();
}

Future<void> _sendResult({required bool result}) async {
  print('sendResult result=$result');
  await _jsSendTestResult(result.toJS).toDart;
}
