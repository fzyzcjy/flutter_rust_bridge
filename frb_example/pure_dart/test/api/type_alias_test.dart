import 'package:frb_example_pure_dart/src/rust/api/type_alias.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handle_type_id', () async {
    final id = await handleTypeAliasId(input: 42);
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_nest_alias_id', () async {
    final id = await handleTypeNestAliasId(input: 42);
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_model', () async {
    final testModel = await handleTypeAliasModel(input: 42);
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.False);
    expect(testModel.aliasStruct.content, true);
  });
}
