// ignore_for_file: unused_import, unused_element

import 'api/array.dart';
import 'api/comment.dart';
import 'api/enumeration.dart';
import 'api/exception.dart';
import 'api/inside_macro.dart';
import 'api/misc_example.dart';
import 'api/misc_type.dart';
import 'api/newtype_pattern.dart';
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
import 'api/raw_string.dart';
import 'api/simple.dart';
import 'api/stream.dart';
import 'api/structure.dart';
import 'auxiliary/sample_types.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<String> api2wire_StringList(List<String> raw) {
    return raw;
  }

  @protected
  List<dynamic> api2wire_a(A raw) {
    return [api2wire_String(raw.a)];
  }

  @protected
  List<dynamic> api2wire_abc(Abc raw) {
    if (raw is Abc_A) {
      return [0, api2wire_box_autoadd_a(raw.field0)];
    }
    if (raw is Abc_B) {
      return [1, api2wire_box_autoadd_b(raw.field0)];
    }
    if (raw is Abc_C) {
      return [2, api2wire_box_autoadd_c(raw.field0)];
    }
    if (raw is Abc_JustInt) {
      return [3, api2wire_i_32(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_b(B raw) {
    return [api2wire_i_32(raw.b)];
  }

  @protected
  List<dynamic> api2wire_blob(Blob raw) {
    return [api2wire_u_8_array_1600(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_box_autoadd_a(A raw) {
    return api2wire_a(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_abc(Abc raw) {
    return api2wire_abc(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_b(B raw) {
    return api2wire_b(raw);
  }

  @protected
  bool api2wire_box_autoadd_bool(bool raw) {
    return api2wire_bool(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_c(C raw) {
    return api2wire_c(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_nested_error_inner_twin_normal(
      CustomNestedErrorInnerTwinNormal raw) {
    return api2wire_custom_nested_error_inner_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_nested_error_inner_twin_sync(
      CustomNestedErrorInnerTwinSync raw) {
    return api2wire_custom_nested_error_inner_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_nested_error_outer_twin_normal(
      CustomNestedErrorOuterTwinNormal raw) {
    return api2wire_custom_nested_error_outer_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_nested_error_outer_twin_sync(
      CustomNestedErrorOuterTwinSync raw) {
    return api2wire_custom_nested_error_outer_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal raw) {
    return api2wire_custom_struct_error_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_custom_struct_error_twin_sync(
      CustomStructErrorTwinSync raw) {
    return api2wire_custom_struct_error_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_mixed_twin_normal(
      EnumWithItemMixedTwinNormal raw) {
    return api2wire_enum_with_item_mixed_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_mixed_twin_sync(
      EnumWithItemMixedTwinSync raw) {
    return api2wire_enum_with_item_mixed_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_struct_twin_normal(
      EnumWithItemStructTwinNormal raw) {
    return api2wire_enum_with_item_struct_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_struct_twin_sync(
      EnumWithItemStructTwinSync raw) {
    return api2wire_enum_with_item_struct_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_tuple_twin_normal(
      EnumWithItemTupleTwinNormal raw) {
    return api2wire_enum_with_item_tuple_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_with_item_tuple_twin_sync(
      EnumWithItemTupleTwinSync raw) {
    return api2wire_enum_with_item_tuple_twin_sync(raw);
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
  List<dynamic> api2wire_box_autoadd_feed_id(FeedId raw) {
    return api2wire_feed_id(raw);
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
  List<dynamic> api2wire_box_autoadd_macro_struct(MacroStruct raw) {
    return api2wire_macro_struct(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_measure(Measure raw) {
    return api2wire_measure(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_message_id(MessageId raw) {
    return api2wire_message_id(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_my_nested_struct(MyNestedStruct raw) {
    return api2wire_my_nested_struct(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_my_tree_node(MyTreeNode raw) {
    return api2wire_my_tree_node(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_new_type_int(NewTypeInt raw) {
    return api2wire_new_type_int(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_note(Note raw) {
    return api2wire_note(raw);
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
  List<dynamic> api2wire_box_autoadd_struct_with_enum(StructWithEnum raw) {
    return api2wire_struct_with_enum(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_one_field_twin_normal(
      StructWithOneFieldTwinNormal raw) {
    return api2wire_struct_with_one_field_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_one_field_twin_sync(
      StructWithOneFieldTwinSync raw) {
    return api2wire_struct_with_one_field_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_two_field_twin_normal(
      StructWithTwoFieldTwinNormal raw) {
    return api2wire_struct_with_two_field_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_two_field_twin_sync(
      StructWithTwoFieldTwinSync raw) {
    return api2wire_struct_with_two_field_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_zero_field_twin_normal(
      StructWithZeroFieldTwinNormal raw) {
    return api2wire_struct_with_zero_field_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_struct_with_zero_field_twin_sync(
      StructWithZeroFieldTwinSync raw) {
    return api2wire_struct_with_zero_field_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_test_id(TestId raw) {
    return api2wire_test_id(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_tuple_struct_with_one_field_twin_normal(
      TupleStructWithOneFieldTwinNormal raw) {
    return api2wire_tuple_struct_with_one_field_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_tuple_struct_with_one_field_twin_sync(
      TupleStructWithOneFieldTwinSync raw) {
    return api2wire_tuple_struct_with_one_field_twin_sync(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_tuple_struct_with_two_field_twin_normal(
      TupleStructWithTwoFieldTwinNormal raw) {
    return api2wire_tuple_struct_with_two_field_twin_normal(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_tuple_struct_with_two_field_twin_sync(
      TupleStructWithTwoFieldTwinSync raw) {
    return api2wire_tuple_struct_with_two_field_twin_sync(raw);
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
  List<dynamic> api2wire_box_blob(Blob raw) {
    return api2wire_blob(raw);
  }

  @protected
  List<dynamic> api2wire_box_distance(Distance raw) {
    return api2wire_distance(raw);
  }

  @protected
  List<dynamic> api2wire_box_speed(Speed raw) {
    return api2wire_speed(raw);
  }

  @protected
  Uint8List api2wire_box_u_8_array_1600(U8Array1600 raw) {
    return api2wire_u_8_array_1600(raw);
  }

  @protected
  int api2wire_box_weekdays(Weekdays raw) {
    return api2wire_weekdays(raw);
  }

  @protected
  List<dynamic> api2wire_c(C raw) {
    return [api2wire_bool(raw.c)];
  }

  @protected
  List<dynamic> api2wire_custom_nested_error_inner_twin_normal(
      CustomNestedErrorInnerTwinNormal raw) {
    if (raw is CustomNestedErrorInnerTwinNormal_Three) {
      return [0, api2wire_String(raw.field0)];
    }
    if (raw is CustomNestedErrorInnerTwinNormal_Four) {
      return [1, api2wire_u_32(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_custom_nested_error_inner_twin_sync(
      CustomNestedErrorInnerTwinSync raw) {
    if (raw is CustomNestedErrorInnerTwinSync_Three) {
      return [0, api2wire_String(raw.field0)];
    }
    if (raw is CustomNestedErrorInnerTwinSync_Four) {
      return [1, api2wire_u_32(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_custom_nested_error_outer_twin_normal(
      CustomNestedErrorOuterTwinNormal raw) {
    if (raw is CustomNestedErrorOuterTwinNormal_One) {
      return [0, api2wire_String(raw.field0)];
    }
    if (raw is CustomNestedErrorOuterTwinNormal_Two) {
      return [
        1,
        api2wire_box_autoadd_custom_nested_error_inner_twin_normal(raw.field0)
      ];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_custom_nested_error_outer_twin_sync(
      CustomNestedErrorOuterTwinSync raw) {
    if (raw is CustomNestedErrorOuterTwinSync_One) {
      return [0, api2wire_String(raw.field0)];
    }
    if (raw is CustomNestedErrorOuterTwinSync_Two) {
      return [
        1,
        api2wire_box_autoadd_custom_nested_error_inner_twin_sync(raw.field0)
      ];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal raw) {
    return [api2wire_String(raw.a)];
  }

  @protected
  List<dynamic> api2wire_custom_struct_error_twin_sync(
      CustomStructErrorTwinSync raw) {
    return [api2wire_String(raw.a)];
  }

  @protected
  List<dynamic> api2wire_distance(Distance raw) {
    if (raw is Distance_Unknown) {
      return [0];
    }
    if (raw is Distance_Map) {
      return [1, api2wire_f_64(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_mixed_twin_normal(
      EnumWithItemMixedTwinNormal raw) {
    if (raw is EnumWithItemMixedTwinNormal_A) {
      return [0];
    }
    if (raw is EnumWithItemMixedTwinNormal_B) {
      return [1, api2wire_list_prim_u_8(raw.field0)];
    }
    if (raw is EnumWithItemMixedTwinNormal_C) {
      return [2, api2wire_String(raw.cField)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_mixed_twin_sync(
      EnumWithItemMixedTwinSync raw) {
    if (raw is EnumWithItemMixedTwinSync_A) {
      return [0];
    }
    if (raw is EnumWithItemMixedTwinSync_B) {
      return [1, api2wire_list_prim_u_8(raw.field0)];
    }
    if (raw is EnumWithItemMixedTwinSync_C) {
      return [2, api2wire_String(raw.cField)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_struct_twin_normal(
      EnumWithItemStructTwinNormal raw) {
    if (raw is EnumWithItemStructTwinNormal_A) {
      return [0, api2wire_list_prim_u_8(raw.aField)];
    }
    if (raw is EnumWithItemStructTwinNormal_B) {
      return [1, api2wire_list_prim_i_32(raw.bField)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_struct_twin_sync(
      EnumWithItemStructTwinSync raw) {
    if (raw is EnumWithItemStructTwinSync_A) {
      return [0, api2wire_list_prim_u_8(raw.aField)];
    }
    if (raw is EnumWithItemStructTwinSync_B) {
      return [1, api2wire_list_prim_i_32(raw.bField)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_tuple_twin_normal(
      EnumWithItemTupleTwinNormal raw) {
    if (raw is EnumWithItemTupleTwinNormal_A) {
      return [0, api2wire_list_prim_u_8(raw.field0)];
    }
    if (raw is EnumWithItemTupleTwinNormal_B) {
      return [1, api2wire_list_prim_i_32(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_with_item_tuple_twin_sync(
      EnumWithItemTupleTwinSync raw) {
    if (raw is EnumWithItemTupleTwinSync_A) {
      return [0, api2wire_list_prim_u_8(raw.field0)];
    }
    if (raw is EnumWithItemTupleTwinSync_B) {
      return [1, api2wire_list_prim_i_32(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  Float64List api2wire_f_64_array_16(F64Array16 raw) {
    return Float64List.fromList(raw);
  }

  @protected
  List<dynamic> api2wire_feed_id(FeedId raw) {
    return [api2wire_u_8_array_8(raw.field0)];
  }

  @protected
  Int32List api2wire_i_32_array_2(I32Array2 raw) {
    return Int32List.fromList(raw);
  }

  @protected
  Object api2wire_i_64(BigInt raw) {
    return castNativeBigInt(raw);
  }

  @protected
  List<dynamic> api2wire_list_bool(List<bool> raw) {
    return raw.map(api2wire_bool).toList();
  }

  @protected
  List<dynamic> api2wire_list_my_size(List<MySize> raw) {
    return raw.map(api2wire_my_size).toList();
  }

  @protected
  List<dynamic> api2wire_list_my_tree_node(List<MyTreeNode> raw) {
    return raw.map(api2wire_my_tree_node).toList();
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
  List<dynamic> api2wire_list_test_id(List<TestId> raw) {
    return raw.map(api2wire_test_id).toList();
  }

  @protected
  List<dynamic> api2wire_list_weekdays(List<Weekdays> raw) {
    return raw.map(api2wire_weekdays).toList();
  }

  @protected
  List<dynamic> api2wire_macro_struct(MacroStruct raw) {
    return [api2wire_i_32(raw.data)];
  }

  @protected
  List<dynamic> api2wire_measure(Measure raw) {
    if (raw is Measure_Speed) {
      return [0, api2wire_box_speed(raw.field0)];
    }
    if (raw is Measure_Distance) {
      return [1, api2wire_box_distance(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_message_id(MessageId raw) {
    return [api2wire_u_8_array_32(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_my_nested_struct(MyNestedStruct raw) {
    return [
      api2wire_my_tree_node(raw.treeNode),
      api2wire_weekdays(raw.weekday)
    ];
  }

  @protected
  List<dynamic> api2wire_my_size(MySize raw) {
    return [api2wire_i_32(raw.width), api2wire_i_32(raw.height)];
  }

  @protected
  List<dynamic> api2wire_my_tree_node(MyTreeNode raw) {
    return [
      api2wire_i_32(raw.valueI32),
      api2wire_list_prim_u_8(raw.valueVecU8),
      api2wire_bool(raw.valueBoolean),
      api2wire_list_my_tree_node(raw.children)
    ];
  }

  @protected
  List<dynamic> api2wire_new_type_int(NewTypeInt raw) {
    return [api2wire_i_64(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_note(Note raw) {
    return [api2wire_box_weekdays(raw.day), api2wire_String(raw.body)];
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
  List<dynamic> api2wire_speed(Speed raw) {
    if (raw is Speed_Unknown) {
      return [0];
    }
    if (raw is Speed_GPS) {
      return [1, api2wire_f_64(raw.field0)];
    }

    throw Exception('unreachable');
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
  List<dynamic> api2wire_struct_with_enum(StructWithEnum raw) {
    return [api2wire_abc(raw.abc1), api2wire_abc(raw.abc2)];
  }

  @protected
  List<dynamic> api2wire_struct_with_one_field_twin_normal(
      StructWithOneFieldTwinNormal raw) {
    return [api2wire_i_32(raw.a)];
  }

  @protected
  List<dynamic> api2wire_struct_with_one_field_twin_sync(
      StructWithOneFieldTwinSync raw) {
    return [api2wire_i_32(raw.a)];
  }

  @protected
  List<dynamic> api2wire_struct_with_two_field_twin_normal(
      StructWithTwoFieldTwinNormal raw) {
    return [api2wire_i_32(raw.a), api2wire_i_32(raw.b)];
  }

  @protected
  List<dynamic> api2wire_struct_with_two_field_twin_sync(
      StructWithTwoFieldTwinSync raw) {
    return [api2wire_i_32(raw.a), api2wire_i_32(raw.b)];
  }

  @protected
  List<dynamic> api2wire_struct_with_zero_field_twin_normal(
      StructWithZeroFieldTwinNormal raw) {
    return [];
  }

  @protected
  List<dynamic> api2wire_struct_with_zero_field_twin_sync(
      StructWithZeroFieldTwinSync raw) {
    return [];
  }

  @protected
  List<dynamic> api2wire_test_id(TestId raw) {
    return [api2wire_i_32_array_2(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_test_id_array_4(TestIdArray4 raw) {
    return api2wire_list_test_id(raw);
  }

  @protected
  List<dynamic> api2wire_tuple_struct_with_one_field_twin_normal(
      TupleStructWithOneFieldTwinNormal raw) {
    return [api2wire_i_32(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_tuple_struct_with_one_field_twin_sync(
      TupleStructWithOneFieldTwinSync raw) {
    return [api2wire_i_32(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_tuple_struct_with_two_field_twin_normal(
      TupleStructWithTwoFieldTwinNormal raw) {
    return [api2wire_i_32(raw.field0), api2wire_i_32(raw.field1)];
  }

  @protected
  List<dynamic> api2wire_tuple_struct_with_two_field_twin_sync(
      TupleStructWithTwoFieldTwinSync raw) {
    return [api2wire_i_32(raw.field0), api2wire_i_32(raw.field1)];
  }

  @protected
  Object api2wire_u_64(BigInt raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_u_8_array_1600(U8Array1600 raw) {
    return Uint8List.fromList(raw);
  }

  @protected
  Uint8List api2wire_u_8_array_32(U8Array32 raw) {
    return Uint8List.fromList(raw);
  }

  @protected
  Uint8List api2wire_u_8_array_8(U8Array8 raw) {
    return Uint8List.fromList(raw);
  }
}

// Section: boilerplate

class RustLibWire extends BaseWire {
  // TODO
}
