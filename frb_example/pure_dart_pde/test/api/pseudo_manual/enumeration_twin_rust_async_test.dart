// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:typed_data';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/enumeration_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/misc_example_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinRustAsync, [
    EnumSimpleTwinRustAsync.a,
    EnumSimpleTwinRustAsync.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinRustAsync, [
    EnumWithItemMixedTwinRustAsync.a(),
    EnumWithItemMixedTwinRustAsync.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinRustAsync.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinRustAsync, [
    EnumWithItemStructTwinRustAsync.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinRustAsync.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinRustAsync, [
    EnumWithItemTupleTwinRustAsync.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinRustAsync.b(Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinRustAsync, [
    EnumWithDiscriminantTwinRustAsync.oneHundred,
    EnumWithDiscriminantTwinRustAsync.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinRustAsync(input: "Tuesday"),
          WeekdaysTwinRustAsync.tuesday);
      expect(await handleReturnEnumTwinRustAsync(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinRustAsync(
              weekday: WeekdaysTwinRustAsync.saturday),
          WeekdaysTwinRustAsync.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(
          await handleEnumStructTwinRustAsync(
              val: KitchenSinkTwinRustAsync_Empty()),
          KitchenSinkTwinRustAsync_Empty());
      expect(
        await handleEnumStructTwinRustAsync(
          val: KitchenSinkTwinRustAsync_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinRustAsync_Primitives(
            int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinRustAsync(
            val: KitchenSinkTwinRustAsync_Optional(null, 0)),
        KitchenSinkTwinRustAsync_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinRustAsync(
            val: KitchenSinkTwinRustAsync_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinRustAsync_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinRustAsync(
            val: KitchenSinkTwinRustAsync_Enums(WeekdaysTwinRustAsync.monday)),
        KitchenSinkTwinRustAsync_Enums(WeekdaysTwinRustAsync.tuesday),
      );
      expect(
        await handleEnumStructTwinRustAsync(
            val: const KitchenSinkTwinRustAsync.nested(
                0, KitchenSinkTwinRustAsync.empty())),
        const KitchenSinkTwinRustAsync.nested(
            1, KitchenSinkTwinRustAsync.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinRustAsync(
            measure: MeasureTwinRustAsync.speed(SpeedTwinRustAsync_GPS(10.0))),
        MeasureTwinRustAsync.speed(SpeedTwinRustAsync_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinRustAsync(
            measure: MeasureTwinRustAsync.speed(SpeedTwinRustAsync_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect(
          (SpeedTwinRustAsync_Unknown).toString(), 'SpeedTwinRustAsync_Unknown',
          skip: skipMinified);
      expect((SpeedTwinRustAsync_GPS).toString(), 'SpeedTwinRustAsync_GPS',
          skip: skipMinified);
      expect((DistanceTwinRustAsync_Unknown).toString(),
          'DistanceTwinRustAsync_Unknown',
          skip: skipMinified);
      expect(
          (DistanceTwinRustAsync_Map).toString(), 'DistanceTwinRustAsync_Map',
          skip: skipMinified);
    });
  });
}
