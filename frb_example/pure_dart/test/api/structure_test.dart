import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcStructWithZeroFieldTwinNormal', () async {
    expect(await funcEnumSimpleTwinNormal(arg: EnumSimple.A), EnumSimple.A);
    expect(await funcEnumSimpleTwinNormal(arg: EnumSimple.B), EnumSimple.B);
  });
}
