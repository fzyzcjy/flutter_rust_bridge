// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/enumeration_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_example_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcEnumSimpleTwinRustAsyncSse, [
    EnumSimpleTwinRustAsyncSse.a,
    EnumSimpleTwinRustAsyncSse.b,
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemMixedTwinRustAsyncSse, [
    EnumWithItemMixedTwinRustAsyncSse.a(),
    EnumWithItemMixedTwinRustAsyncSse.b(Uint8List.fromList([42])),
    EnumWithItemMixedTwinRustAsyncSse.c(cField: 'hi'),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemStructTwinRustAsyncSse, [
    EnumWithItemStructTwinRustAsyncSse.a(aField: Uint8List.fromList([42])),
    EnumWithItemStructTwinRustAsyncSse.b(bField: Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithItemTupleTwinRustAsyncSse, [
    EnumWithItemTupleTwinRustAsyncSse.a(Uint8List.fromList([42])),
    EnumWithItemTupleTwinRustAsyncSse.b(Int32List.fromList([200])),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinRustAsyncSse, [
    EnumWithDiscriminantTwinRustAsyncSse.oneHundred,
    EnumWithDiscriminantTwinRustAsyncSse.fifty,
  ]);

  group('example-based tests', () {
    test('dart call handleReturnEnum', () async {
      expect(await handleReturnEnumTwinRustAsyncSse(input: "Tuesday"),
          WeekdaysTwinRustAsyncSse.tuesday);
      expect(await handleReturnEnumTwinRustAsyncSse(input: "Foreverday"), null);
    });

    test('dart call handleEnumParameter', () async {
      expect(
          await handleEnumParameterTwinRustAsyncSse(
              weekday: WeekdaysTwinRustAsyncSse.saturday),
          WeekdaysTwinRustAsyncSse.saturday);
    });

    test('dart call handleEnumStruct', () async {
      expect(
          await handleEnumStructTwinRustAsyncSse(
              val: KitchenSinkTwinRustAsyncSse_Empty()),
          KitchenSinkTwinRustAsyncSse_Empty());
      expect(
        await handleEnumStructTwinRustAsyncSse(
          val: KitchenSinkTwinRustAsyncSse_Primitives(
              int32: 0, float64: 1, boolean: false),
        ),
        KitchenSinkTwinRustAsyncSse_Primitives(
            int32: 1, float64: 2, boolean: true),
      );
      expect(
        await handleEnumStructTwinRustAsyncSse(
            val: KitchenSinkTwinRustAsyncSse_Optional(null, 0)),
        KitchenSinkTwinRustAsyncSse_Optional(null, 1),
      );
      expect(
        await handleEnumStructTwinRustAsyncSse(
            val: KitchenSinkTwinRustAsyncSse_Buffer(Uint8List.fromList([]))),
        KitchenSinkTwinRustAsyncSse_Buffer(Uint8List.fromList([1])),
      );
      expect(
        await handleEnumStructTwinRustAsyncSse(
            val: KitchenSinkTwinRustAsyncSse_Enums(
                WeekdaysTwinRustAsyncSse.monday)),
        KitchenSinkTwinRustAsyncSse_Enums(WeekdaysTwinRustAsyncSse.tuesday),
      );
      expect(
        await handleEnumStructTwinRustAsyncSse(
            val: const KitchenSinkTwinRustAsyncSse.nested(
                0, KitchenSinkTwinRustAsyncSse.empty())),
        const KitchenSinkTwinRustAsyncSse.nested(
            1, KitchenSinkTwinRustAsyncSse.empty()),
      );
    });

    test('dart call multiplyByTen()', () async {
      expect(
        await multiplyByTenTwinRustAsyncSse(
            measure:
                MeasureTwinRustAsyncSse.speed(SpeedTwinRustAsyncSse_GPS(10.0))),
        MeasureTwinRustAsyncSse.speed(SpeedTwinRustAsyncSse_GPS(100.0)),
      );
      expect(
        await multiplyByTenTwinRustAsyncSse(
            measure:
                MeasureTwinRustAsyncSse.speed(SpeedTwinRustAsyncSse_Unknown())),
        null,
      );
      final skipMinified =
          releaseMode ? skipWeb('Minified names cannot be compared.') : null;
      expect((SpeedTwinRustAsyncSse_Unknown).toString(),
          'SpeedTwinRustAsyncSse_Unknown',
          skip: skipMinified);
      expect(
          (SpeedTwinRustAsyncSse_GPS).toString(), 'SpeedTwinRustAsyncSse_GPS',
          skip: skipMinified);
      expect((DistanceTwinRustAsyncSse_Unknown).toString(),
          'DistanceTwinRustAsyncSse_Unknown',
          skip: skipMinified);
      expect((DistanceTwinRustAsyncSse_Map).toString(),
          'DistanceTwinRustAsyncSse_Map',
          skip: skipMinified);
    });
  });
}
