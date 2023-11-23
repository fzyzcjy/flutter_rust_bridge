import 'package:frb_example_pure_dart/src/rust/api/misc_type.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcStringTwinNormal, ['', 'hello', '😂']);

  test('call funcReturnUnitTwinNormal', () async {
    // nothing to assert - it returns nothing
    await funcReturnUnitTwinNormal();
  });
}
