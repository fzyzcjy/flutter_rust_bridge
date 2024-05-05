// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/method_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinRustAsyncSse concatenateWith =
        ConcatenateWithTwinRustAsyncSse(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinRustAsyncSse(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinRustAsyncSse.concatenateStaticTwinRustAsyncSse(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinRustAsyncSse.newTwinRustAsyncSse(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinRustAsyncSse(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinRustAsyncSse sumWith = SumWithTwinRustAsyncSse(x: 3);
    final int sum = await sumWith.sumTwinRustAsyncSse(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinRustAsyncSse sumWith =
        await getSumStructTwinRustAsyncSse();
    final int sum = await sumWith.sumTwinRustAsyncSse(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinRustAsyncSse> sumWithList =
        await getSumArrayTwinRustAsyncSse(a: 12, b: 23, c: 67);
    expect(
        await sumWithList[0].sumTwinRustAsyncSse(y: 23, z: 67), 12 + 23 + 67);
    expect(
        await sumWithList[1].sumTwinRustAsyncSse(y: 12, z: 67), 12 + 23 + 67);
    expect(
        await sumWithList[2].sumTwinRustAsyncSse(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinRustAsyncSse concatenateWith =
        ConcatenateWithTwinRustAsyncSse(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream = concatenateWith.handleSomeStreamSinkTwinRustAsyncSse(
        key: key, max: max);
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
    final stream = ConcatenateWithTwinRustAsyncSse
        .handleSomeStaticStreamSinkTwinRustAsyncSse(key: key, max: max);
    int cnt = 0;
    await for (final value in stream) {
      print("output from ConcatenateWith's static stream: $value");
      expect(value.value, "$cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink at 1 test', () async {
    final stream = ConcatenateWithTwinRustAsyncSse
        .handleSomeStaticStreamSinkSingleArgTwinRustAsyncSse();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinRustAsyncSse(a: "apple");
    expect(await concatenateWith.simpleGetterTwinRustAsyncSse, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinRustAsyncSse(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect(
          (await SimpleStructTwinRustAsyncSse.returnSelfTwinRustAsyncSse(
                  one: 'One'))
              .one,
          'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinRustAsyncSse(one: 'a');
      expect(await obj.receiverBorrowTwinRustAsyncSse(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinRustAsyncSse(one: 'a');
      expect(await obj.receiverOwnTwinRustAsyncSse(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinRustAsyncSse(one: 'a');
      final b = SimpleStructTwinRustAsyncSse(one: 'b');
      expect(
          await SimpleStructTwinRustAsyncSse.argSelfTwinRustAsyncSse(
              a: a, b: b),
          'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinRustAsyncSse(one: 'a');
      final b = SimpleStructTwinRustAsyncSse(one: 'b');
      expect(
          await SimpleStructTwinRustAsyncSse.vecSelfTwinRustAsyncSse(
              arg: [a, b]),
          ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj =
        await SimpleEnumTwinRustAsyncSse.returnSelfTwinRustAsyncSse(one: 'A');
    expect(await obj.simpleMethodTwinRustAsyncSse(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(
        await SimplePrimitiveEnumTwinRustAsyncSse.second
            .simpleMethodTwinRustAsyncSse(),
        200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinRustAsyncSse.staticMethodTwinRustAsyncSse(a: 42),
        42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinRustAsyncSse.staticGetterTwinRustAsyncSse,
        42);
  });
}
