// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:typed_data';

import 'package:frb_example_pure_dart_pde/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/map_and_set.dart';
import 'package:frb_example_pure_dart_pde/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcHashMapI32I32TwinNormal, <Map<int, int>>[
    {},
    {10: 20},
    {10: 20, 30: 40},
  ]);
  addTestsIdentityFunctionCall(funcHashSetI32TwinNormal, <Set<int>>[
    {},
    {10},
    {10, 20},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringStringTwinNormal, <Map<String, String>>[
    {},
    {'a': 'b'},
    {'a': 'b', 'c': 'd'},
  ]);
  addTestsIdentityFunctionCall(funcHashSetStringTwinNormal, <Set<String>>[
    {},
    {'a'},
    {'a', 'b'},
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringBytesTwinNormal, <Map<String, Uint8List>>[
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
      funcHashMapStringStructTwinNormal, <Map<String, MySize>>[
    {},
    {'a': MySize(width: 1, height: 2)},
    {
      'a': MySize(width: 1, height: 2),
      'b': MySize(width: 3, height: 4),
    },
  ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringSimpleEnumTwinNormal,
      <Map<String, EnumSimpleTwinNormal>>[
        {},
        {'a': EnumSimpleTwinNormal.a},
        {
          'a': EnumSimpleTwinNormal.a,
          'b': EnumSimpleTwinNormal.b,
        },
      ]);

  addTestsIdentityFunctionCall(
      funcHashMapStringComplexEnumTwinNormal,
      <Map<String, KitchenSinkTwinNormal>>[
        {},
        {'a': KitchenSinkTwinNormal.empty()},
        {
          'a': KitchenSinkTwinNormal.buffer(Uint8List.fromList([10, 20])),
          'b': KitchenSinkTwinNormal.nested(42),
        },
      ]);
}
