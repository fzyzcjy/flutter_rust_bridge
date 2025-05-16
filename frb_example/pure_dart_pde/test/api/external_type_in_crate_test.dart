// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/external_type_in_crate.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call useImportedStruct()', () async {
    expect(
      await useImportedStructTwinNormal(myStruct: MyStruct(content: false)),
      false,
    );
    expect(
      await useImportedStructTwinNormal(myStruct: MyStruct(content: true)),
      true,
    );
  });

  test('dart call useImportedEnum()', () async {
    expect(
      await useImportedEnumTwinNormal(myEnum: MyEnum.false_),
      false,
    );
    expect(
      await useImportedEnumTwinNormal(myEnum: MyEnum.true_),
      true,
    );
  });

  test('resolve module for old module system', () async {
    final o = await callOldModuleSystemTwinNormal();
    expect(o.field, 2);
  });

  test('resolve module for new module system', () async {
    final n = await callNewModuleSystemTwinNormal();
    expect(n.field, 1);
  });
}
