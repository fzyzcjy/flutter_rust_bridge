// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_list_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_list_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_twin_sync_sse.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_list', () {
    addTestsIdentityFunctionCall(exampleBasicListTypeI8TwinSyncSse, <Int8List>[
      Int8List.fromList([]),
      Int8List.fromList([0]),
      Int8List.fromList([-128]),
      Int8List.fromList([127])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI16TwinSyncSse, <Int16List>[
      Int16List.fromList([]),
      Int16List.fromList([0]),
      Int16List.fromList([-32768]),
      Int16List.fromList([32767])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI32TwinSyncSse, <Int32List>[
      Int32List.fromList([]),
      Int32List.fromList([0]),
      Int32List.fromList([-2147483648]),
      Int32List.fromList([2147483647])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeI64TwinSyncSse, <Int64List>[
      Int64List.fromList([]),
      Int64List.fromList([0]),
      Int64List.fromList([-9007199254740992]),
      Int64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeU8TwinSyncSse, <Uint8List>[
      Uint8List.fromList([]),
      Uint8List.fromList([0]),
      Uint8List.fromList([255])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU16TwinSyncSse, <Uint16List>[
      Uint16List.fromList([]),
      Uint16List.fromList([0]),
      Uint16List.fromList([65535])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU32TwinSyncSse, <Uint32List>[
      Uint32List.fromList([]),
      Uint32List.fromList([0]),
      Uint32List.fromList([4294967295])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeU64TwinSyncSse, <Uint64List>[
      Uint64List.fromList([]),
      Uint64List.fromList([0]),
      Uint64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeF32TwinSyncSse, <Float32List>[
      Float32List.fromList([]),
      Float32List.fromList([0]),
      Float32List.fromList([-42.5]),
      Float32List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeF64TwinSyncSse, <Float64List>[
      Float64List.fromList([]),
      Float64List.fromList([0]),
      Float64List.fromList([-42.5]),
      Float64List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBoolTwinSyncSse, <List<bool>>[
      <bool>[],
      <bool>[false],
      <bool>[true]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeStringTwinSyncSse, <List<String>>[
      [],
      [""],
      ["hello"],
      ["😂"]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBytesTwinSyncSse, <List<Uint8List>>[
      [],
      [Uint8List.fromList([])],
      [
        Uint8List.fromList([255, 0])
      ],
      [
        Uint8List.fromList([10, 20, 30, 40])
      ]
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicPrimitiveEnumTwinSyncSseTwinSyncSse,
        <List<BasicPrimitiveEnumTwinSyncSse>>[
          [],
          [BasicPrimitiveEnumTwinSyncSse.apple],
          [BasicPrimitiveEnumTwinSyncSse.orange]
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicGeneralEnumTwinSyncSseTwinSyncSse,
        <List<BasicGeneralEnumTwinSyncSse>>[
          [],
          [BasicGeneralEnumTwinSyncSse.apple(field: "one")],
          [BasicGeneralEnumTwinSyncSse.orange()]
        ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicStructTwinSyncSseTwinSyncSse,
        <List<BasicStructTwinSyncSse>>[
          [],
          [BasicStructTwinSyncSse(apple: null, orange: null)],
          [BasicStructTwinSyncSse(apple: "one", orange: 42)]
        ]);
  });
}
