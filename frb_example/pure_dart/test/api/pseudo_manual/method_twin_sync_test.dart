// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/method_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinSync concatenateWith =
        ConcatenateWithTwinSync(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinSync(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinSync.concatenateStaticTwinSync(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinSync.newTwinSync(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinSync(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinSync sumWith = SumWithTwinSync(x: 3);
    final int sum = await sumWith.sumTwinSync(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinSync sumWith = await getSumStructTwinSync();
    final int sum = await sumWith.sumTwinSync(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinSync> sumWithList =
        await getSumArrayTwinSync(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sumTwinSync(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sumTwinSync(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sumTwinSync(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinSync concatenateWith =
        ConcatenateWithTwinSync(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream =
        concatenateWith.handleSomeStreamSinkTwinSync(key: key, max: max);
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
    final stream = ConcatenateWithTwinSync.handleSomeStaticStreamSinkTwinSync(
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
        ConcatenateWithTwinSync.handleSomeStaticStreamSinkSingleArgTwinSync();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinSync(a: "apple");
    expect(await concatenateWith.simpleGetterTwinSync, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinSync(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect((await SimpleStructTwinSync.returnSelfTwinSync(one: 'One')).one,
          'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinSync(one: 'a');
      expect(await obj.receiverBorrowTwinSync(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinSync(one: 'a');
      expect(await obj.receiverOwnTwinSync(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinSync(one: 'a');
      final b = SimpleStructTwinSync(one: 'b');
      expect(await SimpleStructTwinSync.argSelfTwinSync(a: a, b: b), 'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinSync(one: 'a');
      final b = SimpleStructTwinSync(one: 'b');
      expect(
          await SimpleStructTwinSync.vecSelfTwinSync(arg: [a, b]), ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj = await SimpleEnumTwinSync.returnSelfTwinSync(one: 'A');
    expect(await obj.simpleMethodTwinSync(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(
        await SimplePrimitiveEnumTwinSync.second.simpleMethodTwinSync(), 200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinSync.staticMethodTwinSync(a: 42), 42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinSync.staticGetterTwinSync, 42);
  });
}
