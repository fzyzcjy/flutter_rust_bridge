import 'package:frb_example_pure_dart/src/rust/api/ownership.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(borrowStringTwinNormal, ['a']);
  addTestsIdentityFunctionCall(borrowStrTwinNormal, ['a']);
  addTestsIdentityFunctionCall(borrowI32TwinNormal, [100]);
  addTestsIdentityFunctionCall(
      borrowStructTwinNormal, [SimpleStructForBorrowTwinNormal(one: 'a')]);
}
