// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/attribute_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart check that non-final field is modifiable', () {
    var customized = CustomizedTwinRustAsyncSse(
        finalField: "finalField", nonFinalField: "nonFinalField");
    expect(customized.nonFinalField, "nonFinalField");
    customized.nonFinalField = "changed";
    expect(customized.nonFinalField, "changed");
  });

  test('dart call next_user_id to test metadata annotations', () async {
    UserIdTwinRustAsyncSse userId = UserIdTwinRustAsyncSse(value: 11);
    expect(await nextUserIdTwinRustAsyncSse(userId: userId),
        UserIdTwinRustAsyncSse(value: 12));
  });
}
