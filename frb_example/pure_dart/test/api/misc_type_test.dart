import 'package:frb_example_pure_dart/src/rust/api/misc_type.dart';
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
}
