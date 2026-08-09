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
    final testModel = await handleTypeAliasModelTwinNormal(
      input: BigInt.from(42),
    );
    expect(testModel.id.toInt(), 42);
    expect(testModel.name, "TestModel");
    expect(testModel.aliasEnum, MyEnum.false_);
    expect(testModel.aliasStruct.content, true);
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

  test('chained generic type alias resolves to Ok value (#3071)', () async {
    final value = await genericResultAliasChainedOkTwinNormal();
    expect(value, 43);
  });

  test('chained generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasChainedErrTwinNormal(),
      throwsA(isA<GenericAliasErrorTwinNormal>()),
    );
  });

  test('two-parameter generic type alias resolves to Ok value (#3071)',
      () async {
    final value = await genericResultAliasTwoParamsOkTwinNormal();
    expect(value, 44);
  });

  test(
      'two-parameter generic type alias Err translates to a Dart exception (#3071)',
      () async {
    await expectLater(
      () async => genericResultAliasTwoParamsErrTwinNormal(),
      throwsA(isA<GenericAliasErrorTwinNormal>()),
    );
  });

  test('generic Option alias resolves in return position (#3071)', () async {
    expect(await genericOptionAliasReturnTwinNormal(input: 45), 45);
    expect(await genericOptionAliasReturnTwinNormal(input: -1), isNull);
  });

  test('generic Option alias resolves in argument position (#3071)', () async {
    expect(await genericOptionAliasArgTwinNormal(input: 46), 46);
    expect(await genericOptionAliasArgTwinNormal(input: null), -1);
  });
}
