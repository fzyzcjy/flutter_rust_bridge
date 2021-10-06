// ignore_for_file: avoid_print

import 'dart:developer';
import 'dart:io';
import 'dart:isolate';
import 'dart:math';
import 'dart:typed_data';

import 'package:flutter_rust_bridge_example/generated_api.dart';
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
      for (var i = 0; i < 100; ++i) {
        await tester.pumpAndSettle();
        expect(find.textContaining('Hi this string is from Rust'), findsOneWidget);

        for (var j = 0; j < 5; ++j) {
          await _callFfiWithBigArrayToDetectMemoryProblems();
        }
        for (var j = 0; j < 20; ++j) {
          await _callFfiWithComplexStructToDetectMemoryProblems();
        }

        await _maybeGC();
      }
    });
  });
}

Future<void> _callFfiWithBigArrayToDetectMemoryProblems() async {
  print('Call FFI with big array: start');
  final input = Uint8List(1000000);
  input[0] = 42;
  final output = await app.api.workOnBigArray(input: input);
  expect(output[0], 255 - input[0]);
  expect(output.length, input.length);
  print('Call FFI with big array: end (output.length=${output.length})');
}

Future<void> _callFfiWithComplexStructToDetectMemoryProblems() async {
  print('Call FFI with complex struct: start');
  final input = _createBigTree(4, 10);
  final result = await app.api.passingComplexStructs(root: input);
  print('Call FFI with complex struct: end (result.length=${result.length})');
}

TreeNode _createBigTree(int maxDepth, int fanOut) {
  print('create big tree with maxDepth=$maxDepth fanOut=$fanOut => totally ${pow(fanOut, maxDepth)} nodes');
  return _createBigTreeInner(0, maxDepth, fanOut);
}

TreeNode _createBigTreeInner(int currDepth, int maxDepth, int fanOut) {
  return TreeNode(
      name: 'TreeNodeOfDepth$currDepth',
      children: currDepth == maxDepth
          ? []
          : [for (var i = 0; i < fanOut; ++i) _createBigTreeInner(currDepth + 1, maxDepth, fanOut)]);
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

  final profileWithoutExplicitGc = await vmService.getAllocationProfile(isolateId, gc: true);
  print('Memory usage without explicit GC (but may have implicit): ${profileWithoutExplicitGc.memoryUsage}');

  final profileAfterMaybeGc = await vmService.getAllocationProfile(isolateId, gc: true);
  print('Memory usage after maybe GC: ${profileAfterMaybeGc.memoryUsage}');
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
