// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception.freezed.dart';

Future<int> funcReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinNormal(hint: hint);

Future<int> funcTypeFalliblePanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcTypeFalliblePanicTwinNormal(hint: hint);

Future<int> funcTypeInfalliblePanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcTypeInfalliblePanicTwinNormal(hint: hint);

Future<int> customEnumErrorReturnOkTwinNormal(
        {required int arg, dynamic hint}) =>
    RustLib.instance.api
        .customEnumErrorReturnOkTwinNormal(arg: arg, hint: hint);

Future<void> customEnumErrorPanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorPanicTwinNormal(hint: hint);

Future<int> customEnumErrorReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnErrorTwinNormal(hint: hint);

Future<void> customNestedErrorReturnErrorTwinNormal(
        {required CustomNestedErrorOuterTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .customNestedErrorReturnErrorTwinNormal(arg: arg, hint: hint);

Future<void> customStructErrorReturnErrorTwinNormal(
        {required CustomStructErrorTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .customStructErrorReturnErrorTwinNormal(arg: arg, hint: hint);

Future<int> returnErrCustomError({dynamic hint}) =>
    RustLib.instance.api.returnErrCustomError(hint: hint);

Future<int> returnOkCustomError({dynamic hint}) =>
    RustLib.instance.api.returnOkCustomError(hint: hint);

Future<int> returnErrorVariant({required int variant, dynamic hint}) =>
    RustLib.instance.api.returnErrorVariant(variant: variant, hint: hint);

Future<void> returnCustomNestedError1({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1(hint: hint);

Future<void> returnCustomNestedError1Variant1({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1Variant1(hint: hint);

Future<void> returnCustomNestedError2({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError2(hint: hint);

Future<void> returnCustomStructError({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructError(hint: hint);

void syncReturnCustomStructError({dynamic hint}) =>
    RustLib.instance.api.syncReturnCustomStructError(hint: hint);

Future<int> returnCustomStructOk({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructOk(hint: hint);

Future<void> throwAnyhow({dynamic hint}) =>
    RustLib.instance.api.throwAnyhow(hint: hint);

Future<void> panicWithCustomResult({dynamic hint}) =>
    RustLib.instance.api.panicWithCustomResult(hint: hint);

Stream<String> streamSinkThrowAnyhow({dynamic hint}) =>
    RustLib.instance.api.streamSinkThrowAnyhow(hint: hint);

@freezed
sealed class CustomEnumErrorTwinNormal
    with _$CustomEnumErrorTwinNormal
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.one({
    required String message,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_One;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.two({
    required int message,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_Two;
}

@freezed
sealed class CustomError with _$CustomError implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomError.error0({
    required String e,
    required String backtrace,
  }) = CustomError_Error0;
  @Implements<FrbBacktracedException>()
  const factory CustomError.error1({
    required int e,
    required String backtrace,
  }) = CustomError_Error1;
}

@freezed
sealed class CustomNestedError1
    with _$CustomNestedError1
    implements FrbException {
  const factory CustomNestedError1.customNested1(
    String field0,
  ) = CustomNestedError1_CustomNested1;
  const factory CustomNestedError1.errorNested(
    CustomNestedError2 field0,
  ) = CustomNestedError1_ErrorNested;
}

@freezed
sealed class CustomNestedError2 with _$CustomNestedError2 {
  const factory CustomNestedError2.customNested2(
    String field0,
  ) = CustomNestedError2_CustomNested2;
  const factory CustomNestedError2.customNested2Number(
    int field0,
  ) = CustomNestedError2_CustomNested2Number;
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

class CustomStruct {
  final String message;

  const CustomStruct({
    required this.message,
  });

  static Future<CustomStruct> newCustomStruct(
          {required String message, dynamic hint}) =>
      RustLib.instance.api.customStructNew(message: message, hint: hint);

  Future<void> nonstaticReturnCustomStructError({dynamic hint}) =>
      RustLib.instance.api.customStructNonstaticReturnCustomStructError(
        that: this,
      );

  Future<int> nonstaticReturnCustomStructOk({dynamic hint}) =>
      RustLib.instance.api.customStructNonstaticReturnCustomStructOk(
        that: this,
      );

  static Future<void> staticReturnCustomStructError({dynamic hint}) =>
      RustLib.instance.api
          .customStructStaticReturnCustomStructError(hint: hint);

  static Future<int> staticReturnCustomStructOk({dynamic hint}) =>
      RustLib.instance.api.customStructStaticReturnCustomStructOk(hint: hint);

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStruct &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class CustomStructError implements FrbException {
  final String message;

  const CustomStructError({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructError &&
          runtimeType == other.runtimeType &&
          message == other.message;
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

class SomeStruct {
  final int value;

  const SomeStruct({
    required this.value,
  });

  static Future<SomeStruct> newSomeStruct({required int value, dynamic hint}) =>
      RustLib.instance.api.someStructNew(value: value, hint: hint);

  Future<int> nonStaticReturnErrCustomError({dynamic hint}) =>
      RustLib.instance.api.someStructNonStaticReturnErrCustomError(
        that: this,
      );

  Future<int> nonStaticReturnOkCustomError({dynamic hint}) =>
      RustLib.instance.api.someStructNonStaticReturnOkCustomError(
        that: this,
      );

  static Future<int> staticReturnErrCustomError({dynamic hint}) =>
      RustLib.instance.api.someStructStaticReturnErrCustomError(hint: hint);

  static Future<int> staticReturnOkCustomError({dynamic hint}) =>
      RustLib.instance.api.someStructStaticReturnOkCustomError(hint: hint);

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SomeStruct &&
          runtimeType == other.runtimeType &&
          value == other.value;
}
