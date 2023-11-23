// ignore_for_file: unused_import, unused_element

import 'api/comment.dart';
import 'api/enumeration.dart';
import 'api/exception.dart';
import 'api/inside_macro.dart';
import 'api/misc_type.dart';
import 'api/pseudo_manual/comment_twin_sync.dart';
import 'api/pseudo_manual/enumeration_twin_sync.dart';
import 'api/pseudo_manual/exception_twin_sync.dart';
import 'api/pseudo_manual/misc_type_twin_sync.dart';
import 'api/pseudo_manual/optional_primitive.dart';
import 'api/pseudo_manual/optional_primitive_twin_sync.dart';
import 'api/pseudo_manual/primitive.dart';
import 'api/pseudo_manual/primitive_list.dart';
import 'api/pseudo_manual/primitive_list_twin_sync.dart';
import 'api/pseudo_manual/primitive_twin_sync.dart';
import 'api/pseudo_manual/simple_twin_sync.dart';
import 'api/pseudo_manual/structure_twin_sync.dart';
import 'api/simple.dart';
import 'api/stream.dart';
import 'api/structure.dart';
import 'dart:async';
import 'dart:convert';
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
  ffi.Pointer<wire_list_prim_u_8> api2wire_String(String raw) {
    return api2wire_list_prim_u_8(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<ffi.Bool> api2wire_box_autoadd_bool(bool raw) {
    return wire.new_box_autoadd_bool(api2wire_bool(raw));
  }

  @protected
  ffi.Pointer<wire_custom_nested_error_inner_twin_normal>
      api2wire_box_autoadd_custom_nested_error_inner_twin_normal(
          CustomNestedErrorInnerTwinNormal raw) {
    final ptr = wire.new_box_autoadd_custom_nested_error_inner_twin_normal();
    _api_fill_to_wire_custom_nested_error_inner_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_custom_nested_error_outer_twin_normal>
      api2wire_box_autoadd_custom_nested_error_outer_twin_normal(
          CustomNestedErrorOuterTwinNormal raw) {
    final ptr = wire.new_box_autoadd_custom_nested_error_outer_twin_normal();
    _api_fill_to_wire_custom_nested_error_outer_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_custom_struct_error_twin_normal>
      api2wire_box_autoadd_custom_struct_error_twin_normal(
          CustomStructErrorTwinNormal raw) {
    final ptr = wire.new_box_autoadd_custom_struct_error_twin_normal();
    _api_fill_to_wire_custom_struct_error_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_mixed_twin_normal>
      api2wire_box_autoadd_enum_with_item_mixed_twin_normal(
          EnumWithItemMixedTwinNormal raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_mixed_twin_normal();
    _api_fill_to_wire_enum_with_item_mixed_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_mixed_twin_sync>
      api2wire_box_autoadd_enum_with_item_mixed_twin_sync(
          EnumWithItemMixedTwinSync raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_mixed_twin_sync();
    _api_fill_to_wire_enum_with_item_mixed_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_struct_twin_normal>
      api2wire_box_autoadd_enum_with_item_struct_twin_normal(
          EnumWithItemStructTwinNormal raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_struct_twin_normal();
    _api_fill_to_wire_enum_with_item_struct_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_struct_twin_sync>
      api2wire_box_autoadd_enum_with_item_struct_twin_sync(
          EnumWithItemStructTwinSync raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_struct_twin_sync();
    _api_fill_to_wire_enum_with_item_struct_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_tuple_twin_normal>
      api2wire_box_autoadd_enum_with_item_tuple_twin_normal(
          EnumWithItemTupleTwinNormal raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_tuple_twin_normal();
    _api_fill_to_wire_enum_with_item_tuple_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_with_item_tuple_twin_sync>
      api2wire_box_autoadd_enum_with_item_tuple_twin_sync(
          EnumWithItemTupleTwinSync raw) {
    final ptr = wire.new_box_autoadd_enum_with_item_tuple_twin_sync();
    _api_fill_to_wire_enum_with_item_tuple_twin_sync(raw, ptr.ref);
    return ptr;
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
  ffi.Pointer<wire_macro_struct> api2wire_box_autoadd_macro_struct(
      MacroStruct raw) {
    final ptr = wire.new_box_autoadd_macro_struct();
    _api_fill_to_wire_macro_struct(raw, ptr.ref);
    return ptr;
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
  ffi.Pointer<wire_struct_with_one_field_twin_normal>
      api2wire_box_autoadd_struct_with_one_field_twin_normal(
          StructWithOneFieldTwinNormal raw) {
    final ptr = wire.new_box_autoadd_struct_with_one_field_twin_normal();
    _api_fill_to_wire_struct_with_one_field_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_one_field_twin_sync>
      api2wire_box_autoadd_struct_with_one_field_twin_sync(
          StructWithOneFieldTwinSync raw) {
    final ptr = wire.new_box_autoadd_struct_with_one_field_twin_sync();
    _api_fill_to_wire_struct_with_one_field_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_two_field_twin_normal>
      api2wire_box_autoadd_struct_with_two_field_twin_normal(
          StructWithTwoFieldTwinNormal raw) {
    final ptr = wire.new_box_autoadd_struct_with_two_field_twin_normal();
    _api_fill_to_wire_struct_with_two_field_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_two_field_twin_sync>
      api2wire_box_autoadd_struct_with_two_field_twin_sync(
          StructWithTwoFieldTwinSync raw) {
    final ptr = wire.new_box_autoadd_struct_with_two_field_twin_sync();
    _api_fill_to_wire_struct_with_two_field_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_zero_field_twin_normal>
      api2wire_box_autoadd_struct_with_zero_field_twin_normal(
          StructWithZeroFieldTwinNormal raw) {
    final ptr = wire.new_box_autoadd_struct_with_zero_field_twin_normal();

    return ptr;
  }

  @protected
  ffi.Pointer<wire_struct_with_zero_field_twin_sync>
      api2wire_box_autoadd_struct_with_zero_field_twin_sync(
          StructWithZeroFieldTwinSync raw) {
    final ptr = wire.new_box_autoadd_struct_with_zero_field_twin_sync();

    return ptr;
  }

  @protected
  ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>
      api2wire_box_autoadd_tuple_struct_with_one_field_twin_normal(
          TupleStructWithOneFieldTwinNormal raw) {
    final ptr = wire.new_box_autoadd_tuple_struct_with_one_field_twin_normal();
    _api_fill_to_wire_tuple_struct_with_one_field_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync>
      api2wire_box_autoadd_tuple_struct_with_one_field_twin_sync(
          TupleStructWithOneFieldTwinSync raw) {
    final ptr = wire.new_box_autoadd_tuple_struct_with_one_field_twin_sync();
    _api_fill_to_wire_tuple_struct_with_one_field_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>
      api2wire_box_autoadd_tuple_struct_with_two_field_twin_normal(
          TupleStructWithTwoFieldTwinNormal raw) {
    final ptr = wire.new_box_autoadd_tuple_struct_with_two_field_twin_normal();
    _api_fill_to_wire_tuple_struct_with_two_field_twin_normal(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync>
      api2wire_box_autoadd_tuple_struct_with_two_field_twin_sync(
          TupleStructWithTwoFieldTwinSync raw) {
    final ptr = wire.new_box_autoadd_tuple_struct_with_two_field_twin_sync();
    _api_fill_to_wire_tuple_struct_with_two_field_twin_sync(raw, ptr.ref);
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

  void _api_fill_to_wire_box_autoadd_custom_nested_error_inner_twin_normal(
      CustomNestedErrorInnerTwinNormal apiObj,
      ffi.Pointer<wire_custom_nested_error_inner_twin_normal> wireObj) {
    _api_fill_to_wire_custom_nested_error_inner_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_nested_error_outer_twin_normal(
      CustomNestedErrorOuterTwinNormal apiObj,
      ffi.Pointer<wire_custom_nested_error_outer_twin_normal> wireObj) {
    _api_fill_to_wire_custom_nested_error_outer_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal apiObj,
      ffi.Pointer<wire_custom_struct_error_twin_normal> wireObj) {
    _api_fill_to_wire_custom_struct_error_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_mixed_twin_normal(
      EnumWithItemMixedTwinNormal apiObj,
      ffi.Pointer<wire_enum_with_item_mixed_twin_normal> wireObj) {
    _api_fill_to_wire_enum_with_item_mixed_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_mixed_twin_sync(
      EnumWithItemMixedTwinSync apiObj,
      ffi.Pointer<wire_enum_with_item_mixed_twin_sync> wireObj) {
    _api_fill_to_wire_enum_with_item_mixed_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_struct_twin_normal(
      EnumWithItemStructTwinNormal apiObj,
      ffi.Pointer<wire_enum_with_item_struct_twin_normal> wireObj) {
    _api_fill_to_wire_enum_with_item_struct_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_struct_twin_sync(
      EnumWithItemStructTwinSync apiObj,
      ffi.Pointer<wire_enum_with_item_struct_twin_sync> wireObj) {
    _api_fill_to_wire_enum_with_item_struct_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_tuple_twin_normal(
      EnumWithItemTupleTwinNormal apiObj,
      ffi.Pointer<wire_enum_with_item_tuple_twin_normal> wireObj) {
    _api_fill_to_wire_enum_with_item_tuple_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_with_item_tuple_twin_sync(
      EnumWithItemTupleTwinSync apiObj,
      ffi.Pointer<wire_enum_with_item_tuple_twin_sync> wireObj) {
    _api_fill_to_wire_enum_with_item_tuple_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_macro_struct(
      MacroStruct apiObj, ffi.Pointer<wire_macro_struct> wireObj) {
    _api_fill_to_wire_macro_struct(apiObj, wireObj.ref);
  }

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

  void _api_fill_to_wire_box_autoadd_struct_with_one_field_twin_normal(
      StructWithOneFieldTwinNormal apiObj,
      ffi.Pointer<wire_struct_with_one_field_twin_normal> wireObj) {
    _api_fill_to_wire_struct_with_one_field_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_struct_with_one_field_twin_sync(
      StructWithOneFieldTwinSync apiObj,
      ffi.Pointer<wire_struct_with_one_field_twin_sync> wireObj) {
    _api_fill_to_wire_struct_with_one_field_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_struct_with_two_field_twin_normal(
      StructWithTwoFieldTwinNormal apiObj,
      ffi.Pointer<wire_struct_with_two_field_twin_normal> wireObj) {
    _api_fill_to_wire_struct_with_two_field_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_struct_with_two_field_twin_sync(
      StructWithTwoFieldTwinSync apiObj,
      ffi.Pointer<wire_struct_with_two_field_twin_sync> wireObj) {
    _api_fill_to_wire_struct_with_two_field_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tuple_struct_with_one_field_twin_normal(
      TupleStructWithOneFieldTwinNormal apiObj,
      ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal> wireObj) {
    _api_fill_to_wire_tuple_struct_with_one_field_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tuple_struct_with_one_field_twin_sync(
      TupleStructWithOneFieldTwinSync apiObj,
      ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync> wireObj) {
    _api_fill_to_wire_tuple_struct_with_one_field_twin_sync(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tuple_struct_with_two_field_twin_normal(
      TupleStructWithTwoFieldTwinNormal apiObj,
      ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal> wireObj) {
    _api_fill_to_wire_tuple_struct_with_two_field_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tuple_struct_with_two_field_twin_sync(
      TupleStructWithTwoFieldTwinSync apiObj,
      ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync> wireObj) {
    _api_fill_to_wire_tuple_struct_with_two_field_twin_sync(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_custom_nested_error_inner_twin_normal(
      CustomNestedErrorInnerTwinNormal apiObj,
      wire_custom_nested_error_inner_twin_normal wireObj) {
    if (apiObj is CustomNestedErrorInnerTwinNormal_Three) {
      var pre_field0 = api2wire_String(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_CustomNestedErrorInnerTwinNormal_Three();
      wireObj.kind.ref.Three.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is CustomNestedErrorInnerTwinNormal_Four) {
      var pre_field0 = api2wire_u_32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_CustomNestedErrorInnerTwinNormal_Four();
      wireObj.kind.ref.Four.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_custom_nested_error_outer_twin_normal(
      CustomNestedErrorOuterTwinNormal apiObj,
      wire_custom_nested_error_outer_twin_normal wireObj) {
    if (apiObj is CustomNestedErrorOuterTwinNormal_One) {
      var pre_field0 = api2wire_String(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_CustomNestedErrorOuterTwinNormal_One();
      wireObj.kind.ref.One.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is CustomNestedErrorOuterTwinNormal_Two) {
      var pre_field0 =
          api2wire_box_autoadd_custom_nested_error_inner_twin_normal(
              apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_CustomNestedErrorOuterTwinNormal_Two();
      wireObj.kind.ref.Two.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal apiObj,
      wire_custom_struct_error_twin_normal wireObj) {
    wireObj.a = api2wire_String(apiObj.a);
  }

  void _api_fill_to_wire_enum_with_item_mixed_twin_normal(
      EnumWithItemMixedTwinNormal apiObj,
      wire_enum_with_item_mixed_twin_normal wireObj) {
    if (apiObj is EnumWithItemMixedTwinNormal_A) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is EnumWithItemMixedTwinNormal_B) {
      var pre_field0 = api2wire_list_prim_u_8(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemMixedTwinNormal_B();
      wireObj.kind.ref.B.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumWithItemMixedTwinNormal_C) {
      var pre_c_field = api2wire_String(apiObj.cField);
      wireObj.tag = 2;
      wireObj.kind = wire.inflate_EnumWithItemMixedTwinNormal_C();
      wireObj.kind.ref.C.ref.c_field = pre_c_field;
      return;
    }
  }

  void _api_fill_to_wire_enum_with_item_mixed_twin_sync(
      EnumWithItemMixedTwinSync apiObj,
      wire_enum_with_item_mixed_twin_sync wireObj) {
    if (apiObj is EnumWithItemMixedTwinSync_A) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is EnumWithItemMixedTwinSync_B) {
      var pre_field0 = api2wire_list_prim_u_8(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemMixedTwinSync_B();
      wireObj.kind.ref.B.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumWithItemMixedTwinSync_C) {
      var pre_c_field = api2wire_String(apiObj.cField);
      wireObj.tag = 2;
      wireObj.kind = wire.inflate_EnumWithItemMixedTwinSync_C();
      wireObj.kind.ref.C.ref.c_field = pre_c_field;
      return;
    }
  }

  void _api_fill_to_wire_enum_with_item_struct_twin_normal(
      EnumWithItemStructTwinNormal apiObj,
      wire_enum_with_item_struct_twin_normal wireObj) {
    if (apiObj is EnumWithItemStructTwinNormal_A) {
      var pre_a_field = api2wire_list_prim_u_8(apiObj.aField);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumWithItemStructTwinNormal_A();
      wireObj.kind.ref.A.ref.a_field = pre_a_field;
      return;
    }
    if (apiObj is EnumWithItemStructTwinNormal_B) {
      var pre_b_field = api2wire_list_prim_i_32(apiObj.bField);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemStructTwinNormal_B();
      wireObj.kind.ref.B.ref.b_field = pre_b_field;
      return;
    }
  }

  void _api_fill_to_wire_enum_with_item_struct_twin_sync(
      EnumWithItemStructTwinSync apiObj,
      wire_enum_with_item_struct_twin_sync wireObj) {
    if (apiObj is EnumWithItemStructTwinSync_A) {
      var pre_a_field = api2wire_list_prim_u_8(apiObj.aField);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumWithItemStructTwinSync_A();
      wireObj.kind.ref.A.ref.a_field = pre_a_field;
      return;
    }
    if (apiObj is EnumWithItemStructTwinSync_B) {
      var pre_b_field = api2wire_list_prim_i_32(apiObj.bField);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemStructTwinSync_B();
      wireObj.kind.ref.B.ref.b_field = pre_b_field;
      return;
    }
  }

  void _api_fill_to_wire_enum_with_item_tuple_twin_normal(
      EnumWithItemTupleTwinNormal apiObj,
      wire_enum_with_item_tuple_twin_normal wireObj) {
    if (apiObj is EnumWithItemTupleTwinNormal_A) {
      var pre_field0 = api2wire_list_prim_u_8(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumWithItemTupleTwinNormal_A();
      wireObj.kind.ref.A.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumWithItemTupleTwinNormal_B) {
      var pre_field0 = api2wire_list_prim_i_32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemTupleTwinNormal_B();
      wireObj.kind.ref.B.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_enum_with_item_tuple_twin_sync(
      EnumWithItemTupleTwinSync apiObj,
      wire_enum_with_item_tuple_twin_sync wireObj) {
    if (apiObj is EnumWithItemTupleTwinSync_A) {
      var pre_field0 = api2wire_list_prim_u_8(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumWithItemTupleTwinSync_A();
      wireObj.kind.ref.A.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumWithItemTupleTwinSync_B) {
      var pre_field0 = api2wire_list_prim_i_32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumWithItemTupleTwinSync_B();
      wireObj.kind.ref.B.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_macro_struct(
      MacroStruct apiObj, wire_macro_struct wireObj) {
    wireObj.data = api2wire_i_32(apiObj.data);
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

  void _api_fill_to_wire_struct_with_one_field_twin_normal(
      StructWithOneFieldTwinNormal apiObj,
      wire_struct_with_one_field_twin_normal wireObj) {
    wireObj.a = api2wire_i_32(apiObj.a);
  }

  void _api_fill_to_wire_struct_with_one_field_twin_sync(
      StructWithOneFieldTwinSync apiObj,
      wire_struct_with_one_field_twin_sync wireObj) {
    wireObj.a = api2wire_i_32(apiObj.a);
  }

  void _api_fill_to_wire_struct_with_two_field_twin_normal(
      StructWithTwoFieldTwinNormal apiObj,
      wire_struct_with_two_field_twin_normal wireObj) {
    wireObj.a = api2wire_i_32(apiObj.a);
    wireObj.b = api2wire_i_32(apiObj.b);
  }

  void _api_fill_to_wire_struct_with_two_field_twin_sync(
      StructWithTwoFieldTwinSync apiObj,
      wire_struct_with_two_field_twin_sync wireObj) {
    wireObj.a = api2wire_i_32(apiObj.a);
    wireObj.b = api2wire_i_32(apiObj.b);
  }

  void _api_fill_to_wire_struct_with_zero_field_twin_normal(
      StructWithZeroFieldTwinNormal apiObj,
      wire_struct_with_zero_field_twin_normal wireObj) {}
  void _api_fill_to_wire_struct_with_zero_field_twin_sync(
      StructWithZeroFieldTwinSync apiObj,
      wire_struct_with_zero_field_twin_sync wireObj) {}
  void _api_fill_to_wire_tuple_struct_with_one_field_twin_normal(
      TupleStructWithOneFieldTwinNormal apiObj,
      wire_tuple_struct_with_one_field_twin_normal wireObj) {
    wireObj.field0 = api2wire_i_32(apiObj.field0);
  }

  void _api_fill_to_wire_tuple_struct_with_one_field_twin_sync(
      TupleStructWithOneFieldTwinSync apiObj,
      wire_tuple_struct_with_one_field_twin_sync wireObj) {
    wireObj.field0 = api2wire_i_32(apiObj.field0);
  }

  void _api_fill_to_wire_tuple_struct_with_two_field_twin_normal(
      TupleStructWithTwoFieldTwinNormal apiObj,
      wire_tuple_struct_with_two_field_twin_normal wireObj) {
    wireObj.field0 = api2wire_i_32(apiObj.field0);
    wireObj.field1 = api2wire_i_32(apiObj.field1);
  }

  void _api_fill_to_wire_tuple_struct_with_two_field_twin_sync(
      TupleStructWithTwoFieldTwinSync apiObj,
      wire_tuple_struct_with_two_field_twin_sync wireObj) {
    wireObj.field0 = api2wire_i_32(apiObj.field0);
    wireObj.field1 = api2wire_i_32(apiObj.field1);
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

  void wire_func_enum_simple_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_func_enum_simple_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_enum_simple_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_func_enum_simple_twin_normal');
  late final _wire_func_enum_simple_twin_normal =
      _wire_func_enum_simple_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_func_enum_with_item_mixed_twin_normal(
    int port_,
    ffi.Pointer<wire_enum_with_item_mixed_twin_normal> arg,
  ) {
    return _wire_func_enum_with_item_mixed_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_enum_with_item_mixed_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_enum_with_item_mixed_twin_normal>)>>(
      'wire_func_enum_with_item_mixed_twin_normal');
  late final _wire_func_enum_with_item_mixed_twin_normal =
      _wire_func_enum_with_item_mixed_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_enum_with_item_mixed_twin_normal>)>();

  void wire_func_enum_with_item_struct_twin_normal(
    int port_,
    ffi.Pointer<wire_enum_with_item_struct_twin_normal> arg,
  ) {
    return _wire_func_enum_with_item_struct_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_enum_with_item_struct_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_enum_with_item_struct_twin_normal>)>>(
      'wire_func_enum_with_item_struct_twin_normal');
  late final _wire_func_enum_with_item_struct_twin_normal =
      _wire_func_enum_with_item_struct_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_enum_with_item_struct_twin_normal>)>();

  void wire_func_enum_with_item_tuple_twin_normal(
    int port_,
    ffi.Pointer<wire_enum_with_item_tuple_twin_normal> arg,
  ) {
    return _wire_func_enum_with_item_tuple_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_enum_with_item_tuple_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_enum_with_item_tuple_twin_normal>)>>(
      'wire_func_enum_with_item_tuple_twin_normal');
  late final _wire_func_enum_with_item_tuple_twin_normal =
      _wire_func_enum_with_item_tuple_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_enum_with_item_tuple_twin_normal>)>();

  void wire_custom_enum_error_panic_twin_normal(
    int port_,
  ) {
    return _wire_custom_enum_error_panic_twin_normal(
      port_,
    );
  }

  late final _wire_custom_enum_error_panic_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_custom_enum_error_panic_twin_normal');
  late final _wire_custom_enum_error_panic_twin_normal =
      _wire_custom_enum_error_panic_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_custom_enum_error_return_error_twin_normal(
    int port_,
  ) {
    return _wire_custom_enum_error_return_error_twin_normal(
      port_,
    );
  }

  late final _wire_custom_enum_error_return_error_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_custom_enum_error_return_error_twin_normal');
  late final _wire_custom_enum_error_return_error_twin_normal =
      _wire_custom_enum_error_return_error_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_custom_enum_error_return_ok_twin_normal(
    int port_,
    int arg,
  ) {
    return _wire_custom_enum_error_return_ok_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_custom_enum_error_return_ok_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint32)>>(
          'wire_custom_enum_error_return_ok_twin_normal');
  late final _wire_custom_enum_error_return_ok_twin_normal =
      _wire_custom_enum_error_return_ok_twin_normalPtr
          .asFunction<void Function(int, int)>();

  void wire_custom_nested_error_return_error_twin_normal(
    int port_,
    ffi.Pointer<wire_custom_nested_error_outer_twin_normal> arg,
  ) {
    return _wire_custom_nested_error_return_error_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_custom_nested_error_return_error_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_custom_nested_error_outer_twin_normal>)>>(
      'wire_custom_nested_error_return_error_twin_normal');
  late final _wire_custom_nested_error_return_error_twin_normal =
      _wire_custom_nested_error_return_error_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_custom_nested_error_outer_twin_normal>)>();

  void wire_custom_struct_error_return_error_twin_normal(
    int port_,
    ffi.Pointer<wire_custom_struct_error_twin_normal> arg,
  ) {
    return _wire_custom_struct_error_return_error_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_custom_struct_error_return_error_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_custom_struct_error_twin_normal>)>>(
      'wire_custom_struct_error_return_error_twin_normal');
  late final _wire_custom_struct_error_return_error_twin_normal =
      _wire_custom_struct_error_return_error_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_custom_struct_error_twin_normal>)>();

  void wire_func_return_error_twin_normal(
    int port_,
  ) {
    return _wire_func_return_error_twin_normal(
      port_,
    );
  }

  late final _wire_func_return_error_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_return_error_twin_normal');
  late final _wire_func_return_error_twin_normal =
      _wire_func_return_error_twin_normalPtr.asFunction<void Function(int)>();

  void wire_func_type_fallible_panic_twin_normal(
    int port_,
  ) {
    return _wire_func_type_fallible_panic_twin_normal(
      port_,
    );
  }

  late final _wire_func_type_fallible_panic_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_type_fallible_panic_twin_normal');
  late final _wire_func_type_fallible_panic_twin_normal =
      _wire_func_type_fallible_panic_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_func_type_infallible_panic_twin_normal(
    int port_,
  ) {
    return _wire_func_type_infallible_panic_twin_normal(
      port_,
    );
  }

  late final _wire_func_type_infallible_panic_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_type_infallible_panic_twin_normal');
  late final _wire_func_type_infallible_panic_twin_normal =
      _wire_func_type_infallible_panic_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_func_macro_struct(
    int port_,
    ffi.Pointer<wire_macro_struct> arg,
  ) {
    return _wire_func_macro_struct(
      port_,
      arg,
    );
  }

  late final _wire_func_macro_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_macro_struct>)>>('wire_func_macro_struct');
  late final _wire_func_macro_struct = _wire_func_macro_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_macro_struct>)>();

  void wire_func_return_unit_twin_normal(
    int port_,
  ) {
    return _wire_func_return_unit_twin_normal(
      port_,
    );
  }

  late final _wire_func_return_unit_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_return_unit_twin_normal');
  late final _wire_func_return_unit_twin_normal =
      _wire_func_return_unit_twin_normalPtr.asFunction<void Function(int)>();

  void wire_func_string_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> arg,
  ) {
    return _wire_func_string_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_string_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_func_string_twin_normal');
  late final _wire_func_string_twin_normal = _wire_func_string_twin_normalPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

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

  WireSyncReturn wire_func_enum_simple_twin_sync(
    int arg,
  ) {
    return _wire_func_enum_simple_twin_sync(
      arg,
    );
  }

  late final _wire_func_enum_simple_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Int32)>>(
          'wire_func_enum_simple_twin_sync');
  late final _wire_func_enum_simple_twin_sync =
      _wire_func_enum_simple_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_func_enum_with_item_mixed_twin_sync(
    ffi.Pointer<wire_enum_with_item_mixed_twin_sync> arg,
  ) {
    return _wire_func_enum_with_item_mixed_twin_sync(
      arg,
    );
  }

  late final _wire_func_enum_with_item_mixed_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_enum_with_item_mixed_twin_sync>)>>(
      'wire_func_enum_with_item_mixed_twin_sync');
  late final _wire_func_enum_with_item_mixed_twin_sync =
      _wire_func_enum_with_item_mixed_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_enum_with_item_mixed_twin_sync>)>();

  WireSyncReturn wire_func_enum_with_item_struct_twin_sync(
    ffi.Pointer<wire_enum_with_item_struct_twin_sync> arg,
  ) {
    return _wire_func_enum_with_item_struct_twin_sync(
      arg,
    );
  }

  late final _wire_func_enum_with_item_struct_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_enum_with_item_struct_twin_sync>)>>(
      'wire_func_enum_with_item_struct_twin_sync');
  late final _wire_func_enum_with_item_struct_twin_sync =
      _wire_func_enum_with_item_struct_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_enum_with_item_struct_twin_sync>)>();

  WireSyncReturn wire_func_enum_with_item_tuple_twin_sync(
    ffi.Pointer<wire_enum_with_item_tuple_twin_sync> arg,
  ) {
    return _wire_func_enum_with_item_tuple_twin_sync(
      arg,
    );
  }

  late final _wire_func_enum_with_item_tuple_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_enum_with_item_tuple_twin_sync>)>>(
      'wire_func_enum_with_item_tuple_twin_sync');
  late final _wire_func_enum_with_item_tuple_twin_sync =
      _wire_func_enum_with_item_tuple_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_enum_with_item_tuple_twin_sync>)>();

  WireSyncReturn wire_func_return_error_twin_sync() {
    return _wire_func_return_error_twin_sync();
  }

  late final _wire_func_return_error_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_return_error_twin_sync');
  late final _wire_func_return_error_twin_sync =
      _wire_func_return_error_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_func_return_panic_twin_sync() {
    return _wire_func_return_panic_twin_sync();
  }

  late final _wire_func_return_panic_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_return_panic_twin_sync');
  late final _wire_func_return_panic_twin_sync =
      _wire_func_return_panic_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_func_return_unit_twin_sync() {
    return _wire_func_return_unit_twin_sync();
  }

  late final _wire_func_return_unit_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_return_unit_twin_sync');
  late final _wire_func_return_unit_twin_sync =
      _wire_func_return_unit_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_func_string_twin_sync(
    ffi.Pointer<wire_list_prim_u_8> arg,
  ) {
    return _wire_func_string_twin_sync(
      arg,
    );
  }

  late final _wire_func_string_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_func_string_twin_sync');
  late final _wire_func_string_twin_sync = _wire_func_string_twin_syncPtr
      .asFunction<WireSyncReturn Function(ffi.Pointer<wire_list_prim_u_8>)>();

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

  WireSyncReturn wire_func_struct_with_one_field_twin_sync(
    ffi.Pointer<wire_struct_with_one_field_twin_sync> arg,
  ) {
    return _wire_func_struct_with_one_field_twin_sync(
      arg,
    );
  }

  late final _wire_func_struct_with_one_field_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_struct_with_one_field_twin_sync>)>>(
      'wire_func_struct_with_one_field_twin_sync');
  late final _wire_func_struct_with_one_field_twin_sync =
      _wire_func_struct_with_one_field_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_struct_with_one_field_twin_sync>)>();

  WireSyncReturn wire_func_struct_with_two_field_twin_sync(
    ffi.Pointer<wire_struct_with_two_field_twin_sync> arg,
  ) {
    return _wire_func_struct_with_two_field_twin_sync(
      arg,
    );
  }

  late final _wire_func_struct_with_two_field_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_struct_with_two_field_twin_sync>)>>(
      'wire_func_struct_with_two_field_twin_sync');
  late final _wire_func_struct_with_two_field_twin_sync =
      _wire_func_struct_with_two_field_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_struct_with_two_field_twin_sync>)>();

  WireSyncReturn wire_func_struct_with_zero_field_twin_sync(
    ffi.Pointer<wire_struct_with_zero_field_twin_sync> arg,
  ) {
    return _wire_func_struct_with_zero_field_twin_sync(
      arg,
    );
  }

  late final _wire_func_struct_with_zero_field_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_struct_with_zero_field_twin_sync>)>>(
      'wire_func_struct_with_zero_field_twin_sync');
  late final _wire_func_struct_with_zero_field_twin_sync =
      _wire_func_struct_with_zero_field_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_struct_with_zero_field_twin_sync>)>();

  WireSyncReturn wire_func_tuple_struct_with_one_field_twin_sync(
    ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync> arg,
  ) {
    return _wire_func_tuple_struct_with_one_field_twin_sync(
      arg,
    );
  }

  late final _wire_func_tuple_struct_with_one_field_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync>)>>(
      'wire_func_tuple_struct_with_one_field_twin_sync');
  late final _wire_func_tuple_struct_with_one_field_twin_sync =
      _wire_func_tuple_struct_with_one_field_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync>)>();

  WireSyncReturn wire_func_tuple_struct_with_two_field_twin_sync(
    ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync> arg,
  ) {
    return _wire_func_tuple_struct_with_two_field_twin_sync(
      arg,
    );
  }

  late final _wire_func_tuple_struct_with_two_field_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync>)>>(
      'wire_func_tuple_struct_with_two_field_twin_sync');
  late final _wire_func_tuple_struct_with_two_field_twin_sync =
      _wire_func_tuple_struct_with_two_field_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync>)>();

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

  void wire_func_stream_realistic_twin_normal(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> arg,
  ) {
    return _wire_func_stream_realistic_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_stream_realistic_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_func_stream_realistic_twin_normal');
  late final _wire_func_stream_realistic_twin_normal =
      _wire_func_stream_realistic_twin_normalPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_func_stream_return_error_twin_normal(
    int port_,
  ) {
    return _wire_func_stream_return_error_twin_normal(
      port_,
    );
  }

  late final _wire_func_stream_return_error_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_stream_return_error_twin_normal');
  late final _wire_func_stream_return_error_twin_normal =
      _wire_func_stream_return_error_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_func_stream_return_panic_twin_normal(
    int port_,
  ) {
    return _wire_func_stream_return_panic_twin_normal(
      port_,
    );
  }

  late final _wire_func_stream_return_panic_twin_normalPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_func_stream_return_panic_twin_normal');
  late final _wire_func_stream_return_panic_twin_normal =
      _wire_func_stream_return_panic_twin_normalPtr
          .asFunction<void Function(int)>();

  void wire_func_stream_sink_arg_position_twin_normal(
    int port_,
    int a,
    int b,
  ) {
    return _wire_func_stream_sink_arg_position_twin_normal(
      port_,
      a,
      b,
    );
  }

  late final _wire_func_stream_sink_arg_position_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Uint32,
              ffi.Uint32)>>('wire_func_stream_sink_arg_position_twin_normal');
  late final _wire_func_stream_sink_arg_position_twin_normal =
      _wire_func_stream_sink_arg_position_twin_normalPtr
          .asFunction<void Function(int, int, int)>();

  void wire_func_struct_with_one_field_twin_normal(
    int port_,
    ffi.Pointer<wire_struct_with_one_field_twin_normal> arg,
  ) {
    return _wire_func_struct_with_one_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_struct_with_one_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_struct_with_one_field_twin_normal>)>>(
      'wire_func_struct_with_one_field_twin_normal');
  late final _wire_func_struct_with_one_field_twin_normal =
      _wire_func_struct_with_one_field_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_struct_with_one_field_twin_normal>)>();

  void wire_func_struct_with_two_field_twin_normal(
    int port_,
    ffi.Pointer<wire_struct_with_two_field_twin_normal> arg,
  ) {
    return _wire_func_struct_with_two_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_struct_with_two_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_struct_with_two_field_twin_normal>)>>(
      'wire_func_struct_with_two_field_twin_normal');
  late final _wire_func_struct_with_two_field_twin_normal =
      _wire_func_struct_with_two_field_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_struct_with_two_field_twin_normal>)>();

  void wire_func_struct_with_zero_field_twin_normal(
    int port_,
    ffi.Pointer<wire_struct_with_zero_field_twin_normal> arg,
  ) {
    return _wire_func_struct_with_zero_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_struct_with_zero_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_struct_with_zero_field_twin_normal>)>>(
      'wire_func_struct_with_zero_field_twin_normal');
  late final _wire_func_struct_with_zero_field_twin_normal =
      _wire_func_struct_with_zero_field_twin_normalPtr.asFunction<
          void Function(
              int, ffi.Pointer<wire_struct_with_zero_field_twin_normal>)>();

  void wire_func_tuple_struct_with_one_field_twin_normal(
    int port_,
    ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal> arg,
  ) {
    return _wire_func_tuple_struct_with_one_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_tuple_struct_with_one_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>)>>(
      'wire_func_tuple_struct_with_one_field_twin_normal');
  late final _wire_func_tuple_struct_with_one_field_twin_normal =
      _wire_func_tuple_struct_with_one_field_twin_normalPtr.asFunction<
          void Function(int,
              ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>)>();

  void wire_func_tuple_struct_with_two_field_twin_normal(
    int port_,
    ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal> arg,
  ) {
    return _wire_func_tuple_struct_with_two_field_twin_normal(
      port_,
      arg,
    );
  }

  late final _wire_func_tuple_struct_with_two_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64,
                  ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>)>>(
      'wire_func_tuple_struct_with_two_field_twin_normal');
  late final _wire_func_tuple_struct_with_two_field_twin_normal =
      _wire_func_tuple_struct_with_two_field_twin_normalPtr.asFunction<
          void Function(int,
              ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>)>();

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

  ffi.Pointer<wire_custom_nested_error_inner_twin_normal>
      new_box_autoadd_custom_nested_error_inner_twin_normal() {
    return _new_box_autoadd_custom_nested_error_inner_twin_normal();
  }

  late final _new_box_autoadd_custom_nested_error_inner_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_custom_nested_error_inner_twin_normal>
                      Function()>>(
          'new_box_autoadd_custom_nested_error_inner_twin_normal');
  late final _new_box_autoadd_custom_nested_error_inner_twin_normal =
      _new_box_autoadd_custom_nested_error_inner_twin_normalPtr.asFunction<
          ffi.Pointer<wire_custom_nested_error_inner_twin_normal> Function()>();

  ffi.Pointer<wire_custom_nested_error_outer_twin_normal>
      new_box_autoadd_custom_nested_error_outer_twin_normal() {
    return _new_box_autoadd_custom_nested_error_outer_twin_normal();
  }

  late final _new_box_autoadd_custom_nested_error_outer_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_custom_nested_error_outer_twin_normal>
                      Function()>>(
          'new_box_autoadd_custom_nested_error_outer_twin_normal');
  late final _new_box_autoadd_custom_nested_error_outer_twin_normal =
      _new_box_autoadd_custom_nested_error_outer_twin_normalPtr.asFunction<
          ffi.Pointer<wire_custom_nested_error_outer_twin_normal> Function()>();

  ffi.Pointer<wire_custom_struct_error_twin_normal>
      new_box_autoadd_custom_struct_error_twin_normal() {
    return _new_box_autoadd_custom_struct_error_twin_normal();
  }

  late final _new_box_autoadd_custom_struct_error_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_custom_struct_error_twin_normal>
              Function()>>('new_box_autoadd_custom_struct_error_twin_normal');
  late final _new_box_autoadd_custom_struct_error_twin_normal =
      _new_box_autoadd_custom_struct_error_twin_normalPtr.asFunction<
          ffi.Pointer<wire_custom_struct_error_twin_normal> Function()>();

  ffi.Pointer<wire_enum_with_item_mixed_twin_normal>
      new_box_autoadd_enum_with_item_mixed_twin_normal() {
    return _new_box_autoadd_enum_with_item_mixed_twin_normal();
  }

  late final _new_box_autoadd_enum_with_item_mixed_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_mixed_twin_normal>
              Function()>>('new_box_autoadd_enum_with_item_mixed_twin_normal');
  late final _new_box_autoadd_enum_with_item_mixed_twin_normal =
      _new_box_autoadd_enum_with_item_mixed_twin_normalPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_mixed_twin_normal> Function()>();

  ffi.Pointer<wire_enum_with_item_mixed_twin_sync>
      new_box_autoadd_enum_with_item_mixed_twin_sync() {
    return _new_box_autoadd_enum_with_item_mixed_twin_sync();
  }

  late final _new_box_autoadd_enum_with_item_mixed_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_mixed_twin_sync>
              Function()>>('new_box_autoadd_enum_with_item_mixed_twin_sync');
  late final _new_box_autoadd_enum_with_item_mixed_twin_sync =
      _new_box_autoadd_enum_with_item_mixed_twin_syncPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_mixed_twin_sync> Function()>();

  ffi.Pointer<wire_enum_with_item_struct_twin_normal>
      new_box_autoadd_enum_with_item_struct_twin_normal() {
    return _new_box_autoadd_enum_with_item_struct_twin_normal();
  }

  late final _new_box_autoadd_enum_with_item_struct_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_struct_twin_normal>
              Function()>>('new_box_autoadd_enum_with_item_struct_twin_normal');
  late final _new_box_autoadd_enum_with_item_struct_twin_normal =
      _new_box_autoadd_enum_with_item_struct_twin_normalPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_struct_twin_normal> Function()>();

  ffi.Pointer<wire_enum_with_item_struct_twin_sync>
      new_box_autoadd_enum_with_item_struct_twin_sync() {
    return _new_box_autoadd_enum_with_item_struct_twin_sync();
  }

  late final _new_box_autoadd_enum_with_item_struct_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_struct_twin_sync>
              Function()>>('new_box_autoadd_enum_with_item_struct_twin_sync');
  late final _new_box_autoadd_enum_with_item_struct_twin_sync =
      _new_box_autoadd_enum_with_item_struct_twin_syncPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_struct_twin_sync> Function()>();

  ffi.Pointer<wire_enum_with_item_tuple_twin_normal>
      new_box_autoadd_enum_with_item_tuple_twin_normal() {
    return _new_box_autoadd_enum_with_item_tuple_twin_normal();
  }

  late final _new_box_autoadd_enum_with_item_tuple_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_tuple_twin_normal>
              Function()>>('new_box_autoadd_enum_with_item_tuple_twin_normal');
  late final _new_box_autoadd_enum_with_item_tuple_twin_normal =
      _new_box_autoadd_enum_with_item_tuple_twin_normalPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_tuple_twin_normal> Function()>();

  ffi.Pointer<wire_enum_with_item_tuple_twin_sync>
      new_box_autoadd_enum_with_item_tuple_twin_sync() {
    return _new_box_autoadd_enum_with_item_tuple_twin_sync();
  }

  late final _new_box_autoadd_enum_with_item_tuple_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_enum_with_item_tuple_twin_sync>
              Function()>>('new_box_autoadd_enum_with_item_tuple_twin_sync');
  late final _new_box_autoadd_enum_with_item_tuple_twin_sync =
      _new_box_autoadd_enum_with_item_tuple_twin_syncPtr.asFunction<
          ffi.Pointer<wire_enum_with_item_tuple_twin_sync> Function()>();

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

  ffi.Pointer<wire_macro_struct> new_box_autoadd_macro_struct() {
    return _new_box_autoadd_macro_struct();
  }

  late final _new_box_autoadd_macro_structPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_macro_struct> Function()>>(
          'new_box_autoadd_macro_struct');
  late final _new_box_autoadd_macro_struct = _new_box_autoadd_macro_structPtr
      .asFunction<ffi.Pointer<wire_macro_struct> Function()>();

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

  ffi.Pointer<wire_struct_with_one_field_twin_normal>
      new_box_autoadd_struct_with_one_field_twin_normal() {
    return _new_box_autoadd_struct_with_one_field_twin_normal();
  }

  late final _new_box_autoadd_struct_with_one_field_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_one_field_twin_normal>
              Function()>>('new_box_autoadd_struct_with_one_field_twin_normal');
  late final _new_box_autoadd_struct_with_one_field_twin_normal =
      _new_box_autoadd_struct_with_one_field_twin_normalPtr.asFunction<
          ffi.Pointer<wire_struct_with_one_field_twin_normal> Function()>();

  ffi.Pointer<wire_struct_with_one_field_twin_sync>
      new_box_autoadd_struct_with_one_field_twin_sync() {
    return _new_box_autoadd_struct_with_one_field_twin_sync();
  }

  late final _new_box_autoadd_struct_with_one_field_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_one_field_twin_sync>
              Function()>>('new_box_autoadd_struct_with_one_field_twin_sync');
  late final _new_box_autoadd_struct_with_one_field_twin_sync =
      _new_box_autoadd_struct_with_one_field_twin_syncPtr.asFunction<
          ffi.Pointer<wire_struct_with_one_field_twin_sync> Function()>();

  ffi.Pointer<wire_struct_with_two_field_twin_normal>
      new_box_autoadd_struct_with_two_field_twin_normal() {
    return _new_box_autoadd_struct_with_two_field_twin_normal();
  }

  late final _new_box_autoadd_struct_with_two_field_twin_normalPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_two_field_twin_normal>
              Function()>>('new_box_autoadd_struct_with_two_field_twin_normal');
  late final _new_box_autoadd_struct_with_two_field_twin_normal =
      _new_box_autoadd_struct_with_two_field_twin_normalPtr.asFunction<
          ffi.Pointer<wire_struct_with_two_field_twin_normal> Function()>();

  ffi.Pointer<wire_struct_with_two_field_twin_sync>
      new_box_autoadd_struct_with_two_field_twin_sync() {
    return _new_box_autoadd_struct_with_two_field_twin_sync();
  }

  late final _new_box_autoadd_struct_with_two_field_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_two_field_twin_sync>
              Function()>>('new_box_autoadd_struct_with_two_field_twin_sync');
  late final _new_box_autoadd_struct_with_two_field_twin_sync =
      _new_box_autoadd_struct_with_two_field_twin_syncPtr.asFunction<
          ffi.Pointer<wire_struct_with_two_field_twin_sync> Function()>();

  ffi.Pointer<wire_struct_with_zero_field_twin_normal>
      new_box_autoadd_struct_with_zero_field_twin_normal() {
    return _new_box_autoadd_struct_with_zero_field_twin_normal();
  }

  late final _new_box_autoadd_struct_with_zero_field_twin_normalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_struct_with_zero_field_twin_normal> Function()>>(
      'new_box_autoadd_struct_with_zero_field_twin_normal');
  late final _new_box_autoadd_struct_with_zero_field_twin_normal =
      _new_box_autoadd_struct_with_zero_field_twin_normalPtr.asFunction<
          ffi.Pointer<wire_struct_with_zero_field_twin_normal> Function()>();

  ffi.Pointer<wire_struct_with_zero_field_twin_sync>
      new_box_autoadd_struct_with_zero_field_twin_sync() {
    return _new_box_autoadd_struct_with_zero_field_twin_sync();
  }

  late final _new_box_autoadd_struct_with_zero_field_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_struct_with_zero_field_twin_sync>
              Function()>>('new_box_autoadd_struct_with_zero_field_twin_sync');
  late final _new_box_autoadd_struct_with_zero_field_twin_sync =
      _new_box_autoadd_struct_with_zero_field_twin_syncPtr.asFunction<
          ffi.Pointer<wire_struct_with_zero_field_twin_sync> Function()>();

  ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>
      new_box_autoadd_tuple_struct_with_one_field_twin_normal() {
    return _new_box_autoadd_tuple_struct_with_one_field_twin_normal();
  }

  late final _new_box_autoadd_tuple_struct_with_one_field_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>
                      Function()>>(
          'new_box_autoadd_tuple_struct_with_one_field_twin_normal');
  late final _new_box_autoadd_tuple_struct_with_one_field_twin_normal =
      _new_box_autoadd_tuple_struct_with_one_field_twin_normalPtr.asFunction<
          ffi.Pointer<wire_tuple_struct_with_one_field_twin_normal>
              Function()>();

  ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync>
      new_box_autoadd_tuple_struct_with_one_field_twin_sync() {
    return _new_box_autoadd_tuple_struct_with_one_field_twin_sync();
  }

  late final _new_box_autoadd_tuple_struct_with_one_field_twin_syncPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync>
                      Function()>>(
          'new_box_autoadd_tuple_struct_with_one_field_twin_sync');
  late final _new_box_autoadd_tuple_struct_with_one_field_twin_sync =
      _new_box_autoadd_tuple_struct_with_one_field_twin_syncPtr.asFunction<
          ffi.Pointer<wire_tuple_struct_with_one_field_twin_sync> Function()>();

  ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>
      new_box_autoadd_tuple_struct_with_two_field_twin_normal() {
    return _new_box_autoadd_tuple_struct_with_two_field_twin_normal();
  }

  late final _new_box_autoadd_tuple_struct_with_two_field_twin_normalPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>
                      Function()>>(
          'new_box_autoadd_tuple_struct_with_two_field_twin_normal');
  late final _new_box_autoadd_tuple_struct_with_two_field_twin_normal =
      _new_box_autoadd_tuple_struct_with_two_field_twin_normalPtr.asFunction<
          ffi.Pointer<wire_tuple_struct_with_two_field_twin_normal>
              Function()>();

  ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync>
      new_box_autoadd_tuple_struct_with_two_field_twin_sync() {
    return _new_box_autoadd_tuple_struct_with_two_field_twin_sync();
  }

  late final _new_box_autoadd_tuple_struct_with_two_field_twin_syncPtr =
      _lookup<
              ffi.NativeFunction<
                  ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync>
                      Function()>>(
          'new_box_autoadd_tuple_struct_with_two_field_twin_sync');
  late final _new_box_autoadd_tuple_struct_with_two_field_twin_sync =
      _new_box_autoadd_tuple_struct_with_two_field_twin_syncPtr.asFunction<
          ffi.Pointer<wire_tuple_struct_with_two_field_twin_sync> Function()>();

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

  ffi.Pointer<CustomNestedErrorInnerTwinNormalKind>
      inflate_CustomNestedErrorInnerTwinNormal_Three() {
    return _inflate_CustomNestedErrorInnerTwinNormal_Three();
  }

  late final _inflate_CustomNestedErrorInnerTwinNormal_ThreePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinNormalKind>
              Function()>>('inflate_CustomNestedErrorInnerTwinNormal_Three');
  late final _inflate_CustomNestedErrorInnerTwinNormal_Three =
      _inflate_CustomNestedErrorInnerTwinNormal_ThreePtr.asFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinNormalKind> Function()>();

  ffi.Pointer<CustomNestedErrorInnerTwinNormalKind>
      inflate_CustomNestedErrorInnerTwinNormal_Four() {
    return _inflate_CustomNestedErrorInnerTwinNormal_Four();
  }

  late final _inflate_CustomNestedErrorInnerTwinNormal_FourPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinNormalKind>
              Function()>>('inflate_CustomNestedErrorInnerTwinNormal_Four');
  late final _inflate_CustomNestedErrorInnerTwinNormal_Four =
      _inflate_CustomNestedErrorInnerTwinNormal_FourPtr.asFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinNormalKind> Function()>();

  ffi.Pointer<CustomNestedErrorOuterTwinNormalKind>
      inflate_CustomNestedErrorOuterTwinNormal_One() {
    return _inflate_CustomNestedErrorOuterTwinNormal_One();
  }

  late final _inflate_CustomNestedErrorOuterTwinNormal_OnePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinNormalKind>
              Function()>>('inflate_CustomNestedErrorOuterTwinNormal_One');
  late final _inflate_CustomNestedErrorOuterTwinNormal_One =
      _inflate_CustomNestedErrorOuterTwinNormal_OnePtr.asFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinNormalKind> Function()>();

  ffi.Pointer<CustomNestedErrorOuterTwinNormalKind>
      inflate_CustomNestedErrorOuterTwinNormal_Two() {
    return _inflate_CustomNestedErrorOuterTwinNormal_Two();
  }

  late final _inflate_CustomNestedErrorOuterTwinNormal_TwoPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinNormalKind>
              Function()>>('inflate_CustomNestedErrorOuterTwinNormal_Two');
  late final _inflate_CustomNestedErrorOuterTwinNormal_Two =
      _inflate_CustomNestedErrorOuterTwinNormal_TwoPtr.asFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemMixedTwinNormalKind>
      inflate_EnumWithItemMixedTwinNormal_B() {
    return _inflate_EnumWithItemMixedTwinNormal_B();
  }

  late final _inflate_EnumWithItemMixedTwinNormal_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemMixedTwinNormalKind>
              Function()>>('inflate_EnumWithItemMixedTwinNormal_B');
  late final _inflate_EnumWithItemMixedTwinNormal_B =
      _inflate_EnumWithItemMixedTwinNormal_BPtr.asFunction<
          ffi.Pointer<EnumWithItemMixedTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemMixedTwinNormalKind>
      inflate_EnumWithItemMixedTwinNormal_C() {
    return _inflate_EnumWithItemMixedTwinNormal_C();
  }

  late final _inflate_EnumWithItemMixedTwinNormal_CPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemMixedTwinNormalKind>
              Function()>>('inflate_EnumWithItemMixedTwinNormal_C');
  late final _inflate_EnumWithItemMixedTwinNormal_C =
      _inflate_EnumWithItemMixedTwinNormal_CPtr.asFunction<
          ffi.Pointer<EnumWithItemMixedTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemMixedTwinSyncKind>
      inflate_EnumWithItemMixedTwinSync_B() {
    return _inflate_EnumWithItemMixedTwinSync_B();
  }

  late final _inflate_EnumWithItemMixedTwinSync_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemMixedTwinSyncKind>
              Function()>>('inflate_EnumWithItemMixedTwinSync_B');
  late final _inflate_EnumWithItemMixedTwinSync_B =
      _inflate_EnumWithItemMixedTwinSync_BPtr
          .asFunction<ffi.Pointer<EnumWithItemMixedTwinSyncKind> Function()>();

  ffi.Pointer<EnumWithItemMixedTwinSyncKind>
      inflate_EnumWithItemMixedTwinSync_C() {
    return _inflate_EnumWithItemMixedTwinSync_C();
  }

  late final _inflate_EnumWithItemMixedTwinSync_CPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemMixedTwinSyncKind>
              Function()>>('inflate_EnumWithItemMixedTwinSync_C');
  late final _inflate_EnumWithItemMixedTwinSync_C =
      _inflate_EnumWithItemMixedTwinSync_CPtr
          .asFunction<ffi.Pointer<EnumWithItemMixedTwinSyncKind> Function()>();

  ffi.Pointer<EnumWithItemStructTwinNormalKind>
      inflate_EnumWithItemStructTwinNormal_A() {
    return _inflate_EnumWithItemStructTwinNormal_A();
  }

  late final _inflate_EnumWithItemStructTwinNormal_APtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemStructTwinNormalKind>
              Function()>>('inflate_EnumWithItemStructTwinNormal_A');
  late final _inflate_EnumWithItemStructTwinNormal_A =
      _inflate_EnumWithItemStructTwinNormal_APtr.asFunction<
          ffi.Pointer<EnumWithItemStructTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemStructTwinNormalKind>
      inflate_EnumWithItemStructTwinNormal_B() {
    return _inflate_EnumWithItemStructTwinNormal_B();
  }

  late final _inflate_EnumWithItemStructTwinNormal_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemStructTwinNormalKind>
              Function()>>('inflate_EnumWithItemStructTwinNormal_B');
  late final _inflate_EnumWithItemStructTwinNormal_B =
      _inflate_EnumWithItemStructTwinNormal_BPtr.asFunction<
          ffi.Pointer<EnumWithItemStructTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemStructTwinSyncKind>
      inflate_EnumWithItemStructTwinSync_A() {
    return _inflate_EnumWithItemStructTwinSync_A();
  }

  late final _inflate_EnumWithItemStructTwinSync_APtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemStructTwinSyncKind>
              Function()>>('inflate_EnumWithItemStructTwinSync_A');
  late final _inflate_EnumWithItemStructTwinSync_A =
      _inflate_EnumWithItemStructTwinSync_APtr
          .asFunction<ffi.Pointer<EnumWithItemStructTwinSyncKind> Function()>();

  ffi.Pointer<EnumWithItemStructTwinSyncKind>
      inflate_EnumWithItemStructTwinSync_B() {
    return _inflate_EnumWithItemStructTwinSync_B();
  }

  late final _inflate_EnumWithItemStructTwinSync_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemStructTwinSyncKind>
              Function()>>('inflate_EnumWithItemStructTwinSync_B');
  late final _inflate_EnumWithItemStructTwinSync_B =
      _inflate_EnumWithItemStructTwinSync_BPtr
          .asFunction<ffi.Pointer<EnumWithItemStructTwinSyncKind> Function()>();

  ffi.Pointer<EnumWithItemTupleTwinNormalKind>
      inflate_EnumWithItemTupleTwinNormal_A() {
    return _inflate_EnumWithItemTupleTwinNormal_A();
  }

  late final _inflate_EnumWithItemTupleTwinNormal_APtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemTupleTwinNormalKind>
              Function()>>('inflate_EnumWithItemTupleTwinNormal_A');
  late final _inflate_EnumWithItemTupleTwinNormal_A =
      _inflate_EnumWithItemTupleTwinNormal_APtr.asFunction<
          ffi.Pointer<EnumWithItemTupleTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemTupleTwinNormalKind>
      inflate_EnumWithItemTupleTwinNormal_B() {
    return _inflate_EnumWithItemTupleTwinNormal_B();
  }

  late final _inflate_EnumWithItemTupleTwinNormal_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemTupleTwinNormalKind>
              Function()>>('inflate_EnumWithItemTupleTwinNormal_B');
  late final _inflate_EnumWithItemTupleTwinNormal_B =
      _inflate_EnumWithItemTupleTwinNormal_BPtr.asFunction<
          ffi.Pointer<EnumWithItemTupleTwinNormalKind> Function()>();

  ffi.Pointer<EnumWithItemTupleTwinSyncKind>
      inflate_EnumWithItemTupleTwinSync_A() {
    return _inflate_EnumWithItemTupleTwinSync_A();
  }

  late final _inflate_EnumWithItemTupleTwinSync_APtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemTupleTwinSyncKind>
              Function()>>('inflate_EnumWithItemTupleTwinSync_A');
  late final _inflate_EnumWithItemTupleTwinSync_A =
      _inflate_EnumWithItemTupleTwinSync_APtr
          .asFunction<ffi.Pointer<EnumWithItemTupleTwinSyncKind> Function()>();

  ffi.Pointer<EnumWithItemTupleTwinSyncKind>
      inflate_EnumWithItemTupleTwinSync_B() {
    return _inflate_EnumWithItemTupleTwinSync_B();
  }

  late final _inflate_EnumWithItemTupleTwinSync_BPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<EnumWithItemTupleTwinSyncKind>
              Function()>>('inflate_EnumWithItemTupleTwinSync_B');
  late final _inflate_EnumWithItemTupleTwinSync_B =
      _inflate_EnumWithItemTupleTwinSync_BPtr
          .asFunction<ffi.Pointer<EnumWithItemTupleTwinSyncKind> Function()>();

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

final class wire_EnumWithItemMixedTwinNormal_A extends ffi.Opaque {}

final class wire_list_prim_u_8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_EnumWithItemMixedTwinNormal_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_EnumWithItemMixedTwinNormal_C extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> c_field;
}

final class EnumWithItemMixedTwinNormalKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemMixedTwinNormal_A> A;

  external ffi.Pointer<wire_EnumWithItemMixedTwinNormal_B> B;

  external ffi.Pointer<wire_EnumWithItemMixedTwinNormal_C> C;
}

final class wire_enum_with_item_mixed_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemMixedTwinNormalKind> kind;
}

final class wire_EnumWithItemStructTwinNormal_A extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a_field;
}

final class wire_list_prim_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_EnumWithItemStructTwinNormal_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> b_field;
}

final class EnumWithItemStructTwinNormalKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemStructTwinNormal_A> A;

  external ffi.Pointer<wire_EnumWithItemStructTwinNormal_B> B;
}

final class wire_enum_with_item_struct_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemStructTwinNormalKind> kind;
}

final class wire_EnumWithItemTupleTwinNormal_A extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_EnumWithItemTupleTwinNormal_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> field0;
}

final class EnumWithItemTupleTwinNormalKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemTupleTwinNormal_A> A;

  external ffi.Pointer<wire_EnumWithItemTupleTwinNormal_B> B;
}

final class wire_enum_with_item_tuple_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemTupleTwinNormalKind> kind;
}

final class wire_CustomNestedErrorOuterTwinNormal_One extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_CustomNestedErrorInnerTwinNormal_Three extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_CustomNestedErrorInnerTwinNormal_Four extends ffi.Struct {
  @ffi.Uint32()
  external int field0;
}

final class CustomNestedErrorInnerTwinNormalKind extends ffi.Union {
  external ffi.Pointer<wire_CustomNestedErrorInnerTwinNormal_Three> Three;

  external ffi.Pointer<wire_CustomNestedErrorInnerTwinNormal_Four> Four;
}

final class wire_custom_nested_error_inner_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<CustomNestedErrorInnerTwinNormalKind> kind;
}

final class wire_CustomNestedErrorOuterTwinNormal_Two extends ffi.Struct {
  external ffi.Pointer<wire_custom_nested_error_inner_twin_normal> field0;
}

final class CustomNestedErrorOuterTwinNormalKind extends ffi.Union {
  external ffi.Pointer<wire_CustomNestedErrorOuterTwinNormal_One> One;

  external ffi.Pointer<wire_CustomNestedErrorOuterTwinNormal_Two> Two;
}

final class wire_custom_nested_error_outer_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<CustomNestedErrorOuterTwinNormalKind> kind;
}

final class wire_custom_struct_error_twin_normal extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a;
}

final class wire_macro_struct extends ffi.Struct {
  @ffi.Int32()
  external int data;
}

final class wire_struct_with_comments_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int field_with_comments;
}

final class wire_EnumWithItemMixedTwinSync_A extends ffi.Opaque {}

final class wire_EnumWithItemMixedTwinSync_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_EnumWithItemMixedTwinSync_C extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> c_field;
}

final class EnumWithItemMixedTwinSyncKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemMixedTwinSync_A> A;

  external ffi.Pointer<wire_EnumWithItemMixedTwinSync_B> B;

  external ffi.Pointer<wire_EnumWithItemMixedTwinSync_C> C;
}

final class wire_enum_with_item_mixed_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemMixedTwinSyncKind> kind;
}

final class wire_EnumWithItemStructTwinSync_A extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a_field;
}

final class wire_EnumWithItemStructTwinSync_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> b_field;
}

final class EnumWithItemStructTwinSyncKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemStructTwinSync_A> A;

  external ffi.Pointer<wire_EnumWithItemStructTwinSync_B> B;
}

final class wire_enum_with_item_struct_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemStructTwinSyncKind> kind;
}

final class wire_EnumWithItemTupleTwinSync_A extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_EnumWithItemTupleTwinSync_B extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> field0;
}

final class EnumWithItemTupleTwinSyncKind extends ffi.Union {
  external ffi.Pointer<wire_EnumWithItemTupleTwinSync_A> A;

  external ffi.Pointer<wire_EnumWithItemTupleTwinSync_B> B;
}

final class wire_enum_with_item_tuple_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumWithItemTupleTwinSyncKind> kind;
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

final class wire_struct_with_one_field_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int a;
}

final class wire_struct_with_two_field_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int a;

  @ffi.Int32()
  external int b;
}

final class wire_struct_with_zero_field_twin_sync extends ffi.Opaque {}

final class wire_tuple_struct_with_one_field_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int field0;
}

final class wire_tuple_struct_with_two_field_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int field0;

  @ffi.Int32()
  external int field1;
}

final class wire_struct_with_one_field_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int a;
}

final class wire_struct_with_two_field_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int a;

  @ffi.Int32()
  external int b;
}

final class wire_struct_with_zero_field_twin_normal extends ffi.Opaque {}

final class wire_tuple_struct_with_one_field_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int field0;
}

final class wire_tuple_struct_with_two_field_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int field0;

  @ffi.Int32()
  external int field1;
}
