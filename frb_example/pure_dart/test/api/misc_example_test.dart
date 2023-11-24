import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/misc_example.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  void testComplexStruct(MyTreeNode complexStructResp, {required int arrLen}) {
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  }

  test('dart call handleComplexStruct', () async {
    final arrLen = 5;
    final complexStructResp = await handleComplexStruct(s: _createMyTreeNode(arrLen: arrLen));
    testComplexStruct(complexStructResp, arrLen: arrLen);
  });

  test("dart call list_of_primitive_enums", () async {
    List<Weekdays> days = await listOfPrimitiveEnums(weekdays: Weekdays.values);
    expect(days, Weekdays.values);
  });

  test('dart call handleNestedStruct', () async {
    final r = await handleNestedStruct(s: _createMyNestedStruct());
    testComplexStruct(r.treeNode, arrLen: 5);
    expect(r.weekday, Weekdays.friday);
  });

  test('Lossless big buffers', () async {
    final list = await handleBigBuffers();
    expect(list.int64[0], BigInt.parse('-9223372036854775808'));
    expect(list.int64[1], BigInt.parse('9223372036854775807'));
    expect(list.uint64[0], BigInt.parse('0xFFFFFFFFFFFFFFFF'), reason: 'uint64');
  });

  test('test abc', () async {
    final output1 = await testAbcEnum(abc: Abc.a(A(a: "test")));
    expect((output1 as Abc_A).field0.a, "test");

    final output2 = await testAbcEnum(abc: Abc.b(B(b: 1)));
    expect((output2 as Abc_B).field0.b, 1);

    final output3 = await testAbcEnum(abc: Abc.c(C(c: false)));
    expect((output3 as Abc_C).field0.c, false);

    final output4 = await testAbcEnum(abc: Abc.justInt(1));
    expect((output4 as Abc_JustInt).field0, 1);
  });

  test("dart call struct_with_enum_member", () async {
    final result = await testStructWithEnum(se: StructWithEnum(abc1: Abc.a(A(a: "aaa")), abc2: Abc.b(B(b: 999))));
    expect(result.abc1.whenOrNull(b: (B b) => b.b), 999);
    expect(result.abc2.whenOrNull(a: (A a) => a.a), "aaa");
  });
}

MyTreeNode _createMyTreeNode({required int arrLen}) {
  return MyTreeNode(
    valueI32: 100,
    valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
    valueBoolean: true,
    children: [
      MyTreeNode(
        valueI32: 110,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
        valueBoolean: true,
        children: [
          MyTreeNode(
            valueI32: 111,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
            valueBoolean: true,
            children: [],
          ),
        ],
      ),
      MyTreeNode(
        valueI32: 120,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
        valueBoolean: true,
        children: [],
      ),
    ],
  );
}

MyNestedStruct _createMyNestedStruct() {
  return MyNestedStruct(treeNode: _createMyTreeNode(arrLen: 5), weekday: Weekdays.friday);
}
