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
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @protected
  bool api2wire_box_autoadd_bool(bool raw) {
    return api2wire_bool(raw);
  }

  @protected
  double api2wire_box_autoadd_f_32(double raw) {
    return api2wire_f_32(raw);
  }

  @protected
  double api2wire_box_autoadd_f_64(double raw) {
    return api2wire_f_64(raw);
  }

  @protected
  int api2wire_box_autoadd_i_16(int raw) {
    return api2wire_i_16(raw);
  }

  @protected
  int api2wire_box_autoadd_i_32(int raw) {
    return api2wire_i_32(raw);
  }

  @protected
  Object api2wire_box_autoadd_i_64(BigInt raw) {
    return api2wire_i_64(raw);
  }

  @protected
  int api2wire_box_autoadd_i_8(int raw) {
    return api2wire_i_8(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_comments_twin_normal(
      StructWithCommentsTwinNormal raw) {
    return api2wire_struct_with_comments_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_comments_twin_sync(
      StructWithCommentsTwinSync raw) {
    return api2wire_struct_with_comments_twin_sync(raw);
  }

  @protected
  int api2wire_box_autoadd_u_16(int raw) {
    return api2wire_u_16(raw);
  }

  @protected
  int api2wire_box_autoadd_u_32(int raw) {
    return api2wire_u_32(raw);
  }

  @protected
  Object api2wire_box_autoadd_u_64(BigInt raw) {
    return api2wire_u_64(raw);
  }

  @protected
  int api2wire_box_autoadd_u_8(int raw) {
    return api2wire_u_8(raw);
  }

  @protected
  Object api2wire_i_64(BigInt raw) {
    return castNativeBigInt(raw);
  }

  @protected
  ffi.Pointer<wire_list_bool> api2wire_list_bool(List<bool> raw) {
    return raw.map(api2wire_bool).toList();
  }

  @protected
  Float32List api2wire_list_prim_f_32(Float32List raw) {
    return raw;
  }

  @protected
  Float64List api2wire_list_prim_f_64(Float64List raw) {
    return raw;
  }

  @protected
  Int16List api2wire_list_prim_i_16(Int16List raw) {
    return raw;
  }

  @protected
  Int32List api2wire_list_prim_i_32(Int32List raw) {
    return raw;
  }

  @protected
  Object /* BigInt64Array */ api2wire_list_prim_i_64(Int64List raw) {
    return raw.inner;
  }

  @protected
  Int8List api2wire_list_prim_i_8(Int8List raw) {
    return raw;
  }

  @protected
  Uint16List api2wire_list_prim_u_16(Uint16List raw) {
    return raw;
  }

  @protected
  Uint32List api2wire_list_prim_u_32(Uint32List raw) {
    return raw;
  }

  @protected
  Object /* BigInt64Array */ api2wire_list_prim_u_64(Uint64List raw) {
    return raw.inner;
  }

  @protected
  Uint8List api2wire_list_prim_u_8(Uint8List raw) {
    return raw;
  }

  @protected
  bool? api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? null : api2wire_box_autoadd_bool(raw);
  }

  @protected
  double? api2wire_opt_box_autoadd_f_32(double? raw) {
    return raw == null ? null : api2wire_box_autoadd_f_32(raw);
  }

  @protected
  double? api2wire_opt_box_autoadd_f_64(double? raw) {
    return raw == null ? null : api2wire_box_autoadd_f_64(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_i_16(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_i_16(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_i_32(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_i_32(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_i_64(BigInt? raw) {
    return raw == null ? null : api2wire_box_autoadd_i_64(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_i_8(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_i_8(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u_16(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u_16(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u_32(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u_32(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_u_64(BigInt? raw) {
    return raw == null ? null : api2wire_box_autoadd_u_64(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u_8(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u_8(raw);
  }

  @protected
  List<dynamic> api2wire_struct_with_comments_twin_normal(
      StructWithCommentsTwinNormal raw) {
    return [api2wire_i_32(raw.fieldWithComments)];
  }

  @protected
  List<dynamic> api2wire_struct_with_comments_twin_sync(
      StructWithCommentsTwinSync raw) {
    return [api2wire_i_32(raw.fieldWithComments)];
  }

  @protected
  Object api2wire_u_64(BigInt raw) {
    return castNativeBigInt(raw);
  }
}

// Section: boilerplate

class RustLibWire extends BaseWire {
  // TODO
}
