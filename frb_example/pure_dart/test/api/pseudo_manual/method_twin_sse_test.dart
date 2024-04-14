// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/method_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinSse concatenateWith =
        ConcatenateWithTwinSse(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinSse(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinSse.concatenateStaticTwinSse(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinSse.newTwinSse(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinSse(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinSse sumWith = SumWithTwinSse(x: 3);
    final int sum = await sumWith.sumTwinSse(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinSse sumWith = await getSumStructTwinSse();
    final int sum = await sumWith.sumTwinSse(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinSse> sumWithList =
        await getSumArrayTwinSse(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sumTwinSse(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sumTwinSse(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sumTwinSse(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinSse concatenateWith =
        ConcatenateWithTwinSse(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream =
        concatenateWith.handleSomeStreamSinkTwinSse(key: key, max: max);
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
    final stream = ConcatenateWithTwinSse.handleSomeStaticStreamSinkTwinSse(
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
    final stream =
        ConcatenateWithTwinSse.handleSomeStaticStreamSinkSingleArgTwinSse();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinSse(a: "apple");
    expect(await concatenateWith.simpleGetterTwinSse, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinSse(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect(
          (await SimpleStructTwinSse.returnSelfTwinSse(one: 'One')).one, 'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinSse(one: 'a');
      expect(await obj.receiverBorrowTwinSse(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinSse(one: 'a');
      expect(await obj.receiverOwnTwinSse(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinSse(one: 'a');
      final b = SimpleStructTwinSse(one: 'b');
      expect(await SimpleStructTwinSse.argSelfTwinSse(a: a, b: b), 'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinSse(one: 'a');
      final b = SimpleStructTwinSse(one: 'b');
      expect(await SimpleStructTwinSse.vecSelfTwinSse(arg: [a, b]), ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj = await SimpleEnumTwinSse.returnSelfTwinSse(one: 'A');
    expect(await obj.simpleMethodTwinSse(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(await SimplePrimitiveEnumTwinSse.second.simpleMethodTwinSse(), 200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinSse.staticMethodTwinSse(a: 42), 42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinSse.staticGetterTwinSse, 42);
  });
}
