import 'package:frb_example_pure_dart/src/rust/api/misc_type.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcStringTwinNormal, ['', 'hello', '😂']);

  test('call funcReturnUnitTwinNormal', () async {
    // `as dynamic` to make the generated pseudo_manual test happy
    await (funcReturnUnitTwinNormal() as dynamic);
  });

  test('dart call handleListOfStruct', () async {
    final listOfStructResp =
        await handleListOfStruct(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });

  // TODO rm? since we have auto sync test
  // test('dart call handleListOfStructSync', () {
  //   final listOfStructResp =
  //       handleListOfStructSync(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
  //   expect(listOfStructResp.length, 4);
  //   expect(listOfStructResp[0].width, 42);
  //   expect(listOfStructResp[1].width, 420);
  //   expect(listOfStructResp[2].width, 42);
  //   expect(listOfStructResp[3].width, 420);
  // });

  test('dart call handleStringList', () async {
    final names = await handleStringList(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });

  // TODO rm?
  // test('dart call handleStringListSync', () {
  //   final names = handleStringListSync(names: ['Steve', 'Bob', 'Alex']);
  //   expect(names, ['Steve', 'Bob', 'Alex']);
  // });
}
