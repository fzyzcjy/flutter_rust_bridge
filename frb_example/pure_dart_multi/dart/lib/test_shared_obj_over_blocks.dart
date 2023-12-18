/// This file is responsible for checking that shared objects are correctly
/// behaved across regular blocks;

import 'package:test/test.dart';

import 'bridge_definitions.dart';
import 'bridge_generated_api_block_1.dart';
import 'bridge_generated_api_block_2.dart';
import 'bridge_generated_api_block_3.dart';

void testSharedObjOverBlocks(ApiBlock1ClassImpl api1, ApiBlock2ClassImpl api2, ApiBlock3ClassImpl api3,
    BridgeGeneratedSharedImpl apiShared, List<SharedComplexEnumInAllBlocks> enumList) {
  test('dart call testAllSharedStructInBlock1', () async {
    // Create a shared object using methods from ApiBlock1ClassImpl
    var sharedObj = SharedStructInAllBlocks(
      bridge: apiShared,
      name: "string",
      id: 0,
      num: 2.2,
      enumList: enumList,
    );
    final result1 = await api1.testAllSharedStructInBlock1(custom: sharedObj, s: "string1", i: 1);
    final result2 = await api2.testAllSharedStructInBlock2(custom: sharedObj, s: "string2", i: 2);
    final result3 = await api3.testAllSharedStructInBlock3(custom: sharedObj, s: "string3", i: 3);

    // Assert that the results are as expected
    expect(sharedObj.name, equals("string"));
    expect(sharedObj.id, equals(0));
    expect(sharedObj.num, equals(2.2));
    expect(sharedObj.enumList, equals(enumList));

    // Assert that all results are readable
    expect(result1.name, equals("string1"));
    expect(result1.id, equals(1));
    expect(result1.num, equals(2.2));
    expect(result1.enumList, equals(enumList));

    expect(result2.name, equals("string2"));
    expect(result2.id, equals(2));
    expect(result2.num, equals(2.2));
    expect(result2.enumList, equals(enumList));

    expect(result3!.name, equals("string3"));
    expect(result3.id, equals(3));
    expect(result3.num, equals(2.2));
    expect(result3.enumList, equals(enumList));

    // Assert that all results are in different memory addresses
    expect(identical(sharedObj, result1), isFalse);
    expect(identical(sharedObj, result2), isFalse);
    expect(identical(sharedObj, result3), isFalse);
    expect(identical(result1, result2), isFalse);
    expect(identical(result1, result3), isFalse);
    expect(identical(result2, result3), isFalse);
  });
}
