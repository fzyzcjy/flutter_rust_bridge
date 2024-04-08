// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `external_impl_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/external_impl_twin_sync_sse_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('SimpleTranslatableExternalStructWithMethod', () async {
    expect(
        await SimpleTranslatableExternalStructWithMethod(a: 'hello')
            .simpleExternalMethod(),
        'hello');
  });

  test('SimpleTranslatableExternalStructWithMethod', () async {
    expect(
        await SimpleTranslatableExternalStructWithMethod(a: 'hello')
            .simpleExternalMethod(),
        'hello');
  });
}
