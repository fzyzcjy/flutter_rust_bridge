// ignore_for_file: implementation_imports

import 'dart:convert';
import 'dart:html';

import 'main.dart' as io;
import 'package:js/js.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS()
external void close();

void main() async {
  final result = await directRunTests(
    () => io.main(['stub']),
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      PrintSink(),
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );
  _close(result);
}

void _close(bool result) {
  final socket = WebSocket(Uri.base.replace(scheme: 'ws').toString());
  socket.onOpen.first.then((_) {
    socket.send(jsonEncode({'__result__': result}));
    close();
  });
}
