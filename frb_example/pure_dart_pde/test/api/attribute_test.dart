import 'package:frb_example_pure_dart/src/rust/api/attribute.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart check that non-final field is modifiable', () {
    var customized = CustomizedTwinNormal(
        finalField: "finalField", nonFinalField: "nonFinalField");
    expect(customized.nonFinalField, "nonFinalField");
    customized.nonFinalField = "changed";
    expect(customized.nonFinalField, "changed");
  });

  test('dart call next_user_id to test metadata annotations', () async {
    UserIdTwinNormal userId = UserIdTwinNormal(value: 11);
    expect(await nextUserIdTwinNormal(userId: userId),
        UserIdTwinNormal(value: 12));
  });
}
