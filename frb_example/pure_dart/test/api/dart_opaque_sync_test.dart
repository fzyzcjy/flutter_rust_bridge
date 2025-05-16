// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () {
    var syncBack = syncLoopbackTwinNormal(opaque: f);
    expect((syncOptionLoopbackTwinNormal(opaque: syncBack) as dynamic)(),
        'Test_String');
    expect(syncOptionLoopbackTwinNormal(opaque: null), isNull);
  });

  test('sync option', () {
    var data4 = syncOptionDartOpaqueTwinNormal(opaque: () => () => 'magic');
    expect(data4, isNotNull);
  });

  test('drop', () async {
    expect(syncAcceptDartOpaqueTwinNormal(opaque: createLargeList(mb: 200)),
        'test');
  });

  test('unwrap', () async {
    expect(
        unwrapDartOpaqueTwinNormal(opaque: createLargeList(mb: 200)), 'Test');
    await expectLater(
        () => panicUnwrapDartOpaqueTwinNormal(opaque: createLargeList(mb: 200)),
        throwsA(isA<PanicException>()));
  });

  // `returnNonDroppableDartOpaqueTwinNormal` is removed
  // test('unwrapped dart opaque', () async {
  //   String f() => "magic";
  //   var res = returnNonDroppableDartOpaqueTwinNormal(opaque: f);
  //   expect(identical(res, f), isTrue);
  //   var syncBack = syncLoopbackTwinNormal(opaque: f);
  //   expect(
  //       identical(syncOptionLoopbackTwinNormal(opaque: syncBack), f), isTrue);
  //   expect(syncOptionLoopbackTwinNormal(opaque: null), isNull);
  // });
}
