// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/attribute_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart check that non-final field is modifiable', () {
    var customized = CustomizedTwinSync(
        finalField: "finalField", nonFinalField: "nonFinalField");
    expect(customized.nonFinalField, "nonFinalField");
    customized.nonFinalField = "changed";
    expect(customized.nonFinalField, "changed");
  });

  test('dart call next_user_id to test metadata annotations', () async {
    UserIdTwinSync userId = UserIdTwinSync(value: 11);
    expect(await nextUserIdTwinSync(userId: userId), UserIdTwinSync(value: 12));
  });
}
