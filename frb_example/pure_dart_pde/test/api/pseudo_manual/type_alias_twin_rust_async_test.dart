// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/type_alias_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
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

  test('chained generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasChainedOkTwinRustAsync();
    expect(value, 43);
  });

  test('chained generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasChainedErrTwinRustAsync(),
      throwsA(isA<GenericAliasErrorTwinRustAsync>()),
    );
  });

  test('two-parameter generic type alias resolves to Ok value (#3071)',
      () async {
    final value = await genericResultAliasTwoParamsOkTwinRustAsync();
    expect(value, 44);
  });

  test(
      'two-parameter generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasTwoParamsErrTwinRustAsync(),
      throwsA(isA<GenericAliasErrorTwinRustAsync>()),
    );
  });

  test('generic Option alias resolves in return position (#3071)', () async {
    expect(await genericOptionAliasReturnTwinRustAsync(input: 45), 45);
    expect(await genericOptionAliasReturnTwinRustAsync(input: -1), isNull);
  });

  test('generic Option alias resolves in argument position (#3071)', () async {
    expect(await genericOptionAliasArgTwinRustAsync(input: 46), 46);
    expect(await genericOptionAliasArgTwinRustAsync(input: null), -1);
  });
}
