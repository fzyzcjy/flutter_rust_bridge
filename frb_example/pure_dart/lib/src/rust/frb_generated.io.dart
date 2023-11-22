// ignore_for_file: unused_import, unused_element

import 'api/comment.dart';
import 'api/pseudo_manual/comment_twin_sync.dart';
import 'api/pseudo_manual/optional_primitive.dart';
import 'api/pseudo_manual/optional_primitive_twin_sync.dart';
import 'api/pseudo_manual/primitive.dart';
import 'api/pseudo_manual/primitive_list.dart';
import 'api/pseudo_manual/primitive_list_twin_sync.dart';
import 'api/pseudo_manual/primitive_twin_sync.dart';
import 'api/pseudo_manual/simple_twin_sync.dart';
import 'api/simple.dart';
import 'api/structure.dart';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @protected
  ffi.Pointer<ffi.Bool> api2wire_box_autoadd_bool(bool raw) {
    return wire.new_box_autoadd_bool(api2wire_bool(raw));
  }

  @protected
  ffi.Pointer<ffi.Float> api2wire_box_autoadd_f_32(double raw) {
    return wire.new_box_autoadd_f_32(api2wire_f_32(raw));
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_box_autoadd_f_64(double raw) {
    return wire.new_box_autoadd_f_64(api2wire_f_64(raw));
  }

  @protected
  ffi.Pointer<ffi.Int16> api2wire_box_autoadd_i_16(int raw) {
    return wire.new_box_autoadd_i_16(api2wire_i_16(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_i_32(int raw) {
    return wire.new_box_autoadd_i_32(api2wire_i_32(raw));
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_box_autoadd_i_64(BigInt raw) {
    return wire.new_box_autoadd_i_64(api2wire_i_64(raw));
  }

  @protected
  ffi.Pointer<ffi.Int8> api2wire_box_autoadd_i_8(int raw) {
    return wire.new_box_autoadd_i_8(api2wire_i_8(raw));
  }

  @protected
  ffi.Pointer<wire_struct_with_comments_twin_normal>
      api2wire_box_autoadd_struct_with_comments_twin_normal(
          StructWithCommentsTwinNormal raw) {
    final ptr = wire.new_box_autoadd_struct_with_comments_twin_normal();
    _api_fill_to_wire_struct_with_comments_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_comments_twin_sync>
      api2wire_box_autoadd_struct_with_comments_twin_sync(
          StructWithCommentsTwinSync raw) {
    final ptr = wire.new_box_autoadd_struct_with_comments_twin_sync();
    _api_fill_to_wire_struct_with_comments_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_zero_field>
      api2wire_box_autoadd_struct_with_zero_field(StructWithZeroField raw) {
    final ptr = wire.new_box_autoadd_struct_with_zero_field();

    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Uint16> api2wire_box_autoadd_u_16(int raw) {
    return wire.new_box_autoadd_u_16(api2wire_u_16(raw));
  }

  @protected
  ffi.Pointer<ffi.Uint32> api2wire_box_autoadd_u_32(int raw) {
    return wire.new_box_autoadd_u_32(api2wire_u_32(raw));
  }

  @protected
  ffi.Pointer<ffi.Uint64> api2wire_box_autoadd_u_64(BigInt raw) {
    return wire.new_box_autoadd_u_64(api2wire_u_64(raw));
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_box_autoadd_u_8(int raw) {
    return wire.new_box_autoadd_u_8(api2wire_u_8(raw));
  }

  @protected
  int api2wire_i_64(BigInt raw) {
    return raw.toInt();
  }

  @protected
  ffi.Pointer<wire_list_bool> api2wire_list_bool(List<bool> raw) {
    final ans = wire.new_list_bool(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      ans.ref.ptr[i] = api2wire_bool(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_f_32> api2wire_list_prim_f_32(Float32List raw) {
    final ans = wire.new_list_prim_f_32(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_f_64> api2wire_list_prim_f_64(Float64List raw) {
    final ans = wire.new_list_prim_f_64(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_i_16> api2wire_list_prim_i_16(Int16List raw) {
    final ans = wire.new_list_prim_i_16(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_i_32> api2wire_list_prim_i_32(Int32List raw) {
    final ans = wire.new_list_prim_i_32(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_i_64> api2wire_list_prim_i_64(Int64List raw) {
    final ans = wire.new_list_prim_i_64(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw.inner);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_i_8> api2wire_list_prim_i_8(Int8List raw) {
    final ans = wire.new_list_prim_i_8(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_16> api2wire_list_prim_u_16(Uint16List raw) {
    final ans = wire.new_list_prim_u_16(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_32> api2wire_list_prim_u_32(Uint32List raw) {
    final ans = wire.new_list_prim_u_32(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_64> api2wire_list_prim_u_64(Uint64List raw) {
    final ans = wire.new_list_prim_u_64(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw.inner);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_list_prim_u_8(Uint8List raw) {
    final ans = wire.new_list_prim_u_8(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<ffi.Bool> api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_bool(raw);
  }

  @protected
  ffi.Pointer<ffi.Float> api2wire_opt_box_autoadd_f_32(double? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_f_32(raw);
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_opt_box_autoadd_f_64(double? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_f_64(raw);
  }

  @protected
  ffi.Pointer<ffi.Int16> api2wire_opt_box_autoadd_i_16(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i_16(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_autoadd_i_32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i_32(raw);
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_opt_box_autoadd_i_64(BigInt? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i_64(raw);
  }

  @protected
  ffi.Pointer<ffi.Int8> api2wire_opt_box_autoadd_i_8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i_8(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint16> api2wire_opt_box_autoadd_u_16(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u_16(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint32> api2wire_opt_box_autoadd_u_32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u_32(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint64> api2wire_opt_box_autoadd_u_64(BigInt? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u_64(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_opt_box_autoadd_u_8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u_8(raw);
  }

  @protected
  int api2wire_u_64(BigInt raw) {
    return raw.toInt();
  }
}

// Section: c_binding

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class RustLibWire implements BaseWire {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustLibWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    int port_,
    ffi.Pointer<wire_struct_with_comments_twin_normal> that,
  ) {
    return _wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
      port_,
      that,
    );
  }

  late final _wire_StructWithCommentsTwinNormal_instance_method_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Int64,
                      ffi.Pointer<wire_struct_with_comments_twin_normal>)>>(
          'wire_StructWithCommentsTwinNormal_instance_method_twin_normal');
  late final _wire_StructWithCommentsTwinNormal_instance_method_twin_normal =
      _wire_StructWithCommentsTwinNormal_instance_method_twin_normalPtr
          .asFunction<
              void Function(
                  int, ffi.Pointer<wire_struct_with_comments_twin_normal>)>();

  void wire_StructWithCommentsTwinNormal_static_method_twin_normal(
    int port_,
  ) {
    return _wire_StructWithCommentsTwinNormal_static_method_twin_normal(
      port_,
    );
  }

  late final _wire_StructWithCommentsTwinNormal_static_method_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_StructWithCommentsTwinNormal_static_method_twin_normal');
  late final _wire_StructWithCommentsTwinNormal_static_method_twin_normal =
      _wire_StructWithCommentsTwinNormal_static_method_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_function_with_comments_slash_star_star_twin_normal(
    int port_,
  ) {
    return _wire_function_with_comments_slash_star_star_twin_normal(
      port_,
    );
  }

  late final _wire_function_with_comments_slash_star_star_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_function_with_comments_slash_star_star_twin_normal');
  late final _wire_function_with_comments_slash_star_star_twin_normal =
      _wire_function_with_comments_slash_star_star_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_function_with_comments_triple_slash_multi_line_twin_normal(
    int port_,
  ) {
    return _wire_function_with_comments_triple_slash_multi_line_twin_normal(
      port_,
    );
  }

  late final _wire_function_with_comments_triple_slash_multi_line_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_function_with_comments_triple_slash_multi_line_twin_normal');
  late final _wire_function_with_comments_triple_slash_multi_line_twin_normal =
      _wire_function_with_comments_triple_slash_multi_line_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_function_with_comments_triple_slash_single_line_twin_normal(
    int port_,
  ) {
    return _wire_function_with_comments_triple_slash_single_line_twin_normal(
      port_,
    );
  }

  late final _wire_function_with_comments_triple_slash_single_line_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_function_with_comments_triple_slash_single_line_twin_normal');
  late final _wire_function_with_comments_triple_slash_single_line_twin_normal =
      _wire_function_with_comments_triple_slash_single_line_twin_normalPtr
          .asFunction<void Function(int)>();

  WireSyncReturn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    ffi.Pointer<wire_struct_with_comments_twin_sync> that,
  ) {
    return _wire_StructWithCommentsTwinSync_instance_method_twin_sync(
      that,
    );
  }

  late final _wire_StructWithCommentsTwinSync_instance_method_twin_syncPtr =
      _lookup<
              ffi.NativeFunction<
                  WireSyncReturn Function(
                      ffi.Pointer<wire_struct_with_comments_twin_sync>)>>(
          'wire_StructWithCommentsTwinSync_instance_method_twin_sync');
  late final _wire_StructWithCommentsTwinSync_instance_method_twin_sync =
      _wire_StructWithCommentsTwinSync_instance_method_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_struct_with_comments_twin_sync>)>();

  WireSyncReturn wire_StructWithCommentsTwinSync_static_method_twin_sync() {
    return _wire_StructWithCommentsTwinSync_static_method_twin_sync();
  }

  late final _wire_StructWithCommentsTwinSync_static_method_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_StructWithCommentsTwinSync_static_method_twin_sync');
  late final _wire_StructWithCommentsTwinSync_static_method_twin_sync =
      _wire_StructWithCommentsTwinSync_static_method_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_function_with_comments_slash_star_star_twin_sync() {
    return _wire_function_with_comments_slash_star_star_twin_sync();
  }

  late final _wire_function_with_comments_slash_star_star_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_function_with_comments_slash_star_star_twin_sync');
  late final _wire_function_with_comments_slash_star_star_twin_sync =
      _wire_function_with_comments_slash_star_star_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn
      wire_function_with_comments_triple_slash_multi_line_twin_sync() {
    return _wire_function_with_comments_triple_slash_multi_line_twin_sync();
  }

  late final _wire_function_with_comments_triple_slash_multi_line_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_function_with_comments_triple_slash_multi_line_twin_sync');
  late final _wire_function_with_comments_triple_slash_multi_line_twin_sync =
      _wire_function_with_comments_triple_slash_multi_line_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn
      wire_function_with_comments_triple_slash_single_line_twin_sync() {
    return _wire_function_with_comments_triple_slash_single_line_twin_sync();
  }

  late final _wire_function_with_comments_triple_slash_single_line_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_function_with_comments_triple_slash_single_line_twin_sync');
  late final _wire_function_with_comments_triple_slash_single_line_twin_sync =
      _wire_function_with_comments_triple_slash_single_line_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  void wire_example_optional_primitive_type_bool_twin_normal(
    int port_,
    ffi.Pointer<ffi.Bool> arg,
  ) {
    return _wire_example_optional_primitive_type_bool_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_bool_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Bool>)>>(
          'wire_example_optional_primitive_type_bool_twin_normal');
  late final _wire_example_optional_primitive_type_bool_twin_normal =
      _wire_example_optional_primitive_type_bool_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Bool>)>();

  void wire_example_optional_primitive_type_f32_twin_normal(
    int port_,
    ffi.Pointer<ffi.Float> arg,
  ) {
    return _wire_example_optional_primitive_type_f32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_f32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Float>)>>(
      'wire_example_optional_primitive_type_f32_twin_normal');
  late final _wire_example_optional_primitive_type_f32_twin_normal =
      _wire_example_optional_primitive_type_f32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Float>)>();

  void wire_example_optional_primitive_type_f64_twin_normal(
    int port_,
    ffi.Pointer<ffi.Double> arg,
  ) {
    return _wire_example_optional_primitive_type_f64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_f64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Double>)>>(
      'wire_example_optional_primitive_type_f64_twin_normal');
  late final _wire_example_optional_primitive_type_f64_twin_normal =
      _wire_example_optional_primitive_type_f64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Double>)>();

  void wire_example_optional_primitive_type_i16_twin_normal(
    int port_,
    ffi.Pointer<ffi.Int16> arg,
  ) {
    return _wire_example_optional_primitive_type_i16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i16_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Int16>)>>(
      'wire_example_optional_primitive_type_i16_twin_normal');
  late final _wire_example_optional_primitive_type_i16_twin_normal =
      _wire_example_optional_primitive_type_i16_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Int16>)>();

  void wire_example_optional_primitive_type_i32_twin_normal(
    int port_,
    ffi.Pointer<ffi.Int32> arg,
  ) {
    return _wire_example_optional_primitive_type_i32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Int32>)>>(
      'wire_example_optional_primitive_type_i32_twin_normal');
  late final _wire_example_optional_primitive_type_i32_twin_normal =
      _wire_example_optional_primitive_type_i32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Int32>)>();

  void wire_example_optional_primitive_type_i64_twin_normal(
    int port_,
    ffi.Pointer<ffi.Int64> arg,
  ) {
    return _wire_example_optional_primitive_type_i64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Int64>)>>(
      'wire_example_optional_primitive_type_i64_twin_normal');
  late final _wire_example_optional_primitive_type_i64_twin_normal =
      _wire_example_optional_primitive_type_i64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Int64>)>();

  void wire_example_optional_primitive_type_i8_twin_normal(
    int port_,
    ffi.Pointer<ffi.Int8> arg,
  ) {
    return _wire_example_optional_primitive_type_i8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i8_twin_normalPtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Int8>)>>(
      'wire_example_optional_primitive_type_i8_twin_normal');
  late final _wire_example_optional_primitive_type_i8_twin_normal =
      _wire_example_optional_primitive_type_i8_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Int8>)>();

  void wire_example_optional_primitive_type_u16_twin_normal(
    int port_,
    ffi.Pointer<ffi.Uint16> arg,
  ) {
    return _wire_example_optional_primitive_type_u16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u16_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Uint16>)>>(
      'wire_example_optional_primitive_type_u16_twin_normal');
  late final _wire_example_optional_primitive_type_u16_twin_normal =
      _wire_example_optional_primitive_type_u16_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Uint16>)>();

  void wire_example_optional_primitive_type_u32_twin_normal(
    int port_,
    ffi.Pointer<ffi.Uint32> arg,
  ) {
    return _wire_example_optional_primitive_type_u32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Uint32>)>>(
      'wire_example_optional_primitive_type_u32_twin_normal');
  late final _wire_example_optional_primitive_type_u32_twin_normal =
      _wire_example_optional_primitive_type_u32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Uint32>)>();

  void wire_example_optional_primitive_type_u64_twin_normal(
    int port_,
    ffi.Pointer<ffi.Uint64> arg,
  ) {
    return _wire_example_optional_primitive_type_u64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Uint64>)>>(
      'wire_example_optional_primitive_type_u64_twin_normal');
  late final _wire_example_optional_primitive_type_u64_twin_normal =
      _wire_example_optional_primitive_type_u64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Uint64>)>();

  void wire_example_optional_primitive_type_u8_twin_normal(
    int port_,
    ffi.Pointer<ffi.Uint8> arg,
  ) {
    return _wire_example_optional_primitive_type_u8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u8_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Uint8>)>>(
      'wire_example_optional_primitive_type_u8_twin_normal');
  late final _wire_example_optional_primitive_type_u8_twin_normal =
      _wire_example_optional_primitive_type_u8_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Uint8>)>();

  WireSyncReturn wire_example_optional_primitive_type_bool_twin_sync(
    ffi.Pointer<ffi.Bool> arg,
  ) {
    return _wire_example_optional_primitive_type_bool_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_bool_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Bool>)>>(
      'wire_example_optional_primitive_type_bool_twin_sync');
  late final _wire_example_optional_primitive_type_bool_twin_sync =
      _wire_example_optional_primitive_type_bool_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Bool>)>();

  WireSyncReturn wire_example_optional_primitive_type_f32_twin_sync(
    ffi.Pointer<ffi.Float> arg,
  ) {
    return _wire_example_optional_primitive_type_f32_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_f32_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Float>)>>(
      'wire_example_optional_primitive_type_f32_twin_sync');
  late final _wire_example_optional_primitive_type_f32_twin_sync =
      _wire_example_optional_primitive_type_f32_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Float>)>();

  WireSyncReturn wire_example_optional_primitive_type_f64_twin_sync(
    ffi.Pointer<ffi.Double> arg,
  ) {
    return _wire_example_optional_primitive_type_f64_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_f64_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Double>)>>(
      'wire_example_optional_primitive_type_f64_twin_sync');
  late final _wire_example_optional_primitive_type_f64_twin_sync =
      _wire_example_optional_primitive_type_f64_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Double>)>();

  WireSyncReturn wire_example_optional_primitive_type_i16_twin_sync(
    ffi.Pointer<ffi.Int16> arg,
  ) {
    return _wire_example_optional_primitive_type_i16_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i16_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int16>)>>(
      'wire_example_optional_primitive_type_i16_twin_sync');
  late final _wire_example_optional_primitive_type_i16_twin_sync =
      _wire_example_optional_primitive_type_i16_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int16>)>();

  WireSyncReturn wire_example_optional_primitive_type_i32_twin_sync(
    ffi.Pointer<ffi.Int32> arg,
  ) {
    return _wire_example_optional_primitive_type_i32_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i32_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int32>)>>(
      'wire_example_optional_primitive_type_i32_twin_sync');
  late final _wire_example_optional_primitive_type_i32_twin_sync =
      _wire_example_optional_primitive_type_i32_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int32>)>();

  WireSyncReturn wire_example_optional_primitive_type_i64_twin_sync(
    ffi.Pointer<ffi.Int64> arg,
  ) {
    return _wire_example_optional_primitive_type_i64_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i64_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int64>)>>(
      'wire_example_optional_primitive_type_i64_twin_sync');
  late final _wire_example_optional_primitive_type_i64_twin_sync =
      _wire_example_optional_primitive_type_i64_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int64>)>();

  WireSyncReturn wire_example_optional_primitive_type_i8_twin_sync(
    ffi.Pointer<ffi.Int8> arg,
  ) {
    return _wire_example_optional_primitive_type_i8_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_i8_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int8>)>>(
      'wire_example_optional_primitive_type_i8_twin_sync');
  late final _wire_example_optional_primitive_type_i8_twin_sync =
      _wire_example_optional_primitive_type_i8_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Int8>)>();

  WireSyncReturn wire_example_optional_primitive_type_u16_twin_sync(
    ffi.Pointer<ffi.Uint16> arg,
  ) {
    return _wire_example_optional_primitive_type_u16_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u16_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint16>)>>(
      'wire_example_optional_primitive_type_u16_twin_sync');
  late final _wire_example_optional_primitive_type_u16_twin_sync =
      _wire_example_optional_primitive_type_u16_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint16>)>();

  WireSyncReturn wire_example_optional_primitive_type_u32_twin_sync(
    ffi.Pointer<ffi.Uint32> arg,
  ) {
    return _wire_example_optional_primitive_type_u32_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u32_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint32>)>>(
      'wire_example_optional_primitive_type_u32_twin_sync');
  late final _wire_example_optional_primitive_type_u32_twin_sync =
      _wire_example_optional_primitive_type_u32_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint32>)>();

  WireSyncReturn wire_example_optional_primitive_type_u64_twin_sync(
    ffi.Pointer<ffi.Uint64> arg,
  ) {
    return _wire_example_optional_primitive_type_u64_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u64_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint64>)>>(
      'wire_example_optional_primitive_type_u64_twin_sync');
  late final _wire_example_optional_primitive_type_u64_twin_sync =
      _wire_example_optional_primitive_type_u64_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint64>)>();

  WireSyncReturn wire_example_optional_primitive_type_u8_twin_sync(
    ffi.Pointer<ffi.Uint8> arg,
  ) {
    return _wire_example_optional_primitive_type_u8_twin_sync(
      arg,
    );
  }

  late final _wire_example_optional_primitive_type_u8_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint8>)>>(
      'wire_example_optional_primitive_type_u8_twin_sync');
  late final _wire_example_optional_primitive_type_u8_twin_sync =
      _wire_example_optional_primitive_type_u8_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<ffi.Uint8>)>();

  void wire_example_primitive_type_bool_twin_normal(
    int port_,
    bool arg,
  ) {
    return _wire_example_primitive_type_bool_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_bool_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Bool)>>(
          'wire_example_primitive_type_bool_twin_normal');
  late final _wire_example_primitive_type_bool_twin_normal =
      _wire_example_primitive_type_bool_twin_normalPtr
          .asFunction<void Function(int, bool)>();

  void wire_example_primitive_type_f32_twin_normal(
    int port_,
    double arg,
  ) {
    return _wire_example_primitive_type_f32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_f32_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Float)>>(
          'wire_example_primitive_type_f32_twin_normal');
  late final _wire_example_primitive_type_f32_twin_normal =
      _wire_example_primitive_type_f32_twin_normalPtr
          .asFunction<void Function(int, double)>();

  void wire_example_primitive_type_f64_twin_normal(
    int port_,
    double arg,
  ) {
    return _wire_example_primitive_type_f64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_f64_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Double)>>(
          'wire_example_primitive_type_f64_twin_normal');
  late final _wire_example_primitive_type_f64_twin_normal =
      _wire_example_primitive_type_f64_twin_normalPtr
          .asFunction<void Function(int, double)>();

  void wire_example_primitive_type_i16_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_i16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_i16_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int16)>>(
          'wire_example_primitive_type_i16_twin_normal');
  late final _wire_example_primitive_type_i16_twin_normal =
      _wire_example_primitive_type_i16_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_i32_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_i32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_i32_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_example_primitive_type_i32_twin_normal');
  late final _wire_example_primitive_type_i32_twin_normal =
      _wire_example_primitive_type_i32_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_i64_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_i64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_i64_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int64)>>(
          'wire_example_primitive_type_i64_twin_normal');
  late final _wire_example_primitive_type_i64_twin_normal =
      _wire_example_primitive_type_i64_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_i8_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_i8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_i8_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int8)>>(
          'wire_example_primitive_type_i8_twin_normal');
  late final _wire_example_primitive_type_i8_twin_normal =
      _wire_example_primitive_type_i8_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_u16_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_u16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_u16_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint16)>>(
          'wire_example_primitive_type_u16_twin_normal');
  late final _wire_example_primitive_type_u16_twin_normal =
      _wire_example_primitive_type_u16_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_u32_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_u32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_u32_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint32)>>(
          'wire_example_primitive_type_u32_twin_normal');
  late final _wire_example_primitive_type_u32_twin_normal =
      _wire_example_primitive_type_u32_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_u64_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_u64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_u64_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>(
          'wire_example_primitive_type_u64_twin_normal');
  late final _wire_example_primitive_type_u64_twin_normal =
      _wire_example_primitive_type_u64_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_type_u8_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_example_primitive_type_u8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_type_u8_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint8)>>(
          'wire_example_primitive_type_u8_twin_normal');
  late final _wire_example_primitive_type_u8_twin_normal =
      _wire_example_primitive_type_u8_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_example_primitive_list_type_bool_twin_normal(
    int port_,
    ffi.Pointer<wire_list_bool> arg,
  ) {
    return _wire_example_primitive_list_type_bool_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_bool_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_bool>)>>(
      'wire_example_primitive_list_type_bool_twin_normal');
  late final _wire_example_primitive_list_type_bool_twin_normal =
      _wire_example_primitive_list_type_bool_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_bool>)>();

  void wire_example_primitive_list_type_f32_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_f_32> arg,
  ) {
    return _wire_example_primitive_list_type_f32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_f32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_f_32>)>>(
      'wire_example_primitive_list_type_f32_twin_normal');
  late final _wire_example_primitive_list_type_f32_twin_normal =
      _wire_example_primitive_list_type_f32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_f_32>)>();

  void wire_example_primitive_list_type_f64_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_f_64> arg,
  ) {
    return _wire_example_primitive_list_type_f64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_f64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_f_64>)>>(
      'wire_example_primitive_list_type_f64_twin_normal');
  late final _wire_example_primitive_list_type_f64_twin_normal =
      _wire_example_primitive_list_type_f64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_f_64>)>();

  void wire_example_primitive_list_type_i16_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_i_16> arg,
  ) {
    return _wire_example_primitive_list_type_i16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i16_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_16>)>>(
      'wire_example_primitive_list_type_i16_twin_normal');
  late final _wire_example_primitive_list_type_i16_twin_normal =
      _wire_example_primitive_list_type_i16_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_16>)>();

  void wire_example_primitive_list_type_i32_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_i_32> arg,
  ) {
    return _wire_example_primitive_list_type_i32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_32>)>>(
      'wire_example_primitive_list_type_i32_twin_normal');
  late final _wire_example_primitive_list_type_i32_twin_normal =
      _wire_example_primitive_list_type_i32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_32>)>();

  void wire_example_primitive_list_type_i64_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_i_64> arg,
  ) {
    return _wire_example_primitive_list_type_i64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_64>)>>(
      'wire_example_primitive_list_type_i64_twin_normal');
  late final _wire_example_primitive_list_type_i64_twin_normal =
      _wire_example_primitive_list_type_i64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_64>)>();

  void wire_example_primitive_list_type_i8_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_i_8> arg,
  ) {
    return _wire_example_primitive_list_type_i8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i8_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_8>)>>(
      'wire_example_primitive_list_type_i8_twin_normal');
  late final _wire_example_primitive_list_type_i8_twin_normal =
      _wire_example_primitive_list_type_i8_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_8>)>();

  void wire_example_primitive_list_type_u16_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_16> arg,
  ) {
    return _wire_example_primitive_list_type_u16_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u16_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_16>)>>(
      'wire_example_primitive_list_type_u16_twin_normal');
  late final _wire_example_primitive_list_type_u16_twin_normal =
      _wire_example_primitive_list_type_u16_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_16>)>();

  void wire_example_primitive_list_type_u32_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_32> arg,
  ) {
    return _wire_example_primitive_list_type_u32_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u32_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_32>)>>(
      'wire_example_primitive_list_type_u32_twin_normal');
  late final _wire_example_primitive_list_type_u32_twin_normal =
      _wire_example_primitive_list_type_u32_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_32>)>();

  void wire_example_primitive_list_type_u64_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_64> arg,
  ) {
    return _wire_example_primitive_list_type_u64_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u64_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_64>)>>(
      'wire_example_primitive_list_type_u64_twin_normal');
  late final _wire_example_primitive_list_type_u64_twin_normal =
      _wire_example_primitive_list_type_u64_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_64>)>();

  void wire_example_primitive_list_type_u8_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> arg,
  ) {
    return _wire_example_primitive_list_type_u8_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u8_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_example_primitive_list_type_u8_twin_normal');
  late final _wire_example_primitive_list_type_u8_twin_normal =
      _wire_example_primitive_list_type_u8_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  WireSyncReturn wire_example_primitive_list_type_bool_twin_sync(
    ffi.Pointer<wire_list_bool> arg,
  ) {
    return _wire_example_primitive_list_type_bool_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_bool_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_bool>)>>(
      'wire_example_primitive_list_type_bool_twin_sync');
  late final _wire_example_primitive_list_type_bool_twin_sync =
      _wire_example_primitive_list_type_bool_twin_syncPtr
          .asFunction<WireSyncReturn Function(ffi.Pointer<wire_list_bool>)>();

  WireSyncReturn wire_example_primitive_list_type_f32_twin_sync(
    ffi.Pointer<wire_list_prim_f_32> arg,
  ) {
    return _wire_example_primitive_list_type_f32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_f32_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_f_32>)>>(
      'wire_example_primitive_list_type_f32_twin_sync');
  late final _wire_example_primitive_list_type_f32_twin_sync =
      _wire_example_primitive_list_type_f32_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_f_32>)>();

  WireSyncReturn wire_example_primitive_list_type_f64_twin_sync(
    ffi.Pointer<wire_list_prim_f_64> arg,
  ) {
    return _wire_example_primitive_list_type_f64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_f64_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_f_64>)>>(
      'wire_example_primitive_list_type_f64_twin_sync');
  late final _wire_example_primitive_list_type_f64_twin_sync =
      _wire_example_primitive_list_type_f64_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_f_64>)>();

  WireSyncReturn wire_example_primitive_list_type_i16_twin_sync(
    ffi.Pointer<wire_list_prim_i_16> arg,
  ) {
    return _wire_example_primitive_list_type_i16_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i16_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_16>)>>(
      'wire_example_primitive_list_type_i16_twin_sync');
  late final _wire_example_primitive_list_type_i16_twin_sync =
      _wire_example_primitive_list_type_i16_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_16>)>();

  WireSyncReturn wire_example_primitive_list_type_i32_twin_sync(
    ffi.Pointer<wire_list_prim_i_32> arg,
  ) {
    return _wire_example_primitive_list_type_i32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i32_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_32>)>>(
      'wire_example_primitive_list_type_i32_twin_sync');
  late final _wire_example_primitive_list_type_i32_twin_sync =
      _wire_example_primitive_list_type_i32_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_32>)>();

  WireSyncReturn wire_example_primitive_list_type_i64_twin_sync(
    ffi.Pointer<wire_list_prim_i_64> arg,
  ) {
    return _wire_example_primitive_list_type_i64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i64_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_64>)>>(
      'wire_example_primitive_list_type_i64_twin_sync');
  late final _wire_example_primitive_list_type_i64_twin_sync =
      _wire_example_primitive_list_type_i64_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_64>)>();

  WireSyncReturn wire_example_primitive_list_type_i8_twin_sync(
    ffi.Pointer<wire_list_prim_i_8> arg,
  ) {
    return _wire_example_primitive_list_type_i8_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_i8_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_8>)>>(
      'wire_example_primitive_list_type_i8_twin_sync');
  late final _wire_example_primitive_list_type_i8_twin_sync =
      _wire_example_primitive_list_type_i8_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_i_8>)>();

  WireSyncReturn wire_example_primitive_list_type_u16_twin_sync(
    ffi.Pointer<wire_list_prim_u_16> arg,
  ) {
    return _wire_example_primitive_list_type_u16_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u16_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_16>)>>(
      'wire_example_primitive_list_type_u16_twin_sync');
  late final _wire_example_primitive_list_type_u16_twin_sync =
      _wire_example_primitive_list_type_u16_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_16>)>();

  WireSyncReturn wire_example_primitive_list_type_u32_twin_sync(
    ffi.Pointer<wire_list_prim_u_32> arg,
  ) {
    return _wire_example_primitive_list_type_u32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u32_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_32>)>>(
      'wire_example_primitive_list_type_u32_twin_sync');
  late final _wire_example_primitive_list_type_u32_twin_sync =
      _wire_example_primitive_list_type_u32_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_32>)>();

  WireSyncReturn wire_example_primitive_list_type_u64_twin_sync(
    ffi.Pointer<wire_list_prim_u_64> arg,
  ) {
    return _wire_example_primitive_list_type_u64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u64_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_64>)>>(
      'wire_example_primitive_list_type_u64_twin_sync');
  late final _wire_example_primitive_list_type_u64_twin_sync =
      _wire_example_primitive_list_type_u64_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_64>)>();

  WireSyncReturn wire_example_primitive_list_type_u8_twin_sync(
    ffi.Pointer<wire_list_prim_u_8> arg,
  ) {
    return _wire_example_primitive_list_type_u8_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_list_type_u8_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_example_primitive_list_type_u8_twin_sync');
  late final _wire_example_primitive_list_type_u8_twin_sync =
      _wire_example_primitive_list_type_u8_twin_syncPtr.asFunction<
          WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_8>)>();

  WireSyncReturn wire_example_primitive_type_bool_twin_sync(
    bool arg,
  ) {
    return _wire_example_primitive_type_bool_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_bool_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Bool)>>(
          'wire_example_primitive_type_bool_twin_sync');
  late final _wire_example_primitive_type_bool_twin_sync =
      _wire_example_primitive_type_bool_twin_syncPtr
          .asFunction<WireSyncReturn Function(bool)>();

  WireSyncReturn wire_example_primitive_type_f32_twin_sync(
    double arg,
  ) {
    return _wire_example_primitive_type_f32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_f32_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Float)>>(
          'wire_example_primitive_type_f32_twin_sync');
  late final _wire_example_primitive_type_f32_twin_sync =
      _wire_example_primitive_type_f32_twin_syncPtr
          .asFunction<WireSyncReturn Function(double)>();

  WireSyncReturn wire_example_primitive_type_f64_twin_sync(
    double arg,
  ) {
    return _wire_example_primitive_type_f64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_f64_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Double)>>(
          'wire_example_primitive_type_f64_twin_sync');
  late final _wire_example_primitive_type_f64_twin_sync =
      _wire_example_primitive_type_f64_twin_syncPtr
          .asFunction<WireSyncReturn Function(double)>();

  WireSyncReturn wire_example_primitive_type_i16_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_i16_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_i16_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Int16)>>(
          'wire_example_primitive_type_i16_twin_sync');
  late final _wire_example_primitive_type_i16_twin_sync =
      _wire_example_primitive_type_i16_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_i32_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_i32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_i32_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Int32)>>(
          'wire_example_primitive_type_i32_twin_sync');
  late final _wire_example_primitive_type_i32_twin_sync =
      _wire_example_primitive_type_i32_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_i64_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_i64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_i64_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Int64)>>(
          'wire_example_primitive_type_i64_twin_sync');
  late final _wire_example_primitive_type_i64_twin_sync =
      _wire_example_primitive_type_i64_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_i8_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_i8_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_i8_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Int8)>>(
          'wire_example_primitive_type_i8_twin_sync');
  late final _wire_example_primitive_type_i8_twin_sync =
      _wire_example_primitive_type_i8_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_u16_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_u16_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_u16_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint16)>>(
          'wire_example_primitive_type_u16_twin_sync');
  late final _wire_example_primitive_type_u16_twin_sync =
      _wire_example_primitive_type_u16_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_u32_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_u32_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_u32_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint32)>>(
          'wire_example_primitive_type_u32_twin_sync');
  late final _wire_example_primitive_type_u32_twin_sync =
      _wire_example_primitive_type_u32_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_u64_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_u64_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_u64_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint64)>>(
          'wire_example_primitive_type_u64_twin_sync');
  late final _wire_example_primitive_type_u64_twin_sync =
      _wire_example_primitive_type_u64_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_example_primitive_type_u8_twin_sync(
    int arg,
  ) {
    return _wire_example_primitive_type_u8_twin_sync(
      arg,
    );
  }

  late final _wire_example_primitive_type_u8_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint8)>>(
          'wire_example_primitive_type_u8_twin_sync');
  late final _wire_example_primitive_type_u8_twin_sync =
      _wire_example_primitive_type_u8_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_simple_adder_twin_sync(
    int a,
    int b,
  ) {
    return _wire_simple_adder_twin_sync(
      a,
      b,
    );
  }

  late final _wire_simple_adder_twin_syncPtr = _lookup<
          ffi.NativeFunction<WireSyncReturn Function(ffi.Int32, ffi.Int32)>>(
      'wire_simple_adder_twin_sync');
  late final _wire_simple_adder_twin_sync = _wire_simple_adder_twin_syncPtr
      .asFunction<WireSyncReturn Function(int, int)>();

  void wire_simple_adder_twin_normal(
    int port_,
    int a,
    int b,
  ) {
    return _wire_simple_adder_twin_normal(
      port_,
      a,
      b,
    );
  }

  late final _wire_simple_adder_twin_normalPtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32, ffi.Int32)>>(
      'wire_simple_adder_twin_normal');
  late final _wire_simple_adder_twin_normal = _wire_simple_adder_twin_normalPtr
      .asFunction<void Function(int, int, int)>();

  void wire_func_struct_with_zero_field_twin_normal(
    int port_,
    ffi.Pointer<wire_struct_with_zero_field> arg,
  ) {
    return _wire_func_struct_with_zero_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_struct_with_zero_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_struct_with_zero_field>)>>(
      'wire_func_struct_with_zero_field_twin_normal');
  late final _wire_func_struct_with_zero_field_twin_normal =
      _wire_func_struct_with_zero_field_twin_normalPtr.asFunction<
          void Function(int, ffi.Pointer<wire_struct_with_zero_field>)>();

  ffi.Pointer<ffi.Bool> new_box_autoadd_bool(
    bool value,
  ) {
    return _new_box_autoadd_bool(
      value,
    );
  }

  late final _new_box_autoadd_boolPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Bool> Function(ffi.Bool)>>(
          'new_box_autoadd_bool');
  late final _new_box_autoadd_bool = _new_box_autoadd_boolPtr
      .asFunction<ffi.Pointer<ffi.Bool> Function(bool)>();

  ffi.Pointer<ffi.Float> new_box_autoadd_f_32(
    double value,
  ) {
    return _new_box_autoadd_f_32(
      value,
    );
  }

  late final _new_box_autoadd_f_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Float> Function(ffi.Float)>>(
          'new_box_autoadd_f_32');
  late final _new_box_autoadd_f_32 = _new_box_autoadd_f_32Ptr
      .asFunction<ffi.Pointer<ffi.Float> Function(double)>();

  ffi.Pointer<ffi.Double> new_box_autoadd_f_64(
    double value,
  ) {
    return _new_box_autoadd_f_64(
      value,
    );
  }

  late final _new_box_autoadd_f_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Double> Function(ffi.Double)>>(
          'new_box_autoadd_f_64');
  late final _new_box_autoadd_f_64 = _new_box_autoadd_f_64Ptr
      .asFunction<ffi.Pointer<ffi.Double> Function(double)>();

  ffi.Pointer<ffi.Int16> new_box_autoadd_i_16(
    int value,
  ) {
    return _new_box_autoadd_i_16(
      value,
    );
  }

  late final _new_box_autoadd_i_16Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int16> Function(ffi.Int16)>>(
          'new_box_autoadd_i_16');
  late final _new_box_autoadd_i_16 = _new_box_autoadd_i_16Ptr
      .asFunction<ffi.Pointer<ffi.Int16> Function(int)>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_i_32(
    int value,
  ) {
    return _new_box_autoadd_i_32(
      value,
    );
  }

  late final _new_box_autoadd_i_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_i_32');
  late final _new_box_autoadd_i_32 = _new_box_autoadd_i_32Ptr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<ffi.Int64> new_box_autoadd_i_64(
    int value,
  ) {
    return _new_box_autoadd_i_64(
      value,
    );
  }

  late final _new_box_autoadd_i_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int64> Function(ffi.Int64)>>(
          'new_box_autoadd_i_64');
  late final _new_box_autoadd_i_64 = _new_box_autoadd_i_64Ptr
      .asFunction<ffi.Pointer<ffi.Int64> Function(int)>();

  ffi.Pointer<ffi.Int8> new_box_autoadd_i_8(
    int value,
  ) {
    return _new_box_autoadd_i_8(
      value,
    );
  }

  late final _new_box_autoadd_i_8Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int8> Function(ffi.Int8)>>(
          'new_box_autoadd_i_8');
  late final _new_box_autoadd_i_8 =
      _new_box_autoadd_i_8Ptr.asFunction<ffi.Pointer<ffi.Int8> Function(int)>();

  ffi.Pointer<wire_struct_with_comments_twin_normal>
      new_box_autoadd_struct_with_comments_twin_normal() {
    return _new_box_autoadd_struct_with_comments_twin_normal();
  }

  late final _new_box_autoadd_struct_with_comments_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_comments_twin_normal>
              Function()>>('new_box_autoadd_struct_with_comments_twin_normal');
  late final _new_box_autoadd_struct_with_comments_twin_normal =
      _new_box_autoadd_struct_with_comments_twin_normalPtr.asFunction<
          ffi.Pointer<wire_struct_with_comments_twin_normal> Function()>();

  ffi.Pointer<wire_struct_with_comments_twin_sync>
      new_box_autoadd_struct_with_comments_twin_sync() {
    return _new_box_autoadd_struct_with_comments_twin_sync();
  }

  late final _new_box_autoadd_struct_with_comments_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_comments_twin_sync>
              Function()>>('new_box_autoadd_struct_with_comments_twin_sync');
  late final _new_box_autoadd_struct_with_comments_twin_sync =
      _new_box_autoadd_struct_with_comments_twin_syncPtr.asFunction<
          ffi.Pointer<wire_struct_with_comments_twin_sync> Function()>();

  ffi.Pointer<wire_struct_with_zero_field>
      new_box_autoadd_struct_with_zero_field() {
    return _new_box_autoadd_struct_with_zero_field();
  }

  late final _new_box_autoadd_struct_with_zero_fieldPtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_struct_with_zero_field> Function()>>(
      'new_box_autoadd_struct_with_zero_field');
  late final _new_box_autoadd_struct_with_zero_field =
      _new_box_autoadd_struct_with_zero_fieldPtr
          .asFunction<ffi.Pointer<wire_struct_with_zero_field> Function()>();

  ffi.Pointer<ffi.Uint16> new_box_autoadd_u_16(
    int value,
  ) {
    return _new_box_autoadd_u_16(
      value,
    );
  }

  late final _new_box_autoadd_u_16Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint16> Function(ffi.Uint16)>>(
          'new_box_autoadd_u_16');
  late final _new_box_autoadd_u_16 = _new_box_autoadd_u_16Ptr
      .asFunction<ffi.Pointer<ffi.Uint16> Function(int)>();

  ffi.Pointer<ffi.Uint32> new_box_autoadd_u_32(
    int value,
  ) {
    return _new_box_autoadd_u_32(
      value,
    );
  }

  late final _new_box_autoadd_u_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint32> Function(ffi.Uint32)>>(
          'new_box_autoadd_u_32');
  late final _new_box_autoadd_u_32 = _new_box_autoadd_u_32Ptr
      .asFunction<ffi.Pointer<ffi.Uint32> Function(int)>();

  ffi.Pointer<ffi.Uint64> new_box_autoadd_u_64(
    int value,
  ) {
    return _new_box_autoadd_u_64(
      value,
    );
  }

  late final _new_box_autoadd_u_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint64> Function(ffi.Uint64)>>(
          'new_box_autoadd_u_64');
  late final _new_box_autoadd_u_64 = _new_box_autoadd_u_64Ptr
      .asFunction<ffi.Pointer<ffi.Uint64> Function(int)>();

  ffi.Pointer<ffi.Uint8> new_box_autoadd_u_8(
    int value,
  ) {
    return _new_box_autoadd_u_8(
      value,
    );
  }

  late final _new_box_autoadd_u_8Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint8> Function(ffi.Uint8)>>(
          'new_box_autoadd_u_8');
  late final _new_box_autoadd_u_8 = _new_box_autoadd_u_8Ptr
      .asFunction<ffi.Pointer<ffi.Uint8> Function(int)>();

  ffi.Pointer<wire_list_bool> new_list_bool(
    int len,
  ) {
    return _new_list_bool(
      len,
    );
  }

  late final _new_list_boolPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_list_bool> Function(ffi.Int32)>>(
      'new_list_bool');
  late final _new_list_bool =
      _new_list_boolPtr.asFunction<ffi.Pointer<wire_list_bool> Function(int)>();

  ffi.Pointer<wire_list_prim_f_32> new_list_prim_f_32(
    int len,
  ) {
    return _new_list_prim_f_32(
      len,
    );
  }

  late final _new_list_prim_f_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_f_32> Function(
              ffi.Int32)>>('new_list_prim_f_32');
  late final _new_list_prim_f_32 = _new_list_prim_f_32Ptr
      .asFunction<ffi.Pointer<wire_list_prim_f_32> Function(int)>();

  ffi.Pointer<wire_list_prim_f_64> new_list_prim_f_64(
    int len,
  ) {
    return _new_list_prim_f_64(
      len,
    );
  }

  late final _new_list_prim_f_64Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_f_64> Function(
              ffi.Int32)>>('new_list_prim_f_64');
  late final _new_list_prim_f_64 = _new_list_prim_f_64Ptr
      .asFunction<ffi.Pointer<wire_list_prim_f_64> Function(int)>();

  ffi.Pointer<wire_list_prim_i_16> new_list_prim_i_16(
    int len,
  ) {
    return _new_list_prim_i_16(
      len,
    );
  }

  late final _new_list_prim_i_16Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_i_16> Function(
              ffi.Int32)>>('new_list_prim_i_16');
  late final _new_list_prim_i_16 = _new_list_prim_i_16Ptr
      .asFunction<ffi.Pointer<wire_list_prim_i_16> Function(int)>();

  ffi.Pointer<wire_list_prim_i_32> new_list_prim_i_32(
    int len,
  ) {
    return _new_list_prim_i_32(
      len,
    );
  }

  late final _new_list_prim_i_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_i_32> Function(
              ffi.Int32)>>('new_list_prim_i_32');
  late final _new_list_prim_i_32 = _new_list_prim_i_32Ptr
      .asFunction<ffi.Pointer<wire_list_prim_i_32> Function(int)>();

  ffi.Pointer<wire_list_prim_i_64> new_list_prim_i_64(
    int len,
  ) {
    return _new_list_prim_i_64(
      len,
    );
  }

  late final _new_list_prim_i_64Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_i_64> Function(
              ffi.Int32)>>('new_list_prim_i_64');
  late final _new_list_prim_i_64 = _new_list_prim_i_64Ptr
      .asFunction<ffi.Pointer<wire_list_prim_i_64> Function(int)>();

  ffi.Pointer<wire_list_prim_i_8> new_list_prim_i_8(
    int len,
  ) {
    return _new_list_prim_i_8(
      len,
    );
  }

  late final _new_list_prim_i_8Ptr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_list_prim_i_8> Function(ffi.Int32)>>(
      'new_list_prim_i_8');
  late final _new_list_prim_i_8 = _new_list_prim_i_8Ptr
      .asFunction<ffi.Pointer<wire_list_prim_i_8> Function(int)>();

  ffi.Pointer<wire_list_prim_u_16> new_list_prim_u_16(
    int len,
  ) {
    return _new_list_prim_u_16(
      len,
    );
  }

  late final _new_list_prim_u_16Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_u_16> Function(
              ffi.Int32)>>('new_list_prim_u_16');
  late final _new_list_prim_u_16 = _new_list_prim_u_16Ptr
      .asFunction<ffi.Pointer<wire_list_prim_u_16> Function(int)>();

  ffi.Pointer<wire_list_prim_u_32> new_list_prim_u_32(
    int len,
  ) {
    return _new_list_prim_u_32(
      len,
    );
  }

  late final _new_list_prim_u_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_u_32> Function(
              ffi.Int32)>>('new_list_prim_u_32');
  late final _new_list_prim_u_32 = _new_list_prim_u_32Ptr
      .asFunction<ffi.Pointer<wire_list_prim_u_32> Function(int)>();

  ffi.Pointer<wire_list_prim_u_64> new_list_prim_u_64(
    int len,
  ) {
    return _new_list_prim_u_64(
      len,
    );
  }

  late final _new_list_prim_u_64Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_prim_u_64> Function(
              ffi.Int32)>>('new_list_prim_u_64');
  late final _new_list_prim_u_64 = _new_list_prim_u_64Ptr
      .asFunction<ffi.Pointer<wire_list_prim_u_64> Function(int)>();

  ffi.Pointer<wire_list_prim_u_8> new_list_prim_u_8(
    int len,
  ) {
    return _new_list_prim_u_8(
      len,
    );
  }

  late final _new_list_prim_u_8Ptr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_list_prim_u_8> Function(ffi.Int32)>>(
      'new_list_prim_u_8');
  late final _new_list_prim_u_8 = _new_list_prim_u_8Ptr
      .asFunction<ffi.Pointer<wire_list_prim_u_8> Function(int)>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>(
          'dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

final class wire_struct_with_comments_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int field_with_comments;
}

final class wire_struct_with_comments_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int field_with_comments;
}

final class wire_list_bool extends ffi.Struct {
  external ffi.Pointer<ffi.Bool> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_f_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Float> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_f_64 extends ffi.Struct {
  external ffi.Pointer<ffi.Double> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_16 extends ffi.Struct {
  external ffi.Pointer<ffi.Int16> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_64 extends ffi.Struct {
  external ffi.Pointer<ffi.Int64> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_8 extends ffi.Struct {
  external ffi.Pointer<ffi.Int8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_u_16 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint16> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_u_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_u_64 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint64> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_u_8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_struct_with_zero_field extends ffi.Opaque {}

// Section: api_fill_to_wire_funcs

void _api_fill_to_wire_box_autoadd_struct_with_comments_twin_normal(
    StructWithCommentsTwinNormal apiObj,
    ffi.Pointer<wire_struct_with_comments_twin_normal> wireObj) {
  _api_fill_to_wire_struct_with_comments_twin_normal(apiObj, wireObj.ref);
}

void _api_fill_to_wire_box_autoadd_struct_with_comments_twin_sync(
    StructWithCommentsTwinSync apiObj,
    ffi.Pointer<wire_struct_with_comments_twin_sync> wireObj) {
  _api_fill_to_wire_struct_with_comments_twin_sync(apiObj, wireObj.ref);
}

void _api_fill_to_wire_struct_with_comments_twin_normal(
    StructWithCommentsTwinNormal apiObj,
    wire_struct_with_comments_twin_normal wireObj) {
  wireObj.field_with_comments = api2wire_i_32(apiObj.fieldWithComments);
}

void _api_fill_to_wire_struct_with_comments_twin_sync(
    StructWithCommentsTwinSync apiObj,
    wire_struct_with_comments_twin_sync wireObj) {
  wireObj.field_with_comments = api2wire_i_32(apiObj.fieldWithComments);
}

void _api_fill_to_wire_struct_with_zero_field(
    StructWithZeroField apiObj, wire_struct_with_zero_field wireObj) {}
