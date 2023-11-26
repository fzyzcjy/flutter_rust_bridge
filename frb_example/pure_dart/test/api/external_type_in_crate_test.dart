import 'package:frb_example_pure_dart/src/rust/api/external_type_in_crate.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call useImportedStruct()', () async {
    expect(
      await useImportedStruct(myStruct: MyStruct(content: false)),
      false,
    );
    expect(
      await useImportedStruct(myStruct: MyStruct(content: true)),
      true,
    );
  });

  test('dart call useImportedEnum()', () async {
    expect(
      await useImportedEnum(myEnum: MyEnum.False),
      false,
    );
    expect(
      await useImportedEnum(myEnum: MyEnum.True),
      true,
    );
  });

  test('resolve module for old module system', () async {
    final o = await callOldModuleSystem();
    expect(o.field, 2);
  });

  test('resolve module for new module system', () async {
    final n = await callNewModuleSystem();
    expect(n.field, 1);
  });
}
