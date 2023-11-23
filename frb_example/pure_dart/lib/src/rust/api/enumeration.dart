// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration.freezed.dart';

Future<EnumSimpleTwinNormal> funcEnumSimpleTwinNormal(
        {required EnumSimpleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumSimpleTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemMixedTwinNormal> funcEnumWithItemMixedTwinNormal(
        {required EnumWithItemMixedTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemMixedTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemStructTwinNormal> funcEnumWithItemStructTwinNormal(
        {required EnumWithItemStructTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemStructTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemTupleTwinNormal> funcEnumWithItemTupleTwinNormal(
        {required EnumWithItemTupleTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemTupleTwinNormal(arg: arg, hint: hint);

enum EnumSimpleTwinNormal {
  A,
  B,
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
