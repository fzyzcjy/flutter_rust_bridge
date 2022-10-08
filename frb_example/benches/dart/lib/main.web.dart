// ignore_for_file: implementation_imports

import 'dart:convert';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart' hide useJSON;
import 'package:flutter_rust_bridge/src/env/web.dart' show useJSON; // need for const

import 'main.dart' as io;
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS()
external void close();

void main() async {
  final result = await directRunTests(
    // on web, useJSON is not propagated to wasm, so pipe as arg instead
    () => io.main(useJSON ? ['stub', '--json'] : ['stub']),
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      PrintSink(),
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );
  sendAndClose(result);
}

void sendAndClose(bool result) {
  final socket = WebSocket(Uri.base.replace(scheme: 'ws').toString());
  socket.onOpen.first.then((_) {
    socket.send(jsonEncode({'__result__': result}));
    close();
  });
}
