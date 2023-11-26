// ignore_for_file: unused_import, unused_element

import 'api/array.dart';
import 'api/attribute.dart';
import 'api/chrono_type.dart';
import 'api/comment.dart';
import 'api/dart_dynamic.dart';
import 'api/dart_opaque.dart';
import 'api/dart_opaque_sync.dart';
import 'api/enumeration.dart';
import 'api/event_listener.dart';
import 'api/exception.dart';
import 'api/external_type_in_crate.dart';
import 'api/inside_macro.dart';
import 'api/method.dart';
import 'api/mirror.dart';
import 'api/misc_example.dart';
import 'api/misc_type.dart';
import 'api/newtype_pattern.dart';
import 'api/optional.dart';
import 'api/optional_primitive_misc.dart';
import 'api/primitive_list_misc.dart';
import 'api/primitive_list_sync_misc.dart';
import 'api/primitive_misc.dart';
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
import 'api/rust_opaque.dart';
import 'api/rust_opaque_sync.dart';
import 'api/simple.dart';
import 'api/stream.dart';
import 'api/structure.dart';
import 'api/tuple.dart';
import 'api/type_alias.dart';
import 'api/uuid_type.dart';
import 'auxiliary/new_module_system/sub_module.dart';
import 'auxiliary/old_module_system/sub_module.dart';
import 'auxiliary/sample_types.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
import 'package:meta/meta.dart' as meta;
import 'package:uuid/uuid.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.dropPortManager,
  });

  late final mutexHideDataFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_MutexHideData);

  late final rwLockHideDataFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_RwLockHideData);

  late final boxDartDebugFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_box_dynDartDebug);

  late final frbOpaqueReturnFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_frb_opaque_return);

  late final frbOpaqueSyncReturnFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_frb_opaque_sync_return);

  late final hideDataFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_hide_data);

  late final i32Finalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_i_32);

  late final nonCloneDataFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_non_clone_data);

  late final nonSendHideDataFinalizer =
      OpaqueTypeFinalizer(wire.drop_opaque_RustOpaque_non_send_hide_data);

  @protected
  Object api2wire_Chrono_Duration(Duration raw) {
    return api2wire_i_64(BigInt.from(raw.inMilliseconds));
  }

  @protected
  Object /* BigInt64Array */ api2wire_Chrono_DurationList(List<Duration> raw) {
    final ans = Int64List(raw.length);
    for (var i = 0; i < raw.length; ++i)
      ans[i] = api2wire_Chrono_Duration(raw[i]);
    return api2wire_list_prim_i_64(ans);
  }

  @protected
  Object api2wire_Chrono_Local(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.millisecondsSinceEpoch));
  }

  @protected
  Object api2wire_Chrono_Naive(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.millisecondsSinceEpoch));
  }

  @protected
  Object /* BigInt64Array */ api2wire_Chrono_NaiveList(List<DateTime> raw) {
    final ans = Int64List(raw.length);
    for (var i = 0; i < raw.length; ++i) ans[i] = api2wire_Chrono_Naive(raw[i]);
    return api2wire_list_prim_i_64(ans);
  }

  @protected
  Object api2wire_Chrono_Utc(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.millisecondsSinceEpoch));
  }

  @protected
  Object api2wire_DartOpaque(Object raw) {
    return [raw, dropPortManager.dropPort];
  }

  @protected
  List<dynamic> api2wire_DartOpaque_array_1(ObjectArray1 raw) {
    return api2wire_list_DartOpaque(raw);
  }

  @protected
  Object api2wire_RustOpaque_MutexHideData(MutexHideData raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RustOpaque_RwLockHideData(RwLockHideData raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RustOpaque_box_dynDartDebug(BoxDartDebug raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RustOpaque_hide_data(HideData raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  List<dynamic> api2wire_RustOpaque_hide_data_array_2(HideDataArray2 raw) {
    return api2wire_list_RustOpaque_hide_data(raw);
  }

  @protected
  Object api2wire_RustOpaque_i_32(I32 raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RustOpaque_non_clone_data(NonCloneData raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RustOpaque_non_send_hide_data(NonSendHideData raw) {
    // ignore: invalid_use_of_internal_member
    return raw.shareOrMove();
  }

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<String> api2wire_StringList(List<String> raw) {
    return raw;
  }

  @protected
  Uint8List api2wire_Uuid(UuidValue raw) {
    return api2wire_list_prim_u_8(raw.toBytes());
  }

  @protected
  Uint8List api2wire_Uuids(List<UuidValue> raw) {
    final builder = BytesBuilder();
    for (final element in raw) {
      builder.add(element.toBytes());
    }
    return api2wire_list_prim_u_8(builder.toBytes());
  }

  @protected
  Uint8List api2wire_ZeroCopyBuffer_list_prim_u_8(Uint8List raw) {
    return api2wire_list_prim_u_8(raw);
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
  List<dynamic> api2wire_application_env(ApplicationEnv raw) {
    return [api2wire_list_application_env_var(raw.vars)];
  }

  @protected
  List<dynamic> api2wire_application_env_var(ApplicationEnvVar raw) {
    return [api2wire_String(raw.field0), api2wire_bool(raw.field1)];
  }

  @protected
  List<dynamic> api2wire_application_settings(ApplicationSettings raw) {
    return [
      api2wire_String(raw.name),
      api2wire_String(raw.version),
      api2wire_application_mode(raw.mode),
      api2wire_box_application_env(raw.env),
      api2wire_opt_box_autoadd_application_env(raw.envOptional)
    ];
  }

  @protected
  List<dynamic> api2wire_attribute(Attribute raw) {
    return [api2wire_String(raw.key), api2wire_String(raw.value)];
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
  List<dynamic> api2wire_box_application_env(ApplicationEnv raw) {
    return api2wire_application_env(raw);
  }

  @protected
  Object api2wire_box_autoadd_Chrono_Utc(DateTime raw) {
    return api2wire_Chrono_Utc(raw);
  }

  @protected
  Object api2wire_box_autoadd_DartOpaque(Object raw) {
    return api2wire_DartOpaque(raw);
  }

  @protected
  Object api2wire_box_autoadd_RustOpaque_hide_data(HideData raw) {
    return api2wire_RustOpaque_hide_data(raw);
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
  List<dynamic> api2wire_box_autoadd_application_env(ApplicationEnv raw) {
    return api2wire_application_env(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_application_settings(
      ApplicationSettings raw) {
    return api2wire_application_settings(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_attribute(Attribute raw) {
    return api2wire_attribute(raw);
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
  List<dynamic> api2wire_box_autoadd_concatenate_with(ConcatenateWith raw) {
    return api2wire_concatenate_with(raw);
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
  List<dynamic> api2wire_box_autoadd_custom_struct(CustomStruct raw) {
    return api2wire_custom_struct(raw);
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
  List<dynamic> api2wire_box_autoadd_customized(Customized raw) {
    return api2wire_customized(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_dart_opaque_nested(DartOpaqueNested raw) {
    return api2wire_dart_opaque_nested(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_empty(Empty raw) {
    return api2wire_empty(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_dart_opaque(EnumDartOpaque raw) {
    return api2wire_enum_dart_opaque(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_enum_opaque(EnumOpaque raw) {
    return api2wire_enum_opaque(raw);
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
  List<dynamic> api2wire_box_autoadd_event(Event raw) {
    return api2wire_event(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_exotic_optionals(ExoticOptionals raw) {
    return api2wire_exotic_optionals(raw);
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
  List<dynamic> api2wire_box_autoadd_feature_chrono(FeatureChrono raw) {
    return api2wire_feature_chrono(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_feature_uuid(FeatureUuid raw) {
    return api2wire_feature_uuid(raw);
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
  List<dynamic> api2wire_box_autoadd_kitchen_sink(KitchenSink raw) {
    return api2wire_kitchen_sink(raw);
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
  List<dynamic> api2wire_box_autoadd_my_size(MySize raw) {
    return api2wire_my_size(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_my_size_freezed(MySizeFreezed raw) {
    return api2wire_my_size_freezed(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_my_struct(MyStruct raw) {
    return api2wire_my_struct(raw);
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
  List<dynamic> api2wire_box_autoadd_numbers(Numbers raw) {
    return api2wire_numbers(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_opaque_nested(OpaqueNested raw) {
    return api2wire_opaque_nested(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_opt_vecs(OptVecs raw) {
    return api2wire_opt_vecs(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_record_string_i_32((String, int) raw) {
    return api2wire_record_string_i_32(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_sequences(Sequences raw) {
    return api2wire_sequences(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_some_struct(SomeStruct raw) {
    return api2wire_some_struct(raw);
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
  List<dynamic> api2wire_box_autoadd_sum_with(SumWith raw) {
    return api2wire_sum_with(raw);
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
  List<dynamic> api2wire_box_autoadd_user_id(UserId raw) {
    return api2wire_user_id(raw);
  }

  @protected
  int api2wire_box_autoadd_weekdays(Weekdays raw) {
    return api2wire_weekdays(raw);
  }

  @protected
  List<dynamic> api2wire_box_blob(Blob raw) {
    return api2wire_blob(raw);
  }

  @protected
  bool api2wire_box_bool(bool raw) {
    return api2wire_bool(raw);
  }

  @protected
  List<dynamic> api2wire_box_distance(Distance raw) {
    return api2wire_distance(raw);
  }

  @protected
  List<dynamic> api2wire_box_exotic_optionals(ExoticOptionals raw) {
    return api2wire_exotic_optionals(raw);
  }

  @protected
  double api2wire_box_f_64(double raw) {
    return api2wire_f_64(raw);
  }

  @protected
  int api2wire_box_i_32(int raw) {
    return api2wire_i_32(raw);
  }

  @protected
  Object api2wire_box_i_64(BigInt raw) {
    return api2wire_i_64(raw);
  }

  @protected
  int api2wire_box_i_8(int raw) {
    return api2wire_i_8(raw);
  }

  @protected
  List<dynamic> api2wire_box_kitchen_sink(KitchenSink raw) {
    return api2wire_kitchen_sink(raw);
  }

  @protected
  List<dynamic> api2wire_box_my_size(MySize raw) {
    return api2wire_my_size(raw);
  }

  @protected
  List<dynamic> api2wire_box_my_size_freezed(MySizeFreezed raw) {
    return api2wire_my_size_freezed(raw);
  }

  @protected
  List<dynamic> api2wire_box_speed(Speed raw) {
    return api2wire_speed(raw);
  }

  @protected
  int api2wire_box_u_8(int raw) {
    return api2wire_u_8(raw);
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
  List<dynamic> api2wire_concatenate_with(ConcatenateWith raw) {
    return [api2wire_String(raw.a)];
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
  List<dynamic> api2wire_custom_struct(CustomStruct raw) {
    return [api2wire_String(raw.message)];
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
  List<dynamic> api2wire_customized(Customized raw) {
    return [
      api2wire_String(raw.finalField),
      api2wire_opt_String(raw.nonFinalField)
    ];
  }

  @protected
  List<dynamic> api2wire_dart_opaque_nested(DartOpaqueNested raw) {
    return [api2wire_DartOpaque(raw.first), api2wire_DartOpaque(raw.second)];
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
  List<dynamic> api2wire_empty(Empty raw) {
    return [];
  }

  @protected
  List<dynamic> api2wire_enum_dart_opaque(EnumDartOpaque raw) {
    if (raw is EnumDartOpaque_Primitive) {
      return [0, api2wire_i_32(raw.field0)];
    }
    if (raw is EnumDartOpaque_Opaque) {
      return [1, api2wire_DartOpaque(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_enum_opaque(EnumOpaque raw) {
    if (raw is EnumOpaque_Struct) {
      return [0, api2wire_RustOpaque_hide_data(raw.field0)];
    }
    if (raw is EnumOpaque_Primitive) {
      return [1, api2wire_RustOpaque_i_32(raw.field0)];
    }
    if (raw is EnumOpaque_TraitObj) {
      return [2, api2wire_RustOpaque_box_dynDartDebug(raw.field0)];
    }
    if (raw is EnumOpaque_Mutex) {
      return [3, api2wire_RustOpaque_MutexHideData(raw.field0)];
    }
    if (raw is EnumOpaque_RwLock) {
      return [4, api2wire_RustOpaque_RwLockHideData(raw.field0)];
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
  List<dynamic> api2wire_event(Event raw) {
    return [api2wire_String(raw.address), api2wire_String(raw.payload)];
  }

  @protected
  List<dynamic> api2wire_exotic_optionals(ExoticOptionals raw) {
    return [
      api2wire_opt_box_autoadd_i_32(raw.int32),
      api2wire_opt_box_autoadd_i_64(raw.int64),
      api2wire_opt_box_autoadd_f_64(raw.float64),
      api2wire_opt_box_autoadd_bool(raw.boolean),
      api2wire_opt_ZeroCopyBuffer_list_prim_u_8(raw.zerocopy),
      api2wire_opt_list_prim_i_8(raw.int8List),
      api2wire_opt_list_prim_u_8(raw.uint8List),
      api2wire_opt_list_prim_i_32(raw.int32List),
      api2wire_opt_list_prim_f_32(raw.float32List),
      api2wire_opt_list_prim_f_64(raw.float64List),
      api2wire_opt_list_attribute(raw.attributes),
      api2wire_list_opt_box_autoadd_attribute(raw.attributesNullable),
      api2wire_opt_list_opt_box_autoadd_attribute(raw.nullableAttributes),
      api2wire_opt_box_autoadd_new_type_int(raw.newtypeint)
    ];
  }

  @protected
  Float64List api2wire_f_64_array_16(F64Array16 raw) {
    return Float64List.fromList(raw);
  }

  @protected
  List<dynamic> api2wire_feature_chrono(FeatureChrono raw) {
    return [
      api2wire_Chrono_Utc(raw.utc),
      api2wire_Chrono_Local(raw.local),
      api2wire_Chrono_Duration(raw.duration),
      api2wire_Chrono_Naive(raw.naive)
    ];
  }

  @protected
  List<dynamic> api2wire_feature_uuid(FeatureUuid raw) {
    return [api2wire_Uuid(raw.one), api2wire_Uuids(raw.many)];
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
  List<dynamic> api2wire_kitchen_sink(KitchenSink raw) {
    if (raw is KitchenSink_Empty) {
      return [0];
    }
    if (raw is KitchenSink_Primitives) {
      return [
        1,
        api2wire_i_32(raw.int32),
        api2wire_f_64(raw.float64),
        api2wire_bool(raw.boolean)
      ];
    }
    if (raw is KitchenSink_Nested) {
      return [
        2,
        api2wire_i_32(raw.field0),
        api2wire_box_kitchen_sink(raw.field1)
      ];
    }
    if (raw is KitchenSink_Optional) {
      return [
        3,
        api2wire_opt_box_autoadd_i_32(raw.field0),
        api2wire_opt_box_autoadd_i_32(raw.field1)
      ];
    }
    if (raw is KitchenSink_Buffer) {
      return [4, api2wire_ZeroCopyBuffer_list_prim_u_8(raw.field0)];
    }
    if (raw is KitchenSink_Enums) {
      return [5, api2wire_weekdays(raw.field0)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_list_DartOpaque(List<Object> raw) {
    return raw.map(api2wire_DartOpaque).toList();
  }

  @protected
  List<dynamic> api2wire_list_RustOpaque_hide_data(List<HideData> raw) {
    return raw.map(api2wire_RustOpaque_hide_data).toList();
  }

  @protected
  List<dynamic> api2wire_list_application_env_var(List<ApplicationEnvVar> raw) {
    return raw.map(api2wire_application_env_var).toList();
  }

  @protected
  List<dynamic> api2wire_list_attribute(List<Attribute> raw) {
    return raw.map(api2wire_attribute).toList();
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
  List<dynamic> api2wire_list_opt_String(List<String?> raw) {
    return mapNonNull(raw, api2wire_String);
  }

  @protected
  List<dynamic> api2wire_list_opt_box_autoadd_attribute(List<Attribute?> raw) {
    return mapNonNull(raw, api2wire_box_autoadd_attribute);
  }

  @protected
  List<dynamic> api2wire_list_opt_box_autoadd_i_32(List<int?> raw) {
    return mapNonNull(raw, api2wire_box_autoadd_i_32);
  }

  @protected
  List<dynamic> api2wire_list_opt_box_autoadd_weekdays(List<Weekdays?> raw) {
    return mapNonNull(raw, api2wire_box_autoadd_weekdays);
  }

  @protected
  List<dynamic> api2wire_list_opt_list_prim_i_32(List<Int32List?> raw) {
    return mapNonNull(raw, api2wire_list_prim_i_32);
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
  List<dynamic> api2wire_list_record_string_i_32(List<(String, int)> raw) {
    return raw.map(api2wire_record_string_i_32).toList();
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
  List<dynamic> api2wire_my_size_freezed(MySizeFreezed raw) {
    return [api2wire_i_32(raw.width), api2wire_i_32(raw.height)];
  }

  @protected
  List<dynamic> api2wire_my_struct(MyStruct raw) {
    return [api2wire_bool(raw.content)];
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
  List<dynamic> api2wire_numbers(Numbers raw) {
    return [api2wire_list_prim_i_32(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_opaque_nested(OpaqueNested raw) {
    return [
      api2wire_RustOpaque_hide_data(raw.first),
      api2wire_RustOpaque_hide_data(raw.second)
    ];
  }

  @protected
  String? api2wire_opt_String(String? raw) {
    return raw == null ? null : api2wire_String(raw);
  }

  @protected
  Uint8List? api2wire_opt_ZeroCopyBuffer_list_prim_u_8(Uint8List? raw) {
    return raw == null ? null : api2wire_ZeroCopyBuffer_list_prim_u_8(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_Chrono_Utc(DateTime? raw) {
    return raw == null ? null : api2wire_box_autoadd_Chrono_Utc(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_DartOpaque(Object? raw) {
    return raw == null ? null : api2wire_box_autoadd_DartOpaque(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_RustOpaque_hide_data(HideData? raw) {
    return raw == null ? null : api2wire_box_autoadd_RustOpaque_hide_data(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_autoadd_application_env(ApplicationEnv? raw) {
    return raw == null ? null : api2wire_box_autoadd_application_env(raw);
  }

  @protected
  bool? api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? null : api2wire_box_autoadd_bool(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_autoadd_exotic_optionals(
      ExoticOptionals? raw) {
    return raw == null ? null : api2wire_box_autoadd_exotic_optionals(raw);
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
  List<dynamic>? api2wire_opt_box_autoadd_new_type_int(NewTypeInt? raw) {
    return raw == null ? null : api2wire_box_autoadd_new_type_int(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_autoadd_record_string_i_32(
      (String, int)? raw) {
    return raw == null ? null : api2wire_box_autoadd_record_string_i_32(raw);
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
  bool? api2wire_opt_box_bool(bool? raw) {
    return raw == null ? null : api2wire_box_bool(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_exotic_optionals(ExoticOptionals? raw) {
    return raw == null ? null : api2wire_box_exotic_optionals(raw);
  }

  @protected
  double? api2wire_opt_box_f_64(double? raw) {
    return raw == null ? null : api2wire_box_f_64(raw);
  }

  @protected
  int? api2wire_opt_box_i_32(int? raw) {
    return raw == null ? null : api2wire_box_i_32(raw);
  }

  @protected
  Object? api2wire_opt_box_i_64(BigInt? raw) {
    return raw == null ? null : api2wire_box_i_64(raw);
  }

  @protected
  int? api2wire_opt_box_i_8(int? raw) {
    return raw == null ? null : api2wire_box_i_8(raw);
  }

  @protected
  int? api2wire_opt_box_u_8(int? raw) {
    return raw == null ? null : api2wire_box_u_8(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_list_attribute(List<Attribute>? raw) {
    return raw == null ? null : api2wire_list_attribute(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_list_opt_box_autoadd_attribute(
      List<Attribute?>? raw) {
    return raw == null ? null : api2wire_list_opt_box_autoadd_attribute(raw);
  }

  @protected
  Float32List? api2wire_opt_list_prim_f_32(Float32List? raw) {
    return raw == null ? null : api2wire_list_prim_f_32(raw);
  }

  @protected
  Float64List? api2wire_opt_list_prim_f_64(Float64List? raw) {
    return raw == null ? null : api2wire_list_prim_f_64(raw);
  }

  @protected
  Int32List? api2wire_opt_list_prim_i_32(Int32List? raw) {
    return raw == null ? null : api2wire_list_prim_i_32(raw);
  }

  @protected
  Int8List? api2wire_opt_list_prim_i_8(Int8List? raw) {
    return raw == null ? null : api2wire_list_prim_i_8(raw);
  }

  @protected
  Uint8List? api2wire_opt_list_prim_u_8(Uint8List? raw) {
    return raw == null ? null : api2wire_list_prim_u_8(raw);
  }

  @protected
  List<dynamic> api2wire_opt_vecs(OptVecs raw) {
    return [
      api2wire_list_opt_box_autoadd_i_32(raw.i32),
      api2wire_list_opt_box_autoadd_weekdays(raw.enums),
      api2wire_list_opt_String(raw.strings),
      api2wire_list_opt_list_prim_i_32(raw.buffers)
    ];
  }

  @protected
  List<dynamic> api2wire_record_string_i_32((String, int) raw) {
    return [api2wire_String(raw.$1), api2wire_i_32(raw.$2)];
  }

  @protected
  List<dynamic> api2wire_sequences(Sequences raw) {
    return [api2wire_list_prim_i_32(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_some_struct(SomeStruct raw) {
    return [api2wire_u_32(raw.value)];
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
  List<dynamic> api2wire_sum_with(SumWith raw) {
    return [api2wire_u_32(raw.x)];
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

  @protected
  List<dynamic> api2wire_user_id(UserId raw) {
    return [api2wire_u_32(raw.value)];
  }
}

// Section: wire_class

class RustLibWire extends BaseWire {
  // TODO
  // : super(WasmModule.cast<RustLibWasmModule>(lib.wasmModule));
  RustLibWire.fromExternalLibrary(ExternalLibrary lib) {}

  void wire_boxed_blob(NativePortType port_, Uint8List blob) =>
      wasmModule.wire_boxed_blob(port_, blob);

  void wire_func_test_id(NativePortType port_, List<dynamic> id) =>
      wasmModule.wire_func_test_id(port_, id);

  void wire_get_array(NativePortType port_) => wasmModule.wire_get_array(port_);

  void wire_get_complex_array(NativePortType port_) =>
      wasmModule.wire_get_complex_array(port_);

  void wire_last_number(NativePortType port_, Float64List array) =>
      wasmModule.wire_last_number(port_, array);

  void wire_nested_id(NativePortType port_, List<dynamic> id) =>
      wasmModule.wire_nested_id(port_, id);

  void wire_new_msgid(NativePortType port_, Uint8List id) =>
      wasmModule.wire_new_msgid(port_, id);

  void wire_return_boxed_feed_id(NativePortType port_, Uint8List id) =>
      wasmModule.wire_return_boxed_feed_id(port_, id);

  void wire_return_boxed_raw_feed_id(NativePortType port_, List<dynamic> id) =>
      wasmModule.wire_return_boxed_raw_feed_id(port_, id);

  void wire_use_boxed_blob(NativePortType port_, List<dynamic> blob) =>
      wasmModule.wire_use_boxed_blob(port_, blob);

  void wire_use_msgid(NativePortType port_, List<dynamic> id) =>
      wasmModule.wire_use_msgid(port_, id);

  void wire_handle_customized_struct(NativePortType port_, List<dynamic> val) =>
      wasmModule.wire_handle_customized_struct(port_, val);

  void wire_next_user_id(NativePortType port_, List<dynamic> user_id) =>
      wasmModule.wire_next_user_id(port_, user_id);

  void wire_datetime_local(NativePortType port_, Object d) =>
      wasmModule.wire_datetime_local(port_, d);

  void wire_datetime_utc(NativePortType port_, Object d) =>
      wasmModule.wire_datetime_utc(port_, d);

  void wire_duration(NativePortType port_, Object d) =>
      wasmModule.wire_duration(port_, d);

  void wire_handle_durations(NativePortType port_,
          Object /* BigInt64Array */ durations, Object since) =>
      wasmModule.wire_handle_durations(port_, durations, since);

  void wire_handle_timestamps(NativePortType port_,
          Object /* BigInt64Array */ timestamps, Object epoch) =>
      wasmModule.wire_handle_timestamps(port_, timestamps, epoch);

  void wire_how_long_does_it_take(NativePortType port_, List<dynamic> mine) =>
      wasmModule.wire_how_long_does_it_take(port_, mine);

  void wire_naivedatetime(NativePortType port_, Object d) =>
      wasmModule.wire_naivedatetime(port_, d);

  void wire_optional_empty_datetime_utc(NativePortType port_, Object? d) =>
      wasmModule.wire_optional_empty_datetime_utc(port_, d);

  void wire_test_chrono(NativePortType port_) =>
      wasmModule.wire_test_chrono(port_);

  void wire_test_precise_chrono(NativePortType port_) =>
      wasmModule.wire_test_precise_chrono(port_);

  void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
          port_, that);

  void wire_StructWithCommentsTwinNormal_static_method_twin_normal(
          NativePortType port_) =>
      wasmModule
          .wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_);

  void wire_function_with_comments_slash_star_star_twin_normal(
          NativePortType port_) =>
      wasmModule.wire_function_with_comments_slash_star_star_twin_normal(port_);

  void wire_function_with_comments_triple_slash_multi_line_twin_normal(
          NativePortType port_) =>
      wasmModule
          .wire_function_with_comments_triple_slash_multi_line_twin_normal(
              port_);

  void wire_function_with_comments_triple_slash_single_line_twin_normal(
          NativePortType port_) =>
      wasmModule
          .wire_function_with_comments_triple_slash_single_line_twin_normal(
              port_);

  void wire_return_dart_dynamic(NativePortType port_) =>
      wasmModule.wire_return_dart_dynamic(port_);

  void wire_async_accept_dart_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_async_accept_dart_opaque(port_, opaque);

  void wire_create_enum_dart_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_create_enum_dart_opaque(port_, opaque);

  void wire_create_nested_dart_opaque(
          NativePortType port_, Object opaque1, Object opaque2) =>
      wasmModule.wire_create_nested_dart_opaque(port_, opaque1, opaque2);

  void wire_drop_static_dart_opaque(NativePortType port_) =>
      wasmModule.wire_drop_static_dart_opaque(port_);

  void wire_get_enum_dart_opaque(NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_get_enum_dart_opaque(port_, opaque);

  void wire_get_nested_dart_opaque(
          NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_get_nested_dart_opaque(port_, opaque);

  void wire_loop_back(NativePortType port_, Object opaque) =>
      wasmModule.wire_loop_back(port_, opaque);

  void wire_loop_back_array(NativePortType port_, Object opaque) =>
      wasmModule.wire_loop_back_array(port_, opaque);

  void wire_loop_back_array_get(NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_loop_back_array_get(port_, opaque);

  void wire_loop_back_option(NativePortType port_, Object opaque) =>
      wasmModule.wire_loop_back_option(port_, opaque);

  void wire_loop_back_option_get(NativePortType port_, Object? opaque) =>
      wasmModule.wire_loop_back_option_get(port_, opaque);

  void wire_loop_back_vec(NativePortType port_, Object opaque) =>
      wasmModule.wire_loop_back_vec(port_, opaque);

  void wire_loop_back_vec_get(NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_loop_back_vec_get(port_, opaque);

  void wire_panic_unwrap_dart_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_panic_unwrap_dart_opaque(port_, opaque);

  void wire_set_static_dart_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_set_static_dart_opaque(port_, opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_return_non_droppable_dart_opaque(Object opaque) =>
          wasmModule.wire_return_non_droppable_dart_opaque(opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_accept_dart_opaque(Object opaque) =>
          wasmModule.wire_sync_accept_dart_opaque(opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */ wire_sync_loopback(
          Object opaque) =>
      wasmModule.wire_sync_loopback(opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_dart_opaque(Object opaque) =>
          wasmModule.wire_sync_option_dart_opaque(opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_loopback(Object? opaque) =>
          wasmModule.wire_sync_option_loopback(opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_unwrap_dart_opaque(Object opaque) =>
          wasmModule.wire_unwrap_dart_opaque(opaque);

  void wire_func_enum_simple_twin_normal(NativePortType port_, int arg) =>
      wasmModule.wire_func_enum_simple_twin_normal(port_, arg);

  void wire_func_enum_with_item_mixed_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_enum_with_item_mixed_twin_normal(port_, arg);

  void wire_func_enum_with_item_struct_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_enum_with_item_struct_twin_normal(port_, arg);

  void wire_func_enum_with_item_tuple_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_enum_with_item_tuple_twin_normal(port_, arg);

  void wire_handle_enum_parameter(NativePortType port_, int weekday) =>
      wasmModule.wire_handle_enum_parameter(port_, weekday);

  void wire_handle_enum_struct(NativePortType port_, List<dynamic> val) =>
      wasmModule.wire_handle_enum_struct(port_, val);

  void wire_handle_return_enum(NativePortType port_, String input) =>
      wasmModule.wire_handle_return_enum(port_, input);

  void wire_multiply_by_ten(NativePortType port_, List<dynamic> measure) =>
      wasmModule.wire_multiply_by_ten(port_, measure);

  void wire_print_note(NativePortType port_, List<dynamic> note) =>
      wasmModule.wire_print_note(port_, note);

  void wire_Event_as_string(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_Event_as_string(port_, that);

  void wire_close_event_listener(NativePortType port_) =>
      wasmModule.wire_close_event_listener(port_);

  void wire_create_event(
          NativePortType port_, String address, String payload) =>
      wasmModule.wire_create_event(port_, address, payload);

  void wire_register_event_listener(NativePortType port_) =>
      wasmModule.wire_register_event_listener(port_);

  void wire_CustomStruct_new(NativePortType port_, String message) =>
      wasmModule.wire_CustomStruct_new(port_, message);

  void wire_CustomStruct_nonstatic_return_custom_struct_error(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_CustomStruct_nonstatic_return_custom_struct_error(
          port_, that);

  void wire_CustomStruct_nonstatic_return_custom_struct_ok(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_CustomStruct_nonstatic_return_custom_struct_ok(
          port_, that);

  void wire_CustomStruct_static_return_custom_struct_error(
          NativePortType port_) =>
      wasmModule.wire_CustomStruct_static_return_custom_struct_error(port_);

  void wire_CustomStruct_static_return_custom_struct_ok(NativePortType port_) =>
      wasmModule.wire_CustomStruct_static_return_custom_struct_ok(port_);

  void wire_SomeStruct_new(NativePortType port_, int value) =>
      wasmModule.wire_SomeStruct_new(port_, value);

  void wire_SomeStruct_non_static_return_err_custom_error(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_SomeStruct_non_static_return_err_custom_error(
          port_, that);

  void wire_SomeStruct_non_static_return_ok_custom_error(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_SomeStruct_non_static_return_ok_custom_error(port_, that);

  void wire_SomeStruct_static_return_err_custom_error(NativePortType port_) =>
      wasmModule.wire_SomeStruct_static_return_err_custom_error(port_);

  void wire_SomeStruct_static_return_ok_custom_error(NativePortType port_) =>
      wasmModule.wire_SomeStruct_static_return_ok_custom_error(port_);

  void wire_custom_enum_error_panic_twin_normal(NativePortType port_) =>
      wasmModule.wire_custom_enum_error_panic_twin_normal(port_);

  void wire_custom_enum_error_return_error_twin_normal(NativePortType port_) =>
      wasmModule.wire_custom_enum_error_return_error_twin_normal(port_);

  void wire_custom_enum_error_return_ok_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_custom_enum_error_return_ok_twin_normal(port_, arg);

  void wire_custom_nested_error_return_error_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_custom_nested_error_return_error_twin_normal(port_, arg);

  void wire_custom_struct_error_return_error_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_custom_struct_error_return_error_twin_normal(port_, arg);

  void wire_func_return_error_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_return_error_twin_normal(port_);

  void wire_func_type_fallible_panic_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_type_fallible_panic_twin_normal(port_);

  void wire_func_type_infallible_panic_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_type_infallible_panic_twin_normal(port_);

  void wire_panic_with_custom_result(NativePortType port_) =>
      wasmModule.wire_panic_with_custom_result(port_);

  void wire_return_custom_nested_error_1(NativePortType port_) =>
      wasmModule.wire_return_custom_nested_error_1(port_);

  void wire_return_custom_nested_error_1_variant1(NativePortType port_) =>
      wasmModule.wire_return_custom_nested_error_1_variant1(port_);

  void wire_return_custom_nested_error_2(NativePortType port_) =>
      wasmModule.wire_return_custom_nested_error_2(port_);

  void wire_return_custom_struct_error(NativePortType port_) =>
      wasmModule.wire_return_custom_struct_error(port_);

  void wire_return_custom_struct_ok(NativePortType port_) =>
      wasmModule.wire_return_custom_struct_ok(port_);

  void wire_return_err_custom_error(NativePortType port_) =>
      wasmModule.wire_return_err_custom_error(port_);

  void wire_return_error_variant(NativePortType port_, int variant) =>
      wasmModule.wire_return_error_variant(port_, variant);

  void wire_return_ok_custom_error(NativePortType port_) =>
      wasmModule.wire_return_ok_custom_error(port_);

  void wire_stream_sink_throw_anyhow(NativePortType port_) =>
      wasmModule.wire_stream_sink_throw_anyhow(port_);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_return_custom_struct_error() =>
          wasmModule.wire_sync_return_custom_struct_error();

  void wire_throw_anyhow(NativePortType port_) =>
      wasmModule.wire_throw_anyhow(port_);

  void wire_call_new_module_system(NativePortType port_) =>
      wasmModule.wire_call_new_module_system(port_);

  void wire_call_old_module_system(NativePortType port_) =>
      wasmModule.wire_call_old_module_system(port_);

  void wire_use_imported_enum(NativePortType port_, int my_enum) =>
      wasmModule.wire_use_imported_enum(port_, my_enum);

  void wire_use_imported_struct(
          NativePortType port_, List<dynamic> my_struct) =>
      wasmModule.wire_use_imported_struct(port_, my_struct);

  void wire_another_macro_struct(NativePortType port_) =>
      wasmModule.wire_another_macro_struct(port_);

  void wire_func_macro_struct(NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_macro_struct(port_, arg);

  void wire_ConcatenateWith_concatenate(
          NativePortType port_, List<dynamic> that, String b) =>
      wasmModule.wire_ConcatenateWith_concatenate(port_, that, b);

  void wire_ConcatenateWith_concatenate_static(
          NativePortType port_, String a, String b) =>
      wasmModule.wire_ConcatenateWith_concatenate_static(port_, a, b);

  void wire_ConcatenateWith_handle_some_static_stream_sink(
          NativePortType port_, int key, int max) =>
      wasmModule.wire_ConcatenateWith_handle_some_static_stream_sink(
          port_, key, max);

  void wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
          NativePortType port_) =>
      wasmModule.wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
          port_);

  void wire_ConcatenateWith_handle_some_stream_sink(
          NativePortType port_, List<dynamic> that, int key, int max) =>
      wasmModule.wire_ConcatenateWith_handle_some_stream_sink(
          port_, that, key, max);

  void wire_ConcatenateWith_handle_some_stream_sink_at_1(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_ConcatenateWith_handle_some_stream_sink_at_1(port_, that);

  void wire_ConcatenateWith_new(NativePortType port_, String a) =>
      wasmModule.wire_ConcatenateWith_new(port_, a);

  void wire_SumWith_sum(
          NativePortType port_, List<dynamic> that, int y, int z) =>
      wasmModule.wire_SumWith_sum(port_, that, y, z);

  void wire_get_sum_array(NativePortType port_, int a, int b, int c) =>
      wasmModule.wire_get_sum_array(port_, a, b, c);

  void wire_get_sum_struct(NativePortType port_) =>
      wasmModule.wire_get_sum_struct(port_);

  void wire_app_settings_stream(NativePortType port_) =>
      wasmModule.wire_app_settings_stream(port_);

  void wire_app_settings_vec_stream(NativePortType port_) =>
      wasmModule.wire_app_settings_vec_stream(port_);

  void wire_first_number(NativePortType port_, List<dynamic> nums) =>
      wasmModule.wire_first_number(port_, nums);

  void wire_first_sequence(NativePortType port_, List<dynamic> seqs) =>
      wasmModule.wire_first_sequence(port_, seqs);

  void wire_get_app_settings(NativePortType port_) =>
      wasmModule.wire_get_app_settings(port_);

  void wire_get_fallible_app_settings(NativePortType port_) =>
      wasmModule.wire_get_fallible_app_settings(port_);

  void wire_get_message(NativePortType port_) =>
      wasmModule.wire_get_message(port_);

  void wire_is_app_embedded(NativePortType port_, List<dynamic> app_settings) =>
      wasmModule.wire_is_app_embedded(port_, app_settings);

  void wire_mirror_struct_stream(NativePortType port_) =>
      wasmModule.wire_mirror_struct_stream(port_);

  void wire_mirror_tuple_stream(NativePortType port_) =>
      wasmModule.wire_mirror_tuple_stream(port_);

  void wire_repeat_number(NativePortType port_, int num, int times) =>
      wasmModule.wire_repeat_number(port_, num, times);

  void wire_repeat_sequence(NativePortType port_, int seq, int times) =>
      wasmModule.wire_repeat_sequence(port_, seq, times);

  void wire_test_contains_mirrored_sub_struct(NativePortType port_) =>
      wasmModule.wire_test_contains_mirrored_sub_struct(port_);

  void wire_test_fallible_of_raw_string_mirrored(NativePortType port_) =>
      wasmModule.wire_test_fallible_of_raw_string_mirrored(port_);

  void wire_test_list_of_nested_enums_mirrored(NativePortType port_) =>
      wasmModule.wire_test_list_of_nested_enums_mirrored(port_);

  void wire_test_list_of_raw_nested_string_mirrored(NativePortType port_) =>
      wasmModule.wire_test_list_of_raw_nested_string_mirrored(port_);

  void wire_test_nested_raw_string_mirrored(NativePortType port_) =>
      wasmModule.wire_test_nested_raw_string_mirrored(port_);

  void wire_test_raw_string_enum_mirrored(NativePortType port_, bool nested) =>
      wasmModule.wire_test_raw_string_enum_mirrored(port_, nested);

  void wire_test_raw_string_mirrored(NativePortType port_) =>
      wasmModule.wire_test_raw_string_mirrored(port_);

  void wire_handle_big_buffers(NativePortType port_) =>
      wasmModule.wire_handle_big_buffers(port_);

  void wire_handle_complex_struct(NativePortType port_, List<dynamic> s) =>
      wasmModule.wire_handle_complex_struct(port_, s);

  void wire_handle_nested_struct(NativePortType port_, List<dynamic> s) =>
      wasmModule.wire_handle_nested_struct(port_, s);

  void wire_handle_string(NativePortType port_, String s) =>
      wasmModule.wire_handle_string(port_, s);

  void wire_handle_struct(
          NativePortType port_, List<dynamic> arg, List<dynamic> boxed) =>
      wasmModule.wire_handle_struct(port_, arg, boxed);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_handle_struct_sync_freezed(List<dynamic> arg, List<dynamic> boxed) =>
          wasmModule.wire_handle_struct_sync_freezed(arg, boxed);

  void wire_handle_vec_u8(NativePortType port_, Uint8List v) =>
      wasmModule.wire_handle_vec_u8(port_, v);

  void wire_list_of_primitive_enums(
          NativePortType port_, List<dynamic> weekdays) =>
      wasmModule.wire_list_of_primitive_enums(port_, weekdays);

  void wire_test_abc_enum(NativePortType port_, List<dynamic> abc) =>
      wasmModule.wire_test_abc_enum(port_, abc);

  void wire_test_struct_with_enum(NativePortType port_, List<dynamic> se) =>
      wasmModule.wire_test_struct_with_enum(port_, se);

  void wire_empty_struct(NativePortType port_, List<dynamic> empty) =>
      wasmModule.wire_empty_struct(port_, empty);

  void wire_func_return_unit_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_return_unit_twin_normal(port_);

  void wire_func_string_twin_normal(NativePortType port_, String arg) =>
      wasmModule.wire_func_string_twin_normal(port_, arg);

  void wire_handle_list_of_struct(NativePortType port_, List<dynamic> l) =>
      wasmModule.wire_handle_list_of_struct(port_, l);

  void wire_handle_string_list(NativePortType port_, List<String> names) =>
      wasmModule.wire_handle_string_list(port_, names);

  void wire_handle_newtype(NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_handle_newtype(port_, arg);

  void wire_handle_increment_boxed_optional(
          NativePortType port_, double? opt) =>
      wasmModule.wire_handle_increment_boxed_optional(port_, opt);

  void wire_handle_option_box_arguments(
          NativePortType port_,
          int? i8box,
          int? u8box,
          int? i32box,
          Object? i64box,
          double? f64box,
          bool? boolbox,
          List<dynamic>? structbox) =>
      wasmModule.wire_handle_option_box_arguments(
          port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox);

  void wire_handle_optional_increment(
          NativePortType port_, List<dynamic>? opt) =>
      wasmModule.wire_handle_optional_increment(port_, opt);

  void wire_handle_optional_return(
          NativePortType port_, double left, double right) =>
      wasmModule.wire_handle_optional_return(port_, left, right);

  void wire_handle_optional_struct(NativePortType port_, String? document) =>
      wasmModule.wire_handle_optional_struct(port_, document);

  void wire_handle_vec_of_opts(NativePortType port_, List<dynamic> opt) =>
      wasmModule.wire_handle_vec_of_opts(port_, opt);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option() => wasmModule.wire_sync_option();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_null() => wasmModule.wire_sync_option_null();

  void wire_primitive_optional_types(NativePortType port_, int? my_i32,
          Object? my_i64, double? my_f64, bool? my_bool) =>
      wasmModule.wire_primitive_optional_types(
          port_, my_i32, my_i64, my_f64, my_bool);

  void wire_handle_vec_of_primitive(NativePortType port_, int n) =>
      wasmModule.wire_handle_vec_of_primitive(port_, n);

  void wire_handle_zero_copy_vec_of_primitive(NativePortType port_, int n) =>
      wasmModule.wire_handle_zero_copy_vec_of_primitive(port_, n);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_handle_zero_copy_vec_of_primitive_sync(int n) =>
          wasmModule.wire_handle_zero_copy_vec_of_primitive_sync(n);

  void wire_primitive_types(NativePortType port_, int my_i32, Object my_i64,
          double my_f64, bool my_bool) =>
      wasmModule.wire_primitive_types(port_, my_i32, my_i64, my_f64, my_bool);

  void wire_primitive_u32(NativePortType port_, int my_u32) =>
      wasmModule.wire_primitive_u32(port_, my_u32);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_StructWithCommentsTwinSync_instance_method_twin_sync(
              List<dynamic> that) =>
          wasmModule
              .wire_StructWithCommentsTwinSync_instance_method_twin_sync(that);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_StructWithCommentsTwinSync_static_method_twin_sync() =>
          wasmModule.wire_StructWithCommentsTwinSync_static_method_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_slash_star_star_twin_sync() =>
          wasmModule.wire_function_with_comments_slash_star_star_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_triple_slash_multi_line_twin_sync() =>
          wasmModule
              .wire_function_with_comments_triple_slash_multi_line_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_triple_slash_single_line_twin_sync() =>
          wasmModule
              .wire_function_with_comments_triple_slash_single_line_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_simple_twin_sync(int arg) =>
          wasmModule.wire_func_enum_simple_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_mixed_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_enum_with_item_mixed_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_struct_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_enum_with_item_struct_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_tuple_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_enum_with_item_tuple_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_panic_twin_sync() =>
          wasmModule.wire_custom_enum_error_panic_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_return_error_twin_sync() =>
          wasmModule.wire_custom_enum_error_return_error_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_return_ok_twin_sync(int arg) =>
          wasmModule.wire_custom_enum_error_return_ok_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_nested_error_return_error_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_custom_nested_error_return_error_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_struct_error_return_error_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_custom_struct_error_return_error_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_return_error_twin_sync() =>
          wasmModule.wire_func_return_error_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_type_fallible_panic_twin_sync() =>
          wasmModule.wire_func_type_fallible_panic_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_type_infallible_panic_twin_sync() =>
          wasmModule.wire_func_type_infallible_panic_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_return_unit_twin_sync() =>
          wasmModule.wire_func_return_unit_twin_sync();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_string_twin_sync(String arg) =>
          wasmModule.wire_func_string_twin_sync(arg);

  void wire_example_optional_primitive_type_bool_twin_normal(
          NativePortType port_, bool? arg) =>
      wasmModule.wire_example_optional_primitive_type_bool_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_f32_twin_normal(
          NativePortType port_, double? arg) =>
      wasmModule.wire_example_optional_primitive_type_f32_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_f64_twin_normal(
          NativePortType port_, double? arg) =>
      wasmModule.wire_example_optional_primitive_type_f64_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_i16_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_i16_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_i32_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_i32_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_i64_twin_normal(
          NativePortType port_, Object? arg) =>
      wasmModule.wire_example_optional_primitive_type_i64_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_i8_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_i8_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_u16_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_u16_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_u32_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_u32_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_u64_twin_normal(
          NativePortType port_, Object? arg) =>
      wasmModule.wire_example_optional_primitive_type_u64_twin_normal(
          port_, arg);

  void wire_example_optional_primitive_type_u8_twin_normal(
          NativePortType port_, int? arg) =>
      wasmModule.wire_example_optional_primitive_type_u8_twin_normal(
          port_, arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_bool_twin_sync(bool? arg) =>
          wasmModule.wire_example_optional_primitive_type_bool_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_f32_twin_sync(double? arg) =>
          wasmModule.wire_example_optional_primitive_type_f32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_f64_twin_sync(double? arg) =>
          wasmModule.wire_example_optional_primitive_type_f64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i16_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_i16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i32_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_i32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i64_twin_sync(Object? arg) =>
          wasmModule.wire_example_optional_primitive_type_i64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i8_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_i8_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u16_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_u16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u32_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_u32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u64_twin_sync(Object? arg) =>
          wasmModule.wire_example_optional_primitive_type_u64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u8_twin_sync(int? arg) =>
          wasmModule.wire_example_optional_primitive_type_u8_twin_sync(arg);

  void wire_example_primitive_type_bool_twin_normal(
          NativePortType port_, bool arg) =>
      wasmModule.wire_example_primitive_type_bool_twin_normal(port_, arg);

  void wire_example_primitive_type_f32_twin_normal(
          NativePortType port_, double arg) =>
      wasmModule.wire_example_primitive_type_f32_twin_normal(port_, arg);

  void wire_example_primitive_type_f64_twin_normal(
          NativePortType port_, double arg) =>
      wasmModule.wire_example_primitive_type_f64_twin_normal(port_, arg);

  void wire_example_primitive_type_i16_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_i16_twin_normal(port_, arg);

  void wire_example_primitive_type_i32_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_i32_twin_normal(port_, arg);

  void wire_example_primitive_type_i64_twin_normal(
          NativePortType port_, Object arg) =>
      wasmModule.wire_example_primitive_type_i64_twin_normal(port_, arg);

  void wire_example_primitive_type_i8_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_i8_twin_normal(port_, arg);

  void wire_example_primitive_type_u16_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_u16_twin_normal(port_, arg);

  void wire_example_primitive_type_u32_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_u32_twin_normal(port_, arg);

  void wire_example_primitive_type_u64_twin_normal(
          NativePortType port_, Object arg) =>
      wasmModule.wire_example_primitive_type_u64_twin_normal(port_, arg);

  void wire_example_primitive_type_u8_twin_normal(
          NativePortType port_, int arg) =>
      wasmModule.wire_example_primitive_type_u8_twin_normal(port_, arg);

  void wire_example_primitive_list_type_bool_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_example_primitive_list_type_bool_twin_normal(port_, arg);

  void wire_example_primitive_list_type_f32_twin_normal(
          NativePortType port_, Float32List arg) =>
      wasmModule.wire_example_primitive_list_type_f32_twin_normal(port_, arg);

  void wire_example_primitive_list_type_f64_twin_normal(
          NativePortType port_, Float64List arg) =>
      wasmModule.wire_example_primitive_list_type_f64_twin_normal(port_, arg);

  void wire_example_primitive_list_type_i16_twin_normal(
          NativePortType port_, Int16List arg) =>
      wasmModule.wire_example_primitive_list_type_i16_twin_normal(port_, arg);

  void wire_example_primitive_list_type_i32_twin_normal(
          NativePortType port_, Int32List arg) =>
      wasmModule.wire_example_primitive_list_type_i32_twin_normal(port_, arg);

  void wire_example_primitive_list_type_i64_twin_normal(
          NativePortType port_, Object /* BigInt64Array */ arg) =>
      wasmModule.wire_example_primitive_list_type_i64_twin_normal(port_, arg);

  void wire_example_primitive_list_type_i8_twin_normal(
          NativePortType port_, Int8List arg) =>
      wasmModule.wire_example_primitive_list_type_i8_twin_normal(port_, arg);

  void wire_example_primitive_list_type_u16_twin_normal(
          NativePortType port_, Uint16List arg) =>
      wasmModule.wire_example_primitive_list_type_u16_twin_normal(port_, arg);

  void wire_example_primitive_list_type_u32_twin_normal(
          NativePortType port_, Uint32List arg) =>
      wasmModule.wire_example_primitive_list_type_u32_twin_normal(port_, arg);

  void wire_example_primitive_list_type_u64_twin_normal(
          NativePortType port_, Object /* BigInt64Array */ arg) =>
      wasmModule.wire_example_primitive_list_type_u64_twin_normal(port_, arg);

  void wire_example_primitive_list_type_u8_twin_normal(
          NativePortType port_, Uint8List arg) =>
      wasmModule.wire_example_primitive_list_type_u8_twin_normal(port_, arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_bool_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_example_primitive_list_type_bool_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_f32_twin_sync(Float32List arg) =>
          wasmModule.wire_example_primitive_list_type_f32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_f64_twin_sync(Float64List arg) =>
          wasmModule.wire_example_primitive_list_type_f64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i16_twin_sync(Int16List arg) =>
          wasmModule.wire_example_primitive_list_type_i16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i32_twin_sync(Int32List arg) =>
          wasmModule.wire_example_primitive_list_type_i32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i64_twin_sync(
              Object /* BigInt64Array */ arg) =>
          wasmModule.wire_example_primitive_list_type_i64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i8_twin_sync(Int8List arg) =>
          wasmModule.wire_example_primitive_list_type_i8_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u16_twin_sync(Uint16List arg) =>
          wasmModule.wire_example_primitive_list_type_u16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u32_twin_sync(Uint32List arg) =>
          wasmModule.wire_example_primitive_list_type_u32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u64_twin_sync(
              Object /* BigInt64Array */ arg) =>
          wasmModule.wire_example_primitive_list_type_u64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u8_twin_sync(Uint8List arg) =>
          wasmModule.wire_example_primitive_list_type_u8_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_bool_twin_sync(bool arg) =>
          wasmModule.wire_example_primitive_type_bool_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_f32_twin_sync(double arg) =>
          wasmModule.wire_example_primitive_type_f32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_f64_twin_sync(double arg) =>
          wasmModule.wire_example_primitive_type_f64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i16_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_i16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i32_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_i32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i64_twin_sync(Object arg) =>
          wasmModule.wire_example_primitive_type_i64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i8_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_i8_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u16_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_u16_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u32_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_u32_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u64_twin_sync(Object arg) =>
          wasmModule.wire_example_primitive_type_u64_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u8_twin_sync(int arg) =>
          wasmModule.wire_example_primitive_type_u8_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_simple_adder_twin_sync(int a, int b) =>
          wasmModule.wire_simple_adder_twin_sync(a, b);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_one_field_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_struct_with_one_field_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_two_field_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_struct_with_two_field_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_zero_field_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_struct_with_zero_field_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_tuple_struct_with_one_field_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_tuple_struct_with_one_field_twin_sync(arg);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_tuple_struct_with_two_field_twin_sync(List<dynamic> arg) =>
          wasmModule.wire_func_tuple_struct_with_two_field_twin_sync(arg);

  void wire_test_more_than_just_one_raw_string_struct(NativePortType port_) =>
      wasmModule.wire_test_more_than_just_one_raw_string_struct(port_);

  void wire_test_raw_string_item_struct(NativePortType port_) =>
      wasmModule.wire_test_raw_string_item_struct(port_);

  void wire_create_array_opaque_enum(NativePortType port_) =>
      wasmModule.wire_create_array_opaque_enum(port_);

  void wire_create_nested_opaque(NativePortType port_) =>
      wasmModule.wire_create_nested_opaque(port_);

  void wire_create_opaque(NativePortType port_) =>
      wasmModule.wire_create_opaque(port_);

  void wire_create_option_opaque(NativePortType port_, Object? opaque) =>
      wasmModule.wire_create_option_opaque(port_, opaque);

  void wire_create_sync_opaque(NativePortType port_) =>
      wasmModule.wire_create_sync_opaque(port_);

  void wire_frb_generator_test(NativePortType port_) =>
      wasmModule.wire_frb_generator_test(port_);

  void wire_opaque_array(NativePortType port_) =>
      wasmModule.wire_opaque_array(port_);

  void wire_opaque_array_run(NativePortType port_, List<dynamic> data) =>
      wasmModule.wire_opaque_array_run(port_, data);

  void wire_opaque_vec(NativePortType port_) =>
      wasmModule.wire_opaque_vec(port_);

  void wire_opaque_vec_run(NativePortType port_, List<dynamic> data) =>
      wasmModule.wire_opaque_vec_run(port_, data);

  void wire_run_enum_opaque(NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_run_enum_opaque(port_, opaque);

  void wire_run_nested_opaque(NativePortType port_, List<dynamic> opaque) =>
      wasmModule.wire_run_nested_opaque(port_, opaque);

  void wire_run_non_clone(NativePortType port_, Object clone) =>
      wasmModule.wire_run_non_clone(port_, clone);

  void wire_run_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_run_opaque(port_, opaque);

  void wire_run_opaque_with_delay(NativePortType port_, Object opaque) =>
      wasmModule.wire_run_opaque_with_delay(port_, opaque);

  void wire_unwrap_rust_opaque(NativePortType port_, Object opaque) =>
      wasmModule.wire_unwrap_rust_opaque(port_, opaque);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_frb_sync_generator_test() =>
          wasmModule.wire_frb_sync_generator_test();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_non_clone() => wasmModule.wire_sync_create_non_clone();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_opaque() => wasmModule.wire_sync_create_opaque();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_sync_opaque() =>
          wasmModule.wire_sync_create_sync_opaque();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_rust_opaque() =>
          wasmModule.wire_sync_option_rust_opaque();

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_run_opaque(Object opaque) =>
          wasmModule.wire_sync_run_opaque(opaque);

  void wire_simple_adder_twin_normal(NativePortType port_, int a, int b) =>
      wasmModule.wire_simple_adder_twin_normal(port_, a, b);

  void wire_func_stream_realistic_twin_normal(
          NativePortType port_, String arg) =>
      wasmModule.wire_func_stream_realistic_twin_normal(port_, arg);

  void wire_func_stream_return_error_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_stream_return_error_twin_normal(port_);

  void wire_func_stream_return_panic_twin_normal(NativePortType port_) =>
      wasmModule.wire_func_stream_return_panic_twin_normal(port_);

  void wire_func_stream_sink_arg_position_twin_normal(
          NativePortType port_, int a, int b) =>
      wasmModule.wire_func_stream_sink_arg_position_twin_normal(port_, a, b);

  void wire_handle_stream_of_struct(NativePortType port_) =>
      wasmModule.wire_handle_stream_of_struct(port_);

  void wire_handle_stream_sink_at_1(NativePortType port_, int key, int max) =>
      wasmModule.wire_handle_stream_sink_at_1(port_, key, max);

  void wire_handle_stream_sink_at_2(NativePortType port_, int key, int max) =>
      wasmModule.wire_handle_stream_sink_at_2(port_, key, max);

  void wire_handle_stream_sink_at_3(NativePortType port_, int key, int max) =>
      wasmModule.wire_handle_stream_sink_at_3(port_, key, max);

  void wire_func_struct_with_one_field_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_struct_with_one_field_twin_normal(port_, arg);

  void wire_func_struct_with_two_field_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_struct_with_two_field_twin_normal(port_, arg);

  void wire_func_struct_with_zero_field_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_struct_with_zero_field_twin_normal(port_, arg);

  void wire_func_tuple_struct_with_one_field_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_tuple_struct_with_one_field_twin_normal(port_, arg);

  void wire_func_tuple_struct_with_two_field_twin_normal(
          NativePortType port_, List<dynamic> arg) =>
      wasmModule.wire_func_tuple_struct_with_two_field_twin_normal(port_, arg);

  void wire_test_tuple(NativePortType port_, List<dynamic>? value) =>
      wasmModule.wire_test_tuple(port_, value);

  void wire_test_tuple_2(NativePortType port_, List<dynamic> value) =>
      wasmModule.wire_test_tuple_2(port_, value);

  void wire_handle_type_alias_id(NativePortType port_, Object input) =>
      wasmModule.wire_handle_type_alias_id(port_, input);

  void wire_handle_type_alias_model(NativePortType port_, Object input) =>
      wasmModule.wire_handle_type_alias_model(port_, input);

  void wire_handle_type_nest_alias_id(NativePortType port_, Object input) =>
      wasmModule.wire_handle_type_nest_alias_id(port_, input);

  void wire_handle_nested_uuids(NativePortType port_, List<dynamic> ids) =>
      wasmModule.wire_handle_nested_uuids(port_, ids);

  void wire_handle_uuid(NativePortType port_, Uint8List id) =>
      wasmModule.wire_handle_uuid(port_, id);

  void wire_handle_uuids(NativePortType port_, Uint8List ids) =>
      wasmModule.wire_handle_uuids(port_, ids);

  void drop_opaque_RustOpaque_MutexHideData(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_MutexHideData(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_MutexHideData(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_MutexHideData(ptr);

  void drop_opaque_RustOpaque_RwLockHideData(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_RwLockHideData(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_RwLockHideData(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_RwLockHideData(ptr);

  void drop_opaque_RustOpaque_box_dynDartDebug(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_box_dynDartDebug(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_box_dynDartDebug(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_box_dynDartDebug(ptr);

  void drop_opaque_RustOpaque_frb_opaque_return(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_frb_opaque_return(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_frb_opaque_return(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_frb_opaque_return(ptr);

  void drop_opaque_RustOpaque_frb_opaque_sync_return(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_frb_opaque_sync_return(ptr);

  int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_frb_opaque_sync_return(dynamic ptr) =>
          wasmModule.share_opaque_RustOpaque_frb_opaque_sync_return(ptr);

  void drop_opaque_RustOpaque_hide_data(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_hide_data(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_hide_data(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_hide_data(ptr);

  void drop_opaque_RustOpaque_i_32(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_i_32(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_i_32(dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_i_32(ptr);

  void drop_opaque_RustOpaque_non_clone_data(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_non_clone_data(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_non_clone_data(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_non_clone_data(ptr);

  void drop_opaque_RustOpaque_non_send_hide_data(dynamic ptr) =>
      wasmModule.drop_opaque_RustOpaque_non_send_hide_data(ptr);

  int /* *const std::ffi::c_void */ share_opaque_RustOpaque_non_send_hide_data(
          dynamic ptr) =>
      wasmModule.share_opaque_RustOpaque_non_send_hide_data(ptr);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);

  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external void wire_boxed_blob(NativePortType port_, Uint8List blob);

  external void wire_func_test_id(NativePortType port_, List<dynamic> id);

  external void wire_get_array(NativePortType port_);

  external void wire_get_complex_array(NativePortType port_);

  external void wire_last_number(NativePortType port_, Float64List array);

  external void wire_nested_id(NativePortType port_, List<dynamic> id);

  external void wire_new_msgid(NativePortType port_, Uint8List id);

  external void wire_return_boxed_feed_id(NativePortType port_, Uint8List id);

  external void wire_return_boxed_raw_feed_id(
      NativePortType port_, List<dynamic> id);

  external void wire_use_boxed_blob(NativePortType port_, List<dynamic> blob);

  external void wire_use_msgid(NativePortType port_, List<dynamic> id);

  external void wire_handle_customized_struct(
      NativePortType port_, List<dynamic> val);

  external void wire_next_user_id(NativePortType port_, List<dynamic> user_id);

  external void wire_datetime_local(NativePortType port_, Object d);

  external void wire_datetime_utc(NativePortType port_, Object d);

  external void wire_duration(NativePortType port_, Object d);

  external void wire_handle_durations(
      NativePortType port_, Object /* BigInt64Array */ durations, Object since);

  external void wire_handle_timestamps(NativePortType port_,
      Object /* BigInt64Array */ timestamps, Object epoch);

  external void wire_how_long_does_it_take(
      NativePortType port_, List<dynamic> mine);

  external void wire_naivedatetime(NativePortType port_, Object d);

  external void wire_optional_empty_datetime_utc(
      NativePortType port_, Object? d);

  external void wire_test_chrono(NativePortType port_);

  external void wire_test_precise_chrono(NativePortType port_);

  external void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
      NativePortType port_, List<dynamic> that);

  external void wire_StructWithCommentsTwinNormal_static_method_twin_normal(
      NativePortType port_);

  external void wire_function_with_comments_slash_star_star_twin_normal(
      NativePortType port_);

  external void wire_function_with_comments_triple_slash_multi_line_twin_normal(
      NativePortType port_);

  external void
      wire_function_with_comments_triple_slash_single_line_twin_normal(
          NativePortType port_);

  external void wire_return_dart_dynamic(NativePortType port_);

  external void wire_async_accept_dart_opaque(
      NativePortType port_, Object opaque);

  external void wire_create_enum_dart_opaque(
      NativePortType port_, Object opaque);

  external void wire_create_nested_dart_opaque(
      NativePortType port_, Object opaque1, Object opaque2);

  external void wire_drop_static_dart_opaque(NativePortType port_);

  external void wire_get_enum_dart_opaque(
      NativePortType port_, List<dynamic> opaque);

  external void wire_get_nested_dart_opaque(
      NativePortType port_, List<dynamic> opaque);

  external void wire_loop_back(NativePortType port_, Object opaque);

  external void wire_loop_back_array(NativePortType port_, Object opaque);

  external void wire_loop_back_array_get(
      NativePortType port_, List<dynamic> opaque);

  external void wire_loop_back_option(NativePortType port_, Object opaque);

  external void wire_loop_back_option_get(NativePortType port_, Object? opaque);

  external void wire_loop_back_vec(NativePortType port_, Object opaque);

  external void wire_loop_back_vec_get(
      NativePortType port_, List<dynamic> opaque);

  external void wire_panic_unwrap_dart_opaque(
      NativePortType port_, Object opaque);

  external void wire_set_static_dart_opaque(
      NativePortType port_, Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_return_non_droppable_dart_opaque(Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_accept_dart_opaque(Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_loopback(Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_dart_opaque(Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_loopback(Object? opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_unwrap_dart_opaque(Object opaque);

  external void wire_func_enum_simple_twin_normal(
      NativePortType port_, int arg);

  external void wire_func_enum_with_item_mixed_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_enum_with_item_struct_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_enum_with_item_tuple_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_handle_enum_parameter(NativePortType port_, int weekday);

  external void wire_handle_enum_struct(
      NativePortType port_, List<dynamic> val);

  external void wire_handle_return_enum(NativePortType port_, String input);

  external void wire_multiply_by_ten(
      NativePortType port_, List<dynamic> measure);

  external void wire_print_note(NativePortType port_, List<dynamic> note);

  external void wire_Event_as_string(NativePortType port_, List<dynamic> that);

  external void wire_close_event_listener(NativePortType port_);

  external void wire_create_event(
      NativePortType port_, String address, String payload);

  external void wire_register_event_listener(NativePortType port_);

  external void wire_CustomStruct_new(NativePortType port_, String message);

  external void wire_CustomStruct_nonstatic_return_custom_struct_error(
      NativePortType port_, List<dynamic> that);

  external void wire_CustomStruct_nonstatic_return_custom_struct_ok(
      NativePortType port_, List<dynamic> that);

  external void wire_CustomStruct_static_return_custom_struct_error(
      NativePortType port_);

  external void wire_CustomStruct_static_return_custom_struct_ok(
      NativePortType port_);

  external void wire_SomeStruct_new(NativePortType port_, int value);

  external void wire_SomeStruct_non_static_return_err_custom_error(
      NativePortType port_, List<dynamic> that);

  external void wire_SomeStruct_non_static_return_ok_custom_error(
      NativePortType port_, List<dynamic> that);

  external void wire_SomeStruct_static_return_err_custom_error(
      NativePortType port_);

  external void wire_SomeStruct_static_return_ok_custom_error(
      NativePortType port_);

  external void wire_custom_enum_error_panic_twin_normal(NativePortType port_);

  external void wire_custom_enum_error_return_error_twin_normal(
      NativePortType port_);

  external void wire_custom_enum_error_return_ok_twin_normal(
      NativePortType port_, int arg);

  external void wire_custom_nested_error_return_error_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_custom_struct_error_return_error_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_return_error_twin_normal(NativePortType port_);

  external void wire_func_type_fallible_panic_twin_normal(NativePortType port_);

  external void wire_func_type_infallible_panic_twin_normal(
      NativePortType port_);

  external void wire_panic_with_custom_result(NativePortType port_);

  external void wire_return_custom_nested_error_1(NativePortType port_);

  external void wire_return_custom_nested_error_1_variant1(
      NativePortType port_);

  external void wire_return_custom_nested_error_2(NativePortType port_);

  external void wire_return_custom_struct_error(NativePortType port_);

  external void wire_return_custom_struct_ok(NativePortType port_);

  external void wire_return_err_custom_error(NativePortType port_);

  external void wire_return_error_variant(NativePortType port_, int variant);

  external void wire_return_ok_custom_error(NativePortType port_);

  external void wire_stream_sink_throw_anyhow(NativePortType port_);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_return_custom_struct_error();

  external void wire_throw_anyhow(NativePortType port_);

  external void wire_call_new_module_system(NativePortType port_);

  external void wire_call_old_module_system(NativePortType port_);

  external void wire_use_imported_enum(NativePortType port_, int my_enum);

  external void wire_use_imported_struct(
      NativePortType port_, List<dynamic> my_struct);

  external void wire_another_macro_struct(NativePortType port_);

  external void wire_func_macro_struct(NativePortType port_, List<dynamic> arg);

  external void wire_ConcatenateWith_concatenate(
      NativePortType port_, List<dynamic> that, String b);

  external void wire_ConcatenateWith_concatenate_static(
      NativePortType port_, String a, String b);

  external void wire_ConcatenateWith_handle_some_static_stream_sink(
      NativePortType port_, int key, int max);

  external void wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
      NativePortType port_);

  external void wire_ConcatenateWith_handle_some_stream_sink(
      NativePortType port_, List<dynamic> that, int key, int max);

  external void wire_ConcatenateWith_handle_some_stream_sink_at_1(
      NativePortType port_, List<dynamic> that);

  external void wire_ConcatenateWith_new(NativePortType port_, String a);

  external void wire_SumWith_sum(
      NativePortType port_, List<dynamic> that, int y, int z);

  external void wire_get_sum_array(NativePortType port_, int a, int b, int c);

  external void wire_get_sum_struct(NativePortType port_);

  external void wire_app_settings_stream(NativePortType port_);

  external void wire_app_settings_vec_stream(NativePortType port_);

  external void wire_first_number(NativePortType port_, List<dynamic> nums);

  external void wire_first_sequence(NativePortType port_, List<dynamic> seqs);

  external void wire_get_app_settings(NativePortType port_);

  external void wire_get_fallible_app_settings(NativePortType port_);

  external void wire_get_message(NativePortType port_);

  external void wire_is_app_embedded(
      NativePortType port_, List<dynamic> app_settings);

  external void wire_mirror_struct_stream(NativePortType port_);

  external void wire_mirror_tuple_stream(NativePortType port_);

  external void wire_repeat_number(NativePortType port_, int num, int times);

  external void wire_repeat_sequence(NativePortType port_, int seq, int times);

  external void wire_test_contains_mirrored_sub_struct(NativePortType port_);

  external void wire_test_fallible_of_raw_string_mirrored(NativePortType port_);

  external void wire_test_list_of_nested_enums_mirrored(NativePortType port_);

  external void wire_test_list_of_raw_nested_string_mirrored(
      NativePortType port_);

  external void wire_test_nested_raw_string_mirrored(NativePortType port_);

  external void wire_test_raw_string_enum_mirrored(
      NativePortType port_, bool nested);

  external void wire_test_raw_string_mirrored(NativePortType port_);

  external void wire_handle_big_buffers(NativePortType port_);

  external void wire_handle_complex_struct(
      NativePortType port_, List<dynamic> s);

  external void wire_handle_nested_struct(
      NativePortType port_, List<dynamic> s);

  external void wire_handle_string(NativePortType port_, String s);

  external void wire_handle_struct(
      NativePortType port_, List<dynamic> arg, List<dynamic> boxed);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_handle_struct_sync_freezed(List<dynamic> arg, List<dynamic> boxed);

  external void wire_handle_vec_u8(NativePortType port_, Uint8List v);

  external void wire_list_of_primitive_enums(
      NativePortType port_, List<dynamic> weekdays);

  external void wire_test_abc_enum(NativePortType port_, List<dynamic> abc);

  external void wire_test_struct_with_enum(
      NativePortType port_, List<dynamic> se);

  external void wire_empty_struct(NativePortType port_, List<dynamic> empty);

  external void wire_func_return_unit_twin_normal(NativePortType port_);

  external void wire_func_string_twin_normal(NativePortType port_, String arg);

  external void wire_handle_list_of_struct(
      NativePortType port_, List<dynamic> l);

  external void wire_handle_string_list(
      NativePortType port_, List<String> names);

  external void wire_handle_newtype(NativePortType port_, List<dynamic> arg);

  external void wire_handle_increment_boxed_optional(
      NativePortType port_, double? opt);

  external void wire_handle_option_box_arguments(
      NativePortType port_,
      int? i8box,
      int? u8box,
      int? i32box,
      Object? i64box,
      double? f64box,
      bool? boolbox,
      List<dynamic>? structbox);

  external void wire_handle_optional_increment(
      NativePortType port_, List<dynamic>? opt);

  external void wire_handle_optional_return(
      NativePortType port_, double left, double right);

  external void wire_handle_optional_struct(
      NativePortType port_, String? document);

  external void wire_handle_vec_of_opts(
      NativePortType port_, List<dynamic> opt);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_null();

  external void wire_primitive_optional_types(NativePortType port_, int? my_i32,
      Object? my_i64, double? my_f64, bool? my_bool);

  external void wire_handle_vec_of_primitive(NativePortType port_, int n);

  external void wire_handle_zero_copy_vec_of_primitive(
      NativePortType port_, int n);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_handle_zero_copy_vec_of_primitive_sync(int n);

  external void wire_primitive_types(NativePortType port_, int my_i32,
      Object my_i64, double my_f64, bool my_bool);

  external void wire_primitive_u32(NativePortType port_, int my_u32);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_StructWithCommentsTwinSync_instance_method_twin_sync(
          List<dynamic> that);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_StructWithCommentsTwinSync_static_method_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_slash_star_star_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_triple_slash_multi_line_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_function_with_comments_triple_slash_single_line_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_simple_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_mixed_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_struct_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_enum_with_item_tuple_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_panic_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_return_error_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_enum_error_return_ok_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_nested_error_return_error_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_custom_struct_error_return_error_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_return_error_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_type_fallible_panic_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_type_infallible_panic_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_return_unit_twin_sync();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_string_twin_sync(String arg);

  external void wire_example_optional_primitive_type_bool_twin_normal(
      NativePortType port_, bool? arg);

  external void wire_example_optional_primitive_type_f32_twin_normal(
      NativePortType port_, double? arg);

  external void wire_example_optional_primitive_type_f64_twin_normal(
      NativePortType port_, double? arg);

  external void wire_example_optional_primitive_type_i16_twin_normal(
      NativePortType port_, int? arg);

  external void wire_example_optional_primitive_type_i32_twin_normal(
      NativePortType port_, int? arg);

  external void wire_example_optional_primitive_type_i64_twin_normal(
      NativePortType port_, Object? arg);

  external void wire_example_optional_primitive_type_i8_twin_normal(
      NativePortType port_, int? arg);

  external void wire_example_optional_primitive_type_u16_twin_normal(
      NativePortType port_, int? arg);

  external void wire_example_optional_primitive_type_u32_twin_normal(
      NativePortType port_, int? arg);

  external void wire_example_optional_primitive_type_u64_twin_normal(
      NativePortType port_, Object? arg);

  external void wire_example_optional_primitive_type_u8_twin_normal(
      NativePortType port_, int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_bool_twin_sync(bool? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_f32_twin_sync(double? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_f64_twin_sync(double? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i16_twin_sync(int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i32_twin_sync(int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i64_twin_sync(Object? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_i8_twin_sync(int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u16_twin_sync(int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u32_twin_sync(int? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u64_twin_sync(Object? arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_optional_primitive_type_u8_twin_sync(int? arg);

  external void wire_example_primitive_type_bool_twin_normal(
      NativePortType port_, bool arg);

  external void wire_example_primitive_type_f32_twin_normal(
      NativePortType port_, double arg);

  external void wire_example_primitive_type_f64_twin_normal(
      NativePortType port_, double arg);

  external void wire_example_primitive_type_i16_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_type_i32_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_type_i64_twin_normal(
      NativePortType port_, Object arg);

  external void wire_example_primitive_type_i8_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_type_u16_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_type_u32_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_type_u64_twin_normal(
      NativePortType port_, Object arg);

  external void wire_example_primitive_type_u8_twin_normal(
      NativePortType port_, int arg);

  external void wire_example_primitive_list_type_bool_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_example_primitive_list_type_f32_twin_normal(
      NativePortType port_, Float32List arg);

  external void wire_example_primitive_list_type_f64_twin_normal(
      NativePortType port_, Float64List arg);

  external void wire_example_primitive_list_type_i16_twin_normal(
      NativePortType port_, Int16List arg);

  external void wire_example_primitive_list_type_i32_twin_normal(
      NativePortType port_, Int32List arg);

  external void wire_example_primitive_list_type_i64_twin_normal(
      NativePortType port_, Object /* BigInt64Array */ arg);

  external void wire_example_primitive_list_type_i8_twin_normal(
      NativePortType port_, Int8List arg);

  external void wire_example_primitive_list_type_u16_twin_normal(
      NativePortType port_, Uint16List arg);

  external void wire_example_primitive_list_type_u32_twin_normal(
      NativePortType port_, Uint32List arg);

  external void wire_example_primitive_list_type_u64_twin_normal(
      NativePortType port_, Object /* BigInt64Array */ arg);

  external void wire_example_primitive_list_type_u8_twin_normal(
      NativePortType port_, Uint8List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_bool_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_f32_twin_sync(Float32List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_f64_twin_sync(Float64List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i16_twin_sync(Int16List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i32_twin_sync(Int32List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i64_twin_sync(
          Object /* BigInt64Array */ arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_i8_twin_sync(Int8List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u16_twin_sync(Uint16List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u32_twin_sync(Uint32List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u64_twin_sync(
          Object /* BigInt64Array */ arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_list_type_u8_twin_sync(Uint8List arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_bool_twin_sync(bool arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_f32_twin_sync(double arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_f64_twin_sync(double arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i16_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i32_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i64_twin_sync(Object arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_i8_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u16_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u32_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u64_twin_sync(Object arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_example_primitive_type_u8_twin_sync(int arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_simple_adder_twin_sync(int a, int b);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_one_field_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_two_field_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_struct_with_zero_field_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_tuple_struct_with_one_field_twin_sync(List<dynamic> arg);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_func_tuple_struct_with_two_field_twin_sync(List<dynamic> arg);

  external void wire_test_more_than_just_one_raw_string_struct(
      NativePortType port_);

  external void wire_test_raw_string_item_struct(NativePortType port_);

  external void wire_create_array_opaque_enum(NativePortType port_);

  external void wire_create_nested_opaque(NativePortType port_);

  external void wire_create_opaque(NativePortType port_);

  external void wire_create_option_opaque(NativePortType port_, Object? opaque);

  external void wire_create_sync_opaque(NativePortType port_);

  external void wire_frb_generator_test(NativePortType port_);

  external void wire_opaque_array(NativePortType port_);

  external void wire_opaque_array_run(NativePortType port_, List<dynamic> data);

  external void wire_opaque_vec(NativePortType port_);

  external void wire_opaque_vec_run(NativePortType port_, List<dynamic> data);

  external void wire_run_enum_opaque(
      NativePortType port_, List<dynamic> opaque);

  external void wire_run_nested_opaque(
      NativePortType port_, List<dynamic> opaque);

  external void wire_run_non_clone(NativePortType port_, Object clone);

  external void wire_run_opaque(NativePortType port_, Object opaque);

  external void wire_run_opaque_with_delay(NativePortType port_, Object opaque);

  external void wire_unwrap_rust_opaque(NativePortType port_, Object opaque);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_frb_sync_generator_test();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_non_clone();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_opaque();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_create_sync_opaque();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_option_rust_opaque();

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_sync_run_opaque(Object opaque);

  external void wire_simple_adder_twin_normal(
      NativePortType port_, int a, int b);

  external void wire_func_stream_realistic_twin_normal(
      NativePortType port_, String arg);

  external void wire_func_stream_return_error_twin_normal(NativePortType port_);

  external void wire_func_stream_return_panic_twin_normal(NativePortType port_);

  external void wire_func_stream_sink_arg_position_twin_normal(
      NativePortType port_, int a, int b);

  external void wire_handle_stream_of_struct(NativePortType port_);

  external void wire_handle_stream_sink_at_1(
      NativePortType port_, int key, int max);

  external void wire_handle_stream_sink_at_2(
      NativePortType port_, int key, int max);

  external void wire_handle_stream_sink_at_3(
      NativePortType port_, int key, int max);

  external void wire_func_struct_with_one_field_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_struct_with_two_field_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_struct_with_zero_field_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_tuple_struct_with_one_field_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_func_tuple_struct_with_two_field_twin_normal(
      NativePortType port_, List<dynamic> arg);

  external void wire_test_tuple(NativePortType port_, List<dynamic>? value);

  external void wire_test_tuple_2(NativePortType port_, List<dynamic> value);

  external void wire_handle_type_alias_id(NativePortType port_, Object input);

  external void wire_handle_type_alias_model(
      NativePortType port_, Object input);

  external void wire_handle_type_nest_alias_id(
      NativePortType port_, Object input);

  external void wire_handle_nested_uuids(
      NativePortType port_, List<dynamic> ids);

  external void wire_handle_uuid(NativePortType port_, Uint8List id);

  external void wire_handle_uuids(NativePortType port_, Uint8List ids);

  external void drop_opaque_RustOpaque_MutexHideData(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_MutexHideData(dynamic ptr);

  external void drop_opaque_RustOpaque_RwLockHideData(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_RwLockHideData(dynamic ptr);

  external void drop_opaque_RustOpaque_box_dynDartDebug(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_box_dynDartDebug(dynamic ptr);

  external void drop_opaque_RustOpaque_frb_opaque_return(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_frb_opaque_return(dynamic ptr);

  external void drop_opaque_RustOpaque_frb_opaque_sync_return(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_frb_opaque_sync_return(dynamic ptr);

  external void drop_opaque_RustOpaque_hide_data(dynamic ptr);

  external int /* *const std::ffi::c_void */ share_opaque_RustOpaque_hide_data(
      dynamic ptr);

  external void drop_opaque_RustOpaque_i_32(dynamic ptr);

  external int /* *const std::ffi::c_void */ share_opaque_RustOpaque_i_32(
      dynamic ptr);

  external void drop_opaque_RustOpaque_non_clone_data(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_non_clone_data(dynamic ptr);

  external void drop_opaque_RustOpaque_non_send_hide_data(dynamic ptr);

  external int /* *const std::ffi::c_void */
      share_opaque_RustOpaque_non_send_hide_data(dynamic ptr);
}
