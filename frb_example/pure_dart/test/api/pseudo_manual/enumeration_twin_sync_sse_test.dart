// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/enumeration_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_example_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinSyncSse, [
    EnumSimpleTwinSyncSse.a,
    EnumSimpleTwinSyncSse.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinSyncSse, [
    EnumWithItemMixedTwinSyncSse.a(),
    EnumWithItemMixedTwinSyncSse.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinSyncSse.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinSyncSse, [
    EnumWithItemStructTwinSyncSse.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinSyncSse.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinSyncSse, [
    EnumWithItemTupleTwinSyncSse.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinSyncSse.b(Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinSyncSse, [
    EnumWithDiscriminantTwinSyncSse.oneHundred,
    EnumWithDiscriminantTwinSyncSse.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinSyncSse(input: "Tuesday"),
          WeekdaysTwinSyncSse.tuesday);
      expect(await handleReturnEnumTwinSyncSse(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinSyncSse(
              weekday: WeekdaysTwinSyncSse.saturday),
          WeekdaysTwinSyncSse.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(
          await handleEnumStructTwinSyncSse(
              val: KitchenSinkTwinSyncSse_Empty()),
          KitchenSinkTwinSyncSse_Empty());
      expect(
        await handleEnumStructTwinSyncSse(
          val: KitchenSinkTwinSyncSse_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinSyncSse_Primitives(int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinSyncSse(
            val: KitchenSinkTwinSyncSse_Optional(null, 0)),
        KitchenSinkTwinSyncSse_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinSyncSse(
            val: KitchenSinkTwinSyncSse_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinSyncSse_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinSyncSse(
            val: KitchenSinkTwinSyncSse_Enums(WeekdaysTwinSyncSse.monday)),
        KitchenSinkTwinSyncSse_Enums(WeekdaysTwinSyncSse.tuesday),
      );
      expect(
        await handleEnumStructTwinSyncSse(
            val: const KitchenSinkTwinSyncSse.nested(
                0, KitchenSinkTwinSyncSse.empty())),
        const KitchenSinkTwinSyncSse.nested(1, KitchenSinkTwinSyncSse.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinSyncSse(
            measure: MeasureTwinSyncSse.speed(SpeedTwinSyncSse_GPS(10.0))),
        MeasureTwinSyncSse.speed(SpeedTwinSyncSse_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinSyncSse(
            measure: MeasureTwinSyncSse.speed(SpeedTwinSyncSse_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect((SpeedTwinSyncSse_Unknown).toString(), 'SpeedTwinSyncSse_Unknown',
          skip: skipMinified);
      expect((SpeedTwinSyncSse_GPS).toString(), 'SpeedTwinSyncSse_GPS',
          skip: skipMinified);
      expect((DistanceTwinSyncSse_Unknown).toString(),
          'DistanceTwinSyncSse_Unknown',
          skip: skipMinified);
      expect((DistanceTwinSyncSse_Map).toString(), 'DistanceTwinSyncSse_Map',
          skip: skipMinified);
    });
  });
}
