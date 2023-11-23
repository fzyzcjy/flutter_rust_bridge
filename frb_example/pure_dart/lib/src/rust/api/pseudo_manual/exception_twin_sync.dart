// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception_twin_sync.freezed.dart';

int funcReturnErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinSync(hint: hint);

int funcTypeFalliblePanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcTypeFalliblePanicTwinSync(hint: hint);

int funcTypeInfalliblePanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcTypeInfalliblePanicTwinSync(hint: hint);

int customEnumErrorReturnOkTwinSync({required int arg, dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnOkTwinSync(arg: arg, hint: hint);

void customEnumErrorPanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorPanicTwinSync(hint: hint);

int customEnumErrorReturnErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnErrorTwinSync(hint: hint);

void customNestedErrorReturnErrorTwinSync(
        {required CustomNestedErrorOuterTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .customNestedErrorReturnErrorTwinSync(arg: arg, hint: hint);

void customStructErrorReturnErrorTwinSync(
        {required CustomStructErrorTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .customStructErrorReturnErrorTwinSync(arg: arg, hint: hint);

@freezed
sealed class CustomEnumErrorTwinSync
    with _$CustomEnumErrorTwinSync
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSync.one({
    required String message,
    required String backtrace,
  }) = CustomEnumErrorTwinSync_One;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSync.two({
    required int message,
    required String backtrace,
  }) = CustomEnumErrorTwinSync_Two;
}

@freezed
sealed class CustomNestedErrorInnerTwinSync
    with _$CustomNestedErrorInnerTwinSync {
  const factory CustomNestedErrorInnerTwinSync.three(
    String field0,
  ) = CustomNestedErrorInnerTwinSync_Three;
  const factory CustomNestedErrorInnerTwinSync.four(
    int field0,
  ) = CustomNestedErrorInnerTwinSync_Four;
}

@freezed
sealed class CustomNestedErrorOuterTwinSync
    with _$CustomNestedErrorOuterTwinSync {
  const factory CustomNestedErrorOuterTwinSync.one(
    String field0,
  ) = CustomNestedErrorOuterTwinSync_One;
  const factory CustomNestedErrorOuterTwinSync.two(
    CustomNestedErrorInnerTwinSync field0,
  ) = CustomNestedErrorOuterTwinSync_Two;
}

class CustomStructErrorTwinSync {
  final String a;

  const CustomStructErrorTwinSync({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorTwinSync &&
          runtimeType == other.runtimeType &&
          a == other.a;
}
