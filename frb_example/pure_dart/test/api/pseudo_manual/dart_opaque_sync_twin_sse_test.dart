// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_sync_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () {
    var syncBack = syncLoopbackTwinSse(opaque: f);
    expect((syncOptionLoopbackTwinSse(opaque: syncBack) as dynamic)(),
        'Test_String');
    expect(syncOptionLoopbackTwinSse(opaque: null), isNull);
  });

  test('sync option', () {
    var data4 = syncOptionDartOpaqueTwinSse(opaque: () => () => 'magic');
    expect(data4, isNotNull);
  });

  test('drop', () async {
    expect(
        syncAcceptDartOpaqueTwinSse(opaque: createLargeList(mb: 200)), 'test');
  });

  test('unwrap', () async {
    expect(unwrapDartOpaqueTwinSse(opaque: createLargeList(mb: 200)), 'Test');
    await expectLater(
        () => panicUnwrapDartOpaqueTwinSse(opaque: createLargeList(mb: 200)),
        throwsA(isA<PanicException>()));
  });

  // `returnNonDroppableDartOpaqueTwinSse` is removed
  // test('unwrapped dart opaque', () async {
  //   String f() => "magic";
  //   var res = returnNonDroppableDartOpaqueTwinSse(opaque: f);
  //   expect(identical(res, f), isTrue);
  //   var syncBack = syncLoopbackTwinSse(opaque: f);
  //   expect(
  //       identical(syncOptionLoopbackTwinSse(opaque: syncBack), f), isTrue);
  //   expect(syncOptionLoopbackTwinSse(opaque: null), isNull);
  // });
}
