// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_sync_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () {
    var syncBack = syncLoopbackTwinRustAsync(opaque: f);
    expect(identical(syncOptionLoopbackTwinRustAsync(opaque: syncBack), f),
        isTrue);
    expect(syncOptionLoopbackTwinRustAsync(opaque: null), isNull);
  });

  test('sync option', () {
    var data4 = syncOptionDartOpaqueTwinRustAsync(opaque: () => () => 'magic');
    expect(data4, isNotNull);
  });

  test('drop', () async {
    expect(syncAcceptDartOpaqueTwinRustAsync(opaque: createLargeList(mb: 200)),
        'test');
  });

  test('unwrap', () async {
    expect(unwrapDartOpaqueTwinRustAsync(opaque: createLargeList(mb: 200)),
        'Test');
    await expectLater(
        () => panicUnwrapDartOpaqueTwinRustAsync(
            opaque: createLargeList(mb: 200)),
        throwsA(isA<PanicException>()));
  });

  test('unwrapped dart opaque', () async {
    String f() => "magic";
    var res = returnNonDroppableDartOpaqueTwinRustAsync(opaque: f);
    expect(identical(res, f), isTrue);
    var syncBack = syncLoopbackTwinRustAsync(opaque: f);
    expect(identical(syncOptionLoopbackTwinRustAsync(opaque: syncBack), f),
        isTrue);
    expect(syncOptionLoopbackTwinRustAsync(opaque: null), isNull);
  });
}
