import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/enumeration.dart';
import 'package:frb_example_pure_dart/src/rust/api/misc_example.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

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

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinNormal, [
    EnumWithDiscriminantTwinNormal.oneHundred,
    EnumWithDiscriminantTwinNormal.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinNormal(input: "Tuesday"),
          WeekdaysTwinNormal.tuesday);
      expect(await handleReturnEnumTwinNormal(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinNormal(
              weekday: WeekdaysTwinNormal.saturday),
          WeekdaysTwinNormal.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(
          await handleEnumStructTwinNormal(val: KitchenSinkTwinNormal_Empty()),
          KitchenSinkTwinNormal_Empty());
      expect(
        await handleEnumStructTwinNormal(
          val: KitchenSinkTwinNormal_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinNormal_Primitives(int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinNormal(
            val: KitchenSinkTwinNormal_Optional(null, 0)),
        KitchenSinkTwinNormal_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinNormal(
            val: KitchenSinkTwinNormal_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinNormal_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinNormal(
            val: KitchenSinkTwinNormal_Enums(WeekdaysTwinNormal.monday)),
        KitchenSinkTwinNormal_Enums(WeekdaysTwinNormal.tuesday),
      );
      expect(
        await handleEnumStructTwinNormal(
            val: const KitchenSinkTwinNormal.nested(
                0, KitchenSinkTwinNormal.empty())),
        const KitchenSinkTwinNormal.nested(1, KitchenSinkTwinNormal.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinNormal(
            measure: MeasureTwinNormal.speed(SpeedTwinNormal_GPS(10.0))),
        MeasureTwinNormal.speed(SpeedTwinNormal_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinNormal(
            measure: MeasureTwinNormal.speed(SpeedTwinNormal_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect((SpeedTwinNormal_Unknown).toString(), 'SpeedTwinNormal_Unknown',
          skip: skipMinified);
      expect((SpeedTwinNormal_GPS).toString(), 'SpeedTwinNormal_GPS',
          skip: skipMinified);
      expect(
          (DistanceTwinNormal_Unknown).toString(), 'DistanceTwinNormal_Unknown',
          skip: skipMinified);
      expect((DistanceTwinNormal_Map).toString(), 'DistanceTwinNormal_Map',
          skip: skipMinified);
    });
  });
}
