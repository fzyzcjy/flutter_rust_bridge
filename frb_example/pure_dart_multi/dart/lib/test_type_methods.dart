import 'package:test/test.dart';

import 'bridge_definitions.dart';
import 'bridge_generated_api_block_1.dart';

void testTypeMethods(ApiBlock1ClassImpl api1, BridgeGeneratedSharedImpl apiShared) {
  test('dart test (static) methods of `StructDefinedInBlock1`', () async {
    var obj = StructDefinedInBlock1(bridge: api1, name: "newString");
    expect("msg", await obj.testMethod(message: "msg"));
    expect("msg", await StructDefinedInBlock1.testStaticMethod(bridge: api1, message: "msg"));
  });

  test('dart test (static) methods of `StructOnlyForBlock1`', () async {
    var obj = StructOnlyForBlock1(bridge: api1, name: "newString", id: 2, num: 2.0);
    expect("msg_1", await obj.testMethod(message: "msg", num: 1));

    expect("msg", await StructOnlyForBlock1.testStaticMethod(bridge: api1, message: "msg"));
  });

  test('dart test (static) methods of `SharedStructInAllBlocks`', () async {
    var obj = SharedStructInAllBlocks(
      bridge: apiShared,
      id: 0,
      num: 3,
      name: "name",
    );
    expect("msg_2", await obj.testMethod(message: "msg", num: 2));
    expect("msg", await SharedStructOnlyForSyncTest.testStaticMethod(bridge: apiShared, message: "msg"));
  });

  test('dart test (static) methods of `CrossSharedStructInBlock2And3`', () async {
    var obj = CrossSharedStructInBlock2And3(bridge: apiShared, name: "newString");
    expect("msg", await obj.testMethod(message: "msg"));
    expect("msg", await CrossSharedStructInBlock2And3.testStaticMethod(bridge: apiShared, message: "msg"));
  });

  // TODO: Implement method generation for enum types in Dart. Leave this test case for a future pull request.
  // Reference: https://stackoverflow.com/questions/38908285/how-do-i-add-methods-or-values-to-enums-in-dart
  // test('dart test (static) methods of `SharedWeekdaysEnumInAllBlocks`',
  //     () async {
  //   var obj = SharedWeekdaysEnumInAllBlocks.Friday;
  //   expect("msg", await obj.testEnumMethod(message: "msg"));
  //   expect(
  //       "msg",
  //       await SharedWeekdaysEnumInAllBlocks.testStaticEnumMethod(
  //           bridge: apiShared, message: "msg"));
  // });

  // test('dart test (static) methods of `SharedComplexEnumInAllBlocks`',
  //     () async {
  //   var obj = SharedComplexEnumInAllBlocks.empty();
  //   expect("msg", await obj.testEnumMethod(message: "msg"));
  //   expect(
  //       "msg",
  //       await SharedComplexEnumInAllBlocks.testStaticEnumMethod(
  //           bridge: apiShared, message: "msg"));
  // });

  // test('dart test (static) methods of `EnumDefinedInBlock1`', () async {
  //   var obj = EnumDefinedInBlock1.quit();
  //   expect("msg", await obj.testEnumMethod(message: "msg"));
  //   expect(
  //       "msg",
  //       await EnumDefinedInBlock1.testStaticEnumMethod(
  //           bridge: apiShared, message: "msg"));
  // });

  // test('dart test (static) methods of `EnumDefinedInBlock2`', () async {
  //   var obj = EnumDefinedInBlock2.move(x: -1, y: -2);
  //   expect("msg", await obj.testEnumMethod(message: "msg"));
  //   expect(
  //       "msg",
  //       await EnumDefinedInBlock2.testStaticEnumMethod(
  //           bridge: apiShared, message: "msg"));
  // });

  // test('dart test (static) methods of `EnumDefinedInBlock3`', () async {
  //   var obj = EnumDefinedInBlock3.changeColor(1, 2, 3);
  //   expect("msg", await obj.testEnumMethod(message: "msg"));
  //   expect(
  //       "msg",
  //       await EnumDefinedInBlock3.testStaticEnumMethod(
  //           bridge: apiShared, message: "msg"));
  // });
}
