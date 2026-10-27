import 'package:frb_example_pure_dart/src/rust/api/third_party_enum.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:frb_example_pure_dart/src/rust/third_party/frb_example_pure_dart_example_third_party_lib.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(thirdPartyEnumWithFieldsRoundtripTwinNormal, [
    ThirdPartyEnumWithFields.unit(),
    ThirdPartyEnumWithFields.named(id: 42, name: 'hi'),
    ThirdPartyEnumWithFields.tuple(100),
  ]);
}
