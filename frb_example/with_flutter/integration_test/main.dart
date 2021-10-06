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
    testWidgets('repeat call to memoryTestUtilityInputComplexStruct', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester,
          () async => expect(
              await app.api.memoryTestUtilityInputComplexStruct(
                  input: TreeNode(
                      name: 'root', children: [for (var i = 0; i < 10000; ++i) TreeNode(name: 'child', children: [])])),
              10000));
    });
    testWidgets('repeat call to memoryTestUtilityOutputComplexStruct', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(tester,
          () async => expect((await app.api.memoryTestUtilityOutputComplexStruct(len: 10000)).children.length, 10000));
    });

    testWidgets('repeat call to memoryTestUtilityInputVecSize', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester,
          () async => expect(
              await app.api.memoryTestUtilityInputVecOfObject(input: List.filled(100000, Size(width: 42, height: 100))),
              100000));
    });
    testWidgets('repeat call to memoryTestUtilityOutputVecSize', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect((await app.api.memoryTestUtilityOutputVecOfObject(len: 100000)).length, 100000));
    });

    testWidgets('repeat call to memoryTestUtilityInputArray', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect(await app.api.memoryTestUtilityInputArray(input: Uint8List(1000000)), 1000000));
    });
    testWidgets('repeat call to memoryTestUtilityOutputZeroCopyBuffer', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(tester,
          () async => expect((await app.api.memoryTestUtilityOutputZeroCopyBuffer(len: 1000000)).length, 1000000));
    });
    testWidgets('repeat call to memoryTestUtilityOutputVecU8', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect((await app.api.memoryTestUtilityOutputVecU8(len: 1000000)).length, 1000000));
    });

    testWidgets('run and wait to see if there is memory problem', (WidgetTester tester) async {
      app.main();
      await tester.pumpAndSettle();

      // run many times to see memory leaks or other problems
      for (var i = 0; i < 100; ++i) {
        await tester.pumpAndSettle();
        expect(find.textContaining('Hi this string is from Rust'), findsOneWidget);

        for (var j = 0; j < 20; ++j) {
          await _callFfiWithComplexStructToDetectMemoryProblems();
        }

        await _maybeGC();
      }
    });
  });
}

Future<void> _testMemoryProblemForSingleTypeOfMethod(WidgetTester tester, Future<void> Function() callFfi) async {
  app.main();
  await tester.pumpAndSettle();

  final startTime = DateTime.now();
  const kMaxRunTime = Duration(seconds: 60);

  while (true) {
    print('Iteration starts (current time: ${DateTime.now()})');
    for (var j = 0; j < 100; ++j) {
      await callFfi();
    }
    await _maybeGC();

    if (DateTime.now().difference(startTime) > kMaxRunTime) {
      break;
    }
  }
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

  // notice this variable is also large and can consume megabytes of memory...
  final profileAfterMaybeGc = await vmService.getAllocationProfile(isolateId, reset: true, gc: true);
  print('Memory usage after maybe GC: ${profileAfterMaybeGc.memoryUsage} '
      'dateLastServiceGC=${profileAfterMaybeGc.dateLastServiceGC} now=${DateTime.now().millisecondsSinceEpoch}');
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
