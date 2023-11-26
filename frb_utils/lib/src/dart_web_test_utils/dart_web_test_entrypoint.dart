// ignore_for_file: implementation_imports

import 'dart:async';
import 'dart:convert';
import 'dart:html';

import 'package:js/js.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS('close')
external void _jsClose();

Future<void> dartWebTestEntrypoint(FutureOr<void> Function() testMain) async {
  final result = await directRunTests(
    testMain,
    reporterFactory: (engine) =>
        ExpandedReporter.watch(engine, PrintSink(), color: true, printPlatform: false, printPath: false),
  );

  await _sendResult(result: result);

  _jsClose();
}

Future<void> _sendResult({required bool result}) async {
  final url = Uri.base.replace(scheme: 'ws').toString();
  print('sendResult result=$result to url=$url');

  final socket = WebSocket(url);
  await socket.onOpen.first;

  socket.send(jsonEncode({'__result__': result}));
}
