// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_type_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcStringTwinSync, ['', 'hello', '😂']);

  test('call funcReturnUnitTwinSync', () async {
    // `as dynamic` to make the generated pseudo_manual test happy
    await (funcReturnUnitTwinSync() as dynamic);
  });
}
