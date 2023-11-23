// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'dart_opaque.freezed.dart';

Future<String> asyncAcceptDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.asyncAcceptDartOpaque(opaque: opaque, hint: hint);

Future<Object> loopBack({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBack(opaque: opaque, hint: hint);

Future<Object?> loopBackOption({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackOption(opaque: opaque, hint: hint);

Future<ObjectArray1> loopBackArray({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackArray(opaque: opaque, hint: hint);

Future<List<Object>> loopBackVec({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVec(opaque: opaque, hint: hint);

Future<void> loopBackOptionGet({Object? opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackOptionGet(opaque: opaque, hint: hint);

Future<void> loopBackArrayGet({required ObjectArray1 opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackArrayGet(opaque: opaque, hint: hint);

Future<void> loopBackVecGet({required List<Object> opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVecGet(opaque: opaque, hint: hint);

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
Future<void> panicUnwrapDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.panicUnwrapDartOpaque(opaque: opaque, hint: hint);

Future<DartOpaqueNested> createNestedDartOpaque(
        {required Object opaque1, required Object opaque2, dynamic hint}) =>
    RustLib.instance.api
        .createNestedDartOpaque(opaque1: opaque1, opaque2: opaque2, hint: hint);

Future<void> getNestedDartOpaque(
        {required DartOpaqueNested opaque, dynamic hint}) =>
    RustLib.instance.api.getNestedDartOpaque(opaque: opaque, hint: hint);

Future<EnumDartOpaque> createEnumDartOpaque(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api.createEnumDartOpaque(opaque: opaque, hint: hint);

Future<void> getEnumDartOpaque(
        {required EnumDartOpaque opaque, dynamic hint}) =>
    RustLib.instance.api.getEnumDartOpaque(opaque: opaque, hint: hint);

Future<void> setStaticDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.setStaticDartOpaque(opaque: opaque, hint: hint);

Future<void> dropStaticDartOpaque({dynamic hint}) =>
    RustLib.instance.api.dropStaticDartOpaque(hint: hint);

class ObjectArray1 extends NonGrowableListView<Object> {
  static const arraySize = 1;
  ObjectArray1(List<Object> inner)
      : assert(inner.length == arraySize),
        super(inner);
  ObjectArray1.unchecked(List<Object> inner) : super(inner);
  ObjectArray1.init(Object fill) : super(List<Object>.filled(arraySize, fill));
}

class DartOpaqueNested {
  final Object first;
  final Object second;

  const DartOpaqueNested({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DartOpaqueNested &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}

@freezed
sealed class EnumDartOpaque with _$EnumDartOpaque {
  const factory EnumDartOpaque.primitive(
    int field0,
  ) = EnumDartOpaque_Primitive;
  const factory EnumDartOpaque.opaque(
    Object field0,
  ) = EnumDartOpaque_Opaque;
}
