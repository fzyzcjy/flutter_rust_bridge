// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/method_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinRustAsync concatenateWith =
        ConcatenateWithTwinRustAsync(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinRustAsync(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinRustAsync.concatenateStaticTwinRustAsync(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinRustAsync.newTwinRustAsync(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinRustAsync(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinRustAsync sumWith = SumWithTwinRustAsync(x: 3);
    final int sum = await sumWith.sumTwinRustAsync(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinRustAsync sumWith = await getSumStructTwinRustAsync();
    final int sum = await sumWith.sumTwinRustAsync(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinRustAsync> sumWithList =
        await getSumArrayTwinRustAsync(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sumTwinRustAsync(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sumTwinRustAsync(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sumTwinRustAsync(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinRustAsync concatenateWith =
        ConcatenateWithTwinRustAsync(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream =
        concatenateWith.handleSomeStreamSinkTwinRustAsync(key: key, max: max);
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
        ConcatenateWithTwinRustAsync.handleSomeStaticStreamSinkTwinRustAsync(
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
    final stream = ConcatenateWithTwinRustAsync
        .handleSomeStaticStreamSinkSingleArgTwinRustAsync();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinRustAsync(a: "apple");
    expect(await concatenateWith.simpleGetterTwinRustAsync, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinRustAsync(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect(
          (await SimpleStructTwinRustAsync.returnSelfTwinRustAsync(one: 'One'))
              .one,
          'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinRustAsync(one: 'a');
      expect(await obj.receiverBorrowTwinRustAsync(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinRustAsync(one: 'a');
      expect(await obj.receiverOwnTwinRustAsync(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinRustAsync(one: 'a');
      final b = SimpleStructTwinRustAsync(one: 'b');
      expect(await SimpleStructTwinRustAsync.argSelfTwinRustAsync(a: a, b: b),
          'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinRustAsync(one: 'a');
      final b = SimpleStructTwinRustAsync(one: 'b');
      expect(await SimpleStructTwinRustAsync.vecSelfTwinRustAsync(arg: [a, b]),
          ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj = await SimpleEnumTwinRustAsync.returnSelfTwinRustAsync(one: 'A');
    expect(await obj.simpleMethodTwinRustAsync(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(
        await SimplePrimitiveEnumTwinRustAsync.second
            .simpleMethodTwinRustAsync(),
        200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinRustAsync.staticMethodTwinRustAsync(a: 42), 42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinRustAsync.staticGetterTwinRustAsync, 42);
  });
}
