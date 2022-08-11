// ignore_for_file: implementation_imports

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
  fetch('/_test', FetchOptions(method: 'POST', body: '$result'));
  close();
}
