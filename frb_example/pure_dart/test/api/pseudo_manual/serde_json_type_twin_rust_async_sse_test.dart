// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `serde_json_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/serde_json_type_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('serde_json::Value null', () async {
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: null);
    expect(output, isNull);
  });

  test('serde_json::Value number', () async {
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: 42);
    expect(output, 42);
  });

  test('serde_json::Value string', () async {
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: 'hello');
    expect(output, 'hello');
  });

  test('serde_json::Value bool', () async {
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: true);
    expect(output, true);
  });

  test('serde_json::Value list', () async {
    final input = [1, 'two', null, true];
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: input);
    expect(output, input);
  });

  test('serde_json::Value map', () async {
    final input = {
      'name': 'Alice',
      'age': 30,
      'active': true,
      'tags': ['a', 'b'],
    };
    final output = await handleSerdeJsonValueTwinRustAsyncSse(val: input);
    expect(output, input);
  });

  test('Option<serde_json::Value> some', () async {
    final output =
        await handleOptionSerdeJsonValueTwinRustAsyncSse(val: {'key': 'value'});
    expect(output, {'key': 'value'});
  });

  test('Option<serde_json::Value> none', () async {
    final output = await handleOptionSerdeJsonValueTwinRustAsyncSse(val: null);
    expect(output, isNull);
  });

  test('Vec<serde_json::Value>', () async {
    final input = [
      42,
      'hello',
      {'nested': true}
    ];
    final output = await handleVecSerdeJsonValueTwinRustAsyncSse(val: input);
    expect(output, input);
  });

  test('HashMap<String, serde_json::Value>', () async {
    final input = {
      'num': 1,
      'str': 'hello',
      'obj': {'a': true},
    };
    final output = await handleMapSerdeJsonValueTwinRustAsyncSse(val: input);
    expect(output, input);
  });

  test('serde_json::Value nested struct', () async {
    final data = {
      'key': 'value',
      'nested': {'a': 1}
    };
    final wrapper = FeatureSerdeJsonTwinRustAsyncSse(data: data);
    final output = await handleNestedSerdeJsonValueTwinRustAsyncSse(wrapper: wrapper);
    expect(output.data, data);
  });
}
