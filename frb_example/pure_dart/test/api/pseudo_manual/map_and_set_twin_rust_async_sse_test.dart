// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `map_and_set_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/enumeration_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/map_and_set_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcHashMapI32I32TwinRustAsyncSse, <Map<int, int>>[
    {},
    {10: 20},
    {10: 20, 30: 40},
  ]);
  addTestsIdentityFunctionCall(funcHashSetI32TwinRustAsyncSse, <Set<int>>[
    {},
    {10},
    {10, 20},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringStringTwinRustAsyncSse, <Map<String, String>>[
    {},
    {'a': 'b'},
    {'a': 'b', 'c': 'd'},
  ]);
  addTestsIdentityFunctionCall(funcHashSetStringTwinRustAsyncSse, <Set<String>>[
    {},
    {'a'},
    {'a', 'b'},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringBytesTwinRustAsyncSse, <Map<String, Uint8List>>[
    {},
    {
      'a': Uint8List.fromList([10, 20])
    },
    {
      'a': Uint8List.fromList([10, 20]),
      'b': Uint8List.fromList([30, 40, 50]),
    },
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringStructTwinRustAsyncSse, <Map<String, MySize>>[
    {},
    {'a': MySize(width: 1, height: 2)},
    {
      'a': MySize(width: 1, height: 2),
      'b': MySize(width: 3, height: 4),
    },
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringSimpleEnumTwinRustAsyncSse,
      <Map<String, EnumSimpleTwinRustAsyncSse>>[
        {},
        {'a': EnumSimpleTwinRustAsyncSse.a},
        {
          'a': EnumSimpleTwinRustAsyncSse.a,
          'b': EnumSimpleTwinRustAsyncSse.b,
        },
      ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringComplexEnumTwinRustAsyncSse,
      <Map<String, KitchenSinkTwinRustAsyncSse>>[
        {},
        {'a': KitchenSinkTwinRustAsyncSse.empty()},
        {
          'a': KitchenSinkTwinRustAsyncSse.buffer(Uint8List.fromList([10, 20])),
          'b': KitchenSinkTwinRustAsyncSse.nested(42),
        },
      ]);
}
