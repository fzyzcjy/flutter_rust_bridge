// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_example_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:typed_data';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/misc_example_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  void testComplexStruct(MyTreeNodeTwinRustAsync complexStructResp,
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
    final complexStructResp = await handleComplexStructTwinRustAsync(
        s: _createMyTreeNode(arrLen: arrLen));
    testComplexStruct(complexStructResp, arrLen: arrLen);
  });

  test('loop and call many times', () async {
    var obj = _createMyTreeNode(arrLen: 5);
    for (var i = 0; i < 500; ++i) {
      obj = await handleComplexStructTwinRustAsync(s: obj);
    }
  });

  test("dart call list_of_primitive_enums", () async {
    List<WeekdaysTwinRustAsync> days = await listOfPrimitiveEnumsTwinRustAsync(
        weekdays: WeekdaysTwinRustAsync.values);
    expect(days, WeekdaysTwinRustAsync.values);
  });

  test('dart call handleNestedStruct', () async {
    final r = await handleNestedStructTwinRustAsync(s: _createMyNestedStruct());
    testComplexStruct(r.treeNode, arrLen: 5);
    expect(r.weekday, WeekdaysTwinRustAsync.friday);
  });

  test('Lossless big buffers', () async {
    final list = await handleBigBuffersTwinRustAsync();
    expect(list.int64[0], BigInt.parse('-9223372036854775808'));
    expect(list.int64[1], BigInt.parse('9223372036854775807'));
    expect(list.uint64[0], BigInt.parse('0xFFFFFFFFFFFFFFFF'),
        reason: 'uint64');
  });

  test('test abc', () async {
    final output1 = await testAbcEnumTwinRustAsync(
        abc: AbcTwinRustAsync.a(ATwinRustAsync(a: "test")));
    expect((output1 as AbcTwinRustAsync_A).field0.a, "test");

    final output2 = await testAbcEnumTwinRustAsync(
        abc: AbcTwinRustAsync.b(BTwinRustAsync(b: 1)));
    expect((output2 as AbcTwinRustAsync_B).field0.b, 1);

    final output3 = await testAbcEnumTwinRustAsync(
        abc: AbcTwinRustAsync.c(CTwinRustAsync(c: false)));
    expect((output3 as AbcTwinRustAsync_C).field0.c, false);

    final output4 =
        await testAbcEnumTwinRustAsync(abc: AbcTwinRustAsync.justInt(1));
    expect((output4 as AbcTwinRustAsync_JustInt).field0, 1);
  });

  test("dart call struct_with_enum_member", () async {
    final result = await testStructWithEnumTwinRustAsync(
        se: StructWithEnumTwinRustAsync(
            abc1: AbcTwinRustAsync.a(ATwinRustAsync(a: "aaa")),
            abc2: AbcTwinRustAsync.b(BTwinRustAsync(b: 999))));
    expect(result.abc1.whenOrNull(b: (BTwinRustAsync b) => b.b), 999);
    expect(result.abc2.whenOrNull(a: (ATwinRustAsync a) => a.a), "aaa");
  });

  test('dart call handleString', () async {
    expect(await handleStringTwinRustAsync(s: "Hello, world!"),
        "Hello, world!Hello, world!");
  });

  test('dart call handleString with nul-containing string', () async {
    expect(
      await handleStringTwinRustAsync(s: "Hello\u0000world!"),
      anyOf(
        // When web or SSE codec
        "Hello\u0000world!Hello\u0000world!",
        // When CST codec
        "",
      ),
    );
  });

  addTestsIdentityFunctionCall(
      handleCharTwinRustAsync, <String>['a', '\0', '\u{10FFFF}']);

  test('dart call handleVecU8', () async {
    final len = 100000;
    expect(
        await handleVecU8TwinRustAsync(
            v: Uint8List.fromList(List.filled(len, 127))),
        Uint8List.fromList(List.filled(len * 2, 127)));
  });

  test('dart call handleStruct', () async {
    final structResp = await handleStructTwinRustAsync(
        arg: MySize(width: 42, height: 100),
        boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });

  test('positionalArgumentsTwinRustAsync', () async {
    expect(await positionalArgumentsTwinRustAsync(100, 200), 300);
  });
}

MyTreeNodeTwinRustAsync _createMyTreeNode({required int arrLen}) {
  return MyTreeNodeTwinRustAsync(
    valueI32: 100,
    valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
    valueBoolean: true,
    children: [
      MyTreeNodeTwinRustAsync(
        valueI32: 110,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
        valueBoolean: true,
        children: [
          MyTreeNodeTwinRustAsync(
            valueI32: 111,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
            valueBoolean: true,
            children: [],
          ),
        ],
      ),
      MyTreeNodeTwinRustAsync(
        valueI32: 120,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
        valueBoolean: true,
        children: [],
      ),
    ],
  );
}

MyNestedStructTwinRustAsync _createMyNestedStruct() {
  return MyNestedStructTwinRustAsync(
      treeNode: _createMyTreeNode(arrLen: 5),
      weekday: WeekdaysTwinRustAsync.friday);
}
