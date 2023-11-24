// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/enumeration_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinSync, [
    EnumSimpleTwinSync.a,
    EnumSimpleTwinSync.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinSync, [
    EnumWithItemMixedTwinSync.a(),
    EnumWithItemMixedTwinSync.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinSync.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinSync, [
    EnumWithItemStructTwinSync.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinSync.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinSync, [
    EnumWithItemTupleTwinSync.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinSync.b(Int32List.fromList([200])),
  ]);
}
