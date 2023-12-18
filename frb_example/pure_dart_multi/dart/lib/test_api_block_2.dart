import 'dart:typed_data';

import 'package:test/test.dart';

import 'bridge_definitions.dart';
import 'bridge_generated_api_block_2.dart';

void testApiBlock2(
    ApiBlock2ClassImpl api2, BridgeGeneratedSharedImpl apiShared, List<SharedComplexEnumInAllBlocks> enumList) {
  test('dart call testInbuiltTypeInBlock2', () async {
    expect(await api2.testInbuiltTypeInBlock2(a: 42, b: 100.0), 142.0);
  });

  test('dart call testStringInBlock2', () async {
    expect(await api2.testStringInBlock2(s: "string", i: 2), "string_2");
  });

  test('dart call testAllSharedStructInBlock2', () async {
    var expected = SharedStructInAllBlocks(bridge: apiShared, name: "newString", id: 2, num: 2.2, enumList: enumList);
    var actual = await api2.testAllSharedStructInBlock2(
        custom: SharedStructInAllBlocks(bridge: apiShared, name: "string", id: 1, num: 2.2, enumList: enumList),
        s: "newString",
        i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
    expect(expected.enumList, actual.enumList);
  });

  test('dart call testAllSharedStructInSyncInBlock2', () async {
    var expected = SharedStructInAllBlocks(bridge: apiShared, name: "newString", id: 2, num: 2.2, enumList: enumList);
    var actual = api2.testAllSharedStructInSyncInBlock2(
        custom: SharedStructInAllBlocks(bridge: apiShared, name: "string", id: 1, num: 2.2, enumList: enumList),
        s: "newString",
        i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
    expect(expected.enumList, actual.enumList);
  });

  test('dart call testSharedStructInBlock2For1And2', () async {
    var expected = SharedStructInBlock1And2(bridge: apiShared, name: "newString", id: 2, num: 2.2);
    var actual = await api2.testSharedStructInBlock2For1And2(
        custom: SharedStructInBlock1And2(bridge: apiShared, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock2For1And2', () async {
    var expected = CrossSharedStructInBlock2And3(bridge: apiShared, name: "string");
    var actual = await api2.testCrossSharedStructInBlock2For1And2(
      name: "string",
    );
    expect(expected.name, actual.name);
  });

  test('dart call testSharedStructInBlock2For2And3', () async {
    var expected = SharedStructInBlock2And3(bridge: apiShared, name: "newString", id: 2, num: 2.2);
    var actual = await api2.testSharedStructInBlock2For2And3(
        custom: SharedStructInBlock2And3(bridge: apiShared, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock2For2And3', () async {
    expect(
      await api2.testCrossSharedStructInBlock2For2And3(
        custom: CrossSharedStructInBlock2And3(bridge: apiShared, name: "stringTest"),
      ),
      "stringTest",
    );
  });

  test('dart call testUniqueStruct2', () async {
    var expected = StructOnlyForBlock2(bridge: api2, name: "newString", id: 2, num: 2.2);
    var actual = await api2.testUniqueStruct2(
        custom: StructOnlyForBlock2(bridge: api2, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testStructDefinedInBlock2', () async {
    expect(
      await api2.testStructDefinedInBlock2(
        custom: StructDefinedInBlock2(bridge: api2, name: "stringTest"),
      ),
      "stringTest",
    );
  });

  test('dart call testEnumDefinedInBlock2', () async {
    expect(
      await api2.testEnumDefinedInBlock2(custom: EnumDefinedInBlock2.write("content")),
      "write_content",
    );
  });

  test('dart call testListInBlock2', () async {
    // add test case for method `api1.testListInBlock1`
    expect(
        await api2.testListInBlock2(nums: Int32List.fromList([1, 2]), strings: [
          "a",
          "b"
        ], sharedStructs: [
          SharedStructInAllBlocks(
            bridge: apiShared,
            name: "struct1",
            id: 1,
            num: 1.1,
            enumList: enumList,
          ),
          SharedStructInAllBlocks(
            bridge: apiShared,
            name: "struct2",
            id: 2,
            num: 2.2,
            enumList: enumList,
          ),
        ], weekdays: [
          SharedWeekdaysEnumInAllBlocks.Saturday,
          SharedWeekdaysEnumInAllBlocks.Sunday
        ], structList: [
          StructDefinedInBlock2(
            bridge: api2,
            name: "struct1",
          ),
          StructDefinedInBlock2(
            bridge: api2,
            name: "struct2",
          ),
        ], enumList: [
          EnumDefinedInBlock2.quit(),
          EnumDefinedInBlock2.write("write"),
        ]),
        "struct1_struct2_a_b_1_2_Saturday_Sunday_struct1_struct2_Quit_Write(\"write\")");
  });
}
