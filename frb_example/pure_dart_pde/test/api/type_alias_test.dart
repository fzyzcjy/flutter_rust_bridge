// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/type_alias.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
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
    final testModel = await handleTypeAliasModelTwinNormal(
      input: BigInt.from(42),
    );
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.false_);
    expect(testModel.aliasStruct.content, true);
  });

  test('infallible API compiles when user shadows std Result (#1710)',
      () async {
    final value = await infallibleWithResultShadowTwinNormal();
    expect(value, 42);
  });

  test('generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasOkTwinNormal();
    expect(value, 42);
  });

  test('generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasErrTwinNormal(),
      throwsA(isA<GenericAliasErrorTwinNormal>()),
    );
  });
}
