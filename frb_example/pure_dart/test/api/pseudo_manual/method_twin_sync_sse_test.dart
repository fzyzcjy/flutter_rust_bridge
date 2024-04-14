// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/method_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinSyncSse concatenateWith =
        ConcatenateWithTwinSyncSse(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinSyncSse(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinSyncSse.concatenateStaticTwinSyncSse(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinSyncSse.newTwinSyncSse(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinSyncSse(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinSyncSse sumWith = SumWithTwinSyncSse(x: 3);
    final int sum = await sumWith.sumTwinSyncSse(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinSyncSse sumWith = await getSumStructTwinSyncSse();
    final int sum = await sumWith.sumTwinSyncSse(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinSyncSse> sumWithList =
        await getSumArrayTwinSyncSse(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sumTwinSyncSse(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sumTwinSyncSse(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sumTwinSyncSse(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinSyncSse concatenateWith =
        ConcatenateWithTwinSyncSse(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream =
        concatenateWith.handleSomeStreamSinkTwinSyncSse(key: key, max: max);
    int cnt = 0;
    await for (final value in stream) {
      print("output from ConcatenateWith's stream: $value");
      expect(value.value, "hello $cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink test', () async {
    final int key = 10;
    final int max = 5;
    final stream =
        ConcatenateWithTwinSyncSse.handleSomeStaticStreamSinkTwinSyncSse(
            key: key, max: max);
    int cnt = 0;
    await for (final value in stream) {
      print("output from ConcatenateWith's static stream: $value");
      expect(value.value, "$cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink at 1 test', () async {
    final stream = ConcatenateWithTwinSyncSse
        .handleSomeStaticStreamSinkSingleArgTwinSyncSse();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinSyncSse(a: "apple");
    expect(await concatenateWith.simpleGetterTwinSyncSse, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinSyncSse(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect(
          (await SimpleStructTwinSyncSse.returnSelfTwinSyncSse(one: 'One')).one,
          'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinSyncSse(one: 'a');
      expect(await obj.receiverBorrowTwinSyncSse(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinSyncSse(one: 'a');
      expect(await obj.receiverOwnTwinSyncSse(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinSyncSse(one: 'a');
      final b = SimpleStructTwinSyncSse(one: 'b');
      expect(
          await SimpleStructTwinSyncSse.argSelfTwinSyncSse(a: a, b: b), 'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinSyncSse(one: 'a');
      final b = SimpleStructTwinSyncSse(one: 'b');
      expect(await SimpleStructTwinSyncSse.vecSelfTwinSyncSse(arg: [a, b]),
          ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj = await SimpleEnumTwinSyncSse.returnSelfTwinSyncSse(one: 'A');
    expect(await obj.simpleMethodTwinSyncSse(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(
        await SimplePrimitiveEnumTwinSyncSse.second.simpleMethodTwinSyncSse(),
        200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinSyncSse.staticMethodTwinSyncSse(a: 42), 42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinSyncSse.staticGetterTwinSyncSse, 42);
  });
}
