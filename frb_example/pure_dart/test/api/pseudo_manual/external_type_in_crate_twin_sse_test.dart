// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `external_type_in_crate_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/external_type_in_crate_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call useImportedStruct()', () async {
    expect(
      await useImportedStructTwinSse(myStruct: MyStruct(content: false)),
      false,
    );
    expect(
      await useImportedStructTwinSse(myStruct: MyStruct(content: true)),
      true,
    );
  });

  test('dart call useImportedEnum()', () async {
    expect(
      await useImportedEnumTwinSse(myEnum: MyEnum.false_),
      false,
    );
    expect(
      await useImportedEnumTwinSse(myEnum: MyEnum.true_),
      true,
    );
  });

  test('resolve module for old module system', () async {
    final o = await callOldModuleSystemTwinSse();
    expect(o.field, 2);
  });

  test('resolve module for new module system', () async {
    final n = await callNewModuleSystemTwinSse();
    expect(n.field, 1);
  });
}
