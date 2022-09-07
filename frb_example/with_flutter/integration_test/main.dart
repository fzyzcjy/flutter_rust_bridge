// ignore_for_file: avoid_print

import 'dart:developer';
import 'dart:io';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:flutter/material.dart' hide Size;
import 'package:flutter_rust_bridge_example/bridge_definitions.dart';
import 'package:flutter_rust_bridge_example/main.dart' as app;
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:vm_service/vm_service_io.dart';

void main() {
  print('integration_test/main.dart starts!');

  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('end-to-end test', () {
    testWidgets('run and wait to see if there is memory problem', (WidgetTester tester) async {
      app.main();
      await tester.pumpAndSettle();

      // run many times to see memory leaks or other problems
      for (var i = 0; i < 20; ++i) {
        await tester.pumpAndSettle();
        expect(find.textContaining('Hi this string is from Rust'), findsOneWidget);

        Future.delayed(const Duration(milliseconds: 50));

        // see https://github.com/fzyzcjy/flutter_rust_bridge/issues/19 for details
        await _maybeGC(
            '[NOTE: even if `externalUsage` increases, this is NOT a bug! It is because Flutter\'s ImageCache, which will cache about 100MB. See #19]');
      }
    });

    testWidgets('test Rust deliberately have error', (WidgetTester tester) async {
      app.main();
      await tester.pumpAndSettle();

      for (var i = 0; i < 3; ++i) {
        print('call offTopicDeliberatelyReturnError');
        try {
          final ret = await app.api.offTopicDeliberatelyReturnError();
          fail('exception not thrown ret=$ret');
        } catch (e, s) {
          debugPrint('catch expected error e=$e s=$s');
        }

        await Future.delayed(const Duration(milliseconds: 200));
        await tester.pumpAndSettle();

        print('call offTopicDeliberatelyPanic');
        try {
          final ret = await app.api.offTopicDeliberatelyPanic();
          fail('exception not thrown ret=$ret');
        } catch (e, s) {
          debugPrint('catch expected error e=$e s=$s');
        }

        await Future.delayed(const Duration(milliseconds: 200));
        await tester.pumpAndSettle();
      }
    });

    testWidgets('repeat call to offTopicMemoryTestInputComplexStruct', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester,
          () async => expect(
              await app.api.offTopicMemoryTestInputComplexStruct(
                  input: TreeNode(
                      name: 'root', children: [for (var i = 0; i < 2000; ++i) TreeNode(name: 'child', children: [])])),
              2000));
    });
    testWidgets('repeat call to offTopicMemoryTestOutputComplexStruct', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(tester,
          () async => expect((await app.api.offTopicMemoryTestOutputComplexStruct(len: 2000)).children.length, 2000));
    });

    testWidgets('repeat call to offTopicMemoryTestInputVecOfObject', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester,
          () async => expect(
              await app.api
                  .offTopicMemoryTestInputVecOfObject(input: List.filled(100000, Size(width: 42, height: 100))),
              100000));
    });
    testWidgets('repeat call to offTopicMemoryTestOutputVecOfObject', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect((await app.api.offTopicMemoryTestOutputVecOfObject(len: 100000)).length, 100000));
    });

    testWidgets('repeat call to offTopicMemoryTestInputArray', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect(await app.api.offTopicMemoryTestInputArray(input: Uint8List(1000000)), 1000000));
    });
    testWidgets('repeat call to offTopicMemoryTestOutputZeroCopyBuffer', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(tester,
          () async => expect((await app.api.offTopicMemoryTestOutputZeroCopyBuffer(len: 1000000)).length, 1000000));
    });
    testWidgets('repeat call to offTopicMemoryTestOutputVecU8', (WidgetTester tester) async {
      await _testMemoryProblemForSingleTypeOfMethod(
          tester, () async => expect((await app.api.offTopicMemoryTestOutputVecU8(len: 200000)).length, 200000));
    });
  });
}

Future<void> _testMemoryProblemForSingleTypeOfMethod(WidgetTester tester, Future<void> Function() callFfi) async {
  print('testMemoryProblemForSingleTypeOfMethod start');

  app.main();
  await tester.pumpAndSettle();

  final startTime = DateTime.now();
  const kMaxRunTime = Duration(seconds: 5);

  while (true) {
    print('Iteration starts (current time: ${DateTime.now()})');
    for (var j = 0; j < 20; ++j) {
      await callFfi();
    }
    await _maybeGC();

    if (DateTime.now().difference(startTime) > kMaxRunTime) {
      break;
    }
  }

  print('testMemoryProblemForSingleTypeOfMethod end');
}

// Future<void> _callFfiWithComplexStructToDetectMemoryProblems() async {
//   // print('Call FFI with complex struct: start');
//   final input = _createBigTree(4, 10);
//   final result = await app.api.passingComplexStructs(root: input);
//   print('Call FFI with complex struct: end (result.length=${result.length})');
// }
//
// TreeNode _createBigTree(int maxDepth, int fanOut) {
//   print('create big tree with maxDepth=$maxDepth fanOut=$fanOut => totally ${pow(fanOut, maxDepth)} nodes');
//   return _createBigTreeInner(0, maxDepth, fanOut);
// }
//
// TreeNode _createBigTreeInner(int currDepth, int maxDepth, int fanOut) {
//   return TreeNode(
//       name: 'TreeNodeOfDepth$currDepth',
//       children: currDepth == maxDepth
//           ? []
//           : [for (var i = 0; i < fanOut; ++i) _createBigTreeInner(currDepth + 1, maxDepth, fanOut)]);
// }

// https://stackoverflow.com/questions/63730179/can-we-force-the-dart-garbage-collector
Future<void> _maybeGC([String hint = '']) async {
  final serverUri = (await Service.getInfo()).serverUri;

  if (serverUri == null) {
    print('Please run the application with the --observe parameter!');
    exit(1);
  }

  final isolateId = Service.getIsolateID(Isolate.current)!;
  final vmService = await vmServiceConnectUri(_toWebSocket(serverUri));

  // notice this variable is also large and can consume megabytes of memory...
  final profileAfterMaybeGc = await vmService.getAllocationProfile(isolateId, reset: true, gc: true);
  print('Memory usage after maybe GC $hint: ${profileAfterMaybeGc.memoryUsage} '
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
