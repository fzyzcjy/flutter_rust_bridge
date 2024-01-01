import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/map_and_set.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

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
}
