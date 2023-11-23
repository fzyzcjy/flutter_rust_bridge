// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

import 'package:frb_example_pure_dart/src/rust/api/inside_macro.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcMacroStruct, [const MacroStruct(value: 42)]);
}
