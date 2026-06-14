// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/type_alias_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handle_type_id', () async {
    final id = await handleTypeAliasIdTwinRustAsync(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_nest_alias_id', () async {
    final id = await handleTypeNestAliasIdTwinRustAsync(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_model', () async {
    final testModel = await handleTypeAliasModelTwinRustAsync(
      input: BigInt.from(42),
    );
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.false_);
    expect(testModel.aliasStruct.content, true);
  });

  test('infallible API compiles when user shadows std Result (#1710)',
      () async {
    final value = await infallibleWithResultShadowTwinRustAsync();
    expect(value, 42);
  });

  test('generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasOkTwinRustAsync();
    expect(value, 42);
  });

  test('generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasErrTwinRustAsync(),
      throwsA(isA<GenericAliasErrorTwinRustAsync>()),
    );
  });
}
