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
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:meta/meta.dart' as meta;
import 'package:uuid/uuid.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  late final OpaqueTypeFinalizer mutexHideDataFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_MutexHideDataPtr);
  late final OpaqueTypeFinalizer rwLockHideDataFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_RwLockHideDataPtr);
  late final OpaqueTypeFinalizer boxDartDebugFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_BoxDartDebugPtr);
  late final OpaqueTypeFinalizer frbOpaqueReturnFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_FrbOpaqueReturnPtr);
  late final OpaqueTypeFinalizer frbOpaqueSyncReturnFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_FrbOpaqueSyncReturnPtr);
  late final OpaqueTypeFinalizer hideDataFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_HideDataPtr);
  late final OpaqueTypeFinalizer i32Finalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_I32Ptr);
  late final OpaqueTypeFinalizer nonCloneDataFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_NonCloneDataPtr);
  late final OpaqueTypeFinalizer nonSendHideDataFinalizer =
      OpaqueTypeFinalizer(wire._drop_opaque_NonSendHideDataPtr);
  @protected
  int api2wire_Chrono_Duration(Duration raw) {
    return api2wire_i_64(BigInt.from(raw.inMicroseconds));
  }

  @protected
  ffi.Pointer<wire_list_prim_i_64> api2wire_Chrono_DurationList(
      List<Duration> raw) {
    final ans = Int64List(raw.length);
    for (var i = 0; i < raw.length; ++i)
      ans[i] = api2wire_Chrono_Duration(raw[i]);
    return api2wire_list_prim_i_64(ans);
  }

  @protected
  int api2wire_Chrono_Local(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.microsecondsSinceEpoch));
  }

  @protected
  int api2wire_Chrono_Naive(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.microsecondsSinceEpoch));
  }

  @protected
  ffi.Pointer<wire_list_prim_i_64> api2wire_Chrono_NaiveList(
      List<DateTime> raw) {
    final ans = Int64List(raw.length);
    for (var i = 0; i < raw.length; ++i) ans[i] = api2wire_Chrono_Naive(raw[i]);
    return api2wire_list_prim_i_64(ans);
  }

  @protected
  int api2wire_Chrono_Utc(DateTime raw) {
    return api2wire_i_64(BigInt.from(raw.microsecondsSinceEpoch));
  }

  @protected
  wire_DartOpaque api2wire_DartOpaque(Object raw) {
    final ptr = wire.new_DartOpaque();
    _api_fill_to_wire_DartOpaque(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_DartOpaque> api2wire_DartOpaque_array_1(
      ObjectArray1 raw) {
    return api2wire_list_DartOpaque(raw);
  }

  @protected
  wire_RustOpaque_MutexHideData api2wire_RustOpaque_MutexHideData(
      MutexHideData raw) {
    final ptr = wire.new_RustOpaque_MutexHideData();
    _api_fill_to_wire_RustOpaque_MutexHideData(raw, ptr);
    return ptr;
  }

  @protected
  wire_RustOpaque_RwLockHideData api2wire_RustOpaque_RwLockHideData(
      RwLockHideData raw) {
    final ptr = wire.new_RustOpaque_RwLockHideData();
    _api_fill_to_wire_RustOpaque_RwLockHideData(raw, ptr);
    return ptr;
  }

  @protected
  wire_RustOpaque_box_dynDartDebug api2wire_RustOpaque_box_dynDartDebug(
      BoxDartDebug raw) {
    final ptr = wire.new_RustOpaque_box_dynDartDebug();
    _api_fill_to_wire_RustOpaque_box_dynDartDebug(raw, ptr);
    return ptr;
  }

  @protected
  wire_RustOpaque_hide_data api2wire_RustOpaque_hide_data(HideData raw) {
    final ptr = wire.new_RustOpaque_hide_data();
    _api_fill_to_wire_RustOpaque_hide_data(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_RustOpaque_hide_data>
      api2wire_RustOpaque_hide_data_array_2(HideDataArray2 raw) {
    return api2wire_list_RustOpaque_hide_data(raw);
  }

  @protected
  wire_RustOpaque_i_32 api2wire_RustOpaque_i_32(I32 raw) {
    final ptr = wire.new_RustOpaque_i_32();
    _api_fill_to_wire_RustOpaque_i_32(raw, ptr);
    return ptr;
  }

  @protected
  wire_RustOpaque_non_clone_data api2wire_RustOpaque_non_clone_data(
      NonCloneData raw) {
    final ptr = wire.new_RustOpaque_non_clone_data();
    _api_fill_to_wire_RustOpaque_non_clone_data(raw, ptr);
    return ptr;
  }

  @protected
  wire_RustOpaque_non_send_hide_data api2wire_RustOpaque_non_send_hide_data(
      NonSendHideData raw) {
    final ptr = wire.new_RustOpaque_non_send_hide_data();
    _api_fill_to_wire_RustOpaque_non_send_hide_data(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_String(String raw) {
    return api2wire_list_prim_u_8(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_StringList> api2wire_StringList(List<String> raw) {
    final ans = wire.new_StringList(raw.length);
    for (var i = 0; i < raw.length; i++) {
      ans.ref.ptr[i] = api2wire_String(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_Uuid(UuidValue raw) {
    return api2wire_list_prim_u_8(raw.toBytes());
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_Uuids(List<UuidValue> raw) {
    return api2wire_list_prim_u_8(api2wireConcatenateBytes(raw));
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_ZeroCopyBuffer_list_prim_u_8(
      Uint8List raw) {
    return api2wire_list_prim_u_8(raw);
  }

  @protected
  ffi.Pointer<wire_application_env> api2wire_box_application_env(
      ApplicationEnv raw) {
    final ptr = wire.new_box_application_env();
    _api_fill_to_wire_application_env(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_box_autoadd_Chrono_Utc(DateTime raw) {
    return wire.new_box_autoadd_Chrono_Utc(api2wire_Chrono_Utc(raw));
  }

  @protected
  ffi.Pointer<wire_DartOpaque> api2wire_box_autoadd_DartOpaque(Object raw) {
    final ptr = wire.new_box_autoadd_DartOpaque();
    _api_fill_to_wire_DartOpaque(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_RustOpaque_hide_data>
      api2wire_box_autoadd_RustOpaque_hide_data(HideData raw) {
    final ptr = wire.new_box_autoadd_RustOpaque_hide_data();
    _api_fill_to_wire_RustOpaque_hide_data(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_a> api2wire_box_autoadd_a(A raw) {
    final ptr = wire.new_box_autoadd_a();
    _api_fill_to_wire_a(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_abc> api2wire_box_autoadd_abc(Abc raw) {
    final ptr = wire.new_box_autoadd_abc();
    _api_fill_to_wire_abc(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_application_env> api2wire_box_autoadd_application_env(
      ApplicationEnv raw) {
    final ptr = wire.new_box_autoadd_application_env();
    _api_fill_to_wire_application_env(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_application_settings>
      api2wire_box_autoadd_application_settings(ApplicationSettings raw) {
    final ptr = wire.new_box_autoadd_application_settings();
    _api_fill_to_wire_application_settings(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_attribute> api2wire_box_autoadd_attribute(Attribute raw) {
    final ptr = wire.new_box_autoadd_attribute();
    _api_fill_to_wire_attribute(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_b> api2wire_box_autoadd_b(B raw) {
    final ptr = wire.new_box_autoadd_b();
    _api_fill_to_wire_b(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Bool> api2wire_box_autoadd_bool(bool raw) {
    return wire.new_box_autoadd_bool(api2wire_bool(raw));
  }

  @protected
  ffi.Pointer<wire_c> api2wire_box_autoadd_c(C raw) {
    final ptr = wire.new_box_autoadd_c();
    _api_fill_to_wire_c(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_concatenate_with> api2wire_box_autoadd_concatenate_with(
      ConcatenateWith raw) {
    final ptr = wire.new_box_autoadd_concatenate_with();
    _api_fill_to_wire_concatenate_with(raw, ptr.ref);
    return ptr;
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
  ffi.Pointer<wire_custom_nested_error_inner_twin_sync>
      api2wire_box_autoadd_custom_nested_error_inner_twin_sync(
          CustomNestedErrorInnerTwinSync raw) {
    final ptr = wire.new_box_autoadd_custom_nested_error_inner_twin_sync();
    _api_fill_to_wire_custom_nested_error_inner_twin_sync(raw, ptr.ref);
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
  ffi.Pointer<wire_custom_nested_error_outer_twin_sync>
      api2wire_box_autoadd_custom_nested_error_outer_twin_sync(
          CustomNestedErrorOuterTwinSync raw) {
    final ptr = wire.new_box_autoadd_custom_nested_error_outer_twin_sync();
    _api_fill_to_wire_custom_nested_error_outer_twin_sync(raw, ptr.ref);
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
  ffi.Pointer<wire_custom_struct_error_twin_sync>
      api2wire_box_autoadd_custom_struct_error_twin_sync(
          CustomStructErrorTwinSync raw) {
    final ptr = wire.new_box_autoadd_custom_struct_error_twin_sync();
    _api_fill_to_wire_custom_struct_error_twin_sync(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_customized> api2wire_box_autoadd_customized(Customized raw) {
    final ptr = wire.new_box_autoadd_customized();
    _api_fill_to_wire_customized(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_dart_opaque_nested> api2wire_box_autoadd_dart_opaque_nested(
      DartOpaqueNested raw) {
    final ptr = wire.new_box_autoadd_dart_opaque_nested();
    _api_fill_to_wire_dart_opaque_nested(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_dart_opaque> api2wire_box_autoadd_enum_dart_opaque(
      EnumDartOpaque raw) {
    final ptr = wire.new_box_autoadd_enum_dart_opaque();
    _api_fill_to_wire_enum_dart_opaque(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_enum_opaque> api2wire_box_autoadd_enum_opaque(
      EnumOpaque raw) {
    final ptr = wire.new_box_autoadd_enum_opaque();
    _api_fill_to_wire_enum_opaque(raw, ptr.ref);
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
  ffi.Pointer<wire_event> api2wire_box_autoadd_event(Event raw) {
    final ptr = wire.new_box_autoadd_event();
    _api_fill_to_wire_event(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_exotic_optionals> api2wire_box_autoadd_exotic_optionals(
      ExoticOptionals raw) {
    final ptr = wire.new_box_autoadd_exotic_optionals();
    _api_fill_to_wire_exotic_optionals(raw, ptr.ref);
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
  ffi.Pointer<wire_feature_chrono> api2wire_box_autoadd_feature_chrono(
      FeatureChrono raw) {
    final ptr = wire.new_box_autoadd_feature_chrono();
    _api_fill_to_wire_feature_chrono(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_feature_uuid> api2wire_box_autoadd_feature_uuid(
      FeatureUuid raw) {
    final ptr = wire.new_box_autoadd_feature_uuid();
    _api_fill_to_wire_feature_uuid(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_feed_id> api2wire_box_autoadd_feed_id(FeedId raw) {
    final ptr = wire.new_box_autoadd_feed_id();
    _api_fill_to_wire_feed_id(raw, ptr.ref);
    return ptr;
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
  ffi.Pointer<wire_measure> api2wire_box_autoadd_measure(Measure raw) {
    final ptr = wire.new_box_autoadd_measure();
    _api_fill_to_wire_measure(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_message_id> api2wire_box_autoadd_message_id(MessageId raw) {
    final ptr = wire.new_box_autoadd_message_id();
    _api_fill_to_wire_message_id(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_my_nested_struct> api2wire_box_autoadd_my_nested_struct(
      MyNestedStruct raw) {
    final ptr = wire.new_box_autoadd_my_nested_struct();
    _api_fill_to_wire_my_nested_struct(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_my_struct> api2wire_box_autoadd_my_struct(MyStruct raw) {
    final ptr = wire.new_box_autoadd_my_struct();
    _api_fill_to_wire_my_struct(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_my_tree_node> api2wire_box_autoadd_my_tree_node(
      MyTreeNode raw) {
    final ptr = wire.new_box_autoadd_my_tree_node();
    _api_fill_to_wire_my_tree_node(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_new_type_int> api2wire_box_autoadd_new_type_int(
      NewTypeInt raw) {
    final ptr = wire.new_box_autoadd_new_type_int();
    _api_fill_to_wire_new_type_int(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_note> api2wire_box_autoadd_note(Note raw) {
    final ptr = wire.new_box_autoadd_note();
    _api_fill_to_wire_note(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_numbers> api2wire_box_autoadd_numbers(Numbers raw) {
    final ptr = wire.new_box_autoadd_numbers();
    _api_fill_to_wire_numbers(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_opaque_nested> api2wire_box_autoadd_opaque_nested(
      OpaqueNested raw) {
    final ptr = wire.new_box_autoadd_opaque_nested();
    _api_fill_to_wire_opaque_nested(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_opt_vecs> api2wire_box_autoadd_opt_vecs(OptVecs raw) {
    final ptr = wire.new_box_autoadd_opt_vecs();
    _api_fill_to_wire_opt_vecs(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_record_string_i_32> api2wire_box_autoadd_record_string_i_32(
      (String, int) raw) {
    final ptr = wire.new_box_autoadd_record_string_i_32();
    _api_fill_to_wire_record_string_i_32(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_sequences> api2wire_box_autoadd_sequences(Sequences raw) {
    final ptr = wire.new_box_autoadd_sequences();
    _api_fill_to_wire_sequences(raw, ptr.ref);
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
  ffi.Pointer<wire_struct_with_enum> api2wire_box_autoadd_struct_with_enum(
      StructWithEnum raw) {
    final ptr = wire.new_box_autoadd_struct_with_enum();
    _api_fill_to_wire_struct_with_enum(raw, ptr.ref);
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
  ffi.Pointer<wire_sum_with> api2wire_box_autoadd_sum_with(SumWith raw) {
    final ptr = wire.new_box_autoadd_sum_with();
    _api_fill_to_wire_sum_with(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_test_id> api2wire_box_autoadd_test_id(TestId raw) {
    final ptr = wire.new_box_autoadd_test_id();
    _api_fill_to_wire_test_id(raw, ptr.ref);
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
  ffi.Pointer<wire_user_id> api2wire_box_autoadd_user_id(UserId raw) {
    final ptr = wire.new_box_autoadd_user_id();
    _api_fill_to_wire_user_id(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_weekdays(Weekdays raw) {
    return wire.new_box_autoadd_weekdays(api2wire_weekdays(raw));
  }

  @protected
  ffi.Pointer<wire_blob> api2wire_box_blob(Blob raw) {
    final ptr = wire.new_box_blob();
    _api_fill_to_wire_blob(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Bool> api2wire_box_bool(bool raw) {
    return wire.new_box_bool(api2wire_bool(raw));
  }

  @protected
  ffi.Pointer<wire_distance> api2wire_box_distance(Distance raw) {
    final ptr = wire.new_box_distance();
    _api_fill_to_wire_distance(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_exotic_optionals> api2wire_box_exotic_optionals(
      ExoticOptionals raw) {
    final ptr = wire.new_box_exotic_optionals();
    _api_fill_to_wire_exotic_optionals(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_box_f_64(double raw) {
    return wire.new_box_f_64(api2wire_f_64(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_i_32(int raw) {
    return wire.new_box_i_32(api2wire_i_32(raw));
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_box_i_64(BigInt raw) {
    return wire.new_box_i_64(api2wire_i_64(raw));
  }

  @protected
  ffi.Pointer<ffi.Int8> api2wire_box_i_8(int raw) {
    return wire.new_box_i_8(api2wire_i_8(raw));
  }

  @protected
  ffi.Pointer<wire_speed> api2wire_box_speed(Speed raw) {
    final ptr = wire.new_box_speed();
    _api_fill_to_wire_speed(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_box_u_8(int raw) {
    return wire.new_box_u_8(api2wire_u_8(raw));
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_box_u_8_array_1600(U8Array1600 raw) {
    return api2wire_u_8_array_1600(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_weekdays(Weekdays raw) {
    return wire.new_box_weekdays(api2wire_weekdays(raw));
  }

  @protected
  ffi.Pointer<wire_list_prim_f_64> api2wire_f_64_array_16(F64Array16 raw) {
    final ans = wire.new_list_prim_f_64(16);
    ans.ref.ptr.asTypedList(16).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_i_32> api2wire_i_32_array_2(I32Array2 raw) {
    final ans = wire.new_list_prim_i_32(2);
    ans.ref.ptr.asTypedList(2).setAll(0, raw);
    return ans;
  }

  @protected
  int api2wire_i_64(BigInt raw) {
    return raw.toInt();
  }

  @protected
  ffi.Pointer<wire_list_DartOpaque> api2wire_list_DartOpaque(List<Object> raw) {
    final ans = wire.new_list_DartOpaque(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_DartOpaque(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_RustOpaque_hide_data>
      api2wire_list_RustOpaque_hide_data(List<HideData> raw) {
    final ans = wire.new_list_RustOpaque_hide_data(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_RustOpaque_hide_data(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_application_env_var> api2wire_list_application_env_var(
      List<ApplicationEnvVar> raw) {
    final ans = wire.new_list_application_env_var(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_application_env_var(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_attribute> api2wire_list_attribute(
      List<Attribute> raw) {
    final ans = wire.new_list_attribute(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_attribute(raw[i], ans.ref.ptr[i]);
    }
    return ans;
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
  ffi.Pointer<wire_list_my_size> api2wire_list_my_size(List<MySize> raw) {
    final ans = wire.new_list_my_size(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_my_size(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_my_tree_node> api2wire_list_my_tree_node(
      List<MyTreeNode> raw) {
    final ans = wire.new_list_my_tree_node(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_my_tree_node(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_opt_String> api2wire_list_opt_String(
      List<String?> raw) {
    final ans = wire.new_list_opt_String(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      final item = raw[i];
      if (item == null) continue;
      ans.ref.ptr[i] = api2wire_String(item);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_opt_box_autoadd_attribute>
      api2wire_list_opt_box_autoadd_attribute(List<Attribute?> raw) {
    final ans = wire.new_list_opt_box_autoadd_attribute(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      final item = raw[i];
      if (item == null) continue;
      ans.ref.ptr[i] = api2wire_box_autoadd_attribute(item);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_opt_box_autoadd_i_32>
      api2wire_list_opt_box_autoadd_i_32(List<int?> raw) {
    final ans = wire.new_list_opt_box_autoadd_i_32(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      final item = raw[i];
      if (item == null) continue;
      ans.ref.ptr[i] = api2wire_box_autoadd_i_32(item);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_opt_box_autoadd_weekdays>
      api2wire_list_opt_box_autoadd_weekdays(List<Weekdays?> raw) {
    final ans = wire.new_list_opt_box_autoadd_weekdays(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      final item = raw[i];
      if (item == null) continue;
      ans.ref.ptr[i] = api2wire_box_autoadd_weekdays(item);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_opt_list_prim_i_32> api2wire_list_opt_list_prim_i_32(
      List<Int32List?> raw) {
    final ans = wire.new_list_opt_list_prim_i_32(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      final item = raw[i];
      if (item == null) continue;
      ans.ref.ptr[i] = api2wire_list_prim_i_32(item);
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
  ffi.Pointer<wire_list_record_string_i_32> api2wire_list_record_string_i_32(
      List<(String, int)> raw) {
    final ans = wire.new_list_record_string_i_32(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_record_string_i_32(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_test_id> api2wire_list_test_id(List<TestId> raw) {
    final ans = wire.new_list_test_id(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_test_id(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_weekdays> api2wire_list_weekdays(List<Weekdays> raw) {
    final ans = wire.new_list_weekdays(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      ans.ref.ptr[i] = api2wire_weekdays(raw[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_opt_String(String? raw) {
    return raw == null ? ffi.nullptr : api2wire_String(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_opt_ZeroCopyBuffer_list_prim_u_8(
      Uint8List? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_ZeroCopyBuffer_list_prim_u_8(raw);
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_opt_box_autoadd_Chrono_Utc(DateTime? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_Chrono_Utc(raw);
  }

  @protected
  ffi.Pointer<wire_DartOpaque> api2wire_opt_box_autoadd_DartOpaque(
      Object? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_DartOpaque(raw);
  }

  @protected
  ffi.Pointer<wire_RustOpaque_hide_data>
      api2wire_opt_box_autoadd_RustOpaque_hide_data(HideData? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_box_autoadd_RustOpaque_hide_data(raw);
  }

  @protected
  ffi.Pointer<wire_application_env> api2wire_opt_box_autoadd_application_env(
      ApplicationEnv? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_box_autoadd_application_env(raw);
  }

  @protected
  ffi.Pointer<ffi.Bool> api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_bool(raw);
  }

  @protected
  ffi.Pointer<wire_exotic_optionals> api2wire_opt_box_autoadd_exotic_optionals(
      ExoticOptionals? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_box_autoadd_exotic_optionals(raw);
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
  ffi.Pointer<wire_new_type_int> api2wire_opt_box_autoadd_new_type_int(
      NewTypeInt? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_new_type_int(raw);
  }

  @protected
  ffi.Pointer<wire_record_string_i_32>
      api2wire_opt_box_autoadd_record_string_i_32((String, int)? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_box_autoadd_record_string_i_32(raw);
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
  ffi.Pointer<ffi.Bool> api2wire_opt_box_bool(bool? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_bool(raw);
  }

  @protected
  ffi.Pointer<wire_exotic_optionals> api2wire_opt_box_exotic_optionals(
      ExoticOptionals? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_exotic_optionals(raw);
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_opt_box_f_64(double? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_f_64(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_i_32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_i_32(raw);
  }

  @protected
  ffi.Pointer<ffi.Int64> api2wire_opt_box_i_64(BigInt? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_i_64(raw);
  }

  @protected
  ffi.Pointer<ffi.Int8> api2wire_opt_box_i_8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_i_8(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_opt_box_u_8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_u_8(raw);
  }

  @protected
  ffi.Pointer<wire_list_attribute> api2wire_opt_list_attribute(
      List<Attribute>? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_attribute(raw);
  }

  @protected
  ffi.Pointer<wire_list_opt_box_autoadd_attribute>
      api2wire_opt_list_opt_box_autoadd_attribute(List<Attribute?>? raw) {
    return raw == null
        ? ffi.nullptr
        : api2wire_list_opt_box_autoadd_attribute(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_f_32> api2wire_opt_list_prim_f_32(
      Float32List? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_prim_f_32(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_f_64> api2wire_opt_list_prim_f_64(
      Float64List? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_prim_f_64(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_i_32> api2wire_opt_list_prim_i_32(Int32List? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_prim_i_32(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_i_8> api2wire_opt_list_prim_i_8(Int8List? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_prim_i_8(raw);
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_opt_list_prim_u_8(Uint8List? raw) {
    return raw == null ? ffi.nullptr : api2wire_list_prim_u_8(raw);
  }

  @protected
  ffi.Pointer<wire_list_test_id> api2wire_test_id_array_4(TestIdArray4 raw) {
    return api2wire_list_test_id(raw);
  }

  @protected
  int api2wire_u_64(BigInt raw) {
    return raw.toInt();
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_u_8_array_1600(U8Array1600 raw) {
    final ans = wire.new_list_prim_u_8(1600);
    ans.ref.ptr.asTypedList(1600).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_u_8_array_32(U8Array32 raw) {
    final ans = wire.new_list_prim_u_8(32);
    ans.ref.ptr.asTypedList(32).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_prim_u_8> api2wire_u_8_array_8(U8Array8 raw) {
    final ans = wire.new_list_prim_u_8(8);
    ans.ref.ptr.asTypedList(8).setAll(0, raw);
    return ans;
  }

  void _api_fill_to_wire_DartOpaque(Object apiObj, wire_DartOpaque wireObj) {
    wireObj.handle = wire.new_dart_opaque(apiObj);
    wireObj.port = dropPort;
  }

  void _api_fill_to_wire_RustOpaque_MutexHideData(
      MutexHideData apiObj, wire_RustOpaque_MutexHideData wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_RwLockHideData(
      RwLockHideData apiObj, wire_RustOpaque_RwLockHideData wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_box_dynDartDebug(
      BoxDartDebug apiObj, wire_RustOpaque_box_dynDartDebug wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_hide_data(
      HideData apiObj, wire_RustOpaque_hide_data wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_i_32(
      I32 apiObj, wire_RustOpaque_i_32 wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_non_clone_data(
      NonCloneData apiObj, wire_RustOpaque_non_clone_data wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_RustOpaque_non_send_hide_data(
      NonSendHideData apiObj, wire_RustOpaque_non_send_hide_data wireObj) {
    wireObj.ptr = apiObj.shareOrMove();
  }

  void _api_fill_to_wire_a(A apiObj, wire_a wireObj) {
    wireObj.a = api2wire_String(apiObj.a);
  }

  void _api_fill_to_wire_abc(Abc apiObj, wire_abc wireObj) {
    if (apiObj is Abc_A) {
      var pre_field0 = api2wire_box_autoadd_a(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_Abc_A();
      wireObj.kind.ref.A.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is Abc_B) {
      var pre_field0 = api2wire_box_autoadd_b(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_Abc_B();
      wireObj.kind.ref.B.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is Abc_C) {
      var pre_field0 = api2wire_box_autoadd_c(apiObj.field0);
      wireObj.tag = 2;
      wireObj.kind = wire.inflate_Abc_C();
      wireObj.kind.ref.C.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is Abc_JustInt) {
      var pre_field0 = api2wire_i_32(apiObj.field0);
      wireObj.tag = 3;
      wireObj.kind = wire.inflate_Abc_JustInt();
      wireObj.kind.ref.JustInt.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_application_env(
      ApplicationEnv apiObj, wire_application_env wireObj) {
    wireObj.vars = api2wire_list_application_env_var(apiObj.vars);
  }

  void _api_fill_to_wire_application_env_var(
      ApplicationEnvVar apiObj, wire_application_env_var wireObj) {
    wireObj.field0 = api2wire_String(apiObj.field0);
    wireObj.field1 = api2wire_bool(apiObj.field1);
  }

  void _api_fill_to_wire_application_settings(
      ApplicationSettings apiObj, wire_application_settings wireObj) {
    wireObj.name = api2wire_String(apiObj.name);
    wireObj.version = api2wire_String(apiObj.version);
    wireObj.mode = api2wire_application_mode(apiObj.mode);
    wireObj.env = api2wire_box_application_env(apiObj.env);
    wireObj.env_optional =
        api2wire_opt_box_autoadd_application_env(apiObj.envOptional);
  }

  void _api_fill_to_wire_attribute(Attribute apiObj, wire_attribute wireObj) {
    wireObj.key = api2wire_String(apiObj.key);
    wireObj.value = api2wire_String(apiObj.value);
  }

  void _api_fill_to_wire_b(B apiObj, wire_b wireObj) {
    wireObj.b = api2wire_i_32(apiObj.b);
  }

  void _api_fill_to_wire_blob(Blob apiObj, wire_blob wireObj) {
    wireObj.field0 = api2wire_u_8_array_1600(apiObj.field0);
  }

  void _api_fill_to_wire_box_application_env(
      ApplicationEnv apiObj, ffi.Pointer<wire_application_env> wireObj) {
    _api_fill_to_wire_application_env(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_DartOpaque(
      Object apiObj, ffi.Pointer<wire_DartOpaque> wireObj) {
    _api_fill_to_wire_DartOpaque(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_RustOpaque_hide_data(
      HideData apiObj, ffi.Pointer<wire_RustOpaque_hide_data> wireObj) {
    _api_fill_to_wire_RustOpaque_hide_data(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_a(A apiObj, ffi.Pointer<wire_a> wireObj) {
    _api_fill_to_wire_a(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_abc(
      Abc apiObj, ffi.Pointer<wire_abc> wireObj) {
    _api_fill_to_wire_abc(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_application_env(
      ApplicationEnv apiObj, ffi.Pointer<wire_application_env> wireObj) {
    _api_fill_to_wire_application_env(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_application_settings(
      ApplicationSettings apiObj,
      ffi.Pointer<wire_application_settings> wireObj) {
    _api_fill_to_wire_application_settings(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_attribute(
      Attribute apiObj, ffi.Pointer<wire_attribute> wireObj) {
    _api_fill_to_wire_attribute(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_b(B apiObj, ffi.Pointer<wire_b> wireObj) {
    _api_fill_to_wire_b(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_c(C apiObj, ffi.Pointer<wire_c> wireObj) {
    _api_fill_to_wire_c(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_concatenate_with(
      ConcatenateWith apiObj, ffi.Pointer<wire_concatenate_with> wireObj) {
    _api_fill_to_wire_concatenate_with(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_nested_error_inner_twin_normal(
      CustomNestedErrorInnerTwinNormal apiObj,
      ffi.Pointer<wire_custom_nested_error_inner_twin_normal> wireObj) {
    _api_fill_to_wire_custom_nested_error_inner_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_nested_error_inner_twin_sync(
      CustomNestedErrorInnerTwinSync apiObj,
      ffi.Pointer<wire_custom_nested_error_inner_twin_sync> wireObj) {
    _api_fill_to_wire_custom_nested_error_inner_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_nested_error_outer_twin_normal(
      CustomNestedErrorOuterTwinNormal apiObj,
      ffi.Pointer<wire_custom_nested_error_outer_twin_normal> wireObj) {
    _api_fill_to_wire_custom_nested_error_outer_twin_normal(
        apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_nested_error_outer_twin_sync(
      CustomNestedErrorOuterTwinSync apiObj,
      ffi.Pointer<wire_custom_nested_error_outer_twin_sync> wireObj) {
    _api_fill_to_wire_custom_nested_error_outer_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal apiObj,
      ffi.Pointer<wire_custom_struct_error_twin_normal> wireObj) {
    _api_fill_to_wire_custom_struct_error_twin_normal(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_custom_struct_error_twin_sync(
      CustomStructErrorTwinSync apiObj,
      ffi.Pointer<wire_custom_struct_error_twin_sync> wireObj) {
    _api_fill_to_wire_custom_struct_error_twin_sync(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_customized(
      Customized apiObj, ffi.Pointer<wire_customized> wireObj) {
    _api_fill_to_wire_customized(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_dart_opaque_nested(
      DartOpaqueNested apiObj, ffi.Pointer<wire_dart_opaque_nested> wireObj) {
    _api_fill_to_wire_dart_opaque_nested(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_dart_opaque(
      EnumDartOpaque apiObj, ffi.Pointer<wire_enum_dart_opaque> wireObj) {
    _api_fill_to_wire_enum_dart_opaque(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_enum_opaque(
      EnumOpaque apiObj, ffi.Pointer<wire_enum_opaque> wireObj) {
    _api_fill_to_wire_enum_opaque(apiObj, wireObj.ref);
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

  void _api_fill_to_wire_box_autoadd_event(
      Event apiObj, ffi.Pointer<wire_event> wireObj) {
    _api_fill_to_wire_event(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_exotic_optionals(
      ExoticOptionals apiObj, ffi.Pointer<wire_exotic_optionals> wireObj) {
    _api_fill_to_wire_exotic_optionals(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_feature_chrono(
      FeatureChrono apiObj, ffi.Pointer<wire_feature_chrono> wireObj) {
    _api_fill_to_wire_feature_chrono(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_feature_uuid(
      FeatureUuid apiObj, ffi.Pointer<wire_feature_uuid> wireObj) {
    _api_fill_to_wire_feature_uuid(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_feed_id(
      FeedId apiObj, ffi.Pointer<wire_feed_id> wireObj) {
    _api_fill_to_wire_feed_id(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_macro_struct(
      MacroStruct apiObj, ffi.Pointer<wire_macro_struct> wireObj) {
    _api_fill_to_wire_macro_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_measure(
      Measure apiObj, ffi.Pointer<wire_measure> wireObj) {
    _api_fill_to_wire_measure(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_message_id(
      MessageId apiObj, ffi.Pointer<wire_message_id> wireObj) {
    _api_fill_to_wire_message_id(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_my_nested_struct(
      MyNestedStruct apiObj, ffi.Pointer<wire_my_nested_struct> wireObj) {
    _api_fill_to_wire_my_nested_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_my_struct(
      MyStruct apiObj, ffi.Pointer<wire_my_struct> wireObj) {
    _api_fill_to_wire_my_struct(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_my_tree_node(
      MyTreeNode apiObj, ffi.Pointer<wire_my_tree_node> wireObj) {
    _api_fill_to_wire_my_tree_node(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_new_type_int(
      NewTypeInt apiObj, ffi.Pointer<wire_new_type_int> wireObj) {
    _api_fill_to_wire_new_type_int(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_note(
      Note apiObj, ffi.Pointer<wire_note> wireObj) {
    _api_fill_to_wire_note(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_numbers(
      Numbers apiObj, ffi.Pointer<wire_numbers> wireObj) {
    _api_fill_to_wire_numbers(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_opaque_nested(
      OpaqueNested apiObj, ffi.Pointer<wire_opaque_nested> wireObj) {
    _api_fill_to_wire_opaque_nested(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_opt_vecs(
      OptVecs apiObj, ffi.Pointer<wire_opt_vecs> wireObj) {
    _api_fill_to_wire_opt_vecs(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_record_string_i_32(
      (String, int) apiObj, ffi.Pointer<wire_record_string_i_32> wireObj) {
    _api_fill_to_wire_record_string_i_32(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_sequences(
      Sequences apiObj, ffi.Pointer<wire_sequences> wireObj) {
    _api_fill_to_wire_sequences(apiObj, wireObj.ref);
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

  void _api_fill_to_wire_box_autoadd_struct_with_enum(
      StructWithEnum apiObj, ffi.Pointer<wire_struct_with_enum> wireObj) {
    _api_fill_to_wire_struct_with_enum(apiObj, wireObj.ref);
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

  void _api_fill_to_wire_box_autoadd_sum_with(
      SumWith apiObj, ffi.Pointer<wire_sum_with> wireObj) {
    _api_fill_to_wire_sum_with(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_test_id(
      TestId apiObj, ffi.Pointer<wire_test_id> wireObj) {
    _api_fill_to_wire_test_id(apiObj, wireObj.ref);
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

  void _api_fill_to_wire_box_autoadd_user_id(
      UserId apiObj, ffi.Pointer<wire_user_id> wireObj) {
    _api_fill_to_wire_user_id(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_blob(Blob apiObj, ffi.Pointer<wire_blob> wireObj) {
    _api_fill_to_wire_blob(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_distance(
      Distance apiObj, ffi.Pointer<wire_distance> wireObj) {
    _api_fill_to_wire_distance(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_exotic_optionals(
      ExoticOptionals apiObj, ffi.Pointer<wire_exotic_optionals> wireObj) {
    _api_fill_to_wire_exotic_optionals(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_speed(
      Speed apiObj, ffi.Pointer<wire_speed> wireObj) {
    _api_fill_to_wire_speed(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_u_8_array_1600(
      U8Array1600 apiObj, ffi.Pointer<wire_list_prim_u_8> wireObj) {
    wireObj = api2wire_u_8_array_1600(apiObj);
  }

  void _api_fill_to_wire_c(C apiObj, wire_c wireObj) {
    wireObj.c = api2wire_bool(apiObj.c);
  }

  void _api_fill_to_wire_concatenate_with(
      ConcatenateWith apiObj, wire_concatenate_with wireObj) {
    wireObj.a = api2wire_String(apiObj.a);
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

  void _api_fill_to_wire_custom_nested_error_inner_twin_sync(
      CustomNestedErrorInnerTwinSync apiObj,
      wire_custom_nested_error_inner_twin_sync wireObj) {
    if (apiObj is CustomNestedErrorInnerTwinSync_Three) {
      var pre_field0 = api2wire_String(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_CustomNestedErrorInnerTwinSync_Three();
      wireObj.kind.ref.Three.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is CustomNestedErrorInnerTwinSync_Four) {
      var pre_field0 = api2wire_u_32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_CustomNestedErrorInnerTwinSync_Four();
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

  void _api_fill_to_wire_custom_nested_error_outer_twin_sync(
      CustomNestedErrorOuterTwinSync apiObj,
      wire_custom_nested_error_outer_twin_sync wireObj) {
    if (apiObj is CustomNestedErrorOuterTwinSync_One) {
      var pre_field0 = api2wire_String(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_CustomNestedErrorOuterTwinSync_One();
      wireObj.kind.ref.One.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is CustomNestedErrorOuterTwinSync_Two) {
      var pre_field0 = api2wire_box_autoadd_custom_nested_error_inner_twin_sync(
          apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_CustomNestedErrorOuterTwinSync_Two();
      wireObj.kind.ref.Two.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_custom_struct_error_twin_normal(
      CustomStructErrorTwinNormal apiObj,
      wire_custom_struct_error_twin_normal wireObj) {
    wireObj.a = api2wire_String(apiObj.a);
  }

  void _api_fill_to_wire_custom_struct_error_twin_sync(
      CustomStructErrorTwinSync apiObj,
      wire_custom_struct_error_twin_sync wireObj) {
    wireObj.a = api2wire_String(apiObj.a);
  }

  void _api_fill_to_wire_customized(
      Customized apiObj, wire_customized wireObj) {
    wireObj.final_field = api2wire_String(apiObj.finalField);
    wireObj.non_final_field = api2wire_opt_String(apiObj.nonFinalField);
  }

  void _api_fill_to_wire_dart_opaque_nested(
      DartOpaqueNested apiObj, wire_dart_opaque_nested wireObj) {
    wireObj.first = api2wire_DartOpaque(apiObj.first);
    wireObj.second = api2wire_DartOpaque(apiObj.second);
  }

  void _api_fill_to_wire_distance(Distance apiObj, wire_distance wireObj) {
    if (apiObj is Distance_Unknown) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is Distance_Map) {
      var pre_field0 = api2wire_f_64(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_Distance_Map();
      wireObj.kind.ref.Map.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_enum_dart_opaque(
      EnumDartOpaque apiObj, wire_enum_dart_opaque wireObj) {
    if (apiObj is EnumDartOpaque_Primitive) {
      var pre_field0 = api2wire_i_32(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumDartOpaque_Primitive();
      wireObj.kind.ref.Primitive.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumDartOpaque_Opaque) {
      var pre_field0 = api2wire_DartOpaque(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumDartOpaque_Opaque();
      wireObj.kind.ref.Opaque.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_enum_opaque(
      EnumOpaque apiObj, wire_enum_opaque wireObj) {
    if (apiObj is EnumOpaque_Struct) {
      var pre_field0 = api2wire_RustOpaque_hide_data(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_EnumOpaque_Struct();
      wireObj.kind.ref.Struct.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumOpaque_Primitive) {
      var pre_field0 = api2wire_RustOpaque_i_32(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_EnumOpaque_Primitive();
      wireObj.kind.ref.Primitive.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumOpaque_TraitObj) {
      var pre_field0 = api2wire_RustOpaque_box_dynDartDebug(apiObj.field0);
      wireObj.tag = 2;
      wireObj.kind = wire.inflate_EnumOpaque_TraitObj();
      wireObj.kind.ref.TraitObj.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumOpaque_Mutex) {
      var pre_field0 = api2wire_RustOpaque_MutexHideData(apiObj.field0);
      wireObj.tag = 3;
      wireObj.kind = wire.inflate_EnumOpaque_Mutex();
      wireObj.kind.ref.Mutex.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is EnumOpaque_RwLock) {
      var pre_field0 = api2wire_RustOpaque_RwLockHideData(apiObj.field0);
      wireObj.tag = 4;
      wireObj.kind = wire.inflate_EnumOpaque_RwLock();
      wireObj.kind.ref.RwLock.ref.field0 = pre_field0;
      return;
    }
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

  void _api_fill_to_wire_event(Event apiObj, wire_event wireObj) {
    wireObj.address = api2wire_String(apiObj.address);
    wireObj.payload = api2wire_String(apiObj.payload);
  }

  void _api_fill_to_wire_exotic_optionals(
      ExoticOptionals apiObj, wire_exotic_optionals wireObj) {
    wireObj.int32 = api2wire_opt_box_autoadd_i_32(apiObj.int32);
    wireObj.int64 = api2wire_opt_box_autoadd_i_64(apiObj.int64);
    wireObj.float64 = api2wire_opt_box_autoadd_f_64(apiObj.float64);
    wireObj.boolean = api2wire_opt_box_autoadd_bool(apiObj.boolean);
    wireObj.zerocopy =
        api2wire_opt_ZeroCopyBuffer_list_prim_u_8(apiObj.zerocopy);
    wireObj.int8list = api2wire_opt_list_prim_i_8(apiObj.int8List);
    wireObj.uint8list = api2wire_opt_list_prim_u_8(apiObj.uint8List);
    wireObj.int32list = api2wire_opt_list_prim_i_32(apiObj.int32List);
    wireObj.float32list = api2wire_opt_list_prim_f_32(apiObj.float32List);
    wireObj.float64list = api2wire_opt_list_prim_f_64(apiObj.float64List);
    wireObj.attributes = api2wire_opt_list_attribute(apiObj.attributes);
    wireObj.attributes_nullable =
        api2wire_list_opt_box_autoadd_attribute(apiObj.attributesNullable);
    wireObj.nullable_attributes =
        api2wire_opt_list_opt_box_autoadd_attribute(apiObj.nullableAttributes);
    wireObj.newtypeint =
        api2wire_opt_box_autoadd_new_type_int(apiObj.newtypeint);
  }

  void _api_fill_to_wire_feature_chrono(
      FeatureChrono apiObj, wire_feature_chrono wireObj) {
    wireObj.utc = api2wire_Chrono_Utc(apiObj.utc);
    wireObj.local = api2wire_Chrono_Local(apiObj.local);
    wireObj.duration = api2wire_Chrono_Duration(apiObj.duration);
    wireObj.naive = api2wire_Chrono_Naive(apiObj.naive);
  }

  void _api_fill_to_wire_feature_uuid(
      FeatureUuid apiObj, wire_feature_uuid wireObj) {
    wireObj.one = api2wire_Uuid(apiObj.one);
    wireObj.many = api2wire_Uuids(apiObj.many);
  }

  void _api_fill_to_wire_feed_id(FeedId apiObj, wire_feed_id wireObj) {
    wireObj.field0 = api2wire_u_8_array_8(apiObj.field0);
  }

  void _api_fill_to_wire_macro_struct(
      MacroStruct apiObj, wire_macro_struct wireObj) {
    wireObj.data = api2wire_i_32(apiObj.data);
  }

  void _api_fill_to_wire_measure(Measure apiObj, wire_measure wireObj) {
    if (apiObj is Measure_Speed) {
      var pre_field0 = api2wire_box_speed(apiObj.field0);
      wireObj.tag = 0;
      wireObj.kind = wire.inflate_Measure_Speed();
      wireObj.kind.ref.Speed.ref.field0 = pre_field0;
      return;
    }
    if (apiObj is Measure_Distance) {
      var pre_field0 = api2wire_box_distance(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_Measure_Distance();
      wireObj.kind.ref.Distance.ref.field0 = pre_field0;
      return;
    }
  }

  void _api_fill_to_wire_message_id(MessageId apiObj, wire_message_id wireObj) {
    wireObj.field0 = api2wire_u_8_array_32(apiObj.field0);
  }

  void _api_fill_to_wire_my_nested_struct(
      MyNestedStruct apiObj, wire_my_nested_struct wireObj) {
    _api_fill_to_wire_my_tree_node(apiObj.treeNode, wireObj.tree_node);
    wireObj.weekday = api2wire_weekdays(apiObj.weekday);
  }

  void _api_fill_to_wire_my_size(MySize apiObj, wire_my_size wireObj) {
    wireObj.width = api2wire_i_32(apiObj.width);
    wireObj.height = api2wire_i_32(apiObj.height);
  }

  void _api_fill_to_wire_my_struct(MyStruct apiObj, wire_my_struct wireObj) {
    wireObj.content = api2wire_bool(apiObj.content);
  }

  void _api_fill_to_wire_my_tree_node(
      MyTreeNode apiObj, wire_my_tree_node wireObj) {
    wireObj.value_i32 = api2wire_i_32(apiObj.valueI32);
    wireObj.value_vec_u8 = api2wire_list_prim_u_8(apiObj.valueVecU8);
    wireObj.value_boolean = api2wire_bool(apiObj.valueBoolean);
    wireObj.children = api2wire_list_my_tree_node(apiObj.children);
  }

  void _api_fill_to_wire_new_type_int(
      NewTypeInt apiObj, wire_new_type_int wireObj) {
    wireObj.field0 = api2wire_i_64(apiObj.field0);
  }

  void _api_fill_to_wire_note(Note apiObj, wire_note wireObj) {
    wireObj.day = api2wire_box_weekdays(apiObj.day);
    wireObj.body = api2wire_String(apiObj.body);
  }

  void _api_fill_to_wire_numbers(Numbers apiObj, wire_numbers wireObj) {
    wireObj.field0 = api2wire_list_prim_i_32(apiObj.field0);
  }

  void _api_fill_to_wire_opaque_nested(
      OpaqueNested apiObj, wire_opaque_nested wireObj) {
    wireObj.first = api2wire_RustOpaque_hide_data(apiObj.first);
    wireObj.second = api2wire_RustOpaque_hide_data(apiObj.second);
  }

  void _api_fill_to_wire_opt_vecs(OptVecs apiObj, wire_opt_vecs wireObj) {
    wireObj.i32 = api2wire_list_opt_box_autoadd_i_32(apiObj.i32);
    wireObj.enums = api2wire_list_opt_box_autoadd_weekdays(apiObj.enums);
    wireObj.strings = api2wire_list_opt_String(apiObj.strings);
    wireObj.buffers = api2wire_list_opt_list_prim_i_32(apiObj.buffers);
  }

  void _api_fill_to_wire_record_string_i_32(
      (String, int) apiObj, wire_record_string_i_32 wireObj) {
    wireObj.field0 = api2wire_String(apiObj.$1);
    wireObj.field1 = api2wire_i_32(apiObj.$2);
  }

  void _api_fill_to_wire_sequences(Sequences apiObj, wire_sequences wireObj) {
    wireObj.field0 = api2wire_list_prim_i_32(apiObj.field0);
  }

  void _api_fill_to_wire_speed(Speed apiObj, wire_speed wireObj) {
    if (apiObj is Speed_Unknown) {
      wireObj.tag = 0;
      return;
    }
    if (apiObj is Speed_GPS) {
      var pre_field0 = api2wire_f_64(apiObj.field0);
      wireObj.tag = 1;
      wireObj.kind = wire.inflate_Speed_GPS();
      wireObj.kind.ref.GPS.ref.field0 = pre_field0;
      return;
    }
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

  void _api_fill_to_wire_struct_with_enum(
      StructWithEnum apiObj, wire_struct_with_enum wireObj) {
    _api_fill_to_wire_abc(apiObj.abc1, wireObj.abc1);
    _api_fill_to_wire_abc(apiObj.abc2, wireObj.abc2);
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
  void _api_fill_to_wire_sum_with(SumWith apiObj, wire_sum_with wireObj) {
    wireObj.x = api2wire_u_32(apiObj.x);
  }

  void _api_fill_to_wire_test_id(TestId apiObj, wire_test_id wireObj) {
    wireObj.field0 = api2wire_i_32_array_2(apiObj.field0);
  }

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

  void _api_fill_to_wire_user_id(UserId apiObj, wire_user_id wireObj) {
    wireObj.value = api2wire_u_32(apiObj.value);
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

  void wire_boxed_blob(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> blob,
  ) {
    return _wire_boxed_blob(
      port_,
      blob,
    );
  }

  late final _wire_boxed_blobPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>('wire_boxed_blob');
  late final _wire_boxed_blob = _wire_boxed_blobPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_func_test_id(
    int port_,
    ffi.Pointer<wire_test_id> id,
  ) {
    return _wire_func_test_id(
      port_,
      id,
    );
  }

  late final _wire_func_test_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_test_id>)>>('wire_func_test_id');
  late final _wire_func_test_id = _wire_func_test_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_test_id>)>();

  void wire_get_array(
    int port_,
  ) {
    return _wire_get_array(
      port_,
    );
  }

  late final _wire_get_arrayPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_array');
  late final _wire_get_array =
      _wire_get_arrayPtr.asFunction<void Function(int)>();

  void wire_get_complex_array(
    int port_,
  ) {
    return _wire_get_complex_array(
      port_,
    );
  }

  late final _wire_get_complex_arrayPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_complex_array');
  late final _wire_get_complex_array =
      _wire_get_complex_arrayPtr.asFunction<void Function(int)>();

  void wire_last_number(
    int port_,
    ffi.Pointer<wire_list_prim_f_64> array,
  ) {
    return _wire_last_number(
      port_,
      array,
    );
  }

  late final _wire_last_numberPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_f_64>)>>('wire_last_number');
  late final _wire_last_number = _wire_last_numberPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_f_64>)>();

  void wire_nested_id(
    int port_,
    ffi.Pointer<wire_list_test_id> id,
  ) {
    return _wire_nested_id(
      port_,
      id,
    );
  }

  late final _wire_nested_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_list_test_id>)>>('wire_nested_id');
  late final _wire_nested_id = _wire_nested_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_test_id>)>();

  void wire_new_msgid(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> id,
  ) {
    return _wire_new_msgid(
      port_,
      id,
    );
  }

  late final _wire_new_msgidPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>('wire_new_msgid');
  late final _wire_new_msgid = _wire_new_msgidPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_return_boxed_feed_id(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> id,
  ) {
    return _wire_return_boxed_feed_id(
      port_,
      id,
    );
  }

  late final _wire_return_boxed_feed_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_return_boxed_feed_id');
  late final _wire_return_boxed_feed_id = _wire_return_boxed_feed_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_return_boxed_raw_feed_id(
    int port_,
    ffi.Pointer<wire_feed_id> id,
  ) {
    return _wire_return_boxed_raw_feed_id(
      port_,
      id,
    );
  }

  late final _wire_return_boxed_raw_feed_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_feed_id>)>>('wire_return_boxed_raw_feed_id');
  late final _wire_return_boxed_raw_feed_id = _wire_return_boxed_raw_feed_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_feed_id>)>();

  void wire_use_boxed_blob(
    int port_,
    ffi.Pointer<wire_blob> blob,
  ) {
    return _wire_use_boxed_blob(
      port_,
      blob,
    );
  }

  late final _wire_use_boxed_blobPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_blob>)>>('wire_use_boxed_blob');
  late final _wire_use_boxed_blob = _wire_use_boxed_blobPtr
      .asFunction<void Function(int, ffi.Pointer<wire_blob>)>();

  void wire_use_msgid(
    int port_,
    ffi.Pointer<wire_message_id> id,
  ) {
    return _wire_use_msgid(
      port_,
      id,
    );
  }

  late final _wire_use_msgidPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_message_id>)>>('wire_use_msgid');
  late final _wire_use_msgid = _wire_use_msgidPtr
      .asFunction<void Function(int, ffi.Pointer<wire_message_id>)>();

  void wire_handle_customized_struct(
    int port_,
    ffi.Pointer<wire_customized> val,
  ) {
    return _wire_handle_customized_struct(
      port_,
      val,
    );
  }

  late final _wire_handle_customized_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_customized>)>>('wire_handle_customized_struct');
  late final _wire_handle_customized_struct = _wire_handle_customized_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_customized>)>();

  void wire_next_user_id(
    int port_,
    ffi.Pointer<wire_user_id> user_id,
  ) {
    return _wire_next_user_id(
      port_,
      user_id,
    );
  }

  late final _wire_next_user_idPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_user_id>)>>('wire_next_user_id');
  late final _wire_next_user_id = _wire_next_user_idPtr
      .asFunction<void Function(int, ffi.Pointer<wire_user_id>)>();

  void wire_datetime_local(
    int port_,
    int d,
  ) {
    return _wire_datetime_local(
      port_,
      d,
    );
  }

  late final _wire_datetime_localPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int64)>>(
          'wire_datetime_local');
  late final _wire_datetime_local =
      _wire_datetime_localPtr.asFunction<void Function(int, int)>();

  void wire_datetime_utc(
    int port_,
    int d,
  ) {
    return _wire_datetime_utc(
      port_,
      d,
    );
  }

  late final _wire_datetime_utcPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int64)>>(
          'wire_datetime_utc');
  late final _wire_datetime_utc =
      _wire_datetime_utcPtr.asFunction<void Function(int, int)>();

  void wire_duration(
    int port_,
    int d,
  ) {
    return _wire_duration(
      port_,
      d,
    );
  }

  late final _wire_durationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int64)>>(
          'wire_duration');
  late final _wire_duration =
      _wire_durationPtr.asFunction<void Function(int, int)>();

  void wire_handle_durations(
    int port_,
    ffi.Pointer<wire_list_prim_i_64> durations,
    int since,
  ) {
    return _wire_handle_durations(
      port_,
      durations,
      since,
    );
  }

  late final _wire_handle_durationsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_64>,
              ffi.Int64)>>('wire_handle_durations');
  late final _wire_handle_durations = _wire_handle_durationsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_64>, int)>();

  void wire_handle_timestamps(
    int port_,
    ffi.Pointer<wire_list_prim_i_64> timestamps,
    int epoch,
  ) {
    return _wire_handle_timestamps(
      port_,
      timestamps,
      epoch,
    );
  }

  late final _wire_handle_timestampsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_i_64>,
              ffi.Int64)>>('wire_handle_timestamps');
  late final _wire_handle_timestamps = _wire_handle_timestampsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_i_64>, int)>();

  void wire_how_long_does_it_take(
    int port_,
    ffi.Pointer<wire_feature_chrono> mine,
  ) {
    return _wire_how_long_does_it_take(
      port_,
      mine,
    );
  }

  late final _wire_how_long_does_it_takePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_feature_chrono>)>>('wire_how_long_does_it_take');
  late final _wire_how_long_does_it_take = _wire_how_long_does_it_takePtr
      .asFunction<void Function(int, ffi.Pointer<wire_feature_chrono>)>();

  void wire_naivedatetime(
    int port_,
    int d,
  ) {
    return _wire_naivedatetime(
      port_,
      d,
    );
  }

  late final _wire_naivedatetimePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int64)>>(
          'wire_naivedatetime');
  late final _wire_naivedatetime =
      _wire_naivedatetimePtr.asFunction<void Function(int, int)>();

  void wire_optional_empty_datetime_utc(
    int port_,
    ffi.Pointer<ffi.Int64> d,
  ) {
    return _wire_optional_empty_datetime_utc(
      port_,
      d,
    );
  }

  late final _wire_optional_empty_datetime_utcPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<ffi.Int64>)>>('wire_optional_empty_datetime_utc');
  late final _wire_optional_empty_datetime_utc =
      _wire_optional_empty_datetime_utcPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Int64>)>();

  void wire_test_chrono(
    int port_,
  ) {
    return _wire_test_chrono(
      port_,
    );
  }

  late final _wire_test_chronoPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_chrono');
  late final _wire_test_chrono =
      _wire_test_chronoPtr.asFunction<void Function(int)>();

  void wire_test_precise_chrono(
    int port_,
  ) {
    return _wire_test_precise_chrono(
      port_,
    );
  }

  late final _wire_test_precise_chronoPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_precise_chrono');
  late final _wire_test_precise_chrono =
      _wire_test_precise_chronoPtr.asFunction<void Function(int)>();

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

  void wire_return_dart_dynamic(
    int port_,
  ) {
    return _wire_return_dart_dynamic(
      port_,
    );
  }

  late final _wire_return_dart_dynamicPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_return_dart_dynamic');
  late final _wire_return_dart_dynamic =
      _wire_return_dart_dynamicPtr.asFunction<void Function(int)>();

  void wire_async_accept_dart_opaque(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_async_accept_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_async_accept_dart_opaquePtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_async_accept_dart_opaque');
  late final _wire_async_accept_dart_opaque = _wire_async_accept_dart_opaquePtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  void wire_create_enum_dart_opaque(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_create_enum_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_create_enum_dart_opaquePtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_create_enum_dart_opaque');
  late final _wire_create_enum_dart_opaque = _wire_create_enum_dart_opaquePtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  void wire_create_nested_dart_opaque(
    int port_,
    wire_DartOpaque opaque1,
    wire_DartOpaque opaque2,
  ) {
    return _wire_create_nested_dart_opaque(
      port_,
      opaque1,
      opaque2,
    );
  }

  late final _wire_create_nested_dart_opaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, wire_DartOpaque,
              wire_DartOpaque)>>('wire_create_nested_dart_opaque');
  late final _wire_create_nested_dart_opaque =
      _wire_create_nested_dart_opaquePtr
          .asFunction<void Function(int, wire_DartOpaque, wire_DartOpaque)>();

  void wire_drop_static_dart_opaque(
    int port_,
  ) {
    return _wire_drop_static_dart_opaque(
      port_,
    );
  }

  late final _wire_drop_static_dart_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_drop_static_dart_opaque');
  late final _wire_drop_static_dart_opaque =
      _wire_drop_static_dart_opaquePtr.asFunction<void Function(int)>();

  void wire_get_enum_dart_opaque(
    int port_,
    ffi.Pointer<wire_enum_dart_opaque> opaque,
  ) {
    return _wire_get_enum_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_get_enum_dart_opaquePtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_enum_dart_opaque>)>>(
      'wire_get_enum_dart_opaque');
  late final _wire_get_enum_dart_opaque = _wire_get_enum_dart_opaquePtr
      .asFunction<void Function(int, ffi.Pointer<wire_enum_dart_opaque>)>();

  void wire_get_nested_dart_opaque(
    int port_,
    ffi.Pointer<wire_dart_opaque_nested> opaque,
  ) {
    return _wire_get_nested_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_get_nested_dart_opaquePtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_dart_opaque_nested>)>>(
      'wire_get_nested_dart_opaque');
  late final _wire_get_nested_dart_opaque = _wire_get_nested_dart_opaquePtr
      .asFunction<void Function(int, ffi.Pointer<wire_dart_opaque_nested>)>();

  void wire_loop_back(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_loop_back(
      port_,
      opaque,
    );
  }

  late final _wire_loop_backPtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_loop_back');
  late final _wire_loop_back =
      _wire_loop_backPtr.asFunction<void Function(int, wire_DartOpaque)>();

  void wire_loop_back_array(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_loop_back_array(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_arrayPtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_loop_back_array');
  late final _wire_loop_back_array = _wire_loop_back_arrayPtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  void wire_loop_back_array_get(
    int port_,
    ffi.Pointer<wire_list_DartOpaque> opaque,
  ) {
    return _wire_loop_back_array_get(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_array_getPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_DartOpaque>)>>('wire_loop_back_array_get');
  late final _wire_loop_back_array_get = _wire_loop_back_array_getPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_DartOpaque>)>();

  void wire_loop_back_option(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_loop_back_option(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_optionPtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_loop_back_option');
  late final _wire_loop_back_option = _wire_loop_back_optionPtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  void wire_loop_back_option_get(
    int port_,
    ffi.Pointer<wire_DartOpaque> opaque,
  ) {
    return _wire_loop_back_option_get(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_option_getPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_DartOpaque>)>>('wire_loop_back_option_get');
  late final _wire_loop_back_option_get = _wire_loop_back_option_getPtr
      .asFunction<void Function(int, ffi.Pointer<wire_DartOpaque>)>();

  void wire_loop_back_vec(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_loop_back_vec(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_vecPtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_loop_back_vec');
  late final _wire_loop_back_vec =
      _wire_loop_back_vecPtr.asFunction<void Function(int, wire_DartOpaque)>();

  void wire_loop_back_vec_get(
    int port_,
    ffi.Pointer<wire_list_DartOpaque> opaque,
  ) {
    return _wire_loop_back_vec_get(
      port_,
      opaque,
    );
  }

  late final _wire_loop_back_vec_getPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_DartOpaque>)>>('wire_loop_back_vec_get');
  late final _wire_loop_back_vec_get = _wire_loop_back_vec_getPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_DartOpaque>)>();

  void wire_panic_unwrap_dart_opaque(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_panic_unwrap_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_panic_unwrap_dart_opaquePtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_panic_unwrap_dart_opaque');
  late final _wire_panic_unwrap_dart_opaque = _wire_panic_unwrap_dart_opaquePtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  void wire_set_static_dart_opaque(
    int port_,
    wire_DartOpaque opaque,
  ) {
    return _wire_set_static_dart_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_set_static_dart_opaquePtr = _lookup<
          ffi.NativeFunction<ffi.Void Function(ffi.Int64, wire_DartOpaque)>>(
      'wire_set_static_dart_opaque');
  late final _wire_set_static_dart_opaque = _wire_set_static_dart_opaquePtr
      .asFunction<void Function(int, wire_DartOpaque)>();

  WireSyncReturn wire_return_non_droppable_dart_opaque(
    wire_DartOpaque opaque,
  ) {
    return _wire_return_non_droppable_dart_opaque(
      opaque,
    );
  }

  late final _wire_return_non_droppable_dart_opaquePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_DartOpaque)>>(
          'wire_return_non_droppable_dart_opaque');
  late final _wire_return_non_droppable_dart_opaque =
      _wire_return_non_droppable_dart_opaquePtr
          .asFunction<WireSyncReturn Function(wire_DartOpaque)>();

  WireSyncReturn wire_unwrap_dart_opaque(
    wire_DartOpaque opaque,
  ) {
    return _wire_unwrap_dart_opaque(
      opaque,
    );
  }

  late final _wire_unwrap_dart_opaquePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(wire_DartOpaque)>>(
          'wire_unwrap_dart_opaque');
  late final _wire_unwrap_dart_opaque = _wire_unwrap_dart_opaquePtr
      .asFunction<WireSyncReturn Function(wire_DartOpaque)>();

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

  void wire_handle_enum_parameter(
    int port_,
    int weekday,
  ) {
    return _wire_handle_enum_parameter(
      port_,
      weekday,
    );
  }

  late final _wire_handle_enum_parameterPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_handle_enum_parameter');
  late final _wire_handle_enum_parameter =
      _wire_handle_enum_parameterPtr.asFunction<void Function(int, int)>();

  void wire_handle_return_enum(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> input,
  ) {
    return _wire_handle_return_enum(
      port_,
      input,
    );
  }

  late final _wire_handle_return_enumPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_handle_return_enum');
  late final _wire_handle_return_enum = _wire_handle_return_enumPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_multiply_by_ten(
    int port_,
    ffi.Pointer<wire_measure> measure,
  ) {
    return _wire_multiply_by_ten(
      port_,
      measure,
    );
  }

  late final _wire_multiply_by_tenPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_measure>)>>('wire_multiply_by_ten');
  late final _wire_multiply_by_ten = _wire_multiply_by_tenPtr
      .asFunction<void Function(int, ffi.Pointer<wire_measure>)>();

  void wire_print_note(
    int port_,
    ffi.Pointer<wire_note> note,
  ) {
    return _wire_print_note(
      port_,
      note,
    );
  }

  late final _wire_print_notePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_note>)>>('wire_print_note');
  late final _wire_print_note = _wire_print_notePtr
      .asFunction<void Function(int, ffi.Pointer<wire_note>)>();

  void wire_Event_as_string(
    int port_,
    ffi.Pointer<wire_event> that,
  ) {
    return _wire_Event_as_string(
      port_,
      that,
    );
  }

  late final _wire_Event_as_stringPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_event>)>>('wire_Event_as_string');
  late final _wire_Event_as_string = _wire_Event_as_stringPtr
      .asFunction<void Function(int, ffi.Pointer<wire_event>)>();

  void wire_close_event_listener(
    int port_,
  ) {
    return _wire_close_event_listener(
      port_,
    );
  }

  late final _wire_close_event_listenerPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_close_event_listener');
  late final _wire_close_event_listener =
      _wire_close_event_listenerPtr.asFunction<void Function(int)>();

  void wire_create_event(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> address,
    ffi.Pointer<wire_list_prim_u_8> payload,
  ) {
    return _wire_create_event(
      port_,
      address,
      payload,
    );
  }

  late final _wire_create_eventPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_8>,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_create_event');
  late final _wire_create_event = _wire_create_eventPtr.asFunction<
      void Function(int, ffi.Pointer<wire_list_prim_u_8>,
          ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_register_event_listener(
    int port_,
  ) {
    return _wire_register_event_listener(
      port_,
    );
  }

  late final _wire_register_event_listenerPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_register_event_listener');
  late final _wire_register_event_listener =
      _wire_register_event_listenerPtr.asFunction<void Function(int)>();

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

  void wire_call_new_module_system(
    int port_,
  ) {
    return _wire_call_new_module_system(
      port_,
    );
  }

  late final _wire_call_new_module_systemPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_call_new_module_system');
  late final _wire_call_new_module_system =
      _wire_call_new_module_systemPtr.asFunction<void Function(int)>();

  void wire_call_old_module_system(
    int port_,
  ) {
    return _wire_call_old_module_system(
      port_,
    );
  }

  late final _wire_call_old_module_systemPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_call_old_module_system');
  late final _wire_call_old_module_system =
      _wire_call_old_module_systemPtr.asFunction<void Function(int)>();

  void wire_use_imported_enum(
    int port_,
    int my_enum,
  ) {
    return _wire_use_imported_enum(
      port_,
      my_enum,
    );
  }

  late final _wire_use_imported_enumPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_use_imported_enum');
  late final _wire_use_imported_enum =
      _wire_use_imported_enumPtr.asFunction<void Function(int, int)>();

  void wire_use_imported_struct(
    int port_,
    ffi.Pointer<wire_my_struct> my_struct,
  ) {
    return _wire_use_imported_struct(
      port_,
      my_struct,
    );
  }

  late final _wire_use_imported_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_my_struct>)>>('wire_use_imported_struct');
  late final _wire_use_imported_struct = _wire_use_imported_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_my_struct>)>();

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

  void wire_ConcatenateWith_concatenate(
    int port_,
    ffi.Pointer<wire_concatenate_with> that,
    ffi.Pointer<wire_list_prim_u_8> b,
  ) {
    return _wire_ConcatenateWith_concatenate(
      port_,
      that,
      b,
    );
  }

  late final _wire_ConcatenateWith_concatenatePtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_concatenate_with>,
                  ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_ConcatenateWith_concatenate');
  late final _wire_ConcatenateWith_concatenate =
      _wire_ConcatenateWith_concatenatePtr.asFunction<
          void Function(int, ffi.Pointer<wire_concatenate_with>,
              ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_ConcatenateWith_concatenate_static(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> a,
    ffi.Pointer<wire_list_prim_u_8> b,
  ) {
    return _wire_ConcatenateWith_concatenate_static(
      port_,
      a,
      b,
    );
  }

  late final _wire_ConcatenateWith_concatenate_staticPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_prim_u_8>,
                  ffi.Pointer<wire_list_prim_u_8>)>>(
      'wire_ConcatenateWith_concatenate_static');
  late final _wire_ConcatenateWith_concatenate_static =
      _wire_ConcatenateWith_concatenate_staticPtr.asFunction<
          void Function(int, ffi.Pointer<wire_list_prim_u_8>,
              ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_ConcatenateWith_handle_some_static_stream_sink(
    int port_,
    int key,
    int max,
  ) {
    return _wire_ConcatenateWith_handle_some_static_stream_sink(
      port_,
      key,
      max,
    );
  }

  late final _wire_ConcatenateWith_handle_some_static_stream_sinkPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Uint32, ffi.Uint32)>>(
      'wire_ConcatenateWith_handle_some_static_stream_sink');
  late final _wire_ConcatenateWith_handle_some_static_stream_sink =
      _wire_ConcatenateWith_handle_some_static_stream_sinkPtr
          .asFunction<void Function(int, int, int)>();

  void wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
    int port_,
  ) {
    return _wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(
      port_,
    );
  }

  late final _wire_ConcatenateWith_handle_some_static_stream_sink_single_argPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_ConcatenateWith_handle_some_static_stream_sink_single_arg');
  late final _wire_ConcatenateWith_handle_some_static_stream_sink_single_arg =
      _wire_ConcatenateWith_handle_some_static_stream_sink_single_argPtr
          .asFunction<void Function(int)>();

  void wire_ConcatenateWith_handle_some_stream_sink(
    int port_,
    ffi.Pointer<wire_concatenate_with> that,
    int key,
    int max,
  ) {
    return _wire_ConcatenateWith_handle_some_stream_sink(
      port_,
      that,
      key,
      max,
    );
  }

  late final _wire_ConcatenateWith_handle_some_stream_sinkPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_concatenate_with>,
              ffi.Uint32,
              ffi.Uint32)>>('wire_ConcatenateWith_handle_some_stream_sink');
  late final _wire_ConcatenateWith_handle_some_stream_sink =
      _wire_ConcatenateWith_handle_some_stream_sinkPtr.asFunction<
          void Function(int, ffi.Pointer<wire_concatenate_with>, int, int)>();

  void wire_ConcatenateWith_handle_some_stream_sink_at_1(
    int port_,
    ffi.Pointer<wire_concatenate_with> that,
  ) {
    return _wire_ConcatenateWith_handle_some_stream_sink_at_1(
      port_,
      that,
    );
  }

  late final _wire_ConcatenateWith_handle_some_stream_sink_at_1Ptr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_concatenate_with>)>>(
      'wire_ConcatenateWith_handle_some_stream_sink_at_1');
  late final _wire_ConcatenateWith_handle_some_stream_sink_at_1 =
      _wire_ConcatenateWith_handle_some_stream_sink_at_1Ptr
          .asFunction<void Function(int, ffi.Pointer<wire_concatenate_with>)>();

  void wire_ConcatenateWith_new(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> a,
  ) {
    return _wire_ConcatenateWith_new(
      port_,
      a,
    );
  }

  late final _wire_ConcatenateWith_newPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_ConcatenateWith_new');
  late final _wire_ConcatenateWith_new = _wire_ConcatenateWith_newPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_SumWith_sum(
    int port_,
    ffi.Pointer<wire_sum_with> that,
    int y,
    int z,
  ) {
    return _wire_SumWith_sum(
      port_,
      that,
      y,
      z,
    );
  }

  late final _wire_SumWith_sumPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_sum_with>, ffi.Uint32,
              ffi.Uint32)>>('wire_SumWith_sum');
  late final _wire_SumWith_sum = _wire_SumWith_sumPtr
      .asFunction<void Function(int, ffi.Pointer<wire_sum_with>, int, int)>();

  void wire_get_sum_array(
    int port_,
    int a,
    int b,
    int c,
  ) {
    return _wire_get_sum_array(
      port_,
      a,
      b,
      c,
    );
  }

  late final _wire_get_sum_arrayPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Uint32, ffi.Uint32,
              ffi.Uint32)>>('wire_get_sum_array');
  late final _wire_get_sum_array =
      _wire_get_sum_arrayPtr.asFunction<void Function(int, int, int, int)>();

  void wire_get_sum_struct(
    int port_,
  ) {
    return _wire_get_sum_struct(
      port_,
    );
  }

  late final _wire_get_sum_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_sum_struct');
  late final _wire_get_sum_struct =
      _wire_get_sum_structPtr.asFunction<void Function(int)>();

  void wire_app_settings_stream(
    int port_,
  ) {
    return _wire_app_settings_stream(
      port_,
    );
  }

  late final _wire_app_settings_streamPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_app_settings_stream');
  late final _wire_app_settings_stream =
      _wire_app_settings_streamPtr.asFunction<void Function(int)>();

  void wire_app_settings_vec_stream(
    int port_,
  ) {
    return _wire_app_settings_vec_stream(
      port_,
    );
  }

  late final _wire_app_settings_vec_streamPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_app_settings_vec_stream');
  late final _wire_app_settings_vec_stream =
      _wire_app_settings_vec_streamPtr.asFunction<void Function(int)>();

  void wire_first_number(
    int port_,
    ffi.Pointer<wire_numbers> nums,
  ) {
    return _wire_first_number(
      port_,
      nums,
    );
  }

  late final _wire_first_numberPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_numbers>)>>('wire_first_number');
  late final _wire_first_number = _wire_first_numberPtr
      .asFunction<void Function(int, ffi.Pointer<wire_numbers>)>();

  void wire_first_sequence(
    int port_,
    ffi.Pointer<wire_sequences> seqs,
  ) {
    return _wire_first_sequence(
      port_,
      seqs,
    );
  }

  late final _wire_first_sequencePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_sequences>)>>('wire_first_sequence');
  late final _wire_first_sequence = _wire_first_sequencePtr
      .asFunction<void Function(int, ffi.Pointer<wire_sequences>)>();

  void wire_get_app_settings(
    int port_,
  ) {
    return _wire_get_app_settings(
      port_,
    );
  }

  late final _wire_get_app_settingsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_app_settings');
  late final _wire_get_app_settings =
      _wire_get_app_settingsPtr.asFunction<void Function(int)>();

  void wire_get_fallible_app_settings(
    int port_,
  ) {
    return _wire_get_fallible_app_settings(
      port_,
    );
  }

  late final _wire_get_fallible_app_settingsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_fallible_app_settings');
  late final _wire_get_fallible_app_settings =
      _wire_get_fallible_app_settingsPtr.asFunction<void Function(int)>();

  void wire_get_message(
    int port_,
  ) {
    return _wire_get_message(
      port_,
    );
  }

  late final _wire_get_messagePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_get_message');
  late final _wire_get_message =
      _wire_get_messagePtr.asFunction<void Function(int)>();

  void wire_is_app_embedded(
    int port_,
    ffi.Pointer<wire_application_settings> app_settings,
  ) {
    return _wire_is_app_embedded(
      port_,
      app_settings,
    );
  }

  late final _wire_is_app_embeddedPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_application_settings>)>>('wire_is_app_embedded');
  late final _wire_is_app_embedded = _wire_is_app_embeddedPtr
      .asFunction<void Function(int, ffi.Pointer<wire_application_settings>)>();

  void wire_mirror_struct_stream(
    int port_,
  ) {
    return _wire_mirror_struct_stream(
      port_,
    );
  }

  late final _wire_mirror_struct_streamPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_mirror_struct_stream');
  late final _wire_mirror_struct_stream =
      _wire_mirror_struct_streamPtr.asFunction<void Function(int)>();

  void wire_mirror_tuple_stream(
    int port_,
  ) {
    return _wire_mirror_tuple_stream(
      port_,
    );
  }

  late final _wire_mirror_tuple_streamPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_mirror_tuple_stream');
  late final _wire_mirror_tuple_stream =
      _wire_mirror_tuple_streamPtr.asFunction<void Function(int)>();

  void wire_repeat_number(
    int port_,
    int num,
    int times,
  ) {
    return _wire_repeat_number(
      port_,
      num,
      times,
    );
  }

  late final _wire_repeat_numberPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Int32, ffi.UintPtr)>>('wire_repeat_number');
  late final _wire_repeat_number =
      _wire_repeat_numberPtr.asFunction<void Function(int, int, int)>();

  void wire_repeat_sequence(
    int port_,
    int seq,
    int times,
  ) {
    return _wire_repeat_sequence(
      port_,
      seq,
      times,
    );
  }

  late final _wire_repeat_sequencePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Int32, ffi.UintPtr)>>('wire_repeat_sequence');
  late final _wire_repeat_sequence =
      _wire_repeat_sequencePtr.asFunction<void Function(int, int, int)>();

  void wire_test_contains_mirrored_sub_struct(
    int port_,
  ) {
    return _wire_test_contains_mirrored_sub_struct(
      port_,
    );
  }

  late final _wire_test_contains_mirrored_sub_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_contains_mirrored_sub_struct');
  late final _wire_test_contains_mirrored_sub_struct =
      _wire_test_contains_mirrored_sub_structPtr
          .asFunction<void Function(int)>();

  void wire_test_fallible_of_raw_string_mirrored(
    int port_,
  ) {
    return _wire_test_fallible_of_raw_string_mirrored(
      port_,
    );
  }

  late final _wire_test_fallible_of_raw_string_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_fallible_of_raw_string_mirrored');
  late final _wire_test_fallible_of_raw_string_mirrored =
      _wire_test_fallible_of_raw_string_mirroredPtr
          .asFunction<void Function(int)>();

  void wire_test_list_of_nested_enums_mirrored(
    int port_,
  ) {
    return _wire_test_list_of_nested_enums_mirrored(
      port_,
    );
  }

  late final _wire_test_list_of_nested_enums_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_list_of_nested_enums_mirrored');
  late final _wire_test_list_of_nested_enums_mirrored =
      _wire_test_list_of_nested_enums_mirroredPtr
          .asFunction<void Function(int)>();

  void wire_test_list_of_raw_nested_string_mirrored(
    int port_,
  ) {
    return _wire_test_list_of_raw_nested_string_mirrored(
      port_,
    );
  }

  late final _wire_test_list_of_raw_nested_string_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_list_of_raw_nested_string_mirrored');
  late final _wire_test_list_of_raw_nested_string_mirrored =
      _wire_test_list_of_raw_nested_string_mirroredPtr
          .asFunction<void Function(int)>();

  void wire_test_nested_raw_string_mirrored(
    int port_,
  ) {
    return _wire_test_nested_raw_string_mirrored(
      port_,
    );
  }

  late final _wire_test_nested_raw_string_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_nested_raw_string_mirrored');
  late final _wire_test_nested_raw_string_mirrored =
      _wire_test_nested_raw_string_mirroredPtr.asFunction<void Function(int)>();

  void wire_test_raw_string_enum_mirrored(
    int port_,
    bool nested,
  ) {
    return _wire_test_raw_string_enum_mirrored(
      port_,
      nested,
    );
  }

  late final _wire_test_raw_string_enum_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Bool)>>(
          'wire_test_raw_string_enum_mirrored');
  late final _wire_test_raw_string_enum_mirrored =
      _wire_test_raw_string_enum_mirroredPtr
          .asFunction<void Function(int, bool)>();

  void wire_test_raw_string_mirrored(
    int port_,
  ) {
    return _wire_test_raw_string_mirrored(
      port_,
    );
  }

  late final _wire_test_raw_string_mirroredPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_raw_string_mirrored');
  late final _wire_test_raw_string_mirrored =
      _wire_test_raw_string_mirroredPtr.asFunction<void Function(int)>();

  void wire_handle_big_buffers(
    int port_,
  ) {
    return _wire_handle_big_buffers(
      port_,
    );
  }

  late final _wire_handle_big_buffersPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_handle_big_buffers');
  late final _wire_handle_big_buffers =
      _wire_handle_big_buffersPtr.asFunction<void Function(int)>();

  void wire_handle_complex_struct(
    int port_,
    ffi.Pointer<wire_my_tree_node> s,
  ) {
    return _wire_handle_complex_struct(
      port_,
      s,
    );
  }

  late final _wire_handle_complex_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_my_tree_node>)>>('wire_handle_complex_struct');
  late final _wire_handle_complex_struct = _wire_handle_complex_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_my_tree_node>)>();

  void wire_handle_nested_struct(
    int port_,
    ffi.Pointer<wire_my_nested_struct> s,
  ) {
    return _wire_handle_nested_struct(
      port_,
      s,
    );
  }

  late final _wire_handle_nested_structPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_my_nested_struct>)>>(
      'wire_handle_nested_struct');
  late final _wire_handle_nested_struct = _wire_handle_nested_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_my_nested_struct>)>();

  void wire_list_of_primitive_enums(
    int port_,
    ffi.Pointer<wire_list_weekdays> weekdays,
  ) {
    return _wire_list_of_primitive_enums(
      port_,
      weekdays,
    );
  }

  late final _wire_list_of_primitive_enumsPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_weekdays>)>>(
      'wire_list_of_primitive_enums');
  late final _wire_list_of_primitive_enums = _wire_list_of_primitive_enumsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_weekdays>)>();

  void wire_test_abc_enum(
    int port_,
    ffi.Pointer<wire_abc> abc,
  ) {
    return _wire_test_abc_enum(
      port_,
      abc,
    );
  }

  late final _wire_test_abc_enumPtr = _lookup<
          ffi
          .NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_abc>)>>(
      'wire_test_abc_enum');
  late final _wire_test_abc_enum = _wire_test_abc_enumPtr
      .asFunction<void Function(int, ffi.Pointer<wire_abc>)>();

  void wire_test_struct_with_enum(
    int port_,
    ffi.Pointer<wire_struct_with_enum> se,
  ) {
    return _wire_test_struct_with_enum(
      port_,
      se,
    );
  }

  late final _wire_test_struct_with_enumPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_struct_with_enum>)>>(
      'wire_test_struct_with_enum');
  late final _wire_test_struct_with_enum = _wire_test_struct_with_enumPtr
      .asFunction<void Function(int, ffi.Pointer<wire_struct_with_enum>)>();

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

  void wire_handle_list_of_struct(
    int port_,
    ffi.Pointer<wire_list_my_size> l,
  ) {
    return _wire_handle_list_of_struct(
      port_,
      l,
    );
  }

  late final _wire_handle_list_of_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_my_size>)>>('wire_handle_list_of_struct');
  late final _wire_handle_list_of_struct = _wire_handle_list_of_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_my_size>)>();

  void wire_handle_string_list(
    int port_,
    ffi.Pointer<wire_StringList> names,
  ) {
    return _wire_handle_string_list(
      port_,
      names,
    );
  }

  late final _wire_handle_string_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_StringList>)>>('wire_handle_string_list');
  late final _wire_handle_string_list = _wire_handle_string_listPtr
      .asFunction<void Function(int, ffi.Pointer<wire_StringList>)>();

  void wire_handle_newtype(
    int port_,
    ffi.Pointer<wire_new_type_int> arg,
  ) {
    return _wire_handle_newtype(
      port_,
      arg,
    );
  }

  late final _wire_handle_newtypePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_new_type_int>)>>('wire_handle_newtype');
  late final _wire_handle_newtype = _wire_handle_newtypePtr
      .asFunction<void Function(int, ffi.Pointer<wire_new_type_int>)>();

  void wire_handle_increment_boxed_optional(
    int port_,
    ffi.Pointer<ffi.Double> opt,
  ) {
    return _wire_handle_increment_boxed_optional(
      port_,
      opt,
    );
  }

  late final _wire_handle_increment_boxed_optionalPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<ffi.Double>)>>(
      'wire_handle_increment_boxed_optional');
  late final _wire_handle_increment_boxed_optional =
      _wire_handle_increment_boxed_optionalPtr
          .asFunction<void Function(int, ffi.Pointer<ffi.Double>)>();

  void wire_handle_option_box_arguments(
    int port_,
    ffi.Pointer<ffi.Int8> i8box,
    ffi.Pointer<ffi.Uint8> u8box,
    ffi.Pointer<ffi.Int32> i32box,
    ffi.Pointer<ffi.Int64> i64box,
    ffi.Pointer<ffi.Double> f64box,
    ffi.Pointer<ffi.Bool> boolbox,
    ffi.Pointer<wire_exotic_optionals> structbox,
  ) {
    return _wire_handle_option_box_arguments(
      port_,
      i8box,
      u8box,
      i32box,
      i64box,
      f64box,
      boolbox,
      structbox,
    );
  }

  late final _wire_handle_option_box_argumentsPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<ffi.Int8>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Pointer<ffi.Int32>,
                  ffi.Pointer<ffi.Int64>,
                  ffi.Pointer<ffi.Double>,
                  ffi.Pointer<ffi.Bool>,
                  ffi.Pointer<wire_exotic_optionals>)>>(
      'wire_handle_option_box_arguments');
  late final _wire_handle_option_box_arguments =
      _wire_handle_option_box_argumentsPtr.asFunction<
          void Function(
              int,
              ffi.Pointer<ffi.Int8>,
              ffi.Pointer<ffi.Uint8>,
              ffi.Pointer<ffi.Int32>,
              ffi.Pointer<ffi.Int64>,
              ffi.Pointer<ffi.Double>,
              ffi.Pointer<ffi.Bool>,
              ffi.Pointer<wire_exotic_optionals>)>();

  void wire_handle_optional_increment(
    int port_,
    ffi.Pointer<wire_exotic_optionals> opt,
  ) {
    return _wire_handle_optional_increment(
      port_,
      opt,
    );
  }

  late final _wire_handle_optional_incrementPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_exotic_optionals>)>>(
      'wire_handle_optional_increment');
  late final _wire_handle_optional_increment =
      _wire_handle_optional_incrementPtr
          .asFunction<void Function(int, ffi.Pointer<wire_exotic_optionals>)>();

  void wire_handle_optional_return(
    int port_,
    double left,
    double right,
  ) {
    return _wire_handle_optional_return(
      port_,
      left,
      right,
    );
  }

  late final _wire_handle_optional_returnPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Double,
              ffi.Double)>>('wire_handle_optional_return');
  late final _wire_handle_optional_return = _wire_handle_optional_returnPtr
      .asFunction<void Function(int, double, double)>();

  void wire_handle_optional_struct(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> document,
  ) {
    return _wire_handle_optional_struct(
      port_,
      document,
    );
  }

  late final _wire_handle_optional_structPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_handle_optional_struct');
  late final _wire_handle_optional_struct = _wire_handle_optional_structPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_handle_vec_of_opts(
    int port_,
    ffi.Pointer<wire_opt_vecs> opt,
  ) {
    return _wire_handle_vec_of_opts(
      port_,
      opt,
    );
  }

  late final _wire_handle_vec_of_optsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_opt_vecs>)>>('wire_handle_vec_of_opts');
  late final _wire_handle_vec_of_opts = _wire_handle_vec_of_optsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_opt_vecs>)>();

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

  WireSyncReturn wire_custom_enum_error_panic_twin_sync() {
    return _wire_custom_enum_error_panic_twin_sync();
  }

  late final _wire_custom_enum_error_panic_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_custom_enum_error_panic_twin_sync');
  late final _wire_custom_enum_error_panic_twin_sync =
      _wire_custom_enum_error_panic_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_custom_enum_error_return_error_twin_sync() {
    return _wire_custom_enum_error_return_error_twin_sync();
  }

  late final _wire_custom_enum_error_return_error_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_custom_enum_error_return_error_twin_sync');
  late final _wire_custom_enum_error_return_error_twin_sync =
      _wire_custom_enum_error_return_error_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_custom_enum_error_return_ok_twin_sync(
    int arg,
  ) {
    return _wire_custom_enum_error_return_ok_twin_sync(
      arg,
    );
  }

  late final _wire_custom_enum_error_return_ok_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Uint32)>>(
          'wire_custom_enum_error_return_ok_twin_sync');
  late final _wire_custom_enum_error_return_ok_twin_sync =
      _wire_custom_enum_error_return_ok_twin_syncPtr
          .asFunction<WireSyncReturn Function(int)>();

  WireSyncReturn wire_custom_nested_error_return_error_twin_sync(
    ffi.Pointer<wire_custom_nested_error_outer_twin_sync> arg,
  ) {
    return _wire_custom_nested_error_return_error_twin_sync(
      arg,
    );
  }

  late final _wire_custom_nested_error_return_error_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_custom_nested_error_outer_twin_sync>)>>(
      'wire_custom_nested_error_return_error_twin_sync');
  late final _wire_custom_nested_error_return_error_twin_sync =
      _wire_custom_nested_error_return_error_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_custom_nested_error_outer_twin_sync>)>();

  WireSyncReturn wire_custom_struct_error_return_error_twin_sync(
    ffi.Pointer<wire_custom_struct_error_twin_sync> arg,
  ) {
    return _wire_custom_struct_error_return_error_twin_sync(
      arg,
    );
  }

  late final _wire_custom_struct_error_return_error_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturn Function(
                  ffi.Pointer<wire_custom_struct_error_twin_sync>)>>(
      'wire_custom_struct_error_return_error_twin_sync');
  late final _wire_custom_struct_error_return_error_twin_sync =
      _wire_custom_struct_error_return_error_twin_syncPtr.asFunction<
          WireSyncReturn Function(
              ffi.Pointer<wire_custom_struct_error_twin_sync>)>();

  WireSyncReturn wire_func_return_error_twin_sync() {
    return _wire_func_return_error_twin_sync();
  }

  late final _wire_func_return_error_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_return_error_twin_sync');
  late final _wire_func_return_error_twin_sync =
      _wire_func_return_error_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_func_type_fallible_panic_twin_sync() {
    return _wire_func_type_fallible_panic_twin_sync();
  }

  late final _wire_func_type_fallible_panic_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_type_fallible_panic_twin_sync');
  late final _wire_func_type_fallible_panic_twin_sync =
      _wire_func_type_fallible_panic_twin_syncPtr
          .asFunction<WireSyncReturn Function()>();

  WireSyncReturn wire_func_type_infallible_panic_twin_sync() {
    return _wire_func_type_infallible_panic_twin_sync();
  }

  late final _wire_func_type_infallible_panic_twin_syncPtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function()>>(
          'wire_func_type_infallible_panic_twin_sync');
  late final _wire_func_type_infallible_panic_twin_sync =
      _wire_func_type_infallible_panic_twin_syncPtr
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

  void wire_test_more_than_just_one_raw_string_struct(
    int port_,
  ) {
    return _wire_test_more_than_just_one_raw_string_struct(
      port_,
    );
  }

  late final _wire_test_more_than_just_one_raw_string_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_more_than_just_one_raw_string_struct');
  late final _wire_test_more_than_just_one_raw_string_struct =
      _wire_test_more_than_just_one_raw_string_structPtr
          .asFunction<void Function(int)>();

  void wire_test_raw_string_item_struct(
    int port_,
  ) {
    return _wire_test_raw_string_item_struct(
      port_,
    );
  }

  late final _wire_test_raw_string_item_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_test_raw_string_item_struct');
  late final _wire_test_raw_string_item_struct =
      _wire_test_raw_string_item_structPtr.asFunction<void Function(int)>();

  void wire_create_array_opaque_enum(
    int port_,
  ) {
    return _wire_create_array_opaque_enum(
      port_,
    );
  }

  late final _wire_create_array_opaque_enumPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_create_array_opaque_enum');
  late final _wire_create_array_opaque_enum =
      _wire_create_array_opaque_enumPtr.asFunction<void Function(int)>();

  void wire_create_nested_opaque(
    int port_,
  ) {
    return _wire_create_nested_opaque(
      port_,
    );
  }

  late final _wire_create_nested_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_create_nested_opaque');
  late final _wire_create_nested_opaque =
      _wire_create_nested_opaquePtr.asFunction<void Function(int)>();

  void wire_create_opaque(
    int port_,
  ) {
    return _wire_create_opaque(
      port_,
    );
  }

  late final _wire_create_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_create_opaque');
  late final _wire_create_opaque =
      _wire_create_opaquePtr.asFunction<void Function(int)>();

  void wire_create_option_opaque(
    int port_,
    ffi.Pointer<wire_RustOpaque_hide_data> opaque,
  ) {
    return _wire_create_option_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_create_option_opaquePtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_RustOpaque_hide_data>)>>(
      'wire_create_option_opaque');
  late final _wire_create_option_opaque = _wire_create_option_opaquePtr
      .asFunction<void Function(int, ffi.Pointer<wire_RustOpaque_hide_data>)>();

  void wire_create_sync_opaque(
    int port_,
  ) {
    return _wire_create_sync_opaque(
      port_,
    );
  }

  late final _wire_create_sync_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_create_sync_opaque');
  late final _wire_create_sync_opaque =
      _wire_create_sync_opaquePtr.asFunction<void Function(int)>();

  void wire_frb_generator_test(
    int port_,
  ) {
    return _wire_frb_generator_test(
      port_,
    );
  }

  late final _wire_frb_generator_testPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_frb_generator_test');
  late final _wire_frb_generator_test =
      _wire_frb_generator_testPtr.asFunction<void Function(int)>();

  void wire_opaque_array(
    int port_,
  ) {
    return _wire_opaque_array(
      port_,
    );
  }

  late final _wire_opaque_arrayPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_opaque_array');
  late final _wire_opaque_array =
      _wire_opaque_arrayPtr.asFunction<void Function(int)>();

  void wire_opaque_array_run(
    int port_,
    ffi.Pointer<wire_list_RustOpaque_hide_data> data,
  ) {
    return _wire_opaque_array_run(
      port_,
      data,
    );
  }

  late final _wire_opaque_array_runPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_list_RustOpaque_hide_data>)>>(
      'wire_opaque_array_run');
  late final _wire_opaque_array_run = _wire_opaque_array_runPtr.asFunction<
      void Function(int, ffi.Pointer<wire_list_RustOpaque_hide_data>)>();

  void wire_opaque_vec(
    int port_,
  ) {
    return _wire_opaque_vec(
      port_,
    );
  }

  late final _wire_opaque_vecPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_opaque_vec');
  late final _wire_opaque_vec =
      _wire_opaque_vecPtr.asFunction<void Function(int)>();

  void wire_opaque_vec_run(
    int port_,
    ffi.Pointer<wire_list_RustOpaque_hide_data> data,
  ) {
    return _wire_opaque_vec_run(
      port_,
      data,
    );
  }

  late final _wire_opaque_vec_runPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64, ffi.Pointer<wire_list_RustOpaque_hide_data>)>>(
      'wire_opaque_vec_run');
  late final _wire_opaque_vec_run = _wire_opaque_vec_runPtr.asFunction<
      void Function(int, ffi.Pointer<wire_list_RustOpaque_hide_data>)>();

  void wire_run_enum_opaque(
    int port_,
    ffi.Pointer<wire_enum_opaque> opaque,
  ) {
    return _wire_run_enum_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_run_enum_opaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_enum_opaque>)>>('wire_run_enum_opaque');
  late final _wire_run_enum_opaque = _wire_run_enum_opaquePtr
      .asFunction<void Function(int, ffi.Pointer<wire_enum_opaque>)>();

  void wire_run_nested_opaque(
    int port_,
    ffi.Pointer<wire_opaque_nested> opaque,
  ) {
    return _wire_run_nested_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_run_nested_opaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_opaque_nested>)>>('wire_run_nested_opaque');
  late final _wire_run_nested_opaque = _wire_run_nested_opaquePtr
      .asFunction<void Function(int, ffi.Pointer<wire_opaque_nested>)>();

  void wire_run_non_clone(
    int port_,
    wire_RustOpaque_non_clone_data clone,
  ) {
    return _wire_run_non_clone(
      port_,
      clone,
    );
  }

  late final _wire_run_non_clonePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              wire_RustOpaque_non_clone_data)>>('wire_run_non_clone');
  late final _wire_run_non_clone = _wire_run_non_clonePtr
      .asFunction<void Function(int, wire_RustOpaque_non_clone_data)>();

  void wire_run_opaque(
    int port_,
    wire_RustOpaque_hide_data opaque,
  ) {
    return _wire_run_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_run_opaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, wire_RustOpaque_hide_data)>>('wire_run_opaque');
  late final _wire_run_opaque = _wire_run_opaquePtr
      .asFunction<void Function(int, wire_RustOpaque_hide_data)>();

  void wire_run_opaque_with_delay(
    int port_,
    wire_RustOpaque_hide_data opaque,
  ) {
    return _wire_run_opaque_with_delay(
      port_,
      opaque,
    );
  }

  late final _wire_run_opaque_with_delayPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              wire_RustOpaque_hide_data)>>('wire_run_opaque_with_delay');
  late final _wire_run_opaque_with_delay = _wire_run_opaque_with_delayPtr
      .asFunction<void Function(int, wire_RustOpaque_hide_data)>();

  void wire_unwrap_rust_opaque(
    int port_,
    wire_RustOpaque_hide_data opaque,
  ) {
    return _wire_unwrap_rust_opaque(
      port_,
      opaque,
    );
  }

  late final _wire_unwrap_rust_opaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              wire_RustOpaque_hide_data)>>('wire_unwrap_rust_opaque');
  late final _wire_unwrap_rust_opaque = _wire_unwrap_rust_opaquePtr
      .asFunction<void Function(int, wire_RustOpaque_hide_data)>();

  void wire_frb_sync_generator_test(
    int port_,
  ) {
    return _wire_frb_sync_generator_test(
      port_,
    );
  }

  late final _wire_frb_sync_generator_testPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_frb_sync_generator_test');
  late final _wire_frb_sync_generator_test =
      _wire_frb_sync_generator_testPtr.asFunction<void Function(int)>();

  WireSyncReturn wire_sync_run_opaque(
    wire_RustOpaque_non_send_hide_data opaque,
  ) {
    return _wire_sync_run_opaque(
      opaque,
    );
  }

  late final _wire_sync_run_opaquePtr = _lookup<
      ffi.NativeFunction<
          WireSyncReturn Function(
              wire_RustOpaque_non_send_hide_data)>>('wire_sync_run_opaque');
  late final _wire_sync_run_opaque = _wire_sync_run_opaquePtr.asFunction<
      WireSyncReturn Function(wire_RustOpaque_non_send_hide_data)>();

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

  void wire_handle_stream_of_struct(
    int port_,
  ) {
    return _wire_handle_stream_of_struct(
      port_,
    );
  }

  late final _wire_handle_stream_of_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_handle_stream_of_struct');
  late final _wire_handle_stream_of_struct =
      _wire_handle_stream_of_structPtr.asFunction<void Function(int)>();

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

  void wire_test_tuple(
    int port_,
    ffi.Pointer<wire_record_string_i_32> value,
  ) {
    return _wire_test_tuple(
      port_,
      value,
    );
  }

  late final _wire_test_tuplePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_record_string_i_32>)>>('wire_test_tuple');
  late final _wire_test_tuple = _wire_test_tuplePtr
      .asFunction<void Function(int, ffi.Pointer<wire_record_string_i_32>)>();

  void wire_test_tuple_2(
    int port_,
    ffi.Pointer<wire_list_record_string_i_32> value,
  ) {
    return _wire_test_tuple_2(
      port_,
      value,
    );
  }

  late final _wire_test_tuple_2Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_record_string_i_32>)>>('wire_test_tuple_2');
  late final _wire_test_tuple_2 = _wire_test_tuple_2Ptr.asFunction<
      void Function(int, ffi.Pointer<wire_list_record_string_i_32>)>();

  void wire_handle_type_alias_id(
    int port_,
    int input,
  ) {
    return _wire_handle_type_alias_id(
      port_,
      input,
    );
  }

  late final _wire_handle_type_alias_idPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>(
          'wire_handle_type_alias_id');
  late final _wire_handle_type_alias_id =
      _wire_handle_type_alias_idPtr.asFunction<void Function(int, int)>();

  void wire_handle_type_alias_model(
    int port_,
    int input,
  ) {
    return _wire_handle_type_alias_model(
      port_,
      input,
    );
  }

  late final _wire_handle_type_alias_modelPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>(
          'wire_handle_type_alias_model');
  late final _wire_handle_type_alias_model =
      _wire_handle_type_alias_modelPtr.asFunction<void Function(int, int)>();

  void wire_handle_type_nest_alias_id(
    int port_,
    int input,
  ) {
    return _wire_handle_type_nest_alias_id(
      port_,
      input,
    );
  }

  late final _wire_handle_type_nest_alias_idPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>(
          'wire_handle_type_nest_alias_id');
  late final _wire_handle_type_nest_alias_id =
      _wire_handle_type_nest_alias_idPtr.asFunction<void Function(int, int)>();

  void wire_handle_nested_uuids(
    int port_,
    ffi.Pointer<wire_feature_uuid> ids,
  ) {
    return _wire_handle_nested_uuids(
      port_,
      ids,
    );
  }

  late final _wire_handle_nested_uuidsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_feature_uuid>)>>('wire_handle_nested_uuids');
  late final _wire_handle_nested_uuids = _wire_handle_nested_uuidsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_feature_uuid>)>();

  void wire_handle_uuid(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> id,
  ) {
    return _wire_handle_uuid(
      port_,
      id,
    );
  }

  late final _wire_handle_uuidPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_list_prim_u_8>)>>('wire_handle_uuid');
  late final _wire_handle_uuid = _wire_handle_uuidPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  void wire_handle_uuids(
    int port_,
    ffi.Pointer<wire_list_prim_u_8> ids,
  ) {
    return _wire_handle_uuids(
      port_,
      ids,
    );
  }

  late final _wire_handle_uuidsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_list_prim_u_8>)>>('wire_handle_uuids');
  late final _wire_handle_uuids = _wire_handle_uuidsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_list_prim_u_8>)>();

  wire_DartOpaque new_DartOpaque() {
    return _new_DartOpaque();
  }

  late final _new_DartOpaquePtr =
      _lookup<ffi.NativeFunction<wire_DartOpaque Function()>>('new_DartOpaque');
  late final _new_DartOpaque =
      _new_DartOpaquePtr.asFunction<wire_DartOpaque Function()>();

  wire_RustOpaque_MutexHideData new_RustOpaque_MutexHideData() {
    return _new_RustOpaque_MutexHideData();
  }

  late final _new_RustOpaque_MutexHideDataPtr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_MutexHideData Function()>>(
          'new_RustOpaque_MutexHideData');
  late final _new_RustOpaque_MutexHideData = _new_RustOpaque_MutexHideDataPtr
      .asFunction<wire_RustOpaque_MutexHideData Function()>();

  wire_RustOpaque_RwLockHideData new_RustOpaque_RwLockHideData() {
    return _new_RustOpaque_RwLockHideData();
  }

  late final _new_RustOpaque_RwLockHideDataPtr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_RwLockHideData Function()>>(
          'new_RustOpaque_RwLockHideData');
  late final _new_RustOpaque_RwLockHideData = _new_RustOpaque_RwLockHideDataPtr
      .asFunction<wire_RustOpaque_RwLockHideData Function()>();

  wire_RustOpaque_box_dynDartDebug new_RustOpaque_box_dynDartDebug() {
    return _new_RustOpaque_box_dynDartDebug();
  }

  late final _new_RustOpaque_box_dynDartDebugPtr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_box_dynDartDebug Function()>>(
          'new_RustOpaque_box_dynDartDebug');
  late final _new_RustOpaque_box_dynDartDebug =
      _new_RustOpaque_box_dynDartDebugPtr
          .asFunction<wire_RustOpaque_box_dynDartDebug Function()>();

  wire_RustOpaque_hide_data new_RustOpaque_hide_data() {
    return _new_RustOpaque_hide_data();
  }

  late final _new_RustOpaque_hide_dataPtr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_hide_data Function()>>(
          'new_RustOpaque_hide_data');
  late final _new_RustOpaque_hide_data = _new_RustOpaque_hide_dataPtr
      .asFunction<wire_RustOpaque_hide_data Function()>();

  wire_RustOpaque_i_32 new_RustOpaque_i_32() {
    return _new_RustOpaque_i_32();
  }

  late final _new_RustOpaque_i_32Ptr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_i_32 Function()>>(
          'new_RustOpaque_i_32');
  late final _new_RustOpaque_i_32 =
      _new_RustOpaque_i_32Ptr.asFunction<wire_RustOpaque_i_32 Function()>();

  wire_RustOpaque_non_clone_data new_RustOpaque_non_clone_data() {
    return _new_RustOpaque_non_clone_data();
  }

  late final _new_RustOpaque_non_clone_dataPtr =
      _lookup<ffi.NativeFunction<wire_RustOpaque_non_clone_data Function()>>(
          'new_RustOpaque_non_clone_data');
  late final _new_RustOpaque_non_clone_data = _new_RustOpaque_non_clone_dataPtr
      .asFunction<wire_RustOpaque_non_clone_data Function()>();

  wire_RustOpaque_non_send_hide_data new_RustOpaque_non_send_hide_data() {
    return _new_RustOpaque_non_send_hide_data();
  }

  late final _new_RustOpaque_non_send_hide_dataPtr = _lookup<
          ffi.NativeFunction<wire_RustOpaque_non_send_hide_data Function()>>(
      'new_RustOpaque_non_send_hide_data');
  late final _new_RustOpaque_non_send_hide_data =
      _new_RustOpaque_non_send_hide_dataPtr
          .asFunction<wire_RustOpaque_non_send_hide_data Function()>();

  ffi.Pointer<wire_StringList> new_StringList(
    int len,
  ) {
    return _new_StringList(
      len,
    );
  }

  late final _new_StringListPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_StringList> Function(ffi.Int32)>>(
      'new_StringList');
  late final _new_StringList = _new_StringListPtr
      .asFunction<ffi.Pointer<wire_StringList> Function(int)>();

  ffi.Pointer<wire_application_env> new_box_application_env() {
    return _new_box_application_env();
  }

  late final _new_box_application_envPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_application_env> Function()>>(
          'new_box_application_env');
  late final _new_box_application_env = _new_box_application_envPtr
      .asFunction<ffi.Pointer<wire_application_env> Function()>();

  ffi.Pointer<ffi.Int64> new_box_autoadd_Chrono_Utc(
    int value,
  ) {
    return _new_box_autoadd_Chrono_Utc(
      value,
    );
  }

  late final _new_box_autoadd_Chrono_UtcPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int64> Function(ffi.Int64)>>(
          'new_box_autoadd_Chrono_Utc');
  late final _new_box_autoadd_Chrono_Utc = _new_box_autoadd_Chrono_UtcPtr
      .asFunction<ffi.Pointer<ffi.Int64> Function(int)>();

  ffi.Pointer<wire_DartOpaque> new_box_autoadd_DartOpaque() {
    return _new_box_autoadd_DartOpaque();
  }

  late final _new_box_autoadd_DartOpaquePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_DartOpaque> Function()>>(
          'new_box_autoadd_DartOpaque');
  late final _new_box_autoadd_DartOpaque = _new_box_autoadd_DartOpaquePtr
      .asFunction<ffi.Pointer<wire_DartOpaque> Function()>();

  ffi.Pointer<wire_RustOpaque_hide_data>
      new_box_autoadd_RustOpaque_hide_data() {
    return _new_box_autoadd_RustOpaque_hide_data();
  }

  late final _new_box_autoadd_RustOpaque_hide_dataPtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_RustOpaque_hide_data> Function()>>(
      'new_box_autoadd_RustOpaque_hide_data');
  late final _new_box_autoadd_RustOpaque_hide_data =
      _new_box_autoadd_RustOpaque_hide_dataPtr
          .asFunction<ffi.Pointer<wire_RustOpaque_hide_data> Function()>();

  ffi.Pointer<wire_a> new_box_autoadd_a() {
    return _new_box_autoadd_a();
  }

  late final _new_box_autoadd_aPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_a> Function()>>(
          'new_box_autoadd_a');
  late final _new_box_autoadd_a =
      _new_box_autoadd_aPtr.asFunction<ffi.Pointer<wire_a> Function()>();

  ffi.Pointer<wire_abc> new_box_autoadd_abc() {
    return _new_box_autoadd_abc();
  }

  late final _new_box_autoadd_abcPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_abc> Function()>>(
          'new_box_autoadd_abc');
  late final _new_box_autoadd_abc =
      _new_box_autoadd_abcPtr.asFunction<ffi.Pointer<wire_abc> Function()>();

  ffi.Pointer<wire_application_env> new_box_autoadd_application_env() {
    return _new_box_autoadd_application_env();
  }

  late final _new_box_autoadd_application_envPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_application_env> Function()>>(
          'new_box_autoadd_application_env');
  late final _new_box_autoadd_application_env =
      _new_box_autoadd_application_envPtr
          .asFunction<ffi.Pointer<wire_application_env> Function()>();

  ffi.Pointer<wire_application_settings>
      new_box_autoadd_application_settings() {
    return _new_box_autoadd_application_settings();
  }

  late final _new_box_autoadd_application_settingsPtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_application_settings> Function()>>(
      'new_box_autoadd_application_settings');
  late final _new_box_autoadd_application_settings =
      _new_box_autoadd_application_settingsPtr
          .asFunction<ffi.Pointer<wire_application_settings> Function()>();

  ffi.Pointer<wire_attribute> new_box_autoadd_attribute() {
    return _new_box_autoadd_attribute();
  }

  late final _new_box_autoadd_attributePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_attribute> Function()>>(
          'new_box_autoadd_attribute');
  late final _new_box_autoadd_attribute = _new_box_autoadd_attributePtr
      .asFunction<ffi.Pointer<wire_attribute> Function()>();

  ffi.Pointer<wire_b> new_box_autoadd_b() {
    return _new_box_autoadd_b();
  }

  late final _new_box_autoadd_bPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_b> Function()>>(
          'new_box_autoadd_b');
  late final _new_box_autoadd_b =
      _new_box_autoadd_bPtr.asFunction<ffi.Pointer<wire_b> Function()>();

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

  ffi.Pointer<wire_c> new_box_autoadd_c() {
    return _new_box_autoadd_c();
  }

  late final _new_box_autoadd_cPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_c> Function()>>(
          'new_box_autoadd_c');
  late final _new_box_autoadd_c =
      _new_box_autoadd_cPtr.asFunction<ffi.Pointer<wire_c> Function()>();

  ffi.Pointer<wire_concatenate_with> new_box_autoadd_concatenate_with() {
    return _new_box_autoadd_concatenate_with();
  }

  late final _new_box_autoadd_concatenate_withPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_concatenate_with> Function()>>(
      'new_box_autoadd_concatenate_with');
  late final _new_box_autoadd_concatenate_with =
      _new_box_autoadd_concatenate_withPtr
          .asFunction<ffi.Pointer<wire_concatenate_with> Function()>();

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

  ffi.Pointer<wire_custom_nested_error_inner_twin_sync>
      new_box_autoadd_custom_nested_error_inner_twin_sync() {
    return _new_box_autoadd_custom_nested_error_inner_twin_sync();
  }

  late final _new_box_autoadd_custom_nested_error_inner_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_custom_nested_error_inner_twin_sync>
                  Function()>>(
      'new_box_autoadd_custom_nested_error_inner_twin_sync');
  late final _new_box_autoadd_custom_nested_error_inner_twin_sync =
      _new_box_autoadd_custom_nested_error_inner_twin_syncPtr.asFunction<
          ffi.Pointer<wire_custom_nested_error_inner_twin_sync> Function()>();

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

  ffi.Pointer<wire_custom_nested_error_outer_twin_sync>
      new_box_autoadd_custom_nested_error_outer_twin_sync() {
    return _new_box_autoadd_custom_nested_error_outer_twin_sync();
  }

  late final _new_box_autoadd_custom_nested_error_outer_twin_syncPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_custom_nested_error_outer_twin_sync>
                  Function()>>(
      'new_box_autoadd_custom_nested_error_outer_twin_sync');
  late final _new_box_autoadd_custom_nested_error_outer_twin_sync =
      _new_box_autoadd_custom_nested_error_outer_twin_syncPtr.asFunction<
          ffi.Pointer<wire_custom_nested_error_outer_twin_sync> Function()>();

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

  ffi.Pointer<wire_custom_struct_error_twin_sync>
      new_box_autoadd_custom_struct_error_twin_sync() {
    return _new_box_autoadd_custom_struct_error_twin_sync();
  }

  late final _new_box_autoadd_custom_struct_error_twin_syncPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_custom_struct_error_twin_sync>
              Function()>>('new_box_autoadd_custom_struct_error_twin_sync');
  late final _new_box_autoadd_custom_struct_error_twin_sync =
      _new_box_autoadd_custom_struct_error_twin_syncPtr.asFunction<
          ffi.Pointer<wire_custom_struct_error_twin_sync> Function()>();

  ffi.Pointer<wire_customized> new_box_autoadd_customized() {
    return _new_box_autoadd_customized();
  }

  late final _new_box_autoadd_customizedPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_customized> Function()>>(
          'new_box_autoadd_customized');
  late final _new_box_autoadd_customized = _new_box_autoadd_customizedPtr
      .asFunction<ffi.Pointer<wire_customized> Function()>();

  ffi.Pointer<wire_dart_opaque_nested> new_box_autoadd_dart_opaque_nested() {
    return _new_box_autoadd_dart_opaque_nested();
  }

  late final _new_box_autoadd_dart_opaque_nestedPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_dart_opaque_nested> Function()>>(
      'new_box_autoadd_dart_opaque_nested');
  late final _new_box_autoadd_dart_opaque_nested =
      _new_box_autoadd_dart_opaque_nestedPtr
          .asFunction<ffi.Pointer<wire_dart_opaque_nested> Function()>();

  ffi.Pointer<wire_enum_dart_opaque> new_box_autoadd_enum_dart_opaque() {
    return _new_box_autoadd_enum_dart_opaque();
  }

  late final _new_box_autoadd_enum_dart_opaquePtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_enum_dart_opaque> Function()>>(
      'new_box_autoadd_enum_dart_opaque');
  late final _new_box_autoadd_enum_dart_opaque =
      _new_box_autoadd_enum_dart_opaquePtr
          .asFunction<ffi.Pointer<wire_enum_dart_opaque> Function()>();

  ffi.Pointer<wire_enum_opaque> new_box_autoadd_enum_opaque() {
    return _new_box_autoadd_enum_opaque();
  }

  late final _new_box_autoadd_enum_opaquePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_enum_opaque> Function()>>(
          'new_box_autoadd_enum_opaque');
  late final _new_box_autoadd_enum_opaque = _new_box_autoadd_enum_opaquePtr
      .asFunction<ffi.Pointer<wire_enum_opaque> Function()>();

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

  ffi.Pointer<wire_event> new_box_autoadd_event() {
    return _new_box_autoadd_event();
  }

  late final _new_box_autoadd_eventPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_event> Function()>>(
          'new_box_autoadd_event');
  late final _new_box_autoadd_event = _new_box_autoadd_eventPtr
      .asFunction<ffi.Pointer<wire_event> Function()>();

  ffi.Pointer<wire_exotic_optionals> new_box_autoadd_exotic_optionals() {
    return _new_box_autoadd_exotic_optionals();
  }

  late final _new_box_autoadd_exotic_optionalsPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_exotic_optionals> Function()>>(
      'new_box_autoadd_exotic_optionals');
  late final _new_box_autoadd_exotic_optionals =
      _new_box_autoadd_exotic_optionalsPtr
          .asFunction<ffi.Pointer<wire_exotic_optionals> Function()>();

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

  ffi.Pointer<wire_feature_chrono> new_box_autoadd_feature_chrono() {
    return _new_box_autoadd_feature_chrono();
  }

  late final _new_box_autoadd_feature_chronoPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_feature_chrono> Function()>>(
          'new_box_autoadd_feature_chrono');
  late final _new_box_autoadd_feature_chrono =
      _new_box_autoadd_feature_chronoPtr
          .asFunction<ffi.Pointer<wire_feature_chrono> Function()>();

  ffi.Pointer<wire_feature_uuid> new_box_autoadd_feature_uuid() {
    return _new_box_autoadd_feature_uuid();
  }

  late final _new_box_autoadd_feature_uuidPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_feature_uuid> Function()>>(
          'new_box_autoadd_feature_uuid');
  late final _new_box_autoadd_feature_uuid = _new_box_autoadd_feature_uuidPtr
      .asFunction<ffi.Pointer<wire_feature_uuid> Function()>();

  ffi.Pointer<wire_feed_id> new_box_autoadd_feed_id() {
    return _new_box_autoadd_feed_id();
  }

  late final _new_box_autoadd_feed_idPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_feed_id> Function()>>(
          'new_box_autoadd_feed_id');
  late final _new_box_autoadd_feed_id = _new_box_autoadd_feed_idPtr
      .asFunction<ffi.Pointer<wire_feed_id> Function()>();

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

  ffi.Pointer<wire_measure> new_box_autoadd_measure() {
    return _new_box_autoadd_measure();
  }

  late final _new_box_autoadd_measurePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_measure> Function()>>(
          'new_box_autoadd_measure');
  late final _new_box_autoadd_measure = _new_box_autoadd_measurePtr
      .asFunction<ffi.Pointer<wire_measure> Function()>();

  ffi.Pointer<wire_message_id> new_box_autoadd_message_id() {
    return _new_box_autoadd_message_id();
  }

  late final _new_box_autoadd_message_idPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_message_id> Function()>>(
          'new_box_autoadd_message_id');
  late final _new_box_autoadd_message_id = _new_box_autoadd_message_idPtr
      .asFunction<ffi.Pointer<wire_message_id> Function()>();

  ffi.Pointer<wire_my_nested_struct> new_box_autoadd_my_nested_struct() {
    return _new_box_autoadd_my_nested_struct();
  }

  late final _new_box_autoadd_my_nested_structPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_my_nested_struct> Function()>>(
      'new_box_autoadd_my_nested_struct');
  late final _new_box_autoadd_my_nested_struct =
      _new_box_autoadd_my_nested_structPtr
          .asFunction<ffi.Pointer<wire_my_nested_struct> Function()>();

  ffi.Pointer<wire_my_struct> new_box_autoadd_my_struct() {
    return _new_box_autoadd_my_struct();
  }

  late final _new_box_autoadd_my_structPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_my_struct> Function()>>(
          'new_box_autoadd_my_struct');
  late final _new_box_autoadd_my_struct = _new_box_autoadd_my_structPtr
      .asFunction<ffi.Pointer<wire_my_struct> Function()>();

  ffi.Pointer<wire_my_tree_node> new_box_autoadd_my_tree_node() {
    return _new_box_autoadd_my_tree_node();
  }

  late final _new_box_autoadd_my_tree_nodePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_my_tree_node> Function()>>(
          'new_box_autoadd_my_tree_node');
  late final _new_box_autoadd_my_tree_node = _new_box_autoadd_my_tree_nodePtr
      .asFunction<ffi.Pointer<wire_my_tree_node> Function()>();

  ffi.Pointer<wire_new_type_int> new_box_autoadd_new_type_int() {
    return _new_box_autoadd_new_type_int();
  }

  late final _new_box_autoadd_new_type_intPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_new_type_int> Function()>>(
          'new_box_autoadd_new_type_int');
  late final _new_box_autoadd_new_type_int = _new_box_autoadd_new_type_intPtr
      .asFunction<ffi.Pointer<wire_new_type_int> Function()>();

  ffi.Pointer<wire_note> new_box_autoadd_note() {
    return _new_box_autoadd_note();
  }

  late final _new_box_autoadd_notePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_note> Function()>>(
          'new_box_autoadd_note');
  late final _new_box_autoadd_note =
      _new_box_autoadd_notePtr.asFunction<ffi.Pointer<wire_note> Function()>();

  ffi.Pointer<wire_numbers> new_box_autoadd_numbers() {
    return _new_box_autoadd_numbers();
  }

  late final _new_box_autoadd_numbersPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_numbers> Function()>>(
          'new_box_autoadd_numbers');
  late final _new_box_autoadd_numbers = _new_box_autoadd_numbersPtr
      .asFunction<ffi.Pointer<wire_numbers> Function()>();

  ffi.Pointer<wire_opaque_nested> new_box_autoadd_opaque_nested() {
    return _new_box_autoadd_opaque_nested();
  }

  late final _new_box_autoadd_opaque_nestedPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_opaque_nested> Function()>>(
          'new_box_autoadd_opaque_nested');
  late final _new_box_autoadd_opaque_nested = _new_box_autoadd_opaque_nestedPtr
      .asFunction<ffi.Pointer<wire_opaque_nested> Function()>();

  ffi.Pointer<wire_opt_vecs> new_box_autoadd_opt_vecs() {
    return _new_box_autoadd_opt_vecs();
  }

  late final _new_box_autoadd_opt_vecsPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_opt_vecs> Function()>>(
          'new_box_autoadd_opt_vecs');
  late final _new_box_autoadd_opt_vecs = _new_box_autoadd_opt_vecsPtr
      .asFunction<ffi.Pointer<wire_opt_vecs> Function()>();

  ffi.Pointer<wire_record_string_i_32> new_box_autoadd_record_string_i_32() {
    return _new_box_autoadd_record_string_i_32();
  }

  late final _new_box_autoadd_record_string_i_32Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_record_string_i_32> Function()>>(
      'new_box_autoadd_record_string_i_32');
  late final _new_box_autoadd_record_string_i_32 =
      _new_box_autoadd_record_string_i_32Ptr
          .asFunction<ffi.Pointer<wire_record_string_i_32> Function()>();

  ffi.Pointer<wire_sequences> new_box_autoadd_sequences() {
    return _new_box_autoadd_sequences();
  }

  late final _new_box_autoadd_sequencesPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_sequences> Function()>>(
          'new_box_autoadd_sequences');
  late final _new_box_autoadd_sequences = _new_box_autoadd_sequencesPtr
      .asFunction<ffi.Pointer<wire_sequences> Function()>();

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

  ffi.Pointer<wire_struct_with_enum> new_box_autoadd_struct_with_enum() {
    return _new_box_autoadd_struct_with_enum();
  }

  late final _new_box_autoadd_struct_with_enumPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_struct_with_enum> Function()>>(
      'new_box_autoadd_struct_with_enum');
  late final _new_box_autoadd_struct_with_enum =
      _new_box_autoadd_struct_with_enumPtr
          .asFunction<ffi.Pointer<wire_struct_with_enum> Function()>();

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

  ffi.Pointer<wire_sum_with> new_box_autoadd_sum_with() {
    return _new_box_autoadd_sum_with();
  }

  late final _new_box_autoadd_sum_withPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_sum_with> Function()>>(
          'new_box_autoadd_sum_with');
  late final _new_box_autoadd_sum_with = _new_box_autoadd_sum_withPtr
      .asFunction<ffi.Pointer<wire_sum_with> Function()>();

  ffi.Pointer<wire_test_id> new_box_autoadd_test_id() {
    return _new_box_autoadd_test_id();
  }

  late final _new_box_autoadd_test_idPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_test_id> Function()>>(
          'new_box_autoadd_test_id');
  late final _new_box_autoadd_test_id = _new_box_autoadd_test_idPtr
      .asFunction<ffi.Pointer<wire_test_id> Function()>();

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

  ffi.Pointer<wire_user_id> new_box_autoadd_user_id() {
    return _new_box_autoadd_user_id();
  }

  late final _new_box_autoadd_user_idPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_user_id> Function()>>(
          'new_box_autoadd_user_id');
  late final _new_box_autoadd_user_id = _new_box_autoadd_user_idPtr
      .asFunction<ffi.Pointer<wire_user_id> Function()>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_weekdays(
    int value,
  ) {
    return _new_box_autoadd_weekdays(
      value,
    );
  }

  late final _new_box_autoadd_weekdaysPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_weekdays');
  late final _new_box_autoadd_weekdays = _new_box_autoadd_weekdaysPtr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<wire_blob> new_box_blob() {
    return _new_box_blob();
  }

  late final _new_box_blobPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_blob> Function()>>(
          'new_box_blob');
  late final _new_box_blob =
      _new_box_blobPtr.asFunction<ffi.Pointer<wire_blob> Function()>();

  ffi.Pointer<ffi.Bool> new_box_bool(
    bool value,
  ) {
    return _new_box_bool(
      value,
    );
  }

  late final _new_box_boolPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Bool> Function(ffi.Bool)>>(
          'new_box_bool');
  late final _new_box_bool =
      _new_box_boolPtr.asFunction<ffi.Pointer<ffi.Bool> Function(bool)>();

  ffi.Pointer<wire_distance> new_box_distance() {
    return _new_box_distance();
  }

  late final _new_box_distancePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_distance> Function()>>(
          'new_box_distance');
  late final _new_box_distance =
      _new_box_distancePtr.asFunction<ffi.Pointer<wire_distance> Function()>();

  ffi.Pointer<wire_exotic_optionals> new_box_exotic_optionals() {
    return _new_box_exotic_optionals();
  }

  late final _new_box_exotic_optionalsPtr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_exotic_optionals> Function()>>(
      'new_box_exotic_optionals');
  late final _new_box_exotic_optionals = _new_box_exotic_optionalsPtr
      .asFunction<ffi.Pointer<wire_exotic_optionals> Function()>();

  ffi.Pointer<ffi.Double> new_box_f_64(
    double value,
  ) {
    return _new_box_f_64(
      value,
    );
  }

  late final _new_box_f_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Double> Function(ffi.Double)>>(
          'new_box_f_64');
  late final _new_box_f_64 =
      _new_box_f_64Ptr.asFunction<ffi.Pointer<ffi.Double> Function(double)>();

  ffi.Pointer<ffi.Int32> new_box_i_32(
    int value,
  ) {
    return _new_box_i_32(
      value,
    );
  }

  late final _new_box_i_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_i_32');
  late final _new_box_i_32 =
      _new_box_i_32Ptr.asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<ffi.Int64> new_box_i_64(
    int value,
  ) {
    return _new_box_i_64(
      value,
    );
  }

  late final _new_box_i_64Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int64> Function(ffi.Int64)>>(
          'new_box_i_64');
  late final _new_box_i_64 =
      _new_box_i_64Ptr.asFunction<ffi.Pointer<ffi.Int64> Function(int)>();

  ffi.Pointer<ffi.Int8> new_box_i_8(
    int value,
  ) {
    return _new_box_i_8(
      value,
    );
  }

  late final _new_box_i_8Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int8> Function(ffi.Int8)>>(
          'new_box_i_8');
  late final _new_box_i_8 =
      _new_box_i_8Ptr.asFunction<ffi.Pointer<ffi.Int8> Function(int)>();

  ffi.Pointer<wire_speed> new_box_speed() {
    return _new_box_speed();
  }

  late final _new_box_speedPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_speed> Function()>>(
          'new_box_speed');
  late final _new_box_speed =
      _new_box_speedPtr.asFunction<ffi.Pointer<wire_speed> Function()>();

  ffi.Pointer<ffi.Uint8> new_box_u_8(
    int value,
  ) {
    return _new_box_u_8(
      value,
    );
  }

  late final _new_box_u_8Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint8> Function(ffi.Uint8)>>(
          'new_box_u_8');
  late final _new_box_u_8 =
      _new_box_u_8Ptr.asFunction<ffi.Pointer<ffi.Uint8> Function(int)>();

  ffi.Pointer<ffi.Int32> new_box_weekdays(
    int value,
  ) {
    return _new_box_weekdays(
      value,
    );
  }

  late final _new_box_weekdaysPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_weekdays');
  late final _new_box_weekdays =
      _new_box_weekdaysPtr.asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<wire_list_DartOpaque> new_list_DartOpaque(
    int len,
  ) {
    return _new_list_DartOpaque(
      len,
    );
  }

  late final _new_list_DartOpaquePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_DartOpaque> Function(
              ffi.Int32)>>('new_list_DartOpaque');
  late final _new_list_DartOpaque = _new_list_DartOpaquePtr
      .asFunction<ffi.Pointer<wire_list_DartOpaque> Function(int)>();

  ffi.Pointer<wire_list_RustOpaque_hide_data> new_list_RustOpaque_hide_data(
    int len,
  ) {
    return _new_list_RustOpaque_hide_data(
      len,
    );
  }

  late final _new_list_RustOpaque_hide_dataPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_RustOpaque_hide_data> Function(
              ffi.Int32)>>('new_list_RustOpaque_hide_data');
  late final _new_list_RustOpaque_hide_data = _new_list_RustOpaque_hide_dataPtr
      .asFunction<ffi.Pointer<wire_list_RustOpaque_hide_data> Function(int)>();

  ffi.Pointer<wire_list_application_env_var> new_list_application_env_var(
    int len,
  ) {
    return _new_list_application_env_var(
      len,
    );
  }

  late final _new_list_application_env_varPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_application_env_var> Function(
              ffi.Int32)>>('new_list_application_env_var');
  late final _new_list_application_env_var = _new_list_application_env_varPtr
      .asFunction<ffi.Pointer<wire_list_application_env_var> Function(int)>();

  ffi.Pointer<wire_list_attribute> new_list_attribute(
    int len,
  ) {
    return _new_list_attribute(
      len,
    );
  }

  late final _new_list_attributePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_attribute> Function(
              ffi.Int32)>>('new_list_attribute');
  late final _new_list_attribute = _new_list_attributePtr
      .asFunction<ffi.Pointer<wire_list_attribute> Function(int)>();

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

  ffi.Pointer<wire_list_my_size> new_list_my_size(
    int len,
  ) {
    return _new_list_my_size(
      len,
    );
  }

  late final _new_list_my_sizePtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_list_my_size> Function(ffi.Int32)>>(
      'new_list_my_size');
  late final _new_list_my_size = _new_list_my_sizePtr
      .asFunction<ffi.Pointer<wire_list_my_size> Function(int)>();

  ffi.Pointer<wire_list_my_tree_node> new_list_my_tree_node(
    int len,
  ) {
    return _new_list_my_tree_node(
      len,
    );
  }

  late final _new_list_my_tree_nodePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_my_tree_node> Function(
              ffi.Int32)>>('new_list_my_tree_node');
  late final _new_list_my_tree_node = _new_list_my_tree_nodePtr
      .asFunction<ffi.Pointer<wire_list_my_tree_node> Function(int)>();

  ffi.Pointer<wire_list_opt_String> new_list_opt_String(
    int len,
  ) {
    return _new_list_opt_String(
      len,
    );
  }

  late final _new_list_opt_StringPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_opt_String> Function(
              ffi.Int32)>>('new_list_opt_String');
  late final _new_list_opt_String = _new_list_opt_StringPtr
      .asFunction<ffi.Pointer<wire_list_opt_String> Function(int)>();

  ffi.Pointer<wire_list_opt_box_autoadd_attribute>
      new_list_opt_box_autoadd_attribute(
    int len,
  ) {
    return _new_list_opt_box_autoadd_attribute(
      len,
    );
  }

  late final _new_list_opt_box_autoadd_attributePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_opt_box_autoadd_attribute> Function(
              ffi.Int32)>>('new_list_opt_box_autoadd_attribute');
  late final _new_list_opt_box_autoadd_attribute =
      _new_list_opt_box_autoadd_attributePtr.asFunction<
          ffi.Pointer<wire_list_opt_box_autoadd_attribute> Function(int)>();

  ffi.Pointer<wire_list_opt_box_autoadd_i_32> new_list_opt_box_autoadd_i_32(
    int len,
  ) {
    return _new_list_opt_box_autoadd_i_32(
      len,
    );
  }

  late final _new_list_opt_box_autoadd_i_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_opt_box_autoadd_i_32> Function(
              ffi.Int32)>>('new_list_opt_box_autoadd_i_32');
  late final _new_list_opt_box_autoadd_i_32 = _new_list_opt_box_autoadd_i_32Ptr
      .asFunction<ffi.Pointer<wire_list_opt_box_autoadd_i_32> Function(int)>();

  ffi.Pointer<wire_list_opt_box_autoadd_weekdays>
      new_list_opt_box_autoadd_weekdays(
    int len,
  ) {
    return _new_list_opt_box_autoadd_weekdays(
      len,
    );
  }

  late final _new_list_opt_box_autoadd_weekdaysPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_opt_box_autoadd_weekdays> Function(
              ffi.Int32)>>('new_list_opt_box_autoadd_weekdays');
  late final _new_list_opt_box_autoadd_weekdays =
      _new_list_opt_box_autoadd_weekdaysPtr.asFunction<
          ffi.Pointer<wire_list_opt_box_autoadd_weekdays> Function(int)>();

  ffi.Pointer<wire_list_opt_list_prim_i_32> new_list_opt_list_prim_i_32(
    int len,
  ) {
    return _new_list_opt_list_prim_i_32(
      len,
    );
  }

  late final _new_list_opt_list_prim_i_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_opt_list_prim_i_32> Function(
              ffi.Int32)>>('new_list_opt_list_prim_i_32');
  late final _new_list_opt_list_prim_i_32 = _new_list_opt_list_prim_i_32Ptr
      .asFunction<ffi.Pointer<wire_list_opt_list_prim_i_32> Function(int)>();

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

  ffi.Pointer<wire_list_record_string_i_32> new_list_record_string_i_32(
    int len,
  ) {
    return _new_list_record_string_i_32(
      len,
    );
  }

  late final _new_list_record_string_i_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_record_string_i_32> Function(
              ffi.Int32)>>('new_list_record_string_i_32');
  late final _new_list_record_string_i_32 = _new_list_record_string_i_32Ptr
      .asFunction<ffi.Pointer<wire_list_record_string_i_32> Function(int)>();

  ffi.Pointer<wire_list_test_id> new_list_test_id(
    int len,
  ) {
    return _new_list_test_id(
      len,
    );
  }

  late final _new_list_test_idPtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_list_test_id> Function(ffi.Int32)>>(
      'new_list_test_id');
  late final _new_list_test_id = _new_list_test_idPtr
      .asFunction<ffi.Pointer<wire_list_test_id> Function(int)>();

  ffi.Pointer<wire_list_weekdays> new_list_weekdays(
    int len,
  ) {
    return _new_list_weekdays(
      len,
    );
  }

  late final _new_list_weekdaysPtr = _lookup<
          ffi
          .NativeFunction<ffi.Pointer<wire_list_weekdays> Function(ffi.Int32)>>(
      'new_list_weekdays');
  late final _new_list_weekdays = _new_list_weekdaysPtr
      .asFunction<ffi.Pointer<wire_list_weekdays> Function(int)>();

  void drop_opaque_RustOpaque_MutexHideData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_MutexHideData(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_MutexHideDataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_MutexHideData');
  late final _drop_opaque_RustOpaque_MutexHideData =
      _drop_opaque_RustOpaque_MutexHideDataPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_MutexHideData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_MutexHideData(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_MutexHideDataPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RustOpaque_MutexHideData');
  late final _share_opaque_RustOpaque_MutexHideData =
      _share_opaque_RustOpaque_MutexHideDataPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_RwLockHideData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_RwLockHideData(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_RwLockHideDataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_RwLockHideData');
  late final _drop_opaque_RustOpaque_RwLockHideData =
      _drop_opaque_RustOpaque_RwLockHideDataPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_RwLockHideData(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_RwLockHideData(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_RwLockHideDataPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>(
      'share_opaque_RustOpaque_RwLockHideData');
  late final _share_opaque_RustOpaque_RwLockHideData =
      _share_opaque_RustOpaque_RwLockHideDataPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_box_dynDartDebug(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_box_dynDartDebug(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_box_dynDartDebugPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_box_dynDartDebug');
  late final _drop_opaque_RustOpaque_box_dynDartDebug =
      _drop_opaque_RustOpaque_box_dynDartDebugPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_box_dynDartDebug(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_box_dynDartDebug(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_box_dynDartDebugPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>(
      'share_opaque_RustOpaque_box_dynDartDebug');
  late final _share_opaque_RustOpaque_box_dynDartDebug =
      _share_opaque_RustOpaque_box_dynDartDebugPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_frb_opaque_return(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_frb_opaque_return(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_frb_opaque_returnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_frb_opaque_return');
  late final _drop_opaque_RustOpaque_frb_opaque_return =
      _drop_opaque_RustOpaque_frb_opaque_returnPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_frb_opaque_return(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_frb_opaque_return(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_frb_opaque_returnPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>(
      'share_opaque_RustOpaque_frb_opaque_return');
  late final _share_opaque_RustOpaque_frb_opaque_return =
      _share_opaque_RustOpaque_frb_opaque_returnPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_frb_opaque_sync_return(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_frb_opaque_sync_return(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_frb_opaque_sync_returnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_frb_opaque_sync_return');
  late final _drop_opaque_RustOpaque_frb_opaque_sync_return =
      _drop_opaque_RustOpaque_frb_opaque_sync_returnPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_frb_opaque_sync_return(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_frb_opaque_sync_return(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_frb_opaque_sync_returnPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>(
      'share_opaque_RustOpaque_frb_opaque_sync_return');
  late final _share_opaque_RustOpaque_frb_opaque_sync_return =
      _share_opaque_RustOpaque_frb_opaque_sync_returnPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_hide_data(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_hide_data(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_hide_dataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_hide_data');
  late final _drop_opaque_RustOpaque_hide_data =
      _drop_opaque_RustOpaque_hide_dataPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_hide_data(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_hide_data(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_hide_dataPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RustOpaque_hide_data');
  late final _share_opaque_RustOpaque_hide_data =
      _share_opaque_RustOpaque_hide_dataPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_i_32(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_i_32(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_i_32Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_i_32');
  late final _drop_opaque_RustOpaque_i_32 = _drop_opaque_RustOpaque_i_32Ptr
      .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_i_32(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_i_32(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_i_32Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<ffi.Void> Function(
              ffi.Pointer<ffi.Void>)>>('share_opaque_RustOpaque_i_32');
  late final _share_opaque_RustOpaque_i_32 = _share_opaque_RustOpaque_i_32Ptr
      .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  void drop_opaque_RustOpaque_non_send_hide_data(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _drop_opaque_RustOpaque_non_send_hide_data(
      ptr,
    );
  }

  late final _drop_opaque_RustOpaque_non_send_hide_dataPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'drop_opaque_RustOpaque_non_send_hide_data');
  late final _drop_opaque_RustOpaque_non_send_hide_data =
      _drop_opaque_RustOpaque_non_send_hide_dataPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<ffi.Void> share_opaque_RustOpaque_non_send_hide_data(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _share_opaque_RustOpaque_non_send_hide_data(
      ptr,
    );
  }

  late final _share_opaque_RustOpaque_non_send_hide_dataPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>>(
      'share_opaque_RustOpaque_non_send_hide_data');
  late final _share_opaque_RustOpaque_non_send_hide_data =
      _share_opaque_RustOpaque_non_send_hide_dataPtr
          .asFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<AbcKind> inflate_Abc_A() {
    return _inflate_Abc_A();
  }

  late final _inflate_Abc_APtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<AbcKind> Function()>>(
          'inflate_Abc_A');
  late final _inflate_Abc_A =
      _inflate_Abc_APtr.asFunction<ffi.Pointer<AbcKind> Function()>();

  ffi.Pointer<AbcKind> inflate_Abc_B() {
    return _inflate_Abc_B();
  }

  late final _inflate_Abc_BPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<AbcKind> Function()>>(
          'inflate_Abc_B');
  late final _inflate_Abc_B =
      _inflate_Abc_BPtr.asFunction<ffi.Pointer<AbcKind> Function()>();

  ffi.Pointer<AbcKind> inflate_Abc_C() {
    return _inflate_Abc_C();
  }

  late final _inflate_Abc_CPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<AbcKind> Function()>>(
          'inflate_Abc_C');
  late final _inflate_Abc_C =
      _inflate_Abc_CPtr.asFunction<ffi.Pointer<AbcKind> Function()>();

  ffi.Pointer<AbcKind> inflate_Abc_JustInt() {
    return _inflate_Abc_JustInt();
  }

  late final _inflate_Abc_JustIntPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<AbcKind> Function()>>(
          'inflate_Abc_JustInt');
  late final _inflate_Abc_JustInt =
      _inflate_Abc_JustIntPtr.asFunction<ffi.Pointer<AbcKind> Function()>();

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

  ffi.Pointer<CustomNestedErrorInnerTwinSyncKind>
      inflate_CustomNestedErrorInnerTwinSync_Three() {
    return _inflate_CustomNestedErrorInnerTwinSync_Three();
  }

  late final _inflate_CustomNestedErrorInnerTwinSync_ThreePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinSyncKind>
              Function()>>('inflate_CustomNestedErrorInnerTwinSync_Three');
  late final _inflate_CustomNestedErrorInnerTwinSync_Three =
      _inflate_CustomNestedErrorInnerTwinSync_ThreePtr.asFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinSyncKind> Function()>();

  ffi.Pointer<CustomNestedErrorInnerTwinSyncKind>
      inflate_CustomNestedErrorInnerTwinSync_Four() {
    return _inflate_CustomNestedErrorInnerTwinSync_Four();
  }

  late final _inflate_CustomNestedErrorInnerTwinSync_FourPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinSyncKind>
              Function()>>('inflate_CustomNestedErrorInnerTwinSync_Four');
  late final _inflate_CustomNestedErrorInnerTwinSync_Four =
      _inflate_CustomNestedErrorInnerTwinSync_FourPtr.asFunction<
          ffi.Pointer<CustomNestedErrorInnerTwinSyncKind> Function()>();

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

  ffi.Pointer<CustomNestedErrorOuterTwinSyncKind>
      inflate_CustomNestedErrorOuterTwinSync_One() {
    return _inflate_CustomNestedErrorOuterTwinSync_One();
  }

  late final _inflate_CustomNestedErrorOuterTwinSync_OnePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinSyncKind>
              Function()>>('inflate_CustomNestedErrorOuterTwinSync_One');
  late final _inflate_CustomNestedErrorOuterTwinSync_One =
      _inflate_CustomNestedErrorOuterTwinSync_OnePtr.asFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinSyncKind> Function()>();

  ffi.Pointer<CustomNestedErrorOuterTwinSyncKind>
      inflate_CustomNestedErrorOuterTwinSync_Two() {
    return _inflate_CustomNestedErrorOuterTwinSync_Two();
  }

  late final _inflate_CustomNestedErrorOuterTwinSync_TwoPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinSyncKind>
              Function()>>('inflate_CustomNestedErrorOuterTwinSync_Two');
  late final _inflate_CustomNestedErrorOuterTwinSync_Two =
      _inflate_CustomNestedErrorOuterTwinSync_TwoPtr.asFunction<
          ffi.Pointer<CustomNestedErrorOuterTwinSyncKind> Function()>();

  ffi.Pointer<DistanceKind> inflate_Distance_Map() {
    return _inflate_Distance_Map();
  }

  late final _inflate_Distance_MapPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<DistanceKind> Function()>>(
          'inflate_Distance_Map');
  late final _inflate_Distance_Map = _inflate_Distance_MapPtr
      .asFunction<ffi.Pointer<DistanceKind> Function()>();

  ffi.Pointer<EnumDartOpaqueKind> inflate_EnumDartOpaque_Primitive() {
    return _inflate_EnumDartOpaque_Primitive();
  }

  late final _inflate_EnumDartOpaque_PrimitivePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumDartOpaqueKind> Function()>>(
          'inflate_EnumDartOpaque_Primitive');
  late final _inflate_EnumDartOpaque_Primitive =
      _inflate_EnumDartOpaque_PrimitivePtr
          .asFunction<ffi.Pointer<EnumDartOpaqueKind> Function()>();

  ffi.Pointer<EnumDartOpaqueKind> inflate_EnumDartOpaque_Opaque() {
    return _inflate_EnumDartOpaque_Opaque();
  }

  late final _inflate_EnumDartOpaque_OpaquePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumDartOpaqueKind> Function()>>(
          'inflate_EnumDartOpaque_Opaque');
  late final _inflate_EnumDartOpaque_Opaque = _inflate_EnumDartOpaque_OpaquePtr
      .asFunction<ffi.Pointer<EnumDartOpaqueKind> Function()>();

  ffi.Pointer<EnumOpaqueKind> inflate_EnumOpaque_Struct() {
    return _inflate_EnumOpaque_Struct();
  }

  late final _inflate_EnumOpaque_StructPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumOpaqueKind> Function()>>(
          'inflate_EnumOpaque_Struct');
  late final _inflate_EnumOpaque_Struct = _inflate_EnumOpaque_StructPtr
      .asFunction<ffi.Pointer<EnumOpaqueKind> Function()>();

  ffi.Pointer<EnumOpaqueKind> inflate_EnumOpaque_Primitive() {
    return _inflate_EnumOpaque_Primitive();
  }

  late final _inflate_EnumOpaque_PrimitivePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumOpaqueKind> Function()>>(
          'inflate_EnumOpaque_Primitive');
  late final _inflate_EnumOpaque_Primitive = _inflate_EnumOpaque_PrimitivePtr
      .asFunction<ffi.Pointer<EnumOpaqueKind> Function()>();

  ffi.Pointer<EnumOpaqueKind> inflate_EnumOpaque_TraitObj() {
    return _inflate_EnumOpaque_TraitObj();
  }

  late final _inflate_EnumOpaque_TraitObjPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumOpaqueKind> Function()>>(
          'inflate_EnumOpaque_TraitObj');
  late final _inflate_EnumOpaque_TraitObj = _inflate_EnumOpaque_TraitObjPtr
      .asFunction<ffi.Pointer<EnumOpaqueKind> Function()>();

  ffi.Pointer<EnumOpaqueKind> inflate_EnumOpaque_Mutex() {
    return _inflate_EnumOpaque_Mutex();
  }

  late final _inflate_EnumOpaque_MutexPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumOpaqueKind> Function()>>(
          'inflate_EnumOpaque_Mutex');
  late final _inflate_EnumOpaque_Mutex = _inflate_EnumOpaque_MutexPtr
      .asFunction<ffi.Pointer<EnumOpaqueKind> Function()>();

  ffi.Pointer<EnumOpaqueKind> inflate_EnumOpaque_RwLock() {
    return _inflate_EnumOpaque_RwLock();
  }

  late final _inflate_EnumOpaque_RwLockPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<EnumOpaqueKind> Function()>>(
          'inflate_EnumOpaque_RwLock');
  late final _inflate_EnumOpaque_RwLock = _inflate_EnumOpaque_RwLockPtr
      .asFunction<ffi.Pointer<EnumOpaqueKind> Function()>();

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

  ffi.Pointer<MeasureKind> inflate_Measure_Speed() {
    return _inflate_Measure_Speed();
  }

  late final _inflate_Measure_SpeedPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<MeasureKind> Function()>>(
          'inflate_Measure_Speed');
  late final _inflate_Measure_Speed = _inflate_Measure_SpeedPtr
      .asFunction<ffi.Pointer<MeasureKind> Function()>();

  ffi.Pointer<MeasureKind> inflate_Measure_Distance() {
    return _inflate_Measure_Distance();
  }

  late final _inflate_Measure_DistancePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<MeasureKind> Function()>>(
          'inflate_Measure_Distance');
  late final _inflate_Measure_Distance = _inflate_Measure_DistancePtr
      .asFunction<ffi.Pointer<MeasureKind> Function()>();

  ffi.Pointer<SpeedKind> inflate_Speed_GPS() {
    return _inflate_Speed_GPS();
  }

  late final _inflate_Speed_GPSPtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<SpeedKind> Function()>>(
          'inflate_Speed_GPS');
  late final _inflate_Speed_GPS =
      _inflate_Speed_GPSPtr.asFunction<ffi.Pointer<SpeedKind> Function()>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>(
          'dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

final class wire_list_prim_u_8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_test_id extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> field0;
}

final class wire_list_prim_f_64 extends ffi.Struct {
  external ffi.Pointer<ffi.Double> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_test_id extends ffi.Struct {
  external ffi.Pointer<wire_test_id> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_feed_id extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_blob extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_message_id extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_customized extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> final_field;

  external ffi.Pointer<wire_list_prim_u_8> non_final_field;
}

final class wire_user_id extends ffi.Struct {
  @ffi.Uint32()
  external int value;
}

final class wire_list_prim_i_64 extends ffi.Struct {
  external ffi.Pointer<ffi.Int64> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_feature_chrono extends ffi.Struct {
  @ffi.Int64()
  external int utc;

  @ffi.Int64()
  external int local;

  @ffi.Int64()
  external int duration;

  @ffi.Int64()
  external int naive;
}

final class wire_struct_with_comments_twin_normal extends ffi.Struct {
  @ffi.Int32()
  external int field_with_comments;
}

final class wire_DartOpaque extends ffi.Struct {
  @ffi.Int64()
  external int port;

  @ffi.UintPtr()
  external int handle;
}

final class wire_EnumDartOpaque_Primitive extends ffi.Struct {
  @ffi.Int32()
  external int field0;
}

final class wire_EnumDartOpaque_Opaque extends ffi.Struct {
  external wire_DartOpaque field0;
}

final class EnumDartOpaqueKind extends ffi.Union {
  external ffi.Pointer<wire_EnumDartOpaque_Primitive> Primitive;

  external ffi.Pointer<wire_EnumDartOpaque_Opaque> Opaque;
}

final class wire_enum_dart_opaque extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumDartOpaqueKind> kind;
}

final class wire_dart_opaque_nested extends ffi.Struct {
  external wire_DartOpaque first;

  external wire_DartOpaque second;
}

final class wire_list_DartOpaque extends ffi.Struct {
  external ffi.Pointer<wire_DartOpaque> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_EnumWithItemMixedTwinNormal_A extends ffi.Opaque {}

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

final class wire_Speed_Unknown extends ffi.Opaque {}

final class wire_Speed_GPS extends ffi.Struct {
  @ffi.Double()
  external double field0;
}

final class SpeedKind extends ffi.Union {
  external ffi.Pointer<wire_Speed_Unknown> Unknown;

  external ffi.Pointer<wire_Speed_GPS> GPS;
}

final class wire_speed extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<SpeedKind> kind;
}

final class wire_Measure_Speed extends ffi.Struct {
  external ffi.Pointer<wire_speed> field0;
}

final class wire_Distance_Unknown extends ffi.Opaque {}

final class wire_Distance_Map extends ffi.Struct {
  @ffi.Double()
  external double field0;
}

final class DistanceKind extends ffi.Union {
  external ffi.Pointer<wire_Distance_Unknown> Unknown;

  external ffi.Pointer<wire_Distance_Map> Map;
}

final class wire_distance extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<DistanceKind> kind;
}

final class wire_Measure_Distance extends ffi.Struct {
  external ffi.Pointer<wire_distance> field0;
}

final class MeasureKind extends ffi.Union {
  external ffi.Pointer<wire_Measure_Speed> Speed;

  external ffi.Pointer<wire_Measure_Distance> Distance;
}

final class wire_measure extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<MeasureKind> kind;
}

final class wire_note extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> day;

  external ffi.Pointer<wire_list_prim_u_8> body;
}

final class wire_event extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> address;

  external ffi.Pointer<wire_list_prim_u_8> payload;
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

final class wire_my_struct extends ffi.Struct {
  @ffi.Bool()
  external bool content;
}

final class wire_macro_struct extends ffi.Struct {
  @ffi.Int32()
  external int data;
}

final class wire_concatenate_with extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a;
}

final class wire_sum_with extends ffi.Struct {
  @ffi.Uint32()
  external int x;
}

final class wire_numbers extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> field0;
}

final class wire_sequences extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_i_32> field0;
}

final class wire_application_env_var extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;

  @ffi.Bool()
  external bool field1;
}

final class wire_list_application_env_var extends ffi.Struct {
  external ffi.Pointer<wire_application_env_var> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_application_env extends ffi.Struct {
  external ffi.Pointer<wire_list_application_env_var> vars;
}

final class wire_application_settings extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> name;

  external ffi.Pointer<wire_list_prim_u_8> version;

  @ffi.Int32()
  external int mode;

  external ffi.Pointer<wire_application_env> env;

  external ffi.Pointer<wire_application_env> env_optional;
}

final class wire_list_my_tree_node extends ffi.Struct {
  external ffi.Pointer<wire_my_tree_node> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_my_tree_node extends ffi.Struct {
  @ffi.Int32()
  external int value_i32;

  external ffi.Pointer<wire_list_prim_u_8> value_vec_u8;

  @ffi.Bool()
  external bool value_boolean;

  external ffi.Pointer<wire_list_my_tree_node> children;
}

final class wire_my_nested_struct extends ffi.Struct {
  external wire_my_tree_node tree_node;

  @ffi.Int32()
  external int weekday;
}

final class wire_list_weekdays extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_a extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a;
}

final class wire_Abc_A extends ffi.Struct {
  external ffi.Pointer<wire_a> field0;
}

final class wire_b extends ffi.Struct {
  @ffi.Int32()
  external int b;
}

final class wire_Abc_B extends ffi.Struct {
  external ffi.Pointer<wire_b> field0;
}

final class wire_c extends ffi.Struct {
  @ffi.Bool()
  external bool c;
}

final class wire_Abc_C extends ffi.Struct {
  external ffi.Pointer<wire_c> field0;
}

final class wire_Abc_JustInt extends ffi.Struct {
  @ffi.Int32()
  external int field0;
}

final class AbcKind extends ffi.Union {
  external ffi.Pointer<wire_Abc_A> A;

  external ffi.Pointer<wire_Abc_B> B;

  external ffi.Pointer<wire_Abc_C> C;

  external ffi.Pointer<wire_Abc_JustInt> JustInt;
}

final class wire_abc extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<AbcKind> kind;
}

final class wire_struct_with_enum extends ffi.Struct {
  external wire_abc abc1;

  external wire_abc abc2;
}

final class wire_my_size extends ffi.Struct {
  @ffi.Int32()
  external int width;

  @ffi.Int32()
  external int height;
}

final class wire_list_my_size extends ffi.Struct {
  external ffi.Pointer<wire_my_size> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_StringList extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_list_prim_u_8>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_new_type_int extends ffi.Struct {
  @ffi.Int64()
  external int field0;
}

final class wire_list_prim_i_8 extends ffi.Struct {
  external ffi.Pointer<ffi.Int8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_f_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Float> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_attribute extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> key;

  external ffi.Pointer<wire_list_prim_u_8> value;
}

final class wire_list_attribute extends ffi.Struct {
  external ffi.Pointer<wire_attribute> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_opt_box_autoadd_attribute extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_attribute>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_exotic_optionals extends ffi.Struct {
  external ffi.Pointer<ffi.Int32> int32;

  external ffi.Pointer<ffi.Int64> int64;

  external ffi.Pointer<ffi.Double> float64;

  external ffi.Pointer<ffi.Bool> boolean;

  external ffi.Pointer<wire_list_prim_u_8> zerocopy;

  external ffi.Pointer<wire_list_prim_i_8> int8list;

  external ffi.Pointer<wire_list_prim_u_8> uint8list;

  external ffi.Pointer<wire_list_prim_i_32> int32list;

  external ffi.Pointer<wire_list_prim_f_32> float32list;

  external ffi.Pointer<wire_list_prim_f_64> float64list;

  external ffi.Pointer<wire_list_attribute> attributes;

  external ffi.Pointer<wire_list_opt_box_autoadd_attribute> attributes_nullable;

  external ffi.Pointer<wire_list_opt_box_autoadd_attribute> nullable_attributes;

  external ffi.Pointer<wire_new_type_int> newtypeint;
}

final class wire_list_opt_box_autoadd_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<ffi.Int32>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_opt_box_autoadd_weekdays extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<ffi.Int32>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_opt_String extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_list_prim_u_8>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_opt_list_prim_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<wire_list_prim_i_32>> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_opt_vecs extends ffi.Struct {
  external ffi.Pointer<wire_list_opt_box_autoadd_i_32> i32;

  external ffi.Pointer<wire_list_opt_box_autoadd_weekdays> enums;

  external ffi.Pointer<wire_list_opt_String> strings;

  external ffi.Pointer<wire_list_opt_list_prim_i_32> buffers;
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

final class wire_CustomNestedErrorOuterTwinSync_One extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_CustomNestedErrorInnerTwinSync_Three extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;
}

final class wire_CustomNestedErrorInnerTwinSync_Four extends ffi.Struct {
  @ffi.Uint32()
  external int field0;
}

final class CustomNestedErrorInnerTwinSyncKind extends ffi.Union {
  external ffi.Pointer<wire_CustomNestedErrorInnerTwinSync_Three> Three;

  external ffi.Pointer<wire_CustomNestedErrorInnerTwinSync_Four> Four;
}

final class wire_custom_nested_error_inner_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<CustomNestedErrorInnerTwinSyncKind> kind;
}

final class wire_CustomNestedErrorOuterTwinSync_Two extends ffi.Struct {
  external ffi.Pointer<wire_custom_nested_error_inner_twin_sync> field0;
}

final class CustomNestedErrorOuterTwinSyncKind extends ffi.Union {
  external ffi.Pointer<wire_CustomNestedErrorOuterTwinSync_One> One;

  external ffi.Pointer<wire_CustomNestedErrorOuterTwinSync_Two> Two;
}

final class wire_custom_nested_error_outer_twin_sync extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<CustomNestedErrorOuterTwinSyncKind> kind;
}

final class wire_custom_struct_error_twin_sync extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> a;
}

final class wire_list_bool extends ffi.Struct {
  external ffi.Pointer<ffi.Bool> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_list_prim_i_16 extends ffi.Struct {
  external ffi.Pointer<ffi.Int16> ptr;

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

final class wire_RustOpaque_hide_data extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_list_RustOpaque_hide_data extends ffi.Struct {
  external ffi.Pointer<wire_RustOpaque_hide_data> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_EnumOpaque_Struct extends ffi.Struct {
  external wire_RustOpaque_hide_data field0;
}

final class wire_RustOpaque_i_32 extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_EnumOpaque_Primitive extends ffi.Struct {
  external wire_RustOpaque_i_32 field0;
}

final class wire_RustOpaque_box_dynDartDebug extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_EnumOpaque_TraitObj extends ffi.Struct {
  external wire_RustOpaque_box_dynDartDebug field0;
}

final class wire_RustOpaque_MutexHideData extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_EnumOpaque_Mutex extends ffi.Struct {
  external wire_RustOpaque_MutexHideData field0;
}

final class wire_RustOpaque_RwLockHideData extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_EnumOpaque_RwLock extends ffi.Struct {
  external wire_RustOpaque_RwLockHideData field0;
}

final class EnumOpaqueKind extends ffi.Union {
  external ffi.Pointer<wire_EnumOpaque_Struct> Struct;

  external ffi.Pointer<wire_EnumOpaque_Primitive> Primitive;

  external ffi.Pointer<wire_EnumOpaque_TraitObj> TraitObj;

  external ffi.Pointer<wire_EnumOpaque_Mutex> Mutex;

  external ffi.Pointer<wire_EnumOpaque_RwLock> RwLock;
}

final class wire_enum_opaque extends ffi.Struct {
  @ffi.Int32()
  external int tag;

  external ffi.Pointer<EnumOpaqueKind> kind;
}

final class wire_opaque_nested extends ffi.Struct {
  external wire_RustOpaque_hide_data first;

  external wire_RustOpaque_hide_data second;
}

final class wire_RustOpaque_non_clone_data extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

final class wire_RustOpaque_non_send_hide_data extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
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

final class wire_record_string_i_32 extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> field0;

  @ffi.Int32()
  external int field1;
}

final class wire_list_record_string_i_32 extends ffi.Struct {
  external ffi.Pointer<wire_record_string_i_32> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_feature_uuid extends ffi.Struct {
  external ffi.Pointer<wire_list_prim_u_8> one;

  external ffi.Pointer<wire_list_prim_u_8> many;
}
