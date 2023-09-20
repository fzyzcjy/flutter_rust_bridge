import 'dart:typed_data';

import 'package:test/test.dart';

import 'bridge_definitions.dart';
import 'bridge_generated_api_block_1.dart';

void testApiBlock1(
    ApiBlock1ClassImpl api1, BridgeGeneratedSharedImpl apiShared, List<SharedComplexEnumInAllBlocks> enumList) {
  test('dart call testInbuiltTypeInBlock1', () async {
    expect(await api1.testInbuiltTypeInBlock1(a: 42, b: 100.0), 142.0);
  });

  test('dart call testStringInBlock1', () async {
    expect(await api1.testStringInBlock1(s: "string", i: 1), "string_1");
  });

  test('dart call testStringInSyncInBlock1', () async {
    expect(api1.testStringInSyncInBlock1(s: "string", i: 1), "string_1");
  });

  test('dart call testOptionStringInBlock1', () async {
    expect(await api1.testOptionalStringInBlock1(s: "string", i: 0), "string0");
    expect(await api1.testOptionalStringInBlock1(s: null, i: 1), null);
    expect(await api1.testOptionalStringInBlock1(i: 2), null);
  });

  test('dart call testOptionalStringInSyncInBlock1', () async {
    expect(api1.testOptionalStringInSyncInBlock1(s: "string", i: 0), "string0");
    expect(api1.testOptionalStringInSyncInBlock1(s: null, i: 1), null);
    expect(api1.testOptionalStringInSyncInBlock1(i: 2), null);
  });

  test('dart call testSharedStructOnlyForSyncWithSyncReturnInBlock1', () async {
    final name = "name";
    final score = 1.1;
    var expected = SharedStructOnlyForSyncTest(bridge: apiShared, name: "name", score: score);
    var actual = api1.testSharedStructOnlyForSyncWithSyncReturnInBlock1(name: name, score: score);
    expect(expected.name, actual.name);
    expect(expected.score, actual.score);
  });

  test('dart call testAllSharedStructInBlock1', () async {
    var expected = SharedStructInAllBlocks(
      bridge: apiShared,
      name: "newString",
      id: 2,
      num: 2.2,
      enumList: enumList,
    );
    var actual = await api1.testAllSharedStructInBlock1(
        custom: SharedStructInAllBlocks(
          bridge: apiShared,
          name: "string",
          id: 1,
          num: 2.2,
          enumList: enumList,
        ),
        s: "newString",
        i: 2);

    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
    expect(expected.enumList, actual.enumList);
  });

  test('dart call testSharedStructInBlock1For1And2', () async {
    var expected = SharedStructInBlock1And2(bridge: apiShared, name: "newString", id: 2, num: 2.2);
    var actual = await api1.testSharedStructInBlock1For1And2(
        custom: SharedStructInBlock1And2(bridge: apiShared, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock1For1And2', () async {
    expect(
      await api1.testCrossSharedStructInBlock1For1And2(
        custom: CrossSharedStructInBlock1And2(bridge: apiShared, name: "string1"),
      ),
      "string1",
    );
  });

  test('dart call testUniqueStruct1', () async {
    var expected = StructOnlyForBlock1(bridge: api1, name: "newString", id: 2, num: 2.0);
    var actual = await api1.testUniqueStruct1(
        custom: StructOnlyForBlock1(bridge: api1, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testStructDefinedInBlock1', () async {
    expect(
      await api1.testStructDefinedInBlock1(
        custom: StructDefinedInBlock1(bridge: api1, name: "string"),
      ),
      "string",
    );
  });

  test('dart call testEnumDefinedInBlock1', () async {
    expect(
      await api1.testEnumDefinedInBlock1(custom: EnumDefinedInBlock1.move(x: -1, y: 2)),
      "move_-1_2",
    );
  });

  test('dart call testListInBlock1', () async {
    // add test case for method `api1.testListInBlock1`
    expect(
        await api1.testListInBlock1(nums: Int32List.fromList([1, 2]), strings: [
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
          StructDefinedInBlock1(
            bridge: api1,
            name: "struct1",
          ),
          StructDefinedInBlock1(
            bridge: api1,
            name: "struct2",
          ),
        ], enumList: [
          EnumDefinedInBlock1.quit(),
          EnumDefinedInBlock1.write("write"),
        ]),
        "struct1_struct2_a_b_1_2_Saturday_Sunday_struct1_struct2_Quit_Write(\"write\")");
  });
}
