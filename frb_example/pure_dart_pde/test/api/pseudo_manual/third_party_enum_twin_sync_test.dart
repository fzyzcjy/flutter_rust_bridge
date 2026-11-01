// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `third_party_enum_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/third_party_enum_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart_pde/src/rust/third_party/frb_example_pure_dart_example_third_party_lib.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(thirdPartyEnumWithFieldsRoundtripTwinSync, [
    ThirdPartyEnumWithFields.unit(),
    ThirdPartyEnumWithFields.named(id: 42, name: 'hi'),
    ThirdPartyEnumWithFields.tuple(100),
  ]);

  test('third party non-exhaustive struct is opaque', () async {
    final output = await thirdPartyNonExhaustiveStructRoundtripTwinSync(
        arg: await ThirdPartyNonExhaustiveStruct.newInstance(value: 42));
    expect(output.value, 42);
  });
}
