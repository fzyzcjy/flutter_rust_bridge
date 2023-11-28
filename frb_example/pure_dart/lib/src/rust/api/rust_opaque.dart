// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_opaque.freezed.dart';

Future<HideData> createOpaqueTwinNormal({dynamic hint}) =>
    RustLib.instance.api.createOpaqueTwinNormal(hint: hint);

Future<HideData?> createOptionOpaqueTwinNormal(
        {HideData? opaque, dynamic hint}) =>
    RustLib.instance.api
        .createOptionOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<EnumOpaqueTwinNormalArray5> createArrayOpaqueEnumTwinNormal(
        {dynamic hint}) =>
    RustLib.instance.api.createArrayOpaqueEnumTwinNormal(hint: hint);

Future<String> runEnumOpaqueTwinNormal(
        {required EnumOpaqueTwinNormal opaque, dynamic hint}) =>
    RustLib.instance.api.runEnumOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<String> runOpaqueTwinNormal({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<String> runOpaqueWithDelayTwinNormal(
        {required HideData opaque, dynamic hint}) =>
    RustLib.instance.api
        .runOpaqueWithDelayTwinNormal(opaque: opaque, hint: hint);

Future<HideDataArray2> opaqueArrayTwinNormal({dynamic hint}) =>
    RustLib.instance.api.opaqueArrayTwinNormal(hint: hint);

Future<String> runNonCloneTwinNormal(
        {required NonCloneData clone, dynamic hint}) =>
    RustLib.instance.api.runNonCloneTwinNormal(clone: clone, hint: hint);

Future<NonSendHideData> createSyncOpaqueTwinNormal({dynamic hint}) =>
    RustLib.instance.api.createSyncOpaqueTwinNormal(hint: hint);

Future<void> opaqueArrayRunTwinNormal(
        {required HideDataArray2 data, dynamic hint}) =>
    RustLib.instance.api.opaqueArrayRunTwinNormal(data: data, hint: hint);

Future<List<HideData>> opaqueVecTwinNormal({dynamic hint}) =>
    RustLib.instance.api.opaqueVecTwinNormal(hint: hint);

Future<void> opaqueVecRunTwinNormal(
        {required List<HideData> data, dynamic hint}) =>
    RustLib.instance.api.opaqueVecRunTwinNormal(data: data, hint: hint);

Future<OpaqueNestedTwinNormal> createNestedOpaqueTwinNormal({dynamic hint}) =>
    RustLib.instance.api.createNestedOpaqueTwinNormal(hint: hint);

Future<void> runNestedOpaqueTwinNormal(
        {required OpaqueNestedTwinNormal opaque, dynamic hint}) =>
    RustLib.instance.api.runNestedOpaqueTwinNormal(opaque: opaque, hint: hint);

Future<String> unwrapRustOpaqueTwinNormal(
        {required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.unwrapRustOpaqueTwinNormal(opaque: opaque, hint: hint);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
Future<FrbOpaqueReturn> frbGeneratorTestTwinNormal({dynamic hint}) =>
    RustLib.instance.api.frbGeneratorTestTwinNormal(hint: hint);

@sealed
class MutexHideData extends FrbOpaque {
  MutexHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueMutexHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueMutexHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.mutexHideDataFinalizer;
}

@sealed
class RwLockHideData extends FrbOpaque {
  RwLockHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueRwLockHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueRwLockHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.rwLockHideDataFinalizer;
}

@sealed
class BoxDartDebug extends FrbOpaque {
  BoxDartDebug.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueBoxDartDebug;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueBoxDartDebug;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.boxDartDebugFinalizer;
}

@sealed
class FrbOpaqueReturn extends FrbOpaque {
  FrbOpaqueReturn.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueFrbOpaqueReturn;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueFrbOpaqueReturn;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.frbOpaqueReturnFinalizer;
}

@sealed
class HideData extends FrbOpaque {
  HideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueHideData;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.hideDataFinalizer;
}

class HideDataArray2 extends NonGrowableListView<HideData> {
  static const arraySize = 2;
  HideDataArray2(List<HideData> inner)
      : assert(inner.length == arraySize),
        super(inner);
  HideDataArray2.unchecked(List<HideData> inner) : super(inner);
  HideDataArray2.init(HideData fill)
      : super(List<HideData>.filled(arraySize, fill));
}

@sealed
class I32 extends FrbOpaque {
  I32.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueI32;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueI32;

  @override
  OpaqueTypeFinalizer get staticFinalizer => RustLib.instance.api.i32Finalizer;
}

@sealed
class NonCloneData extends FrbOpaque {
  NonCloneData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueNonCloneData;

  @override
  OpaqueShareFnType get shareFn => RustLib.instance.api.shareOpaqueNonCloneData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.nonCloneDataFinalizer;
}

@sealed
class NonSendHideData extends FrbOpaque {
  NonSendHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn => RustLib.instance.api.dropOpaqueNonSendHideData;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueNonSendHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.nonSendHideDataFinalizer;
}

@freezed
sealed class EnumOpaqueTwinNormal with _$EnumOpaqueTwinNormal {
  const factory EnumOpaqueTwinNormal.struct(
    HideData field0,
  ) = EnumOpaqueTwinNormal_Struct;
  const factory EnumOpaqueTwinNormal.primitive(
    I32 field0,
  ) = EnumOpaqueTwinNormal_Primitive;
  const factory EnumOpaqueTwinNormal.traitObj(
    BoxDartDebug field0,
  ) = EnumOpaqueTwinNormal_TraitObj;
  const factory EnumOpaqueTwinNormal.mutex(
    MutexHideData field0,
  ) = EnumOpaqueTwinNormal_Mutex;
  const factory EnumOpaqueTwinNormal.rwLock(
    RwLockHideData field0,
  ) = EnumOpaqueTwinNormal_RwLock;
}

class EnumOpaqueTwinNormalArray5
    extends NonGrowableListView<EnumOpaqueTwinNormal> {
  static const arraySize = 5;
  EnumOpaqueTwinNormalArray5(List<EnumOpaqueTwinNormal> inner)
      : assert(inner.length == arraySize),
        super(inner);
  EnumOpaqueTwinNormalArray5.unchecked(List<EnumOpaqueTwinNormal> inner)
      : super(inner);
  EnumOpaqueTwinNormalArray5.init(EnumOpaqueTwinNormal fill)
      : super(List<EnumOpaqueTwinNormal>.filled(arraySize, fill));
}

/// [`HideData`] has private fields.
class OpaqueNestedTwinNormal {
  final HideData first;
  final HideData second;

  const OpaqueNestedTwinNormal({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNestedTwinNormal &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
