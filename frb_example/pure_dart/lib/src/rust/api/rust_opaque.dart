// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_opaque.freezed.dart';

Future<HideData> createOpaque({dynamic hint}) =>
    RustLib.instance.api.createOpaque(hint: hint);

Future<HideData?> createOptionOpaque({HideData? opaque, dynamic hint}) =>
    RustLib.instance.api.createOptionOpaque(opaque: opaque, hint: hint);

Future<EnumOpaqueArray5> createArrayOpaqueEnum({dynamic hint}) =>
    RustLib.instance.api.createArrayOpaqueEnum(hint: hint);

Future<String> runEnumOpaque({required EnumOpaque opaque, dynamic hint}) =>
    RustLib.instance.api.runEnumOpaque(opaque: opaque, hint: hint);

Future<String> runOpaque({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaque(opaque: opaque, hint: hint);

Future<String> runOpaqueWithDelay({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.runOpaqueWithDelay(opaque: opaque, hint: hint);

Future<HideDataArray2> opaqueArray({dynamic hint}) =>
    RustLib.instance.api.opaqueArray(hint: hint);

Future<String> runNonClone({required NonCloneData clone, dynamic hint}) =>
    RustLib.instance.api.runNonClone(clone: clone, hint: hint);

Future<NonSendHideData> createSyncOpaque({dynamic hint}) =>
    RustLib.instance.api.createSyncOpaque(hint: hint);

Future<void> opaqueArrayRun({required HideDataArray2 data, dynamic hint}) =>
    RustLib.instance.api.opaqueArrayRun(data: data, hint: hint);

Future<List<HideData>> opaqueVec({dynamic hint}) =>
    RustLib.instance.api.opaqueVec(hint: hint);

Future<void> opaqueVecRun({required List<HideData> data, dynamic hint}) =>
    RustLib.instance.api.opaqueVecRun(data: data, hint: hint);

Future<OpaqueNested> createNestedOpaque({dynamic hint}) =>
    RustLib.instance.api.createNestedOpaque(hint: hint);

Future<void> runNestedOpaque({required OpaqueNested opaque, dynamic hint}) =>
    RustLib.instance.api.runNestedOpaque(opaque: opaque, hint: hint);

Future<String> unwrapRustOpaque({required HideData opaque, dynamic hint}) =>
    RustLib.instance.api.unwrapRustOpaque(opaque: opaque, hint: hint);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
Future<FrbOpaqueReturn> frbGeneratorTest({dynamic hint}) =>
    RustLib.instance.api.frbGeneratorTest(hint: hint);

@sealed
class MutexHideData extends FrbOpaque {
  MutexHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueMutexHideData;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueMutexHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.MutexHideDataFinalizer;
}

@sealed
class RwLockHideData extends FrbOpaque {
  RwLockHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueRwLockHideData;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueRwLockHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.RwLockHideDataFinalizer;
}

@sealed
class BoxDartDebug extends FrbOpaque {
  BoxDartDebug.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueBoxDartDebug;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueBoxDartDebug;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.BoxDartDebugFinalizer;
}

@sealed
class FrbOpaqueReturn extends FrbOpaque {
  FrbOpaqueReturn.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueFrbOpaqueReturn;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueFrbOpaqueReturn;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.FrbOpaqueReturnFinalizer;
}

@sealed
class HideData extends FrbOpaque {
  HideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueHideData;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.HideDataFinalizer;
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
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueI32;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueI32;

  @override
  OpaqueTypeFinalizer get staticFinalizer => RustLib.instance.api.I32Finalizer;
}

@sealed
class NonCloneData extends FrbOpaque {
  NonCloneData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueNonCloneData;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueNonCloneData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.NonCloneDataFinalizer;
}

@sealed
class NonSendHideData extends FrbOpaque {
  NonSendHideData.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  DropFnType get dropFn => RustLib.instance.api.dropOpaqueNonSendHideData;

  @override
  ShareFnType get shareFn => RustLib.instance.api.shareOpaqueNonSendHideData;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.NonSendHideDataFinalizer;
}

@freezed
sealed class EnumOpaque with _$EnumOpaque {
  const factory EnumOpaque.struct(
    HideData field0,
  ) = EnumOpaque_Struct;
  const factory EnumOpaque.primitive(
    I32 field0,
  ) = EnumOpaque_Primitive;
  const factory EnumOpaque.traitObj(
    BoxDartDebug field0,
  ) = EnumOpaque_TraitObj;
  const factory EnumOpaque.mutex(
    MutexHideData field0,
  ) = EnumOpaque_Mutex;
  const factory EnumOpaque.rwLock(
    RwLockHideData field0,
  ) = EnumOpaque_RwLock;
}

class EnumOpaqueArray5 extends NonGrowableListView<EnumOpaque> {
  static const arraySize = 5;
  EnumOpaqueArray5(List<EnumOpaque> inner)
      : assert(inner.length == arraySize),
        super(inner);
  EnumOpaqueArray5.unchecked(List<EnumOpaque> inner) : super(inner);
  EnumOpaqueArray5.init(EnumOpaque fill)
      : super(List<EnumOpaque>.filled(arraySize, fill));
}

/// [`HideData`] has private fields.
class OpaqueNested {
  final HideData first;
  final HideData second;

  const OpaqueNested({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNested &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
