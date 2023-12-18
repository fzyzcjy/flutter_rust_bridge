// ignore_for_file: implementation_imports, avoid_print

import 'dart:async';
import 'dart:convert';
import 'dart:html';

import 'package:flutter_rust_bridge_utils/src/dart_web_test_utils/run_test.dart';
import 'package:js/js.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS('close')
external void _jsClose();

Future<void> dartWebTestEntrypoint(FutureOr<void> Function() testMain) async {
  final result = await directRunTests(
    testMain,
    reporterFactory: (engine) => ExpandedReporter.watch(engine, PrintSink(),
        color: true, printPlatform: false, printPath: false),
  );

  // print('hack: sleep forever...');
  // await Future.delayed(Duration(seconds: 10000000));

  await _sendResult(result: result);

  _jsClose();
}

Future<void> _sendResult({required bool result}) async {
  final url = Uri.base.replace(scheme: 'ws').toString();
  print('sendResult result=$result to url=$url');

  final socket = WebSocket(url);
  await socket.onOpen.first;

  socket.send(jsonEncode({kTestResultKey: result}));
}
