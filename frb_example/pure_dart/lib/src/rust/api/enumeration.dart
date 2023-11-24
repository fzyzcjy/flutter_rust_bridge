// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'misc_example.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration.freezed.dart';

Future<EnumSimpleTwinNormal> funcEnumSimpleTwinNormal(
        {required EnumSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumSimpleTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemMixedTwinNormal> funcEnumWithItemMixedTwinNormal(
        {required EnumWithItemMixedTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemMixedTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemTupleTwinNormal> funcEnumWithItemTupleTwinNormal(
        {required EnumWithItemTupleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemTupleTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemStructTwinNormal> funcEnumWithItemStructTwinNormal(
        {required EnumWithItemStructTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemStructTwinNormal(arg: arg, hint: hint);

Future<Uint8List> printNote({required Note note, dynamic hint}) =>
    RustLib.instance.api.printNote(note: note, hint: hint);

Future<Weekdays?> handleReturnEnum({required String input, dynamic hint}) =>
    RustLib.instance.api.handleReturnEnum(input: input, hint: hint);

Future<Weekdays> handleEnumParameter(
        {required Weekdays weekday, dynamic hint}) =>
    RustLib.instance.api.handleEnumParameter(weekday: weekday, hint: hint);

Future<Measure?> multiplyByTen({required Measure measure, dynamic hint}) =>
    RustLib.instance.api.multiplyByTen(measure: measure, hint: hint);

Future<KitchenSink> handleEnumStruct(
        {required KitchenSink val, dynamic hint}) =>
    RustLib.instance.api.handleEnumStruct(val: val, hint: hint);

@freezed
sealed class Distance with _$Distance {
  const factory Distance.unknown() = Distance_Unknown;
  const factory Distance.map(
    double field0,
  ) = Distance_Map;
}

enum EnumSimpleTwinNormal {
  a,
  b,
}

@freezed
sealed class EnumWithItemMixedTwinNormal with _$EnumWithItemMixedTwinNormal {
  const factory EnumWithItemMixedTwinNormal.a() = EnumWithItemMixedTwinNormal_A;
  const factory EnumWithItemMixedTwinNormal.b(
    Uint8List field0,
  ) = EnumWithItemMixedTwinNormal_B;
  const factory EnumWithItemMixedTwinNormal.c({
    required String cField,
  }) = EnumWithItemMixedTwinNormal_C;
}

@freezed
sealed class EnumWithItemStructTwinNormal with _$EnumWithItemStructTwinNormal {
  const factory EnumWithItemStructTwinNormal.a({
    required Uint8List aField,
  }) = EnumWithItemStructTwinNormal_A;
  const factory EnumWithItemStructTwinNormal.b({
    required Int32List bField,
  }) = EnumWithItemStructTwinNormal_B;
}

@freezed
sealed class EnumWithItemTupleTwinNormal with _$EnumWithItemTupleTwinNormal {
  const factory EnumWithItemTupleTwinNormal.a(
    Uint8List field0,
  ) = EnumWithItemTupleTwinNormal_A;
  const factory EnumWithItemTupleTwinNormal.b(
    Int32List field0,
  ) = EnumWithItemTupleTwinNormal_B;
}

@freezed
sealed class KitchenSink with _$KitchenSink {
  /// Comment on variant
  const factory KitchenSink.empty() = KitchenSink_Empty;
  const factory KitchenSink.primitives({
    /// Dart field comment
    @Default(-1) int int32,
    required double float64,
    required bool boolean,
  }) = KitchenSink_Primitives;
  const factory KitchenSink.nested(
    int field0, [
    @Default(KitchenSink.empty()) KitchenSink field1,
  ]) = KitchenSink_Nested;
  const factory KitchenSink.optional([
    /// Comment on anonymous field
    @Default(-1) int? field0,
    int? field1,
  ]) = KitchenSink_Optional;
  const factory KitchenSink.buffer(
    Uint8List field0,
  ) = KitchenSink_Buffer;
  const factory KitchenSink.enums([
    @Default(Weekdays.sunday) Weekdays field0,
  ]) = KitchenSink_Enums;
}

@freezed
sealed class Measure with _$Measure {
  const factory Measure.speed(
    Speed field0,
  ) = Measure_Speed;
  const factory Measure.distance(
    Distance field0,
  ) = Measure_Distance;
}

class Note {
  final Weekdays day;
  final String body;

  const Note({
    this.day = Weekdays.sunday,
    required this.body,
  });

  @override
  int get hashCode => day.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Note &&
          runtimeType == other.runtimeType &&
          day == other.day &&
          body == other.body;
}

@freezed
sealed class Speed with _$Speed {
  const factory Speed.unknown() = Speed_Unknown;
  const factory Speed.gps(
    double field0,
  ) = Speed_GPS;
}
