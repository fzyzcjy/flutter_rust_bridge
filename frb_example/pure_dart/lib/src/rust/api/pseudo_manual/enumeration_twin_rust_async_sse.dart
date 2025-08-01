// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'misc_example_twin_rust_async_sse.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration_twin_rust_async_sse.freezed.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `clone`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`

Future<EnumSimpleTwinRustAsyncSse> funcEnumSimpleTwinRustAsyncSse(
        {required EnumSimpleTwinRustAsyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseFuncEnumSimpleTwinRustAsyncSse(
            arg: arg);

Future<EnumWithItemMixedTwinRustAsyncSse> funcEnumWithItemMixedTwinRustAsyncSse(
        {required EnumWithItemMixedTwinRustAsyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseFuncEnumWithItemMixedTwinRustAsyncSse(
            arg: arg);

Future<EnumWithItemTupleTwinRustAsyncSse> funcEnumWithItemTupleTwinRustAsyncSse(
        {required EnumWithItemTupleTwinRustAsyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseFuncEnumWithItemTupleTwinRustAsyncSse(
            arg: arg);

Future<
    EnumWithItemStructTwinRustAsyncSse> funcEnumWithItemStructTwinRustAsyncSse(
        {required EnumWithItemStructTwinRustAsyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseFuncEnumWithItemStructTwinRustAsyncSse(
            arg: arg);

Future<
    EnumWithDiscriminantTwinRustAsyncSse> funcEnumWithDiscriminantTwinRustAsyncSse(
        {required EnumWithDiscriminantTwinRustAsyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseFuncEnumWithDiscriminantTwinRustAsyncSse(
            arg: arg);

Future<Uint8List> printNoteTwinRustAsyncSse(
        {required NoteTwinRustAsyncSse note}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSsePrintNoteTwinRustAsyncSse(
            note: note);

Future<WeekdaysTwinRustAsyncSse?> handleReturnEnumTwinRustAsyncSse(
        {required String input}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseHandleReturnEnumTwinRustAsyncSse(
            input: input);

Future<WeekdaysTwinRustAsyncSse> handleEnumParameterTwinRustAsyncSse(
        {required WeekdaysTwinRustAsyncSse weekday}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseHandleEnumParameterTwinRustAsyncSse(
            weekday: weekday);

Future<MeasureTwinRustAsyncSse?> multiplyByTenTwinRustAsyncSse(
        {required MeasureTwinRustAsyncSse measure}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseMultiplyByTenTwinRustAsyncSse(
            measure: measure);

Future<KitchenSinkTwinRustAsyncSse> handleEnumStructTwinRustAsyncSse(
        {required KitchenSinkTwinRustAsyncSse val}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinRustAsyncSseHandleEnumStructTwinRustAsyncSse(
            val: val);

@freezed
sealed class DistanceTwinRustAsyncSse with _$DistanceTwinRustAsyncSse {
  const DistanceTwinRustAsyncSse._();

  const factory DistanceTwinRustAsyncSse.unknown() =
      DistanceTwinRustAsyncSse_Unknown;
  const factory DistanceTwinRustAsyncSse.map(
    double field0,
  ) = DistanceTwinRustAsyncSse_Map;
}

enum EnumSimpleTwinRustAsyncSse {
  a,
  b,
  ;
}

enum EnumWithDiscriminantTwinRustAsyncSse {
  oneHundred,
  fifty,
  ;
}

@freezed
sealed class EnumWithItemMixedTwinRustAsyncSse
    with _$EnumWithItemMixedTwinRustAsyncSse {
  const EnumWithItemMixedTwinRustAsyncSse._();

  const factory EnumWithItemMixedTwinRustAsyncSse.a() =
      EnumWithItemMixedTwinRustAsyncSse_A;
  const factory EnumWithItemMixedTwinRustAsyncSse.b(
    Uint8List field0,
  ) = EnumWithItemMixedTwinRustAsyncSse_B;
  const factory EnumWithItemMixedTwinRustAsyncSse.c({
    required String cField,
  }) = EnumWithItemMixedTwinRustAsyncSse_C;
}

@freezed
sealed class EnumWithItemStructTwinRustAsyncSse
    with _$EnumWithItemStructTwinRustAsyncSse {
  const EnumWithItemStructTwinRustAsyncSse._();

  const factory EnumWithItemStructTwinRustAsyncSse.a({
    required Uint8List aField,
  }) = EnumWithItemStructTwinRustAsyncSse_A;
  const factory EnumWithItemStructTwinRustAsyncSse.b({
    required Int32List bField,
  }) = EnumWithItemStructTwinRustAsyncSse_B;
}

@freezed
sealed class EnumWithItemTupleTwinRustAsyncSse
    with _$EnumWithItemTupleTwinRustAsyncSse {
  const EnumWithItemTupleTwinRustAsyncSse._();

  const factory EnumWithItemTupleTwinRustAsyncSse.a(
    Uint8List field0,
  ) = EnumWithItemTupleTwinRustAsyncSse_A;
  const factory EnumWithItemTupleTwinRustAsyncSse.b(
    int field0,
  ) = EnumWithItemTupleTwinRustAsyncSse_B;
}

@freezed
sealed class KitchenSinkTwinRustAsyncSse with _$KitchenSinkTwinRustAsyncSse {
  const KitchenSinkTwinRustAsyncSse._();

  /// Comment on variant
  const factory KitchenSinkTwinRustAsyncSse.empty() =
      KitchenSinkTwinRustAsyncSse_Empty;
  const factory KitchenSinkTwinRustAsyncSse.primitives({
    /// Dart field comment
    @Default(-1) int int32,
    required double float64,
    required bool boolean,
  }) = KitchenSinkTwinRustAsyncSse_Primitives;
  const factory KitchenSinkTwinRustAsyncSse.nested(
    int field0, [
    @Default(KitchenSinkTwinRustAsyncSse.empty())
    KitchenSinkTwinRustAsyncSse field1,
  ]) = KitchenSinkTwinRustAsyncSse_Nested;
  const factory KitchenSinkTwinRustAsyncSse.optional([
    /// Comment on anonymous field
    @Default(-1) int? field0,
    int? field1,
  ]) = KitchenSinkTwinRustAsyncSse_Optional;
  const factory KitchenSinkTwinRustAsyncSse.buffer(
    Uint8List field0,
  ) = KitchenSinkTwinRustAsyncSse_Buffer;
  const factory KitchenSinkTwinRustAsyncSse.enums([
    @Default(WeekdaysTwinRustAsyncSse.sunday) WeekdaysTwinRustAsyncSse field0,
  ]) = KitchenSinkTwinRustAsyncSse_Enums;
}

@freezed
sealed class MeasureTwinRustAsyncSse with _$MeasureTwinRustAsyncSse {
  const MeasureTwinRustAsyncSse._();

  const factory MeasureTwinRustAsyncSse.speed(
    SpeedTwinRustAsyncSse field0,
  ) = MeasureTwinRustAsyncSse_Speed;
  const factory MeasureTwinRustAsyncSse.distance(
    DistanceTwinRustAsyncSse field0,
  ) = MeasureTwinRustAsyncSse_Distance;
}

class NoteTwinRustAsyncSse {
  final WeekdaysTwinRustAsyncSse day;
  final String body;

  const NoteTwinRustAsyncSse({
    this.day = WeekdaysTwinRustAsyncSse.sunday,
    required this.body,
  });

  @override
  int get hashCode => day.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          day == other.day &&
          body == other.body;
}

@freezed
sealed class SpeedTwinRustAsyncSse with _$SpeedTwinRustAsyncSse {
  const SpeedTwinRustAsyncSse._();

  const factory SpeedTwinRustAsyncSse.unknown() = SpeedTwinRustAsyncSse_Unknown;
  const factory SpeedTwinRustAsyncSse.gps(
    double field0,
  ) = SpeedTwinRustAsyncSse_GPS;
}
