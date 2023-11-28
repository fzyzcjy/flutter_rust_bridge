// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_sync_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () {
    var syncBack = syncLoopbackTwinSync(opaque: f);
    expect(identical(syncOptionLoopbackTwinSync(opaque: syncBack), f), isTrue);
    expect(syncOptionLoopbackTwinSync(opaque: null), isNull);
  });

  test('sync option', () {
    var data4 = syncOptionDartOpaqueTwinSync(opaque: () => () => 'magic');
    expect(data4, isNotNull);
  });

  test('drop', () async {
    expect(
        syncAcceptDartOpaqueTwinSync(opaque: createLargeList(mb: 200)), 'test');
  });

  test('unwrap', () async {
    expect(unwrapDartOpaqueTwinSync(opaque: createLargeList(mb: 200)), 'Test');
    await expectLater(
        () => panicUnwrapDartOpaqueTwinSync(opaque: createLargeList(mb: 200)),
        throwsA(isA<PanicException>()));
  });

  test('unwrapped dart opaque', () async {
    String f() => "magic";
    var res = returnNonDroppableDartOpaqueTwinSync(opaque: f);
    expect(identical(res, f), isTrue);
    var syncBack = syncLoopbackTwinSync(opaque: f);
    expect(identical(syncOptionLoopbackTwinSync(opaque: syncBack), f), isTrue);
    expect(syncOptionLoopbackTwinSync(opaque: null), isNull);
  });
}
