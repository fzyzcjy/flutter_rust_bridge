#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturn;
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_list_prim_u_8;

typedef struct wire_list_prim_i_32 {
  int32_t *ptr;
  int32_t len;
} wire_list_prim_i_32;

typedef struct wire_test_id_twin_normal {
  struct wire_list_prim_i_32 *field0;
} wire_test_id_twin_normal;

typedef struct wire_list_prim_f_64 {
  double *ptr;
  int32_t len;
} wire_list_prim_f_64;

typedef struct wire_list_test_id_twin_normal {
  struct wire_test_id_twin_normal *ptr;
  int32_t len;
} wire_list_test_id_twin_normal;

typedef struct wire_feed_id_twin_normal {
  struct wire_list_prim_u_8 *field0;
} wire_feed_id_twin_normal;

typedef struct wire_blob_twin_normal {
  struct wire_list_prim_u_8 *field0;
} wire_blob_twin_normal;

typedef struct wire_message_id_twin_normal {
  struct wire_list_prim_u_8 *field0;
} wire_message_id_twin_normal;

typedef struct wire_customized_twin_normal {
  struct wire_list_prim_u_8 *final_field;
  struct wire_list_prim_u_8 *non_final_field;
} wire_customized_twin_normal;

typedef struct wire_user_id_twin_normal {
  uint32_t value;
} wire_user_id_twin_normal;

typedef struct wire_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_list_prim_i_64;

typedef struct wire_feature_chrono_twin_normal {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_feature_chrono_twin_normal;

typedef struct wire_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_normal;

typedef struct wire_DartOpaque {
  int64_t port;
  uintptr_t handle;
} wire_DartOpaque;

typedef struct wire_EnumDartOpaqueTwinNormal_Primitive {
  int32_t field0;
} wire_EnumDartOpaqueTwinNormal_Primitive;

typedef struct wire_EnumDartOpaqueTwinNormal_Opaque {
  struct wire_DartOpaque field0;
} wire_EnumDartOpaqueTwinNormal_Opaque;

typedef union EnumDartOpaqueTwinNormalKind {
  struct wire_EnumDartOpaqueTwinNormal_Primitive *Primitive;
  struct wire_EnumDartOpaqueTwinNormal_Opaque *Opaque;
} EnumDartOpaqueTwinNormalKind;

typedef struct wire_enum_dart_opaque_twin_normal {
  int32_t tag;
  union EnumDartOpaqueTwinNormalKind *kind;
} wire_enum_dart_opaque_twin_normal;

typedef struct wire_dart_opaque_nested_twin_normal {
  struct wire_DartOpaque first;
  struct wire_DartOpaque second;
} wire_dart_opaque_nested_twin_normal;

typedef struct wire_list_DartOpaque {
  struct wire_DartOpaque *ptr;
  int32_t len;
} wire_list_DartOpaque;

typedef struct wire_EnumWithItemMixedTwinNormal_A {

} wire_EnumWithItemMixedTwinNormal_A;

typedef struct wire_EnumWithItemMixedTwinNormal_B {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemMixedTwinNormal_B;

typedef struct wire_EnumWithItemMixedTwinNormal_C {
  struct wire_list_prim_u_8 *c_field;
} wire_EnumWithItemMixedTwinNormal_C;

typedef union EnumWithItemMixedTwinNormalKind {
  struct wire_EnumWithItemMixedTwinNormal_A *A;
  struct wire_EnumWithItemMixedTwinNormal_B *B;
  struct wire_EnumWithItemMixedTwinNormal_C *C;
} EnumWithItemMixedTwinNormalKind;

typedef struct wire_enum_with_item_mixed_twin_normal {
  int32_t tag;
  union EnumWithItemMixedTwinNormalKind *kind;
} wire_enum_with_item_mixed_twin_normal;

typedef struct wire_EnumWithItemStructTwinNormal_A {
  struct wire_list_prim_u_8 *a_field;
} wire_EnumWithItemStructTwinNormal_A;

typedef struct wire_EnumWithItemStructTwinNormal_B {
  struct wire_list_prim_i_32 *b_field;
} wire_EnumWithItemStructTwinNormal_B;

typedef union EnumWithItemStructTwinNormalKind {
  struct wire_EnumWithItemStructTwinNormal_A *A;
  struct wire_EnumWithItemStructTwinNormal_B *B;
} EnumWithItemStructTwinNormalKind;

typedef struct wire_enum_with_item_struct_twin_normal {
  int32_t tag;
  union EnumWithItemStructTwinNormalKind *kind;
} wire_enum_with_item_struct_twin_normal;

typedef struct wire_EnumWithItemTupleTwinNormal_A {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemTupleTwinNormal_A;

typedef struct wire_EnumWithItemTupleTwinNormal_B {
  struct wire_list_prim_i_32 *field0;
} wire_EnumWithItemTupleTwinNormal_B;

typedef union EnumWithItemTupleTwinNormalKind {
  struct wire_EnumWithItemTupleTwinNormal_A *A;
  struct wire_EnumWithItemTupleTwinNormal_B *B;
} EnumWithItemTupleTwinNormalKind;

typedef struct wire_enum_with_item_tuple_twin_normal {
  int32_t tag;
  union EnumWithItemTupleTwinNormalKind *kind;
} wire_enum_with_item_tuple_twin_normal;

typedef struct wire_KitchenSinkTwinNormal_Empty {

} wire_KitchenSinkTwinNormal_Empty;

typedef struct wire_KitchenSinkTwinNormal_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_KitchenSinkTwinNormal_Primitives;

typedef struct wire_KitchenSinkTwinNormal_Nested {
  int32_t field0;
  struct wire_kitchen_sink_twin_normal *field1;
} wire_KitchenSinkTwinNormal_Nested;

typedef struct wire_KitchenSinkTwinNormal_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_KitchenSinkTwinNormal_Optional;

typedef struct wire_KitchenSinkTwinNormal_Buffer {
  struct wire_list_prim_u_8 *field0;
} wire_KitchenSinkTwinNormal_Buffer;

typedef struct wire_KitchenSinkTwinNormal_Enums {
  int32_t field0;
} wire_KitchenSinkTwinNormal_Enums;

typedef union KitchenSinkTwinNormalKind {
  struct wire_KitchenSinkTwinNormal_Empty *Empty;
  struct wire_KitchenSinkTwinNormal_Primitives *Primitives;
  struct wire_KitchenSinkTwinNormal_Nested *Nested;
  struct wire_KitchenSinkTwinNormal_Optional *Optional;
  struct wire_KitchenSinkTwinNormal_Buffer *Buffer;
  struct wire_KitchenSinkTwinNormal_Enums *Enums;
} KitchenSinkTwinNormalKind;

typedef struct wire_kitchen_sink_twin_normal {
  int32_t tag;
  union KitchenSinkTwinNormalKind *kind;
} wire_kitchen_sink_twin_normal;

typedef struct wire_SpeedTwinNormal_Unknown {

} wire_SpeedTwinNormal_Unknown;

typedef struct wire_SpeedTwinNormal_GPS {
  double field0;
} wire_SpeedTwinNormal_GPS;

typedef union SpeedTwinNormalKind {
  struct wire_SpeedTwinNormal_Unknown *Unknown;
  struct wire_SpeedTwinNormal_GPS *GPS;
} SpeedTwinNormalKind;

typedef struct wire_speed_twin_normal {
  int32_t tag;
  union SpeedTwinNormalKind *kind;
} wire_speed_twin_normal;

typedef struct wire_MeasureTwinNormal_Speed {
  struct wire_speed_twin_normal *field0;
} wire_MeasureTwinNormal_Speed;

typedef struct wire_DistanceTwinNormal_Unknown {

} wire_DistanceTwinNormal_Unknown;

typedef struct wire_DistanceTwinNormal_Map {
  double field0;
} wire_DistanceTwinNormal_Map;

typedef union DistanceTwinNormalKind {
  struct wire_DistanceTwinNormal_Unknown *Unknown;
  struct wire_DistanceTwinNormal_Map *Map;
} DistanceTwinNormalKind;

typedef struct wire_distance_twin_normal {
  int32_t tag;
  union DistanceTwinNormalKind *kind;
} wire_distance_twin_normal;

typedef struct wire_MeasureTwinNormal_Distance {
  struct wire_distance_twin_normal *field0;
} wire_MeasureTwinNormal_Distance;

typedef union MeasureTwinNormalKind {
  struct wire_MeasureTwinNormal_Speed *Speed;
  struct wire_MeasureTwinNormal_Distance *Distance;
} MeasureTwinNormalKind;

typedef struct wire_measure_twin_normal {
  int32_t tag;
  union MeasureTwinNormalKind *kind;
} wire_measure_twin_normal;

typedef struct wire_note_twin_normal {
  int32_t *day;
  struct wire_list_prim_u_8 *body;
} wire_note_twin_normal;

typedef struct wire_event_twin_normal {
  struct wire_list_prim_u_8 *address;
  struct wire_list_prim_u_8 *payload;
} wire_event_twin_normal;

typedef struct wire_custom_struct_twin_normal {
  struct wire_list_prim_u_8 *message;
} wire_custom_struct_twin_normal;

typedef struct wire_some_struct_twin_normal {
  uint32_t value;
} wire_some_struct_twin_normal;

typedef struct wire_CustomNestedErrorOuterTwinNormal_One {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorOuterTwinNormal_One;

typedef struct wire_CustomNestedErrorInnerTwinNormal_Three {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorInnerTwinNormal_Three;

typedef struct wire_CustomNestedErrorInnerTwinNormal_Four {
  uint32_t field0;
} wire_CustomNestedErrorInnerTwinNormal_Four;

typedef union CustomNestedErrorInnerTwinNormalKind {
  struct wire_CustomNestedErrorInnerTwinNormal_Three *Three;
  struct wire_CustomNestedErrorInnerTwinNormal_Four *Four;
} CustomNestedErrorInnerTwinNormalKind;

typedef struct wire_custom_nested_error_inner_twin_normal {
  int32_t tag;
  union CustomNestedErrorInnerTwinNormalKind *kind;
} wire_custom_nested_error_inner_twin_normal;

typedef struct wire_CustomNestedErrorOuterTwinNormal_Two {
  struct wire_custom_nested_error_inner_twin_normal *field0;
} wire_CustomNestedErrorOuterTwinNormal_Two;

typedef union CustomNestedErrorOuterTwinNormalKind {
  struct wire_CustomNestedErrorOuterTwinNormal_One *One;
  struct wire_CustomNestedErrorOuterTwinNormal_Two *Two;
} CustomNestedErrorOuterTwinNormalKind;

typedef struct wire_custom_nested_error_outer_twin_normal {
  int32_t tag;
  union CustomNestedErrorOuterTwinNormalKind *kind;
} wire_custom_nested_error_outer_twin_normal;

typedef struct wire_custom_struct_error_twin_normal {
  struct wire_list_prim_u_8 *a;
} wire_custom_struct_error_twin_normal;

typedef struct wire_my_struct {
  bool content;
} wire_my_struct;

typedef struct wire_macro_struct {
  int32_t data;
} wire_macro_struct;

typedef struct wire_concatenate_with_twin_normal {
  struct wire_list_prim_u_8 *a;
} wire_concatenate_with_twin_normal;

typedef struct wire_sum_with_twin_normal {
  uint32_t x;
} wire_sum_with_twin_normal;

typedef struct wire_numbers {
  struct wire_list_prim_i_32 *field0;
} wire_numbers;

typedef struct wire_sequences {
  struct wire_list_prim_i_32 *field0;
} wire_sequences;

typedef struct wire_application_env_var {
  struct wire_list_prim_u_8 *field0;
  bool field1;
} wire_application_env_var;

typedef struct wire_list_application_env_var {
  struct wire_application_env_var *ptr;
  int32_t len;
} wire_list_application_env_var;

typedef struct wire_application_env {
  struct wire_list_application_env_var *vars;
} wire_application_env;

typedef struct wire_application_settings {
  struct wire_list_prim_u_8 *name;
  struct wire_list_prim_u_8 *version;
  int32_t mode;
  struct wire_application_env *env;
  struct wire_application_env *env_optional;
} wire_application_settings;

typedef struct wire_list_my_tree_node_twin_normal {
  struct wire_my_tree_node_twin_normal *ptr;
  int32_t len;
} wire_list_my_tree_node_twin_normal;

typedef struct wire_my_tree_node_twin_normal {
  int32_t value_i32;
  struct wire_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_list_my_tree_node_twin_normal *children;
} wire_my_tree_node_twin_normal;

typedef struct wire_my_nested_struct_twin_normal {
  struct wire_my_tree_node_twin_normal tree_node;
  int32_t weekday;
} wire_my_nested_struct_twin_normal;

typedef struct wire_my_size_freezed_twin_normal {
  int32_t width;
  int32_t height;
} wire_my_size_freezed_twin_normal;

typedef struct wire_my_size {
  int32_t width;
  int32_t height;
} wire_my_size;

typedef struct wire_list_weekdays_twin_normal {
  int32_t *ptr;
  int32_t len;
} wire_list_weekdays_twin_normal;

typedef struct wire_a_twin_normal {
  struct wire_list_prim_u_8 *a;
} wire_a_twin_normal;

typedef struct wire_AbcTwinNormal_A {
  struct wire_a_twin_normal *field0;
} wire_AbcTwinNormal_A;

typedef struct wire_b_twin_normal {
  int32_t b;
} wire_b_twin_normal;

typedef struct wire_AbcTwinNormal_B {
  struct wire_b_twin_normal *field0;
} wire_AbcTwinNormal_B;

typedef struct wire_c_twin_normal {
  bool c;
} wire_c_twin_normal;

typedef struct wire_AbcTwinNormal_C {
  struct wire_c_twin_normal *field0;
} wire_AbcTwinNormal_C;

typedef struct wire_AbcTwinNormal_JustInt {
  int32_t field0;
} wire_AbcTwinNormal_JustInt;

typedef union AbcTwinNormalKind {
  struct wire_AbcTwinNormal_A *A;
  struct wire_AbcTwinNormal_B *B;
  struct wire_AbcTwinNormal_C *C;
  struct wire_AbcTwinNormal_JustInt *JustInt;
} AbcTwinNormalKind;

typedef struct wire_abc_twin_normal {
  int32_t tag;
  union AbcTwinNormalKind *kind;
} wire_abc_twin_normal;

typedef struct wire_struct_with_enum_twin_normal {
  struct wire_abc_twin_normal abc1;
  struct wire_abc_twin_normal abc2;
} wire_struct_with_enum_twin_normal;

typedef struct wire_empty_twin_normal {

} wire_empty_twin_normal;

typedef struct wire_list_my_size {
  struct wire_my_size *ptr;
  int32_t len;
} wire_list_my_size;

typedef struct wire_StringList {
  struct wire_list_prim_u_8 **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_new_type_int_twin_normal {
  int64_t field0;
} wire_new_type_int_twin_normal;

typedef struct wire_list_prim_i_8 {
  int8_t *ptr;
  int32_t len;
} wire_list_prim_i_8;

typedef struct wire_list_prim_f_32 {
  float *ptr;
  int32_t len;
} wire_list_prim_f_32;

typedef struct wire_attribute_twin_normal {
  struct wire_list_prim_u_8 *key;
  struct wire_list_prim_u_8 *value;
} wire_attribute_twin_normal;

typedef struct wire_list_attribute_twin_normal {
  struct wire_attribute_twin_normal *ptr;
  int32_t len;
} wire_list_attribute_twin_normal;

typedef struct wire_list_opt_box_autoadd_attribute_twin_normal {
  struct wire_attribute_twin_normal **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_attribute_twin_normal;

typedef struct wire_exotic_optionals_twin_normal {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_list_prim_u_8 *zerocopy;
  struct wire_list_prim_i_8 *int8list;
  struct wire_list_prim_u_8 *uint8list;
  struct wire_list_prim_i_32 *int32list;
  struct wire_list_prim_f_32 *float32list;
  struct wire_list_prim_f_64 *float64list;
  struct wire_list_attribute_twin_normal *attributes;
  struct wire_list_opt_box_autoadd_attribute_twin_normal *attributes_nullable;
  struct wire_list_opt_box_autoadd_attribute_twin_normal *nullable_attributes;
  struct wire_new_type_int_twin_normal *newtypeint;
} wire_exotic_optionals_twin_normal;

typedef struct wire_list_opt_box_autoadd_i_32 {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_i_32;

typedef struct wire_list_opt_box_autoadd_weekdays_twin_normal {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_weekdays_twin_normal;

typedef struct wire_list_opt_String {
  struct wire_list_prim_u_8 **ptr;
  int32_t len;
} wire_list_opt_String;

typedef struct wire_list_opt_list_prim_i_32 {
  struct wire_list_prim_i_32 **ptr;
  int32_t len;
} wire_list_opt_list_prim_i_32;

typedef struct wire_opt_vecs_twin_normal {
  struct wire_list_opt_box_autoadd_i_32 *i32;
  struct wire_list_opt_box_autoadd_weekdays_twin_normal *enums;
  struct wire_list_opt_String *strings;
  struct wire_list_opt_list_prim_i_32 *buffers;
} wire_opt_vecs_twin_normal;

typedef struct wire_RustOpaque_hide_data {
  const void *ptr;
} wire_RustOpaque_hide_data;

typedef struct wire_list_RustOpaque_hide_data {
  struct wire_RustOpaque_hide_data *ptr;
  int32_t len;
} wire_list_RustOpaque_hide_data;

typedef struct wire_EnumOpaqueTwinNormal_Struct {
  struct wire_RustOpaque_hide_data field0;
} wire_EnumOpaqueTwinNormal_Struct;

typedef struct wire_RustOpaque_i_32 {
  const void *ptr;
} wire_RustOpaque_i_32;

typedef struct wire_EnumOpaqueTwinNormal_Primitive {
  struct wire_RustOpaque_i_32 field0;
} wire_EnumOpaqueTwinNormal_Primitive;

typedef struct wire_RustOpaque_box_dynDartDebug {
  const void *ptr;
} wire_RustOpaque_box_dynDartDebug;

typedef struct wire_EnumOpaqueTwinNormal_TraitObj {
  struct wire_RustOpaque_box_dynDartDebug field0;
} wire_EnumOpaqueTwinNormal_TraitObj;

typedef struct wire_RustOpaque_MutexHideData {
  const void *ptr;
} wire_RustOpaque_MutexHideData;

typedef struct wire_EnumOpaqueTwinNormal_Mutex {
  struct wire_RustOpaque_MutexHideData field0;
} wire_EnumOpaqueTwinNormal_Mutex;

typedef struct wire_RustOpaque_RwLockHideData {
  const void *ptr;
} wire_RustOpaque_RwLockHideData;

typedef struct wire_EnumOpaqueTwinNormal_RwLock {
  struct wire_RustOpaque_RwLockHideData field0;
} wire_EnumOpaqueTwinNormal_RwLock;

typedef union EnumOpaqueTwinNormalKind {
  struct wire_EnumOpaqueTwinNormal_Struct *Struct;
  struct wire_EnumOpaqueTwinNormal_Primitive *Primitive;
  struct wire_EnumOpaqueTwinNormal_TraitObj *TraitObj;
  struct wire_EnumOpaqueTwinNormal_Mutex *Mutex;
  struct wire_EnumOpaqueTwinNormal_RwLock *RwLock;
} EnumOpaqueTwinNormalKind;

typedef struct wire_enum_opaque_twin_normal {
  int32_t tag;
  union EnumOpaqueTwinNormalKind *kind;
} wire_enum_opaque_twin_normal;

typedef struct wire_opaque_nested_twin_normal {
  struct wire_RustOpaque_hide_data first;
  struct wire_RustOpaque_hide_data second;
} wire_opaque_nested_twin_normal;

typedef struct wire_RustOpaque_non_clone_data {
  const void *ptr;
} wire_RustOpaque_non_clone_data;

typedef struct wire_RustOpaque_non_send_hide_data {
  const void *ptr;
} wire_RustOpaque_non_send_hide_data;

typedef struct wire_struct_with_one_field_twin_normal {
  int32_t a;
} wire_struct_with_one_field_twin_normal;

typedef struct wire_struct_with_two_field_twin_normal {
  int32_t a;
  int32_t b;
} wire_struct_with_two_field_twin_normal;

typedef struct wire_struct_with_zero_field_twin_normal {

} wire_struct_with_zero_field_twin_normal;

typedef struct wire_tuple_struct_with_one_field_twin_normal {
  int32_t field0;
} wire_tuple_struct_with_one_field_twin_normal;

typedef struct wire_tuple_struct_with_two_field_twin_normal {
  int32_t field0;
  int32_t field1;
} wire_tuple_struct_with_two_field_twin_normal;

typedef struct wire_record_string_i_32 {
  struct wire_list_prim_u_8 *field0;
  int32_t field1;
} wire_record_string_i_32;

typedef struct wire_list_record_string_i_32 {
  struct wire_record_string_i_32 *ptr;
  int32_t len;
} wire_list_record_string_i_32;

typedef struct wire_feature_uuid_twin_normal {
  struct wire_list_prim_u_8 *one;
  struct wire_list_prim_u_8 *many;
} wire_feature_uuid_twin_normal;

void wire_boxed_blob_twin_normal(int64_t port_, struct wire_list_prim_u_8 *blob);

void wire_func_test_id_twin_normal(int64_t port_, struct wire_test_id_twin_normal *id);

void wire_get_array_twin_normal(int64_t port_);

void wire_get_complex_array_twin_normal(int64_t port_);

void wire_last_number_twin_normal(int64_t port_, struct wire_list_prim_f_64 *array);

void wire_nested_id_twin_normal(int64_t port_, struct wire_list_test_id_twin_normal *id);

void wire_new_msgid_twin_normal(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_feed_id_twin_normal(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_raw_feed_id_twin_normal(int64_t port_, struct wire_feed_id_twin_normal *id);

void wire_use_boxed_blob_twin_normal(int64_t port_, struct wire_blob_twin_normal *blob);

void wire_use_msgid_twin_normal(int64_t port_, struct wire_message_id_twin_normal *id);

void wire_handle_customized_struct_twin_normal(int64_t port_,
                                               struct wire_customized_twin_normal *val);

void wire_next_user_id_twin_normal(int64_t port_, struct wire_user_id_twin_normal *user_id);

void wire_datetime_local_twin_normal(int64_t port_, int64_t d);

void wire_datetime_utc_twin_normal(int64_t port_, int64_t d);

void wire_duration_twin_normal(int64_t port_, int64_t d);

void wire_handle_durations_twin_normal(int64_t port_,
                                       struct wire_list_prim_i_64 *durations,
                                       int64_t since);

void wire_handle_timestamps_twin_normal(int64_t port_,
                                        struct wire_list_prim_i_64 *timestamps,
                                        int64_t epoch);

void wire_how_long_does_it_take_twin_normal(int64_t port_,
                                            struct wire_feature_chrono_twin_normal *mine);

void wire_naivedatetime_twin_normal(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc_twin_normal(int64_t port_, int64_t *d);

void wire_test_chrono_twin_normal(int64_t port_);

void wire_test_precise_chrono_twin_normal(int64_t port_);

void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(int64_t port_,
                                                                   struct wire_struct_with_comments_twin_normal *that);

void wire_StructWithCommentsTwinNormal_static_method_twin_normal(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_normal(int64_t port_);

void wire_return_dart_dynamic_twin_normal(int64_t port_);

void wire_async_accept_dart_opaque_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_enum_dart_opaque_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_nested_dart_opaque_twin_normal(int64_t port_,
                                                struct wire_DartOpaque opaque1,
                                                struct wire_DartOpaque opaque2);

void wire_drop_static_dart_opaque_twin_normal(int64_t port_);

void wire_get_enum_dart_opaque_twin_normal(int64_t port_,
                                           struct wire_enum_dart_opaque_twin_normal *opaque);

void wire_get_nested_dart_opaque_twin_normal(int64_t port_,
                                             struct wire_dart_opaque_nested_twin_normal *opaque);

void wire_loop_back_array_get_twin_normal(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_array_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_option_get_twin_normal(int64_t port_, struct wire_DartOpaque *opaque);

void wire_loop_back_option_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_vec_get_twin_normal(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_vec_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_panic_unwrap_dart_opaque_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

void wire_set_static_dart_opaque_twin_normal(int64_t port_, struct wire_DartOpaque opaque);

WireSyncReturn wire_return_non_droppable_dart_opaque_twin_normal(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_accept_dart_opaque_twin_normal(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_loopback_twin_normal(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_option_dart_opaque_twin_normal(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_option_loopback_twin_normal(struct wire_DartOpaque *opaque);

WireSyncReturn wire_unwrap_dart_opaque_twin_normal(struct wire_DartOpaque opaque);

void wire_func_enum_simple_twin_normal(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_mixed_twin_normal *arg);

void wire_func_enum_with_item_struct_twin_normal(int64_t port_,
                                                 struct wire_enum_with_item_struct_twin_normal *arg);

void wire_func_enum_with_item_tuple_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_tuple_twin_normal *arg);

void wire_handle_enum_parameter_twin_normal(int64_t port_, int32_t weekday);

void wire_handle_enum_struct_twin_normal(int64_t port_, struct wire_kitchen_sink_twin_normal *val);

void wire_handle_return_enum_twin_normal(int64_t port_, struct wire_list_prim_u_8 *input);

void wire_multiply_by_ten_twin_normal(int64_t port_, struct wire_measure_twin_normal *measure);

void wire_print_note_twin_normal(int64_t port_, struct wire_note_twin_normal *note);

void wire_EventTwinNormal_as_string_twin_normal(int64_t port_, struct wire_event_twin_normal *that);

void wire_close_event_listener_twin_normal(int64_t port_);

void wire_create_event_twin_normal(int64_t port_,
                                   struct wire_list_prim_u_8 *address,
                                   struct wire_list_prim_u_8 *payload);

void wire_register_event_listener_twin_normal(int64_t port_);

void wire_CustomStructTwinNormal_new_twin_normal(int64_t port_, struct wire_list_prim_u_8 *message);

void wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal(int64_t port_,
                                                                                  struct wire_custom_struct_twin_normal *that);

void wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal(int64_t port_,
                                                                               struct wire_custom_struct_twin_normal *that);

void wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal(int64_t port_);

void wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal(int64_t port_);

void wire_SomeStructTwinNormal_new_twin_normal(int64_t port_, uint32_t value);

void wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal(int64_t port_,
                                                                              struct wire_some_struct_twin_normal *that);

void wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal(int64_t port_,
                                                                             struct wire_some_struct_twin_normal *that);

void wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal(int64_t port_);

void wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal(int64_t port_);

void wire_custom_enum_error_panic_twin_normal(int64_t port_);

void wire_custom_enum_error_return_error_twin_normal(int64_t port_);

void wire_custom_enum_error_return_ok_twin_normal(int64_t port_, uint32_t arg);

void wire_custom_nested_error_return_error_twin_normal(int64_t port_,
                                                       struct wire_custom_nested_error_outer_twin_normal *arg);

void wire_custom_struct_error_return_error_twin_normal(int64_t port_,
                                                       struct wire_custom_struct_error_twin_normal *arg);

void wire_func_return_error_twin_normal(int64_t port_);

void wire_func_type_fallible_panic_twin_normal(int64_t port_);

void wire_func_type_infallible_panic_twin_normal(int64_t port_);

void wire_panic_with_custom_result_twin_normal(int64_t port_);

void wire_return_custom_nested_error_1_twin_normal(int64_t port_);

void wire_return_custom_nested_error_1_variant1_twin_normal(int64_t port_);

void wire_return_custom_nested_error_2_twin_normal(int64_t port_);

void wire_return_custom_struct_error_twin_normal(int64_t port_);

void wire_return_custom_struct_ok_twin_normal(int64_t port_);

void wire_return_err_custom_error_twin_normal(int64_t port_);

void wire_return_error_variant_twin_normal(int64_t port_, uint32_t variant);

void wire_return_ok_custom_error_twin_normal(int64_t port_);

void wire_stream_sink_throw_anyhow_twin_normal(int64_t port_);

WireSyncReturn wire_sync_return_custom_struct_error_twin_normal(void);

void wire_throw_anyhow_twin_normal(int64_t port_);

void wire_call_new_module_system_twin_normal(int64_t port_);

void wire_call_old_module_system_twin_normal(int64_t port_);

void wire_use_imported_enum_twin_normal(int64_t port_, int32_t my_enum);

void wire_use_imported_struct_twin_normal(int64_t port_, struct wire_my_struct *my_struct);

void wire_another_macro_struct_twin_normal(int64_t port_);

void wire_func_macro_struct_twin_normal(int64_t port_, struct wire_macro_struct *arg);

void wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal(int64_t port_,
                                                                   struct wire_list_prim_u_8 *a,
                                                                   struct wire_list_prim_u_8 *b);

void wire_ConcatenateWithTwinNormal_concatenate_twin_normal(int64_t port_,
                                                            struct wire_concatenate_with_twin_normal *that,
                                                            struct wire_list_prim_u_8 *b);

void wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal(int64_t port_);

void wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal(int64_t port_,
                                                                               uint32_t key,
                                                                               uint32_t max);

void wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal(int64_t port_,
                                                                             struct wire_concatenate_with_twin_normal *that);

void wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal(int64_t port_,
                                                                        struct wire_concatenate_with_twin_normal *that,
                                                                        uint32_t key,
                                                                        uint32_t max);

void wire_ConcatenateWithTwinNormal_new_twin_normal(int64_t port_, struct wire_list_prim_u_8 *a);

void wire_SumWithTwinNormal_sum_twin_normal(int64_t port_,
                                            struct wire_sum_with_twin_normal *that,
                                            uint32_t y,
                                            uint32_t z);

void wire_get_sum_array_twin_normal(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_get_sum_struct_twin_normal(int64_t port_);

void wire_app_settings_stream_twin_normal(int64_t port_);

void wire_app_settings_vec_stream_twin_normal(int64_t port_);

void wire_first_number_twin_normal(int64_t port_, struct wire_numbers *nums);

void wire_first_sequence_twin_normal(int64_t port_, struct wire_sequences *seqs);

void wire_get_app_settings_twin_normal(int64_t port_);

void wire_get_fallible_app_settings_twin_normal(int64_t port_);

void wire_get_message_twin_normal(int64_t port_);

void wire_is_app_embedded_twin_normal(int64_t port_,
                                      struct wire_application_settings *app_settings);

void wire_mirror_struct_stream_twin_normal(int64_t port_);

void wire_mirror_tuple_stream_twin_normal(int64_t port_);

void wire_repeat_number_twin_normal(int64_t port_, int32_t num, uintptr_t times);

void wire_repeat_sequence_twin_normal(int64_t port_, int32_t seq, uintptr_t times);

void wire_test_contains_mirrored_sub_struct_twin_normal(int64_t port_);

void wire_test_fallible_of_raw_string_mirrored_twin_normal(int64_t port_);

void wire_test_list_of_nested_enums_mirrored_twin_normal(int64_t port_);

void wire_test_list_of_raw_nested_string_mirrored_twin_normal(int64_t port_);

void wire_test_nested_raw_string_mirrored_twin_normal(int64_t port_);

void wire_test_raw_string_enum_mirrored_twin_normal(int64_t port_, bool nested);

void wire_test_raw_string_mirrored_twin_normal(int64_t port_);

void wire_handle_big_buffers_twin_normal(int64_t port_);

void wire_handle_complex_struct_twin_normal(int64_t port_, struct wire_my_tree_node_twin_normal *s);

void wire_handle_nested_struct_twin_normal(int64_t port_,
                                           struct wire_my_nested_struct_twin_normal *s);

void wire_handle_string_twin_normal(int64_t port_, struct wire_list_prim_u_8 *s);

WireSyncReturn wire_handle_struct_sync_freezed_twin_normal(struct wire_my_size_freezed_twin_normal *arg,
                                                           struct wire_my_size_freezed_twin_normal *boxed);

void wire_handle_struct_twin_normal(int64_t port_,
                                    struct wire_my_size *arg,
                                    struct wire_my_size *boxed);

void wire_handle_vec_u8_twin_normal(int64_t port_, struct wire_list_prim_u_8 *v);

void wire_list_of_primitive_enums_twin_normal(int64_t port_,
                                              struct wire_list_weekdays_twin_normal *weekdays);

void wire_test_abc_enum_twin_normal(int64_t port_, struct wire_abc_twin_normal *abc);

void wire_test_struct_with_enum_twin_normal(int64_t port_,
                                            struct wire_struct_with_enum_twin_normal *se);

void wire_empty_struct_twin_normal(int64_t port_, struct wire_empty_twin_normal *empty);

void wire_func_return_unit_twin_normal(int64_t port_);

void wire_func_string_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_handle_list_of_struct_twin_normal(int64_t port_, struct wire_list_my_size *l);

void wire_handle_string_list_twin_normal(int64_t port_, struct wire_StringList *names);

void wire_handle_newtype_twin_normal(int64_t port_, struct wire_new_type_int_twin_normal *arg);

void wire_handle_increment_boxed_optional_twin_normal(int64_t port_, double *opt);

void wire_handle_option_box_arguments_twin_normal(int64_t port_,
                                                  int8_t *i8box,
                                                  uint8_t *u8box,
                                                  int32_t *i32box,
                                                  int64_t *i64box,
                                                  double *f64box,
                                                  bool *boolbox,
                                                  struct wire_exotic_optionals_twin_normal *structbox);

void wire_handle_optional_increment_twin_normal(int64_t port_,
                                                struct wire_exotic_optionals_twin_normal *opt);

void wire_handle_optional_return_twin_normal(int64_t port_, double left, double right);

void wire_handle_optional_struct_twin_normal(int64_t port_, struct wire_list_prim_u_8 *document);

void wire_handle_vec_of_opts_twin_normal(int64_t port_, struct wire_opt_vecs_twin_normal *opt);

WireSyncReturn wire_sync_option_null_twin_normal(void);

WireSyncReturn wire_sync_option_twin_normal(void);

void wire_primitive_optional_types_twin_normal(int64_t port_,
                                               int32_t *my_i32,
                                               int64_t *my_i64,
                                               double *my_f64,
                                               bool *my_bool);

void wire_handle_vec_of_primitive_twin_normal(int64_t port_, int32_t n);

void wire_handle_zero_copy_vec_of_primitive_twin_normal(int64_t port_, int32_t n);

WireSyncReturn wire_handle_zero_copy_vec_of_primitive_sync_twin_normal(int32_t n);

void wire_primitive_types_twin_normal(int64_t port_,
                                      int32_t my_i32,
                                      int64_t my_i64,
                                      double my_f64,
                                      bool my_bool);

void wire_primitive_u32_twin_normal(int64_t port_, uint32_t my_u32);

void wire_test_more_than_just_one_raw_string_struct_twin_normal(int64_t port_);

void wire_test_raw_string_item_struct_twin_normal(int64_t port_);

void wire_create_array_opaque_enum_twin_normal(int64_t port_);

void wire_create_nested_opaque_twin_normal(int64_t port_);

void wire_create_opaque_twin_normal(int64_t port_);

void wire_create_option_opaque_twin_normal(int64_t port_, struct wire_RustOpaque_hide_data *opaque);

void wire_create_sync_opaque_twin_normal(int64_t port_);

void wire_frb_generator_test_twin_normal(int64_t port_);

void wire_opaque_array_run_twin_normal(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_array_twin_normal(int64_t port_);

void wire_opaque_vec_run_twin_normal(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_vec_twin_normal(int64_t port_);

void wire_run_enum_opaque_twin_normal(int64_t port_, struct wire_enum_opaque_twin_normal *opaque);

void wire_run_nested_opaque_twin_normal(int64_t port_,
                                        struct wire_opaque_nested_twin_normal *opaque);

void wire_run_non_clone_twin_normal(int64_t port_, struct wire_RustOpaque_non_clone_data clone);

void wire_run_opaque_twin_normal(int64_t port_, struct wire_RustOpaque_hide_data opaque);

void wire_run_opaque_with_delay_twin_normal(int64_t port_, struct wire_RustOpaque_hide_data opaque);

void wire_unwrap_rust_opaque_twin_normal(int64_t port_, struct wire_RustOpaque_hide_data opaque);

WireSyncReturn wire_frb_sync_generator_test_twin_normal(void);

WireSyncReturn wire_sync_create_non_clone_twin_normal(void);

WireSyncReturn wire_sync_create_opaque_twin_normal(void);

WireSyncReturn wire_sync_create_sync_opaque_twin_normal(void);

WireSyncReturn wire_sync_option_rust_opaque_twin_normal(void);

WireSyncReturn wire_sync_run_opaque_twin_normal(struct wire_RustOpaque_non_send_hide_data opaque);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_stream_realistic_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_func_stream_return_error_twin_normal(int64_t port_);

void wire_func_stream_return_panic_twin_normal(int64_t port_);

void wire_func_stream_sink_arg_position_twin_normal(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct_twin_normal(int64_t port_);

void wire_handle_stream_sink_at_1_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_func_struct_with_one_field_twin_normal(int64_t port_,
                                                 struct wire_struct_with_one_field_twin_normal *arg);

void wire_func_struct_with_two_field_twin_normal(int64_t port_,
                                                 struct wire_struct_with_two_field_twin_normal *arg);

void wire_func_struct_with_zero_field_twin_normal(int64_t port_,
                                                  struct wire_struct_with_zero_field_twin_normal *arg);

void wire_func_tuple_struct_with_one_field_twin_normal(int64_t port_,
                                                       struct wire_tuple_struct_with_one_field_twin_normal *arg);

void wire_func_tuple_struct_with_two_field_twin_normal(int64_t port_,
                                                       struct wire_tuple_struct_with_two_field_twin_normal *arg);

void wire_test_tuple_2_twin_normal(int64_t port_, struct wire_list_record_string_i_32 *value);

void wire_test_tuple_twin_normal(int64_t port_, struct wire_record_string_i_32 *value);

void wire_handle_type_alias_id_twin_normal(int64_t port_, uint64_t input);

void wire_handle_type_alias_model_twin_normal(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id_twin_normal(int64_t port_, uint64_t input);

void wire_handle_nested_uuids_twin_normal(int64_t port_, struct wire_feature_uuid_twin_normal *ids);

void wire_handle_uuid_twin_normal(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_handle_uuids_twin_normal(int64_t port_, struct wire_list_prim_u_8 *ids);

struct wire_DartOpaque new_DartOpaque(void);

struct wire_RustOpaque_MutexHideData new_RustOpaque_MutexHideData(void);

struct wire_RustOpaque_RwLockHideData new_RustOpaque_RwLockHideData(void);

struct wire_RustOpaque_box_dynDartDebug new_RustOpaque_box_dynDartDebug(void);

struct wire_RustOpaque_hide_data new_RustOpaque_hide_data(void);

struct wire_RustOpaque_i_32 new_RustOpaque_i_32(void);

struct wire_RustOpaque_non_clone_data new_RustOpaque_non_clone_data(void);

struct wire_RustOpaque_non_send_hide_data new_RustOpaque_non_send_hide_data(void);

struct wire_StringList *new_StringList(int32_t len);

struct wire_application_env *new_box_application_env(void);

int64_t *new_box_autoadd_Chrono_Utc(int64_t value);

struct wire_DartOpaque *new_box_autoadd_DartOpaque(void);

struct wire_RustOpaque_hide_data *new_box_autoadd_RustOpaque_hide_data(void);

struct wire_a_twin_normal *new_box_autoadd_a_twin_normal(void);

struct wire_abc_twin_normal *new_box_autoadd_abc_twin_normal(void);

struct wire_application_env *new_box_autoadd_application_env(void);

struct wire_application_settings *new_box_autoadd_application_settings(void);

struct wire_attribute_twin_normal *new_box_autoadd_attribute_twin_normal(void);

struct wire_b_twin_normal *new_box_autoadd_b_twin_normal(void);

bool *new_box_autoadd_bool(bool value);

struct wire_c_twin_normal *new_box_autoadd_c_twin_normal(void);

struct wire_concatenate_with_twin_normal *new_box_autoadd_concatenate_with_twin_normal(void);

struct wire_custom_nested_error_inner_twin_normal *new_box_autoadd_custom_nested_error_inner_twin_normal(void);

struct wire_custom_nested_error_outer_twin_normal *new_box_autoadd_custom_nested_error_outer_twin_normal(void);

struct wire_custom_struct_error_twin_normal *new_box_autoadd_custom_struct_error_twin_normal(void);

struct wire_custom_struct_twin_normal *new_box_autoadd_custom_struct_twin_normal(void);

struct wire_customized_twin_normal *new_box_autoadd_customized_twin_normal(void);

struct wire_dart_opaque_nested_twin_normal *new_box_autoadd_dart_opaque_nested_twin_normal(void);

struct wire_empty_twin_normal *new_box_autoadd_empty_twin_normal(void);

struct wire_enum_dart_opaque_twin_normal *new_box_autoadd_enum_dart_opaque_twin_normal(void);

struct wire_enum_opaque_twin_normal *new_box_autoadd_enum_opaque_twin_normal(void);

struct wire_enum_with_item_mixed_twin_normal *new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_enum_with_item_struct_twin_normal *new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_enum_with_item_tuple_twin_normal *new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_event_twin_normal *new_box_autoadd_event_twin_normal(void);

struct wire_exotic_optionals_twin_normal *new_box_autoadd_exotic_optionals_twin_normal(void);

double *new_box_autoadd_f_64(double value);

struct wire_feature_chrono_twin_normal *new_box_autoadd_feature_chrono_twin_normal(void);

struct wire_feature_uuid_twin_normal *new_box_autoadd_feature_uuid_twin_normal(void);

struct wire_feed_id_twin_normal *new_box_autoadd_feed_id_twin_normal(void);

int32_t *new_box_autoadd_i_32(int32_t value);

int64_t *new_box_autoadd_i_64(int64_t value);

struct wire_kitchen_sink_twin_normal *new_box_autoadd_kitchen_sink_twin_normal(void);

struct wire_macro_struct *new_box_autoadd_macro_struct(void);

struct wire_measure_twin_normal *new_box_autoadd_measure_twin_normal(void);

struct wire_message_id_twin_normal *new_box_autoadd_message_id_twin_normal(void);

struct wire_my_nested_struct_twin_normal *new_box_autoadd_my_nested_struct_twin_normal(void);

struct wire_my_size *new_box_autoadd_my_size(void);

struct wire_my_size_freezed_twin_normal *new_box_autoadd_my_size_freezed_twin_normal(void);

struct wire_my_struct *new_box_autoadd_my_struct(void);

struct wire_my_tree_node_twin_normal *new_box_autoadd_my_tree_node_twin_normal(void);

struct wire_new_type_int_twin_normal *new_box_autoadd_new_type_int_twin_normal(void);

struct wire_note_twin_normal *new_box_autoadd_note_twin_normal(void);

struct wire_numbers *new_box_autoadd_numbers(void);

struct wire_opaque_nested_twin_normal *new_box_autoadd_opaque_nested_twin_normal(void);

struct wire_opt_vecs_twin_normal *new_box_autoadd_opt_vecs_twin_normal(void);

struct wire_record_string_i_32 *new_box_autoadd_record_string_i_32(void);

struct wire_sequences *new_box_autoadd_sequences(void);

struct wire_some_struct_twin_normal *new_box_autoadd_some_struct_twin_normal(void);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_struct_with_enum_twin_normal *new_box_autoadd_struct_with_enum_twin_normal(void);

struct wire_struct_with_one_field_twin_normal *new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_struct_with_two_field_twin_normal *new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_struct_with_zero_field_twin_normal *new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_sum_with_twin_normal *new_box_autoadd_sum_with_twin_normal(void);

struct wire_test_id_twin_normal *new_box_autoadd_test_id_twin_normal(void);

struct wire_tuple_struct_with_one_field_twin_normal *new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_tuple_struct_with_two_field_twin_normal *new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_user_id_twin_normal *new_box_autoadd_user_id_twin_normal(void);

int32_t *new_box_autoadd_weekdays_twin_normal(int32_t value);

struct wire_blob_twin_normal *new_box_blob_twin_normal(void);

bool *new_box_bool(bool value);

struct wire_distance_twin_normal *new_box_distance_twin_normal(void);

struct wire_exotic_optionals_twin_normal *new_box_exotic_optionals_twin_normal(void);

double *new_box_f_64(double value);

int32_t *new_box_i_32(int32_t value);

int64_t *new_box_i_64(int64_t value);

int8_t *new_box_i_8(int8_t value);

struct wire_kitchen_sink_twin_normal *new_box_kitchen_sink_twin_normal(void);

struct wire_my_size *new_box_my_size(void);

struct wire_my_size_freezed_twin_normal *new_box_my_size_freezed_twin_normal(void);

struct wire_speed_twin_normal *new_box_speed_twin_normal(void);

uint8_t *new_box_u_8(uint8_t value);

int32_t *new_box_weekdays_twin_normal(int32_t value);

struct wire_list_DartOpaque *new_list_DartOpaque(int32_t len);

struct wire_list_RustOpaque_hide_data *new_list_RustOpaque_hide_data(int32_t len);

struct wire_list_application_env_var *new_list_application_env_var(int32_t len);

struct wire_list_attribute_twin_normal *new_list_attribute_twin_normal(int32_t len);

struct wire_list_my_size *new_list_my_size(int32_t len);

struct wire_list_my_tree_node_twin_normal *new_list_my_tree_node_twin_normal(int32_t len);

struct wire_list_opt_String *new_list_opt_String(int32_t len);

struct wire_list_opt_box_autoadd_attribute_twin_normal *new_list_opt_box_autoadd_attribute_twin_normal(int32_t len);

struct wire_list_opt_box_autoadd_i_32 *new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_list_opt_box_autoadd_weekdays_twin_normal *new_list_opt_box_autoadd_weekdays_twin_normal(int32_t len);

struct wire_list_opt_list_prim_i_32 *new_list_opt_list_prim_i_32(int32_t len);

struct wire_list_prim_f_32 *new_list_prim_f_32(int32_t len);

struct wire_list_prim_f_64 *new_list_prim_f_64(int32_t len);

struct wire_list_prim_i_32 *new_list_prim_i_32(int32_t len);

struct wire_list_prim_i_64 *new_list_prim_i_64(int32_t len);

struct wire_list_prim_i_8 *new_list_prim_i_8(int32_t len);

struct wire_list_prim_u_8 *new_list_prim_u_8(int32_t len);

struct wire_list_record_string_i_32 *new_list_record_string_i_32(int32_t len);

struct wire_list_test_id_twin_normal *new_list_test_id_twin_normal(int32_t len);

struct wire_list_weekdays_twin_normal *new_list_weekdays_twin_normal(int32_t len);

void drop_opaque_RustOpaque_MutexHideData(const void *ptr);

const void *share_opaque_RustOpaque_MutexHideData(const void *ptr);

void drop_opaque_RustOpaque_RwLockHideData(const void *ptr);

const void *share_opaque_RustOpaque_RwLockHideData(const void *ptr);

void drop_opaque_RustOpaque_box_dynDartDebug(const void *ptr);

const void *share_opaque_RustOpaque_box_dynDartDebug(const void *ptr);

void drop_opaque_RustOpaque_frb_opaque_return(const void *ptr);

const void *share_opaque_RustOpaque_frb_opaque_return(const void *ptr);

void drop_opaque_RustOpaque_frb_opaque_sync_return(const void *ptr);

const void *share_opaque_RustOpaque_frb_opaque_sync_return(const void *ptr);

void drop_opaque_RustOpaque_hide_data(const void *ptr);

const void *share_opaque_RustOpaque_hide_data(const void *ptr);

void drop_opaque_RustOpaque_i_32(const void *ptr);

const void *share_opaque_RustOpaque_i_32(const void *ptr);

void drop_opaque_RustOpaque_non_clone_data(const void *ptr);

const void *share_opaque_RustOpaque_non_clone_data(const void *ptr);

void drop_opaque_RustOpaque_non_send_hide_data(const void *ptr);

const void *share_opaque_RustOpaque_non_send_hide_data(const void *ptr);

union AbcTwinNormalKind *inflate_AbcTwinNormal_A(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_B(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_C(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_JustInt(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Three(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Four(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_One(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_Two(void);

union DistanceTwinNormalKind *inflate_DistanceTwinNormal_Map(void);

union EnumDartOpaqueTwinNormalKind *inflate_EnumDartOpaqueTwinNormal_Primitive(void);

union EnumDartOpaqueTwinNormalKind *inflate_EnumDartOpaqueTwinNormal_Opaque(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Struct(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Primitive(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_TraitObj(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Mutex(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_RwLock(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_B(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_C(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_A(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_B(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_A(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_B(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Primitives(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Nested(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Optional(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Buffer(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Enums(void);

union MeasureTwinNormalKind *inflate_MeasureTwinNormal_Speed(void);

union MeasureTwinNormalKind *inflate_MeasureTwinNormal_Distance(void);

union SpeedTwinNormalKind *inflate_SpeedTwinNormal_GPS(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_box_dynDartDebug);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_Two);
    dummy_var ^= ((int64_t) (void*) inflate_DistanceTwinNormal_Map);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinNormal_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinNormal_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinNormal_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_SpeedTwinNormal_GPS);
    dummy_var ^= ((int64_t) (void*) new_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_box_dynDartDebug);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) new_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) new_StringList);
    dummy_var ^= ((int64_t) (void*) new_box_application_env);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_env);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_settings);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_macro_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size_freezed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_numbers);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sequences);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_bool);
    dummy_var ^= ((int64_t) (void*) new_box_distance_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_my_size_freezed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_speed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_list_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_list_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_list_application_env_var);
    dummy_var ^= ((int64_t) (void*) new_list_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_my_size);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_opt_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_f_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_f_64);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_64);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_8);
    dummy_var ^= ((int64_t) (void*) new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) new_list_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_box_dynDartDebug);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) share_opaque_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinNormal_as_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinNormal_sum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_another_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_realistic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_sync_freezed_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_sync_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_non_droppable_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_null_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_normal);
    return dummy_var;
}
