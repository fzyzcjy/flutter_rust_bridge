// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/enumeration_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_example_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinSse, [
    EnumSimpleTwinSse.a,
    EnumSimpleTwinSse.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinSse, [
    EnumWithItemMixedTwinSse.a(),
    EnumWithItemMixedTwinSse.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinSse.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinSse, [
    EnumWithItemStructTwinSse.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinSse.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinSse, [
    EnumWithItemTupleTwinSse.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinSse.b(Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinSse, [
    EnumWithDiscriminantTwinSse.oneHundred,
    EnumWithDiscriminantTwinSse.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinSse(input: "Tuesday"),
          WeekdaysTwinSse.tuesday);
      expect(await handleReturnEnumTwinSse(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinSse(weekday: WeekdaysTwinSse.saturday),
          WeekdaysTwinSse.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(await handleEnumStructTwinSse(val: KitchenSinkTwinSse_Empty()),
          KitchenSinkTwinSse_Empty());
      expect(
        await handleEnumStructTwinSse(
          val: KitchenSinkTwinSse_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinSse_Primitives(int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinSse(
            val: KitchenSinkTwinSse_Optional(null, 0)),
        KitchenSinkTwinSse_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinSse(
            val: KitchenSinkTwinSse_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinSse_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinSse(
            val: KitchenSinkTwinSse_Enums(WeekdaysTwinSse.monday)),
        KitchenSinkTwinSse_Enums(WeekdaysTwinSse.tuesday),
      );
      expect(
        await handleEnumStructTwinSse(
            val:
                const KitchenSinkTwinSse.nested(0, KitchenSinkTwinSse.empty())),
        const KitchenSinkTwinSse.nested(1, KitchenSinkTwinSse.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinSse(
            measure: MeasureTwinSse.speed(SpeedTwinSse_GPS(10.0))),
        MeasureTwinSse.speed(SpeedTwinSse_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinSse(
            measure: MeasureTwinSse.speed(SpeedTwinSse_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect((SpeedTwinSse_Unknown).toString(), 'SpeedTwinSse_Unknown',
          skip: skipMinified);
      expect((SpeedTwinSse_GPS).toString(), 'SpeedTwinSse_GPS',
          skip: skipMinified);
      expect((DistanceTwinSse_Unknown).toString(), 'DistanceTwinSse_Unknown',
          skip: skipMinified);
      expect((DistanceTwinSse_Map).toString(), 'DistanceTwinSse_Map',
          skip: skipMinified);
    });
  });
}
