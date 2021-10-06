// ignore_for_file: avoid_print

import 'dart:developer';
import 'dart:io';
import 'dart:isolate';

import 'package:flutter_rust_bridge_example/main.dart' as app;
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:vm_service/vm_service_io.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('end-to-end test', () {
    testWidgets('run and wait to see if there is memory problem', (WidgetTester tester) async {
      app.main();
      await tester.pumpAndSettle();

      // run many times to see memory leaks or other problems
      for (var i = 0; i < 10; ++i) {
        // wait a bit - should not crash
        await Future.delayed(const Duration(seconds: 3));
        await tester.pumpAndSettle();

        expect(find.textContaining('Hi this string is from Rust'), findsOneWidget);

        _maybeGC();
      }
    });
  });
}

// https://stackoverflow.com/questions/63730179/can-we-force-the-dart-garbage-collector
Future<void> _maybeGC() async {
  final serverUri = (await Service.getInfo()).serverUri;

  if (serverUri == null) {
    print('Please run the application with the --observe parameter!');
    exit(1);
  }

  final isolateId = Service.getIsolateID(Isolate.current)!;
  final vmService = await vmServiceConnectUri(_toWebSocket(serverUri));
  final profile = await vmService.getAllocationProfile(isolateId, gc: true);

  print('Memory usage: ${profile.memoryUsage}');
}

List<String> _cleanupPathSegments(Uri uri) {
  final pathSegments = <String>[];
  if (uri.pathSegments.isNotEmpty) {
    pathSegments.addAll(uri.pathSegments.where(
      (s) => s.isNotEmpty,
    ));
  }
  return pathSegments;
}

String _toWebSocket(Uri uri) {
  final pathSegments = _cleanupPathSegments(uri);
  pathSegments.add('ws');
  return uri.replace(scheme: 'ws', pathSegments: pathSegments).toString();
}
