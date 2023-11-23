// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception.freezed.dart';

Future<void> customEnumErrorPanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorPanicTwinNormal(hint: hint);

Future<int> customEnumErrorReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnErrorTwinNormal(hint: hint);

Future<int> customEnumErrorReturnOkTwinNormal({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnOkTwinNormal(hint: hint);

Future<void> customNestedErrorReturnErrorTwinNormal(
        {required CustomNestedErrorOuterTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .customNestedErrorReturnErrorTwinNormal(arg: arg, hint: hint);

Future<void> customStructErrorReturnErrorTwinNormal(
        {required CustomStructErrorTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .customStructErrorReturnErrorTwinNormal(arg: arg, hint: hint);

Future<int> funcReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinNormal(hint: hint);

Future<int> funcTypeFalliblePanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcTypeFalliblePanicTwinNormal(hint: hint);

Future<int> funcTypeInfalliblePanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcTypeInfalliblePanicTwinNormal(hint: hint);

@freezed
sealed class CustomEnumErrorTwinNormal
    with _$CustomEnumErrorTwinNormal
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.error0({
    required String e,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_Error0;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.error1({
    required int e,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_Error1;
}

@freezed
sealed class CustomNestedErrorInnerTwinNormal
    with _$CustomNestedErrorInnerTwinNormal {
  const factory CustomNestedErrorInnerTwinNormal.three(
    String field0,
  ) = CustomNestedErrorInnerTwinNormal_Three;
  const factory CustomNestedErrorInnerTwinNormal.four(
    int field0,
  ) = CustomNestedErrorInnerTwinNormal_Four;
}

@freezed
sealed class CustomNestedErrorOuterTwinNormal
    with _$CustomNestedErrorOuterTwinNormal {
  const factory CustomNestedErrorOuterTwinNormal.one(
    String field0,
  ) = CustomNestedErrorOuterTwinNormal_One;
  const factory CustomNestedErrorOuterTwinNormal.two(
    CustomNestedErrorInnerTwinNormal field0,
  ) = CustomNestedErrorOuterTwinNormal_Two;
}

class CustomStructErrorTwinNormal {
  final String a;

  const CustomStructErrorTwinNormal({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorTwinNormal &&
          runtimeType == other.runtimeType &&
          a == other.a;
}
