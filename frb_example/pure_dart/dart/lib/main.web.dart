// ignore_for_file: implementation_imports

import 'dart:convert';
import 'dart:html';

import 'main.dart' as io;
import 'package:js/js.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

@JS()
external Object fetch(String path, [FetchOptions? options]);

@JS()
external void close();

@JS()
@anonymous
class FetchOptions {
  external factory FetchOptions({String method, dynamic body});
}

class WebSocketSink extends PrintSink {
  final WebSocket socket;
  final Future<void> opened;
  WebSocketSink._(this.socket) : opened = socket.onOpen.first;
  factory WebSocketSink() => WebSocketSink._(WebSocket(Uri.base.replace(scheme: 'ws').toString()));

  @override
  void write(Object? obj) {
    super.write(obj);
    opened.then((_) {
      socket.sendString(jsonEncode(obj));
    });
  }

  @override
  void writeAll(Iterable objects, [String separator = ' ']) {
    super.writeAll(objects, separator);
    opened.then((_) {
      objects.map(jsonEncode).forEach(socket.sendString);
    });
  }

  @override
  void writeln([Object? obj]) {
    super.writeln(obj);
    opened.then((_) {
      socket.sendString(jsonEncode(obj));
    });
  }

  void close_(bool result) {
    opened.then((_) {
      socket.sendString(jsonEncode({'__result__': result}));
      close();
    });
  }
}

void main() async {
  final sink = WebSocketSink();
  final result = await directRunTests(
    () => io.main(['stub']),
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      sink,
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );
  sink.close_(result);
}
