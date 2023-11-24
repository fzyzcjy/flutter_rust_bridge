import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/api/misc_example.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinNormal, [
    EnumSimpleTwinNormal.a,
    EnumSimpleTwinNormal.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinNormal, [
    EnumWithItemMixedTwinNormal.a(),
    EnumWithItemMixedTwinNormal.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinNormal.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinNormal, [
    EnumWithItemStructTwinNormal.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinNormal.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinNormal, [
    EnumWithItemTupleTwinNormal.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinNormal.b(Int32List.fromList([200])),
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnum(input: "Tuesday"), Weekdays.tuesday);
      expect(await handleReturnEnum(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(await handleEnumParameter(weekday: Weekdays.saturday), Weekdays.saturday);
    });

    test('dart call handleEnumParameter', () async {
      expect(handleEnumSyncFreezed(value: MyEnumFreezed.a(1)), MyEnumFreezed.b('hello'));
    });

    test('dart call handleEnumStruct', () async {
      expect(await handleEnumStruct(val: KitchenSink_Empty()), KitchenSink_Empty());
      expect(
        await handleEnumStruct(
          val: KitchenSink_Primitives(int32: 0, float64: 1, boolean: false),
        ),
        KitchenSink_Primitives(int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStruct(val: KitchenSink_Optional(null, 0)),
        KitchenSink_Optional(null, 1),
      );
      expect(
        await handleEnumStruct(val: KitchenSink_Buffer(Uint8List.fromList([]))),
        KitchenSink_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStruct(val: KitchenSink_Enums(Weekdays.monday)),
        KitchenSink_Enums(Weekdays.tuesday),
      );
      expect(
        await handleEnumStruct(val: const KitchenSink.nested(0, KitchenSink.empty())),
        const KitchenSink.nested(1, KitchenSink.empty()),
      );
    });
  });
}
