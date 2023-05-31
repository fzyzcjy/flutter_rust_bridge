import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated_api_block_1.dart';
import 'bridge_generated_api_block_2.dart';
import 'bridge_generated_api_block_3.dart';
import 'package:test/test.dart';

void main(List<String> args) {
  print(args.length);
  print(args);
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');

  final dylib = loadLibForDart(dylibPath);

  final enumList = [
    EnumType.enums(Weekdays.Friday),
    EnumType.nested(EnumType.enums(Weekdays.Friday)),
    EnumType.empty(),
    EnumType.primitives(int32: 1, float64: 2.0, boolean: true),
    EnumType.optional(null, Uint8List.fromList([1, 2])),
    EnumType.buffer(Float32List.fromList([1.1, 2.2])),
    EnumType.bytesArray(U8Array3(Uint8List.fromList([1, 2, 3])))
  ];

  //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test api block 1↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
  final api1 = ApiBlock1ClassImpl(dylib);

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
    var expected = SharedStructOnlyForSyncTest(name: "name", score: score);
    var actual = api1.testSharedStructOnlyForSyncWithSyncReturnInBlock1(name: name, score: score);
    expect(expected.name, actual.name);
    expect(expected.score, actual.score);
  });

  test('dart call testAllSharedStructInBlock1', () async {
    var expected = SharedStructInAllBlocks(
      name: "newString",
      id: 2,
      num: 2.2,
      enumList: enumList,
    );
    var actual = await api1.testAllSharedStructInBlock1(
        custom: SharedStructInAllBlocks(
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
    var expected = SharedStructInBlock1And2(name: "newString", id: 2, num: 2.2);
    var actual = await api1.testSharedStructInBlock1For1And2(
        custom: SharedStructInBlock1And2(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock1For1And2', () async {
    expect(
      await api1.testCrossSharedStructInBlock1For1And2(
        custom: CrossSharedStructInBlock1And2(name: "string1"),
      ),
      "string1",
    );
  });

  test('dart call testUniqueStruct1', () async {
    var expected = StructOnlyForBlock1(name: "newString", id: 2, num: 2.0);
    var actual = await api1.testUniqueStruct1(
        custom: StructOnlyForBlock1(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
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
  //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test api block 1↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

  //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test api block 2↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
  final api2 = ApiBlock2ClassImpl(dylib);

  test('dart call testInbuiltTypeInBlock2', () async {
    expect(await api2.testInbuiltTypeInBlock2(a: 42, b: 100.0), 142.0);
  });

  test('dart call testStringInBlock2', () async {
    expect(await api2.testStringInBlock2(s: "string", i: 2), "string_2");
  });

  test('dart call testAllSharedStructInBlock2', () async {
    var expected = SharedStructInAllBlocks(name: "newString", id: 2, num: 2.2, enumList: enumList);
    var actual = await api2.testAllSharedStructInBlock2(
        custom: SharedStructInAllBlocks(name: "string", id: 1, num: 2.2, enumList: enumList), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
    expect(expected.enumList, actual.enumList);
  });

  test('dart call testAllSharedStructInSyncInBlock2', () async {
    var expected = SharedStructInAllBlocks(name: "newString", id: 2, num: 2.2, enumList: enumList);
    var actual = api2.testAllSharedStructInSyncInBlock2(
        custom: SharedStructInAllBlocks(name: "string", id: 1, num: 2.2, enumList: enumList), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
    expect(expected.enumList, actual.enumList);
  });

  test('dart call testSharedStructInBlock2For1And2', () async {
    var expected = SharedStructInBlock1And2(name: "newString", id: 2, num: 2.2);
    var actual = await api2.testSharedStructInBlock2For1And2(
        custom: SharedStructInBlock1And2(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock2For1And2', () async {
    var expected = CrossSharedStructInBlock2And3(name: "string");
    var actual = await api2.testCrossSharedStructInBlock2For1And2(
      name: "string",
    );
    expect(expected.name, actual.name);
  });

  test('dart call testSharedStructInBlock2For2And3', () async {
    var expected = SharedStructInBlock2And3(name: "newString", id: 2, num: 2.2);
    var actual = await api2.testSharedStructInBlock2For2And3(
        custom: SharedStructInBlock2And3(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock2For2And3', () async {
    expect(
      await api2.testCrossSharedStructInBlock2For2And3(
        custom: CrossSharedStructInBlock2And3(name: "stringTest"),
      ),
      "stringTest",
    );
  });

  test('dart call testUniqueStruct2', () async {
    var expected = StructOnlyForBlock2(name: "newString", id: 2, num: 2.2);
    var actual = await api2.testUniqueStruct2(
        custom: StructOnlyForBlock2(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
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
  //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test api block 2↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

  //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test api block 3↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
  final api3 = ApiBlock3ClassImpl(dylib);

  test('dart call testInbuiltTypeInBlock3', () async {
    expect(await api3.testInbuiltTypeInBlock3(a: 42, b: 100.0), 142.0);
  });

  test('dart call testStringInBlock3', () async {
    expect(await api3.testStringInBlock3(s: "string", i: 3), "string_3");
  });

  test('dart call testSharedStructOnlyForSyncWithNoSyncReturnInBlock3', () async {
    final name = "name";
    final score = 1.1;
    var expected = SharedStructOnlyForSyncTest(name: name, score: score);
    var actual = await api3.testSharedStructOnlyForSyncWithNoSyncReturnInBlock3(name: name, score: score);
    expect(expected.name, actual.name);
    expect(expected.score, actual.score);
  });

  test('dart call testSharedStructOnlyForSyncAsInputWithNoSyncReturnInBlock3', () async {
    final inputObj = SharedStructOnlyForSyncTest(
      name: "name",
      score: 0.0,
    );
    final inputDefaultScore = 1.1;
    var actual = await api3.testSharedStructOnlyForSyncAsInputWithNoSyncReturnInBlock3(
      obj: inputObj,
      defaultScore: inputDefaultScore,
    );
    var expected = SharedStructOnlyForSyncTest(name: "name", score: inputDefaultScore);
    expect(actual.name, expected.name);
    expect(actual.score, expected.score);
  });

  test('dart call testAllSharedStructInBlock3', () async {
    {
      var expected = SharedStructInAllBlocks(name: "newString", id: 2, num: 2.2);
      var actual = await api3.testAllSharedStructInBlock3(
          custom: SharedStructInAllBlocks(name: "string", id: 1, num: 2.2, enumList: null), s: "newString", i: 2);
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
    var expected = SharedStructInBlock2And3(name: "newString", id: 2, num: 2.2);
    var actual = await api3.testSharedStructInBlock3For2And3(
        custom: SharedStructInBlock2And3(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    expect(expected.name, actual.name);
    expect(expected.id, actual.id);
    expect(expected.num, actual.num);
  });

  test('dart call testCrossSharedStructInBlock3For2And3', () async {
    var expected = CrossSharedStructInBlock2And3(name: "string3");
    var actual = await api3.testCrossSharedStructInBlock3For2And3(
      name: "string3",
    );
    expect(expected.name, actual.name);
  });

  test('dart call testCrossSharedStructInSyncInBlock3For2And3', () async {
    var actual = CrossSharedStructInBlock2And3(name: "string3");
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
        custom: StructOnlyForBlock3(name: "string", id: 1, num: 2.2), s: "newString", i: 2);
    var expected = StructOnlyForBlock3(name: "newString", id: 2, num: 2.2);
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
  //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test api block 3↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

  //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test class methods↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
  // final apiShare = BridgeGeneratedSharesImpl(dylib);
  // test('test struct method of `SharedStruct`', () async {
  //   final obj = SharedStruct(bridge: apiShare, name: "string", id: 1, num: 2.2);
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await SharedStruct.testStaticMethod(bridge: apiShare, message: 'msg'),
  //     "msg",
  //   );
  // });

  // test('test struct method of `CrossSharedStruct`', () async {
  //   final obj = CrossSharedStruct(bridge: apiShare, name: "testName");
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await CrossSharedStruct.testStaticMethod(
  //         bridge: apiShare, message: 'msg'),
  //     "msg",
  //   );
  // });

  // test('test struct method of `OnlyForApi1Struct`', () async {
  //   final obj =
  //       OnlyForApi1Struct(bridge: api1, name: "string", id: 1, num: 2.2);
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await OnlyForApi1Struct.testStaticMethod(bridge: api1, message: 'msg'),
  //     "msg",
  //   );
  // });

  // test('test struct method of `OnlyForApi2Struct`', () async {
  //   final obj =
  //       OnlyForApi2Struct(bridge: api2, name: "string", id: 1, num: 2.2);
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await OnlyForApi2Struct.testStaticMethod(bridge: api2, message: 'msg'),
  //     "msg",
  //   );
  // });

  // test('test struct method of `StructDefinedInApi1`', () async {
  //   final obj = StructDefinedInApi1(bridge: api1, name: "testName");
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await StructDefinedInApi1.testStaticMethod(bridge: api1, message: 'msg'),
  //     "msg",
  //   );
  // });

  // test('test struct method of `StructDefinedInApi2`', () async {
  //   final obj = StructDefinedInApi2(bridge: api2, name: "testName");
  //   expect(
  //     await obj.testMethod(message: 'msg'),
  //     "msg",
  //   );
  //   expect(
  //     await StructDefinedInApi2.testStaticMethod(bridge: api2, message: 'msg'),
  //     "msg",
  //   );
  // });
  //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test class methods↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

  tearDownAll(() {
    api1.dispose();
    api2.dispose();
  });
}
