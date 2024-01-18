// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:typed_data';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/enumeration_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/misc_example_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinSync, [
    EnumSimpleTwinSync.a,
    EnumSimpleTwinSync.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinSync, [
    EnumWithItemMixedTwinSync.a(),
    EnumWithItemMixedTwinSync.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinSync.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinSync, [
    EnumWithItemStructTwinSync.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinSync.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinSync, [
    EnumWithItemTupleTwinSync.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinSync.b(Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinSync, [
    EnumWithDiscriminantTwinSync.oneHundred,
    EnumWithDiscriminantTwinSync.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinSync(input: "Tuesday"),
          WeekdaysTwinSync.tuesday);
      expect(await handleReturnEnumTwinSync(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinSync(weekday: WeekdaysTwinSync.saturday),
          WeekdaysTwinSync.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(await handleEnumStructTwinSync(val: KitchenSinkTwinSync_Empty()),
          KitchenSinkTwinSync_Empty());
      expect(
        await handleEnumStructTwinSync(
          val: KitchenSinkTwinSync_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinSync_Primitives(int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinSync(
            val: KitchenSinkTwinSync_Optional(null, 0)),
        KitchenSinkTwinSync_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinSync(
            val: KitchenSinkTwinSync_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinSync_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinSync(
            val: KitchenSinkTwinSync_Enums(WeekdaysTwinSync.monday)),
        KitchenSinkTwinSync_Enums(WeekdaysTwinSync.tuesday),
      );
      expect(
        await handleEnumStructTwinSync(
            val: const KitchenSinkTwinSync.nested(
                0, KitchenSinkTwinSync.empty())),
        const KitchenSinkTwinSync.nested(1, KitchenSinkTwinSync.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinSync(
            measure: MeasureTwinSync.speed(SpeedTwinSync_GPS(10.0))),
        MeasureTwinSync.speed(SpeedTwinSync_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinSync(
            measure: MeasureTwinSync.speed(SpeedTwinSync_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect((SpeedTwinSync_Unknown).toString(), 'SpeedTwinSync_Unknown',
          skip: skipMinified);
      expect((SpeedTwinSync_GPS).toString(), 'SpeedTwinSync_GPS',
          skip: skipMinified);
      expect((DistanceTwinSync_Unknown).toString(), 'DistanceTwinSync_Unknown',
          skip: skipMinified);
      expect((DistanceTwinSync_Map).toString(), 'DistanceTwinSync_Map',
          skip: skipMinified);
    });
  });
}
