import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinNormal, [
    EnumSimpleTwinNormal.a,
    EnumSimpleTwinNormal.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinNormal, [
    EnumWithItemMixedTwinNormal.a(),
    EnumWithItemMixedTwinNormal.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinNormal.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinNormal, [
    EnumWithItemStructTwinNormal.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinNormal.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinNormal, [
    EnumWithItemTupleTwinNormal.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinNormal.b(Int32List.fromList([200])),
  ]);
}
