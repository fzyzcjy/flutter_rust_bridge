// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `type_alias_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/type_alias_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handle_type_id', () async {
    final id = await handleTypeAliasIdTwinRustAsyncSse(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_nest_alias_id', () async {
    final id =
        await handleTypeNestAliasIdTwinRustAsyncSse(input: BigInt.from(42));
    expect(id.toInt(), 42);
  });

  test('dart call handle_type_model', () async {
    final testModel = await handleTypeAliasModelTwinRustAsyncSse(
      input: BigInt.from(42),
    );
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.false_);
    expect(testModel.aliasStruct.content, true);
  });

  test('infallible API compiles when user shadows std Result (#1710)',
      () async {
    final value = await infallibleWithResultShadowTwinRustAsyncSse();
    expect(value, 42);
  });

  test('generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasOkTwinRustAsyncSse();
    expect(value, 42);
  });

  test('generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasErrTwinRustAsyncSse(),
      throwsA(isA<GenericAliasErrorTwinRustAsyncSse>()),
    );
  });

  test('chained generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasChainedOkTwinRustAsyncSse();
    expect(value, 43);
  });

  test('chained generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasChainedErrTwinRustAsyncSse(),
      throwsA(isA<GenericAliasErrorTwinRustAsyncSse>()),
    );
  });

  test('two-parameter generic type alias resolves to Ok value (#3071)',
      () async {
    final value = await genericResultAliasTwoParamsOkTwinRustAsyncSse();
    expect(value, 44);
  });

  test(
      'two-parameter generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasTwoParamsErrTwinRustAsyncSse(),
      throwsA(isA<GenericAliasErrorTwinRustAsyncSse>()),
    );
  });

  test('generic Option alias resolves in return position (#3071)', () async {
    expect(await genericOptionAliasReturnTwinRustAsyncSse(input: 45), 45);
    expect(await genericOptionAliasReturnTwinRustAsyncSse(input: -1), isNull);
  });

  test('generic Option alias resolves in argument position (#3071)', () async {
    expect(await genericOptionAliasArgTwinRustAsyncSse(input: 46), 46);
    expect(await genericOptionAliasArgTwinRustAsyncSse(input: null), -1);
  });
}
