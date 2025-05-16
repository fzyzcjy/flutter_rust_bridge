// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/method.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ConcatenateWith test', () async {
    final ConcatenateWithTwinNormal concatenateWith =
        ConcatenateWithTwinNormal(a: "hello ");
    final String concatenated =
        await concatenateWith.concatenateTwinNormal(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated =
        await ConcatenateWithTwinNormal.concatenateStaticTwinNormal(
            a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor =
        await ConcatenateWithTwinNormal.newTwinNormal(a: "hello ");
    final String concatenated2 =
        await concatenatedConstructor.concatenateTwinNormal(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWithTwinNormal sumWith = SumWithTwinNormal(x: 3);
    final int sum = await sumWith.sumTwinNormal(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWithTwinNormal sumWith = await getSumStructTwinNormal();
    final int sum = await sumWith.sumTwinNormal(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWithTwinNormal> sumWithList =
        await getSumArrayTwinNormal(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sumTwinNormal(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sumTwinNormal(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sumTwinNormal(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWithTwinNormal concatenateWith =
        ConcatenateWithTwinNormal(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream =
        concatenateWith.handleSomeStreamSinkTwinNormal(key: key, max: max);
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
        ConcatenateWithTwinNormal.handleSomeStaticStreamSinkTwinNormal(
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
    final stream = ConcatenateWithTwinNormal
        .handleSomeStaticStreamSinkSingleArgTwinNormal();
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('getter', () async {
    final concatenateWith = ConcatenateWithTwinNormal(a: "apple");
    expect(await concatenateWith.simpleGetterTwinNormal, equals("apple"));
  });

  test('callable', () async {
    final callable = MyCallableTwinNormal(one: 'One');
    expect(await callable(two: 'Two'), 'OneTwo');
  });

  group('SimpleStruct', () {
    test('returnSelf', () async {
      expect(
          (await SimpleStructTwinNormal.returnSelfTwinNormal(one: 'One')).one,
          'One');
    });

    test('receiverBorrow', () async {
      final obj = SimpleStructTwinNormal(one: 'a');
      expect(await obj.receiverBorrowTwinNormal(), 'a');
    });

    test('receiverOwn', () async {
      final obj = SimpleStructTwinNormal(one: 'a');
      expect(await obj.receiverOwnTwinNormal(), 'a');
    });

    test('argSelf', () async {
      final a = SimpleStructTwinNormal(one: 'a');
      final b = SimpleStructTwinNormal(one: 'b');
      expect(await SimpleStructTwinNormal.argSelfTwinNormal(a: a, b: b), 'ab');
    });

    test('vecSelf', () async {
      final a = SimpleStructTwinNormal(one: 'a');
      final b = SimpleStructTwinNormal(one: 'b');
      expect(await SimpleStructTwinNormal.vecSelfTwinNormal(arg: [a, b]),
          ['a', 'b']);
    });
  });

  test('SimpleEnum', () async {
    final obj = await SimpleEnumTwinNormal.returnSelfTwinNormal(one: 'A');
    expect(await obj.simpleMethodTwinNormal(), 'A');
  });

  test('SimplePrimitiveEnum', () async {
    expect(await SimplePrimitiveEnumTwinNormal.second.simpleMethodTwinNormal(),
        200);
  });

  test('StaticOnly', () async {
    expect(await StaticOnlyTwinNormal.staticMethodTwinNormal(a: 42), 42);
  });

  test('StaticGetterOnly', () async {
    expect(await StaticGetterOnlyTwinNormal.staticGetterTwinNormal, 42);
  });
}
