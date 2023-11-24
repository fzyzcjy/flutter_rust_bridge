// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration_twin_sync.freezed.dart';

EnumSimpleTwinSync funcEnumSimpleTwinSync(
        {required EnumSimpleTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumSimpleTwinSync(arg: arg, hint: hint);

EnumWithItemMixedTwinSync funcEnumWithItemMixedTwinSync(
        {required EnumWithItemMixedTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemMixedTwinSync(arg: arg, hint: hint);

EnumWithItemTupleTwinSync funcEnumWithItemTupleTwinSync(
        {required EnumWithItemTupleTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemTupleTwinSync(arg: arg, hint: hint);

EnumWithItemStructTwinSync funcEnumWithItemStructTwinSync(
        {required EnumWithItemStructTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcEnumWithItemStructTwinSync(arg: arg, hint: hint);

enum EnumSimpleTwinSync {
  a,
  b,
}

@freezed
sealed class EnumWithItemMixedTwinSync with _$EnumWithItemMixedTwinSync {
  const factory EnumWithItemMixedTwinSync.a() = EnumWithItemMixedTwinSync_A;
  const factory EnumWithItemMixedTwinSync.b(
    Uint8List field0,
  ) = EnumWithItemMixedTwinSync_B;
  const factory EnumWithItemMixedTwinSync.c({
    required String cField,
  }) = EnumWithItemMixedTwinSync_C;
}

@freezed
sealed class EnumWithItemStructTwinSync with _$EnumWithItemStructTwinSync {
  const factory EnumWithItemStructTwinSync.a({
    required Uint8List aField,
  }) = EnumWithItemStructTwinSync_A;
  const factory EnumWithItemStructTwinSync.b({
    required Int32List bField,
  }) = EnumWithItemStructTwinSync_B;
}

@freezed
sealed class EnumWithItemTupleTwinSync with _$EnumWithItemTupleTwinSync {
  const factory EnumWithItemTupleTwinSync.a(
    Uint8List field0,
  ) = EnumWithItemTupleTwinSync_A;
  const factory EnumWithItemTupleTwinSync.b(
    Int32List field0,
  ) = EnumWithItemTupleTwinSync_B;
}
