import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinNormal, [
    EnumSimple.A,
    EnumSimple.B,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinNormal, [
    EnumWithItemMixed.a(),
    EnumWithItemMixed.b(Uint8List.fromList([42])),
    EnumWithItemMixed.c(cField: 'hi'),
  ]);

  // TODO
}
