import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
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
    EnumWithItemTupleTwinNormal.b(200),
  ]);

  addTestsIdentityFunctionCall(funcEnumWithDiscriminantTwinNormal, [
    EnumWithDiscriminantTwinNormal.oneHundred,
    EnumWithDiscriminantTwinNormal.fifty,
  ]);

  addTestsIdentityFunctionCall(funcChangeTwinNormal, [
    ChangeStringTwinNormal_Created(data: 'test'),
    ChangeStringTwinNormal_Updated(id: 'id1', data: 'test'),
    ChangeStringTwinNormal_Deleted(id: 'id2'),
  ]);

  addTestsIdentityFunctionCall(funcChangeMapTwinNormal, [
    ChangeMapTwinNormal_Created(data: {'key1': 'value1'}),
    ChangeMapTwinNormal_Updated(id: 'id1', data: {'key1': 'value1', 'key2': 'value2'}),
    ChangeMapTwinNormal_Deleted(id: 'id2'),
  ]);

  // Test case for bug fix: generic enum with ignored template and concrete type aliases
  // This verifies that the generated code uses the concrete type alias names (BlockChangeTwinNormal, MapChangeTwinNormal)
  // instead of the generic template name (ChangeIgnoredTwinNormal) in IntoDart/IntoIntoDart implementations
  addTestsIdentityFunctionCall(funcBlockChangeTwinNormal, [
    BlockChangeTwinNormal_Created(data: 'test'),
    BlockChangeTwinNormal_Updated(id: 'id1', data: 'test'),
    BlockChangeTwinNormal_Deleted(id: 'id2'),
  ]);

  addTestsIdentityFunctionCall(funcMapChangeTwinNormal, [
    MapChangeTwinNormal_Created(data: {'key1': 'value1'}),
    MapChangeTwinNormal_Updated(id: 'id1', data: {'key1': 'value1', 'key2': 'value2'}),
    MapChangeTwinNormal_Deleted(id: 'id2'),
  ]);

  addTestsIdentityFunctionCall(funcBatchTwinNormalChangeMapTwinNormal, [
    BatchTwinNormalChangeMapTwinNormal(items: [
      ChangeIgnoredTwinNormal_Map_String_String.created(data: {'k': 'v'}),
    ]),
    BatchTwinNormalChangeMapTwinNormal(items: [
      ChangeIgnoredTwinNormal_Map_String_String.updated(id: 'id1', data: {'a': 'b'}),
      ChangeIgnoredTwinNormal_Map_String_String.deleted(id: 'id2'),
    ]),
  ]);

  addTestsIdentityFunctionCall(funcAnotherBatchTwinNormalChangeMapTwinNormal, [
    AnotherBatchTwinNormalChangeMapTwinNormal(items: ChangeIgnoredTwinNormal_Map_String_String.created(data: {'k': 'v'})),
    AnotherBatchTwinNormalChangeMapTwinNormal(items: ChangeIgnoredTwinNormal_Map_String_String.deleted(id: 'id1')),
  ]);

  // Test nested generic enum inside StreamSink - verifies auto-naming works for nested generics
  // This tests that Change<HashMap<String, String>> inside StreamSink is correctly instantiated
  test('nested generic enum in StreamSink', () async {
    final sink1 = RustStreamSink<ChangeMapTwinNormal>();
    final sink2 = await funcChangeMapSinkTwinNormal(arg: sink1);

    // Verify that the sink is returned correctly (identity function)
    expect(sink2, isNotNull);

    // Add data to verify the enum type is correct
    sink2.add(ChangeMapTwinNormal_Created(data: {'test': 'value'}));
    sink2.close();

    // Verify we can receive the data
    final received = await sink2.stream.first;
    expect(received, isA<ChangeMapTwinNormal_Created>());
    if (received is ChangeMapTwinNormal_Created) {
      expect(received.data, {'test': 'value'});
    }
  });

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
