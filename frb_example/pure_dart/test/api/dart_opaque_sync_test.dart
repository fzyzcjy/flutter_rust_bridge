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
    var syncBack = syncLoopback(opaque: f);
    expect(identical(syncOptionLoopback(opaque: syncBack), f), isTrue);
    expect(syncOptionLoopback(opaque: null), isNull);
  });

  test('sync option', () {
    var data4 = syncOptionDartOpaque(opaque: () => () => 'magic');
    expect(data4, isNotNull);
  });

  test('drop', () async {
    expect(syncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'test');
  });

  test('unwrap', () async {
    expect(unwrapDartOpaque(opaque: createLargeList(mb: 200)), 'Test');
    await expectLater(
        () => panicUnwrapDartOpaque(opaque: createLargeList(mb: 200)),
        throwsA(isA<PanicException>()));
  });

  test('unwrapped dart opaque', () async {
    String f() => "magic";
    var res = returnNonDroppableDartOpaque(opaque: f);
    expect(identical(res, f), isTrue);
    var syncBack = syncLoopback(opaque: f);
    expect(identical(syncOptionLoopback(opaque: syncBack), f), isTrue);
    expect(syncOptionLoopback(opaque: null), isNull);
  });
}
