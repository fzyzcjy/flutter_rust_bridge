import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  test('unwrap', () async {
    expect(unwrapDartOpaque(opaque: createLargeList(mb: 200)), 'Test');
    await expectLater(() => panicUnwrapDartOpaque(opaque: createLargeList(mb: 200)), throwsA(isA<PanicException>()));
  });

  test('unwrapped dart opaque', () async {
    String f() => "magic";
    var res = returnNonDroppableDartOpaque(opaque: f);
    expect(identical(res, f), isTrue);
  });
}
