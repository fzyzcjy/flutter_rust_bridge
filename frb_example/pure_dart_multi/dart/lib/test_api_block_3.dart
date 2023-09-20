import 'dart:typed_data';

import 'package:test/test.dart';

import 'bridge_definitions.dart';
import 'bridge_generated_api_block_3.dart';

void testApiBlock3(
    ApiBlock3ClassImpl api3, BridgeGeneratedSharedImpl apiShared, List<SharedComplexEnumInAllBlocks> enumList) {
  test('dart call testInbuiltTypeInBlock3', () async {
    expect(await api3.testInbuiltTypeInBlock3(a: 42, b: 100.0), 142.0);
  });

  test('dart call testStringInBlock3', () async {
    expect(await api3.testStringInBlock3(s: "string", i: 3), "string_3");
  });

  test('dart call testSharedStructOnlyForSyncWithNoSyncReturnInBlock3', () async {
    final name = "name";
    final score = 1.1;
    var expected = SharedStructOnlyForSyncTest(bridge: apiShared, name: name, score: score);
    var actual = await api3.testSharedStructOnlyForSyncWithNoSyncReturnInBlock3(name: name, score: score);
    expect(expected.name, actual.name);
    expect(expected.score, actual.score);
  });

  test('dart call testSharedStructOnlyForSyncAsInputWithNoSyncReturnInBlock3', () async {
    final inputObj = SharedStructOnlyForSyncTest(
      bridge: apiShared,
      name: "name",
      score: 0.0,
    );
    final inputDefaultScore = 1.1;
    var actual = await api3.testSharedStructOnlyForSyncAsInputWithNoSyncReturnInBlock3(
      obj: inputObj,
      defaultScore: inputDefaultScore,
    );
    var expected = SharedStructOnlyForSyncTest(bridge: apiShared, name: "name", score: inputDefaultScore);
    expect(actual.name, expected.name);
    expect(actual.score, expected.score);
  });

  test('dart call testAllSharedStructInBlock3', () async {
    {
      var expected = SharedStructInAllBlocks(bridge: apiShared, name: "newString", id: 2, num: 2.2);
      var actual = await api3.testAllSharedStructInBlock3(
          custom: SharedStructInAllBlocks(bridge: apiShared, name: "string", id: 1, num: 2.2, enumList: null),
          s: "newString",
          i: 2);
      expect(expected.name, actual!.name);
      expect(expected.id, actual.id);
      expect(expected.num, actual.num);
      expect(expected.enumList, actual.enumList);
    }

    {
      expect(await api3.testAllSharedStructInBlock3(s: "newString", i: 2), null);
    }
  });

  test('dart call testSharedStructInBlock3For2And3', () async {
    var expected = SharedStructInBlock2And3(bridge: apiShared, name: "newString", id: 2, num: 2.2);
    var actual = await api3.testSharedStructInBlock3For2And3(
        custom: SharedStructInBlock2And3(bridge: apiShared, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock3For2And3', () async {
    var expected = CrossSharedStructInBlock2And3(bridge: apiShared, name: "string3");
    var actual = await api3.testCrossSharedStructInBlock3For2And3(
      name: "string3",
    );
    expect(expected.name, actual.name);
  });

  test('dart call testCrossSharedStructInSyncInBlock3For2And3', () async {
    var actual = CrossSharedStructInBlock2And3(bridge: apiShared, name: "string3");
    var expected = api3.testCrossSharedStructInSyncInBlock3For2And3(
      name: "string3",
    );
    expect(
      expected.name,
      actual.name,
    );
  });

  test('dart call testUniqueStruct3', () async {
    var actual = await api3.testUniqueStruct3(
        custom: StructOnlyForBlock3(bridge: api3, name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    var expected = StructOnlyForBlock3(bridge: api3, name: "newString", id: 2, num: 2.2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testStructDefinedInBlock3', () async {
    expect(
      await api3.testStructDefinedInBlock3(
        custom: StructDefinedInBlock3(bridge: api3, name: "string"),
      ),
      "string",
    );
  });

  test('dart call testEnumDefinedInBlock3', () async {
    expect(
      await api3.testEnumDefinedInBlock3(custom: EnumDefinedInBlock3.changeColor(1, 2, 3)),
      "changeColor_1_2_3",
    );
  });

  test('dart call testListInBlock3', () async {
    // add test case for method `api1.testListInBlock1`
    expect(
        await api3.testListInBlock3(nums: Int32List.fromList([1, 2]), strings: [
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
          StructDefinedInBlock3(
            bridge: api3,
            name: "struct1",
          ),
          StructDefinedInBlock3(
            bridge: api3,
            name: "struct2",
          ),
        ], enumList: [
          EnumDefinedInBlock3.quit(),
          EnumDefinedInBlock3.write("write"),
        ]),
        "struct1_struct2_a_b_1_2_Saturday_Sunday_struct1_struct2_Quit_Write(\"write\")");
  });
}
