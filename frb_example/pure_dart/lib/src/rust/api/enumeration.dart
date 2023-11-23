// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

part 'enumeration.freezed.dart';

Future<EnumSimple> funcEnumSimpleTwinNormal(
        {required EnumSimple arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumSimpleTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemMixed> funcEnumWithItemMixedTwinNormal(
        {required EnumWithItemMixed arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemMixedTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemStruct> funcEnumWithItemStructTwinNormal(
        {required EnumWithItemStruct arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemStructTwinNormal(arg: arg, hint: hint);

Future<EnumWithItemTuple> funcEnumWithItemTupleTwinNormal(
        {required EnumWithItemTuple arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemTupleTwinNormal(arg: arg, hint: hint);

enum EnumSimple {
  A,
  B,
}

@freezed
sealed class EnumWithItemMixed with _$EnumWithItemMixed {
  const factory EnumWithItemMixed.a() = EnumWithItemMixed_A;
  const factory EnumWithItemMixed.b(
    Uint8List field0,
  ) = EnumWithItemMixed_B;
  const factory EnumWithItemMixed.c({
    required String cField,
  }) = EnumWithItemMixed_C;
}

@freezed
sealed class EnumWithItemStruct with _$EnumWithItemStruct {
  const factory EnumWithItemStruct.a({
    required Uint8List aField,
  }) = EnumWithItemStruct_A;
  const factory EnumWithItemStruct.b({
    required Int32List bField,
  }) = EnumWithItemStruct_B;
}

@freezed
sealed class EnumWithItemTuple with _$EnumWithItemTuple {
  const factory EnumWithItemTuple.a(
    Uint8List field0,
  ) = EnumWithItemTuple_A;
  const factory EnumWithItemTuple.b(
    Int32List field0,
  ) = EnumWithItemTuple_B;
}
