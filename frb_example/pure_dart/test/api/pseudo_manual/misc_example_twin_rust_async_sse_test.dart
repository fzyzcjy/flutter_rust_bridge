// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_example_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_example_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  void testComplexStruct(MyTreeNodeTwinRustAsyncSse complexStructResp,
      {required int arrLen}) {
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8,
        List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  }

  test('dart call handleComplexStruct', () async {
    final arrLen = 5;
    final complexStructResp = await handleComplexStructTwinRustAsyncSse(
        s: _createMyTreeNode(arrLen: arrLen));
    testComplexStruct(complexStructResp, arrLen: arrLen);
  });

  test('loop and call many times', () async {
    var obj = _createMyTreeNode(arrLen: 5);
    for (var i = 0; i < 500; ++i) {
      obj = await handleComplexStructTwinRustAsyncSse(s: obj);
    }
  });

  test("dart call list_of_primitive_enums", () async {
    List<WeekdaysTwinRustAsyncSse> days =
        await listOfPrimitiveEnumsTwinRustAsyncSse(
            weekdays: WeekdaysTwinRustAsyncSse.values);
    expect(days, WeekdaysTwinRustAsyncSse.values);
  });

  test('dart call handleNestedStruct', () async {
    final r =
        await handleNestedStructTwinRustAsyncSse(s: _createMyNestedStruct());
    testComplexStruct(r.treeNode, arrLen: 5);
    expect(r.weekday, WeekdaysTwinRustAsyncSse.friday);
  });

  test('Lossless big buffers', () async {
    final list = await handleBigBuffersTwinRustAsyncSse();
    expect(list.int64[0], BigInt.parse('-9223372036854775808'));
    expect(list.int64[1], BigInt.parse('9223372036854775807'));
    expect(list.uint64[0], BigInt.parse('0xFFFFFFFFFFFFFFFF'),
        reason: 'uint64');
  });

  test('test abc', () async {
    final output1 = await testAbcEnumTwinRustAsyncSse(
        abc: AbcTwinRustAsyncSse.a(ATwinRustAsyncSse(a: "test")));
    expect((output1 as AbcTwinRustAsyncSse_A).field0.a, "test");

    final output2 = await testAbcEnumTwinRustAsyncSse(
        abc: AbcTwinRustAsyncSse.b(BTwinRustAsyncSse(b: 1)));
    expect((output2 as AbcTwinRustAsyncSse_B).field0.b, 1);

    final output3 = await testAbcEnumTwinRustAsyncSse(
        abc: AbcTwinRustAsyncSse.c(CTwinRustAsyncSse(c: false)));
    expect((output3 as AbcTwinRustAsyncSse_C).field0.c, false);

    final output4 =
        await testAbcEnumTwinRustAsyncSse(abc: AbcTwinRustAsyncSse.justInt(1));
    expect((output4 as AbcTwinRustAsyncSse_JustInt).field0, 1);
  });

  test("dart call struct_with_enum_member", () async {
    final result = await testStructWithEnumTwinRustAsyncSse(
        se: StructWithEnumTwinRustAsyncSse(
            abc1: AbcTwinRustAsyncSse.a(ATwinRustAsyncSse(a: "aaa")),
            abc2: AbcTwinRustAsyncSse.b(BTwinRustAsyncSse(b: 999))));
    expect(result.abc1.whenOrNull(b: (BTwinRustAsyncSse b) => b.b), 999);
    expect(result.abc2.whenOrNull(a: (ATwinRustAsyncSse a) => a.a), "aaa");
  });

  test('dart call handleString', () async {
    expect(await handleStringTwinRustAsyncSse(s: "Hello, world!"),
        "Hello, world!Hello, world!");
  });

  test('dart call handleString with nul-containing string', () async {
    expect(
      await handleStringTwinRustAsyncSse(s: "Hello\u0000world!"),
      anyOf(
        // When web or SSE codec
        "Hello\u0000world!Hello\u0000world!",
        // When CST codec
        "",
      ),
    );
  });

  addTestsIdentityFunctionCall(
      handleCharTwinRustAsyncSse, <String>['a', '\0', '\u{10FFFF}']);

  test('dart call handleVecU8', () async {
    final len = 100000;
    expect(
        await handleVecU8TwinRustAsyncSse(
            v: Uint8List.fromList(List.filled(len, 127))),
        Uint8List.fromList(List.filled(len * 2, 127)));
  });

  test('dart call handleStruct', () async {
    final structResp = await handleStructTwinRustAsyncSse(
        arg: MySize(width: 42, height: 100),
        boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });

  test('positionalArgumentsTwinRustAsyncSse', () async {
    expect(await positionalArgumentsTwinRustAsyncSse(100, 200), 300);
  });
}

MyTreeNodeTwinRustAsyncSse _createMyTreeNode({required int arrLen}) {
  return MyTreeNodeTwinRustAsyncSse(
    valueI32: 100,
    valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
    valueBoolean: true,
    children: [
      MyTreeNodeTwinRustAsyncSse(
        valueI32: 110,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
        valueBoolean: true,
        children: [
          MyTreeNodeTwinRustAsyncSse(
            valueI32: 111,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
            valueBoolean: true,
            children: [],
          ),
        ],
      ),
      MyTreeNodeTwinRustAsyncSse(
        valueI32: 120,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
        valueBoolean: true,
        children: [],
      ),
    ],
  );
}

MyNestedStructTwinRustAsyncSse _createMyNestedStruct() {
  return MyNestedStructTwinRustAsyncSse(
      treeNode: _createMyTreeNode(arrLen: 5),
      weekday: WeekdaysTwinRustAsyncSse.friday);
}
