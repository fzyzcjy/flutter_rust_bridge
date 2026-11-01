// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/third_party_enum.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart_pde/src/rust/third_party/frb_example_pure_dart_example_third_party_lib.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(thirdPartyEnumWithFieldsRoundtripTwinNormal, [
    ThirdPartyEnumWithFields.unit(),
    ThirdPartyEnumWithFields.named(id: 42, name: 'hi'),
    ThirdPartyEnumWithFields.tuple(100),
  ]);

  test('third party non-exhaustive struct is opaque', () async {
    final output = await thirdPartyNonExhaustiveStructRoundtripTwinNormal(
        arg: await ThirdPartyNonExhaustiveStruct.newInstance(value: 42));
    expect(output.value, 42);
  });
}
