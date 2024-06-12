import 'package:frb_example_pure_dart/src/rust/api/type_alias.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handle_type_id', () async {
    final id = await handleTypeAliasIdTwinNormal(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_nest_alias_id', () async {
    final id = await handleTypeNestAliasIdTwinNormal(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_model', () async {
    final testModel =
        await handleTypeAliasModelTwinNormal(input: BigInt.from(42));
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.false_);
    expect(testModel.aliasStruct.content, true);
  });
}
