// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `map_and_set_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:typed_data';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/enumeration_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/map_and_set_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcHashMapI32I32TwinRustAsync, <Map<int, int>>[
    {},
    {10: 20},
    {10: 20, 30: 40},
  ]);
  addTestsIdentityFunctionCall(funcHashSetI32TwinRustAsync, <Set<int>>[
    {},
    {10},
    {10, 20},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringStringTwinRustAsync, <Map<String, String>>[
    {},
    {'a': 'b'},
    {'a': 'b', 'c': 'd'},
  ]);
  addTestsIdentityFunctionCall(
      funcHashMapStringStringHasherTwinRustAsync, <Map<String, String>>[
    {},
    {'a': 'b'},
    {'a': 'b', 'c': 'd'},
  ]);
  addTestsIdentityFunctionCall(funcHashSetStringTwinRustAsync, <Set<String>>[
    {},
    {'a'},
    {'a', 'b'},
  ]);
  addTestsIdentityFunctionCall(
      funcHashSetStringHasherTwinRustAsync, <Set<String>>[
    {},
    {'a'},
    {'a', 'b'},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringBytesTwinRustAsync, <Map<String, Uint8List>>[
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
      funcHashMapStringStructTwinRustAsync, <Map<String, MySize>>[
    {},
    {'a': MySize(width: 1, height: 2)},
    {
      'a': MySize(width: 1, height: 2),
      'b': MySize(width: 3, height: 4),
    },
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringSimpleEnumTwinRustAsync,
      <Map<String, EnumSimpleTwinRustAsync>>[
        {},
        {'a': EnumSimpleTwinRustAsync.a},
        {
          'a': EnumSimpleTwinRustAsync.a,
          'b': EnumSimpleTwinRustAsync.b,
        },
      ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringComplexEnumTwinRustAsync,
      <Map<String, KitchenSinkTwinRustAsync>>[
        {},
        {'a': KitchenSinkTwinRustAsync.empty()},
        {
          'a': KitchenSinkTwinRustAsync.buffer(Uint8List.fromList([10, 20])),
          'b': KitchenSinkTwinRustAsync.nested(42),
        },
      ]);
}
