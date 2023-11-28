// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_opaque_twin_sync.freezed.dart';

HideData createOpaqueTwinSync({dynamic hint}) =>
    RustLib.instance.api.createOpaqueTwinSync(hint: hint);

HideData? createOptionOpaqueTwinSync({HideData? opaque, dynamic hint}) =>
    RustLib.instance.api.createOptionOpaqueTwinSync(opaque: opaque, hint: hint);

EnumOpaqueTwinSyncArray5 createArrayOpaqueEnumTwinSync({dynamic hint}) =>
    RustLib.instance.api.createArrayOpaqueEnumTwinSync(hint: hint);

String runEnumOpaqueTwinSync(
        {required EnumOpaqueTwinSync opaque, dynamic hint}) =>
    RustLib.instance.api.runEnumOpaqueTwinSync(opaque: opaque, hint: hint);

String runOpaqueTwinSync({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaqueTwinSync(opaque: opaque, hint: hint);

String runOpaqueWithDelayTwinSync({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaqueWithDelayTwinSync(opaque: opaque, hint: hint);

HideDataArray2 opaqueArrayTwinSync({dynamic hint}) =>
    RustLib.instance.api.opaqueArrayTwinSync(hint: hint);

String runNonCloneTwinSync({required NonCloneData clone, dynamic hint}) =>
    RustLib.instance.api.runNonCloneTwinSync(clone: clone, hint: hint);

NonSendHideData createSyncOpaqueTwinSync({dynamic hint}) =>
    RustLib.instance.api.createSyncOpaqueTwinSync(hint: hint);

void opaqueArrayRunTwinSync({required HideDataArray2 data, dynamic hint}) =>
    RustLib.instance.api.opaqueArrayRunTwinSync(data: data, hint: hint);

List<HideData> opaqueVecTwinSync({dynamic hint}) =>
    RustLib.instance.api.opaqueVecTwinSync(hint: hint);

void opaqueVecRunTwinSync({required List<HideData> data, dynamic hint}) =>
    RustLib.instance.api.opaqueVecRunTwinSync(data: data, hint: hint);

OpaqueNestedTwinSync createNestedOpaqueTwinSync({dynamic hint}) =>
    RustLib.instance.api.createNestedOpaqueTwinSync(hint: hint);

void runNestedOpaqueTwinSync(
        {required OpaqueNestedTwinSync opaque, dynamic hint}) =>
    RustLib.instance.api.runNestedOpaqueTwinSync(opaque: opaque, hint: hint);

String unwrapRustOpaqueTwinSync({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.unwrapRustOpaqueTwinSync(opaque: opaque, hint: hint);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
FrbOpaqueReturn frbGeneratorTestTwinSync({dynamic hint}) =>
    RustLib.instance.api.frbGeneratorTestTwinSync(hint: hint);

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
sealed class EnumOpaqueTwinSync with _$EnumOpaqueTwinSync {
  const factory EnumOpaqueTwinSync.struct(
    HideData field0,
  ) = EnumOpaqueTwinSync_Struct;
  const factory EnumOpaqueTwinSync.primitive(
    I32 field0,
  ) = EnumOpaqueTwinSync_Primitive;
  const factory EnumOpaqueTwinSync.traitObj(
    BoxDartDebug field0,
  ) = EnumOpaqueTwinSync_TraitObj;
  const factory EnumOpaqueTwinSync.mutex(
    MutexHideData field0,
  ) = EnumOpaqueTwinSync_Mutex;
  const factory EnumOpaqueTwinSync.rwLock(
    RwLockHideData field0,
  ) = EnumOpaqueTwinSync_RwLock;
}

class EnumOpaqueTwinSyncArray5 extends NonGrowableListView<EnumOpaqueTwinSync> {
  static const arraySize = 5;
  EnumOpaqueTwinSyncArray5(List<EnumOpaqueTwinSync> inner)
      : assert(inner.length == arraySize),
        super(inner);
  EnumOpaqueTwinSyncArray5.unchecked(List<EnumOpaqueTwinSync> inner)
      : super(inner);
  EnumOpaqueTwinSyncArray5.init(EnumOpaqueTwinSync fill)
      : super(List<EnumOpaqueTwinSync>.filled(arraySize, fill));
}

/// [`HideData`] has private fields.
class OpaqueNestedTwinSync {
  final HideData first;
  final HideData second;

  const OpaqueNestedTwinSync({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNestedTwinSync &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
