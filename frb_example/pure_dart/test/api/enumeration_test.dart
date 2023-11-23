import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcStructWithZeroFieldTwinNormal', () async {
    expect(await funcEnumSimpleTwinNormal(arg: EnumSimple.A), EnumSimple.A);
    expect(await funcEnumSimpleTwinNormal(arg: EnumSimple.B), EnumSimple.B);
  });

  test('call funcStructWithZeroFieldTwinNormal', () async {
    expect(await funcEnumWithItemMixedTwinNormal(arg: const EnumWithItemMixed.a()), const EnumWithItemMixed.a());
    expect(await funcEnumWithItemMixedTwinNormal(arg: EnumWithItemMixed.b(Uint8List.fromList([42]))),
        EnumWithItemMixed.b(Uint8List.fromList([42])));
    expect(await funcEnumWithItemMixedTwinNormal(arg: EnumWithItemMixed.c(cField: 'hi')),
        EnumWithItemMixed.c(cField: 'hi'));
  });

  // TODO
}
