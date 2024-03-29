// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_map_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_map_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_rust_async_sse.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_map', () {
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI8TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: -128},
      {42: 127}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI16TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: -32768},
      {42: 32767}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI32TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeI64TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: -9007199254740992},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU8TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: 255}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU16TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: 65535}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU32TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: 4294967295}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeU64TwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeIsizeTwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeUsizeTwinRustAsyncSse, <Map<int, int>>[
      {},
      {42: 0},
      {42: 4294967295}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeF32TwinRustAsyncSse, <Map<int, double>>[
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeF64TwinRustAsyncSse, <Map<int, double>>[
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBoolTwinRustAsyncSse, <Map<int, bool>>[
      {},
      {42: false},
      {42: true}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeStringTwinRustAsyncSse, <Map<int, String>>[
      {},
      {42: ""},
      {42: "hello"},
      {42: "😂"}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBytesTwinRustAsyncSse, <Map<int, Uint8List>>[
      {},
      {42: Uint8List.fromList([])},
      {
        42: Uint8List.fromList([255, 0])
      },
      {
        42: Uint8List.fromList([10, 20, 30, 40])
      }
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicPrimitiveEnumTwinRustAsyncSseTwinRustAsyncSse,
        <Map<int, BasicPrimitiveEnumTwinRustAsyncSse>>[
          {},
          {42: BasicPrimitiveEnumTwinRustAsyncSse.apple},
          {42: BasicPrimitiveEnumTwinRustAsyncSse.orange}
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicGeneralEnumTwinRustAsyncSseTwinRustAsyncSse,
        <Map<int, BasicGeneralEnumTwinRustAsyncSse>>[
          {},
          {42: BasicGeneralEnumTwinRustAsyncSse.apple(field: "one")},
          {42: BasicGeneralEnumTwinRustAsyncSse.orange()}
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicStructTwinRustAsyncSseTwinRustAsyncSse,
        <Map<int, BasicStructTwinRustAsyncSse>>[
          {},
          {42: BasicStructTwinRustAsyncSse(apple: null, orange: null)},
          {42: BasicStructTwinRustAsyncSse(apple: "one", orange: 42)}
        ]);
  });
}
