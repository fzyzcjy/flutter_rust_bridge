// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `serde_json_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:decimal/decimal.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/serde_json_type_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('serde_json::Value null', () async {
    final output = await handleSerdeJsonValueTwinRustAsync(val: null);
    expect(output, isNull);
  });

  test('serde_json::Value number', () async {
    final output = await handleSerdeJsonValueTwinRustAsync(val: 42);
    expect(output, 42);
  });

  test('serde_json::Value string', () async {
    final output = await handleSerdeJsonValueTwinRustAsync(val: 'hello');
    expect(output, 'hello');
  });

  test('serde_json::Value bool', () async {
    final output = await handleSerdeJsonValueTwinRustAsync(val: true);
    expect(output, true);
  });

  test('serde_json::Value list', () async {
    final input = [1, 'two', null, true];
    final output = await handleSerdeJsonValueTwinRustAsync(val: input);
    expect(output, input);
  });

  test('serde_json::Value map', () async {
    final input = {
      'name': 'Alice',
      'age': 30,
      'active': true,
      'tags': ['a', 'b'],
    };
    final output = await handleSerdeJsonValueTwinRustAsync(val: input);
    expect(output, input);
  });

  test('Option<serde_json::Value> some', () async {
    final output =
        await handleOptionSerdeJsonValueTwinRustAsync(val: {'key': 'value'});
    expect(output, {'key': 'value'});
  });

  test('Option<serde_json::Value> none', () async {
    final output = await handleOptionSerdeJsonValueTwinRustAsync(val: null);
    expect(output, isNull);
  });

  test('Vec<serde_json::Value>', () async {
    final input = [
      42,
      'hello',
      {'nested': true}
    ];
    final output = await handleVecSerdeJsonValueTwinRustAsync(val: input);
    expect(output, input);
  });

  test('HashMap<String, serde_json::Value>', () async {
    final input = {
      'num': 1,
      'str': 'hello',
      'obj': {'a': true},
    };
    final output = await handleMapSerdeJsonValueTwinRustAsync(val: input);
    expect(output, input);
  });

  test('serde_json::Value nested struct', () async {
    final data = {
      'key': 'value',
      'nested': {'a': 1}
    };
    final wrapper = FeatureSerdeJsonTwinRustAsync(data: data);
    final output =
        await handleNestedSerdeJsonValueTwinRustAsync(wrapper: wrapper);
    expect(output.data, data);
  });

  test('Big integer and decimal Rust library types', () async {
    final input = FeatureBigNumberTwinRustAsync(
      signed: BigInt.parse('-170141183460469231731687303715884105728'),
      unsigned: BigInt.parse('340282366920938463463374607431768211455'),
      reexportedSigned: BigInt.parse('-123456789123456789123456789'),
      reexportedUnsigned: BigInt.parse('123456789123456789123456789'),
      decimal: Decimal.parse('123456789.123456789'),
      bigDecimal: Decimal.parse('-987654321.987654321'),
    );

    final output = await handleBigNumberTypesTwinRustAsync(input: input);

    expect(output.signed, input.signed);
    expect(output.unsigned, input.unsigned);
    expect(output.reexportedSigned, input.reexportedSigned);
    expect(output.reexportedUnsigned, input.reexportedUnsigned);
    expect(output.decimal, input.decimal);
    expect(output.bigDecimal, input.bigDecimal);
  });
}
