#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct benchmark_raw_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} benchmark_raw_list_prim_u_8;

typedef struct wire_cst_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8;

typedef struct wire_cst_list_prim_i_32 {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_32;

typedef struct wire_cst_test_id_twin_normal {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_normal;

typedef struct wire_cst_list_prim_f_64 {
  double *ptr;
  int32_t len;
} wire_cst_list_prim_f_64;

typedef struct wire_cst_list_test_id_twin_normal {
  struct wire_cst_test_id_twin_normal *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_normal;

typedef struct wire_cst_feed_id_twin_normal {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_normal;

typedef struct wire_cst_blob_twin_normal {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_normal;

typedef struct wire_cst_message_id_twin_normal {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_normal;

typedef struct wire_cst_customized_twin_normal {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_normal;

typedef struct wire_cst_user_id_twin_normal {
  uint32_t value;
} wire_cst_user_id_twin_normal;

typedef struct wire_cst_list_Chrono_Duration {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_Chrono_Duration;

typedef struct wire_cst_list_Chrono_Naive {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_Chrono_Naive;

typedef struct wire_cst_feature_chrono_twin_normal {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_cst_feature_chrono_twin_normal;

typedef struct wire_cst_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_normal;

typedef struct wire_cst_EnumDartOpaqueTwinNormal_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinNormal_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinNormal_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinNormal_Opaque;

typedef union EnumDartOpaqueTwinNormalKind {
  struct wire_cst_EnumDartOpaqueTwinNormal_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinNormal_Opaque *Opaque;
} EnumDartOpaqueTwinNormalKind;

typedef struct wire_cst_enum_dart_opaque_twin_normal {
  int32_t tag;
  union EnumDartOpaqueTwinNormalKind *kind;
} wire_cst_enum_dart_opaque_twin_normal;

typedef struct wire_cst_dart_opaque_nested_twin_normal {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_normal;

typedef struct wire_cst_list_DartOpaque {
  const void **ptr;
  int32_t len;
} wire_cst_list_DartOpaque;

typedef struct wire_cst_EnumWithItemMixedTwinNormal_A {

} wire_cst_EnumWithItemMixedTwinNormal_A;

typedef struct wire_cst_EnumWithItemMixedTwinNormal_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinNormal_B;

typedef struct wire_cst_EnumWithItemMixedTwinNormal_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinNormal_C;

typedef union EnumWithItemMixedTwinNormalKind {
  struct wire_cst_EnumWithItemMixedTwinNormal_A *A;
  struct wire_cst_EnumWithItemMixedTwinNormal_B *B;
  struct wire_cst_EnumWithItemMixedTwinNormal_C *C;
} EnumWithItemMixedTwinNormalKind;

typedef struct wire_cst_enum_with_item_mixed_twin_normal {
  int32_t tag;
  union EnumWithItemMixedTwinNormalKind *kind;
} wire_cst_enum_with_item_mixed_twin_normal;

typedef struct wire_cst_EnumWithItemStructTwinNormal_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinNormal_A;

typedef struct wire_cst_EnumWithItemStructTwinNormal_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinNormal_B;

typedef union EnumWithItemStructTwinNormalKind {
  struct wire_cst_EnumWithItemStructTwinNormal_A *A;
  struct wire_cst_EnumWithItemStructTwinNormal_B *B;
} EnumWithItemStructTwinNormalKind;

typedef struct wire_cst_enum_with_item_struct_twin_normal {
  int32_t tag;
  union EnumWithItemStructTwinNormalKind *kind;
} wire_cst_enum_with_item_struct_twin_normal;

typedef struct wire_cst_EnumWithItemTupleTwinNormal_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinNormal_A;

typedef struct wire_cst_EnumWithItemTupleTwinNormal_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinNormal_B;

typedef union EnumWithItemTupleTwinNormalKind {
  struct wire_cst_EnumWithItemTupleTwinNormal_A *A;
  struct wire_cst_EnumWithItemTupleTwinNormal_B *B;
} EnumWithItemTupleTwinNormalKind;

typedef struct wire_cst_enum_with_item_tuple_twin_normal {
  int32_t tag;
  union EnumWithItemTupleTwinNormalKind *kind;
} wire_cst_enum_with_item_tuple_twin_normal;

typedef struct wire_cst_KitchenSinkTwinNormal_Empty {

} wire_cst_KitchenSinkTwinNormal_Empty;

typedef struct wire_cst_KitchenSinkTwinNormal_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinNormal_Primitives;

typedef struct wire_cst_KitchenSinkTwinNormal_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_normal *field1;
} wire_cst_KitchenSinkTwinNormal_Nested;

typedef struct wire_cst_KitchenSinkTwinNormal_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinNormal_Optional;

typedef struct wire_cst_KitchenSinkTwinNormal_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinNormal_Buffer;

typedef struct wire_cst_KitchenSinkTwinNormal_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinNormal_Enums;

typedef union KitchenSinkTwinNormalKind {
  struct wire_cst_KitchenSinkTwinNormal_Empty *Empty;
  struct wire_cst_KitchenSinkTwinNormal_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinNormal_Nested *Nested;
  struct wire_cst_KitchenSinkTwinNormal_Optional *Optional;
  struct wire_cst_KitchenSinkTwinNormal_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinNormal_Enums *Enums;
} KitchenSinkTwinNormalKind;

typedef struct wire_cst_kitchen_sink_twin_normal {
  int32_t tag;
  union KitchenSinkTwinNormalKind *kind;
} wire_cst_kitchen_sink_twin_normal;

typedef struct wire_cst_SpeedTwinNormal_Unknown {

} wire_cst_SpeedTwinNormal_Unknown;

typedef struct wire_cst_SpeedTwinNormal_GPS {
  double field0;
} wire_cst_SpeedTwinNormal_GPS;

typedef union SpeedTwinNormalKind {
  struct wire_cst_SpeedTwinNormal_Unknown *Unknown;
  struct wire_cst_SpeedTwinNormal_GPS *GPS;
} SpeedTwinNormalKind;

typedef struct wire_cst_speed_twin_normal {
  int32_t tag;
  union SpeedTwinNormalKind *kind;
} wire_cst_speed_twin_normal;

typedef struct wire_cst_MeasureTwinNormal_Speed {
  struct wire_cst_speed_twin_normal *field0;
} wire_cst_MeasureTwinNormal_Speed;

typedef struct wire_cst_DistanceTwinNormal_Unknown {

} wire_cst_DistanceTwinNormal_Unknown;

typedef struct wire_cst_DistanceTwinNormal_Map {
  double field0;
} wire_cst_DistanceTwinNormal_Map;

typedef union DistanceTwinNormalKind {
  struct wire_cst_DistanceTwinNormal_Unknown *Unknown;
  struct wire_cst_DistanceTwinNormal_Map *Map;
} DistanceTwinNormalKind;

typedef struct wire_cst_distance_twin_normal {
  int32_t tag;
  union DistanceTwinNormalKind *kind;
} wire_cst_distance_twin_normal;

typedef struct wire_cst_MeasureTwinNormal_Distance {
  struct wire_cst_distance_twin_normal *field0;
} wire_cst_MeasureTwinNormal_Distance;

typedef union MeasureTwinNormalKind {
  struct wire_cst_MeasureTwinNormal_Speed *Speed;
  struct wire_cst_MeasureTwinNormal_Distance *Distance;
} MeasureTwinNormalKind;

typedef struct wire_cst_measure_twin_normal {
  int32_t tag;
  union MeasureTwinNormalKind *kind;
} wire_cst_measure_twin_normal;

typedef struct wire_cst_note_twin_normal {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_normal;

typedef struct wire_cst_event_twin_normal {
  struct wire_cst_list_prim_u_8 *address;
  struct wire_cst_list_prim_u_8 *payload;
} wire_cst_event_twin_normal;

typedef struct wire_cst_custom_struct_twin_normal {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_normal;

typedef struct wire_cst_some_struct_twin_normal {
  uint32_t value;
} wire_cst_some_struct_twin_normal;

typedef struct wire_cst_CustomNestedErrorOuterTwinNormal_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinNormal_One;

typedef struct wire_cst_CustomNestedErrorInnerTwinNormal_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinNormal_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinNormal_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinNormal_Four;

typedef union CustomNestedErrorInnerTwinNormalKind {
  struct wire_cst_CustomNestedErrorInnerTwinNormal_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinNormal_Four *Four;
} CustomNestedErrorInnerTwinNormalKind;

typedef struct wire_cst_custom_nested_error_inner_twin_normal {
  int32_t tag;
  union CustomNestedErrorInnerTwinNormalKind *kind;
} wire_cst_custom_nested_error_inner_twin_normal;

typedef struct wire_cst_CustomNestedErrorOuterTwinNormal_Two {
  struct wire_cst_custom_nested_error_inner_twin_normal *field0;
} wire_cst_CustomNestedErrorOuterTwinNormal_Two;

typedef union CustomNestedErrorOuterTwinNormalKind {
  struct wire_cst_CustomNestedErrorOuterTwinNormal_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinNormal_Two *Two;
} CustomNestedErrorOuterTwinNormalKind;

typedef struct wire_cst_custom_nested_error_outer_twin_normal {
  int32_t tag;
  union CustomNestedErrorOuterTwinNormalKind *kind;
} wire_cst_custom_nested_error_outer_twin_normal;

typedef struct wire_cst_custom_struct_error_twin_normal {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_normal;

typedef struct wire_cst_my_struct {
  bool content;
} wire_cst_my_struct;

typedef struct wire_cst_macro_struct {
  int32_t data;
} wire_cst_macro_struct;

typedef struct wire_cst_concatenate_with_twin_normal {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_normal;

typedef struct wire_cst_sum_with_twin_normal {
  uint32_t x;
} wire_cst_sum_with_twin_normal;

typedef struct wire_cst_numbers {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_numbers;

typedef struct wire_cst_sequences {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_sequences;

typedef struct wire_cst_application_env_var {
  struct wire_cst_list_prim_u_8 *field0;
  bool field1;
} wire_cst_application_env_var;

typedef struct wire_cst_list_application_env_var {
  struct wire_cst_application_env_var *ptr;
  int32_t len;
} wire_cst_list_application_env_var;

typedef struct wire_cst_application_env {
  struct wire_cst_list_application_env_var *vars;
} wire_cst_application_env;

typedef struct wire_cst_application_settings {
  struct wire_cst_list_prim_u_8 *name;
  struct wire_cst_list_prim_u_8 *version;
  int32_t mode;
  struct wire_cst_application_env *env;
  struct wire_cst_application_env *env_optional;
} wire_cst_application_settings;

typedef struct wire_cst_list_my_tree_node_twin_normal {
  struct wire_cst_my_tree_node_twin_normal *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_normal;

typedef struct wire_cst_my_tree_node_twin_normal {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_normal *children;
} wire_cst_my_tree_node_twin_normal;

typedef struct wire_cst_my_nested_struct_twin_normal {
  struct wire_cst_my_tree_node_twin_normal tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_normal;

typedef struct wire_cst_my_size {
  int32_t width;
  int32_t height;
} wire_cst_my_size;

typedef struct wire_cst_list_weekdays_twin_normal {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_normal;

typedef struct wire_cst_a_twin_normal {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_normal;

typedef struct wire_cst_AbcTwinNormal_A {
  struct wire_cst_a_twin_normal *field0;
} wire_cst_AbcTwinNormal_A;

typedef struct wire_cst_b_twin_normal {
  int32_t b;
} wire_cst_b_twin_normal;

typedef struct wire_cst_AbcTwinNormal_B {
  struct wire_cst_b_twin_normal *field0;
} wire_cst_AbcTwinNormal_B;

typedef struct wire_cst_c_twin_normal {
  bool c;
} wire_cst_c_twin_normal;

typedef struct wire_cst_AbcTwinNormal_C {
  struct wire_cst_c_twin_normal *field0;
} wire_cst_AbcTwinNormal_C;

typedef struct wire_cst_AbcTwinNormal_JustInt {
  int32_t field0;
} wire_cst_AbcTwinNormal_JustInt;

typedef union AbcTwinNormalKind {
  struct wire_cst_AbcTwinNormal_A *A;
  struct wire_cst_AbcTwinNormal_B *B;
  struct wire_cst_AbcTwinNormal_C *C;
  struct wire_cst_AbcTwinNormal_JustInt *JustInt;
} AbcTwinNormalKind;

typedef struct wire_cst_abc_twin_normal {
  int32_t tag;
  union AbcTwinNormalKind *kind;
} wire_cst_abc_twin_normal;

typedef struct wire_cst_struct_with_enum_twin_normal {
  struct wire_cst_abc_twin_normal abc1;
  struct wire_cst_abc_twin_normal abc2;
} wire_cst_struct_with_enum_twin_normal;

typedef struct wire_cst_empty_twin_normal {

} wire_cst_empty_twin_normal;

typedef struct wire_cst_list_my_size {
  struct wire_cst_my_size *ptr;
  int32_t len;
} wire_cst_list_my_size;

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8 **ptr;
  int32_t len;
} wire_cst_list_String;

typedef struct wire_cst_new_type_int_twin_normal {
  int64_t field0;
} wire_cst_new_type_int_twin_normal;

typedef struct wire_cst_list_prim_i_8 {
  int8_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_8;

typedef struct wire_cst_list_prim_f_32 {
  float *ptr;
  int32_t len;
} wire_cst_list_prim_f_32;

typedef struct wire_cst_attribute_twin_normal {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_normal;

typedef struct wire_cst_list_attribute_twin_normal {
  struct wire_cst_attribute_twin_normal *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_normal;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_normal {
  struct wire_cst_attribute_twin_normal **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_normal;

typedef struct wire_cst_exotic_optionals_twin_normal {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_normal *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_normal *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_normal *nullable_attributes;
  struct wire_cst_new_type_int_twin_normal *newtypeint;
} wire_cst_exotic_optionals_twin_normal;

typedef struct wire_cst_list_opt_box_autoadd_i_32 {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_i_32;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_normal {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_normal;

typedef struct wire_cst_list_opt_String {
  struct wire_cst_list_prim_u_8 **ptr;
  int32_t len;
} wire_cst_list_opt_String;

typedef struct wire_cst_list_opt_list_prim_i_32 {
  struct wire_cst_list_prim_i_32 **ptr;
  int32_t len;
} wire_cst_list_opt_list_prim_i_32;

typedef struct wire_cst_opt_vecs_twin_normal {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_normal *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_normal;

typedef struct wire_cst_test_id_twin_rust_async {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_rust_async;

typedef struct wire_cst_list_test_id_twin_rust_async {
  struct wire_cst_test_id_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_rust_async;

typedef struct wire_cst_feed_id_twin_rust_async {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_rust_async;

typedef struct wire_cst_blob_twin_rust_async {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_rust_async;

typedef struct wire_cst_message_id_twin_rust_async {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_rust_async;

typedef struct wire_cst_test_id_twin_sync {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_sync;

typedef struct wire_cst_list_test_id_twin_sync {
  struct wire_cst_test_id_twin_sync *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_sync;

typedef struct wire_cst_feed_id_twin_sync {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_sync;

typedef struct wire_cst_blob_twin_sync {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_sync;

typedef struct wire_cst_message_id_twin_sync {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_sync;

typedef struct wire_cst_customized_twin_rust_async {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_rust_async;

typedef struct wire_cst_user_id_twin_rust_async {
  uint32_t value;
} wire_cst_user_id_twin_rust_async;

typedef struct wire_cst_customized_twin_sync {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_sync;

typedef struct wire_cst_user_id_twin_sync {
  uint32_t value;
} wire_cst_user_id_twin_sync;

typedef struct wire_cst_feature_chrono_twin_rust_async {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_cst_feature_chrono_twin_rust_async;

typedef struct wire_cst_feature_chrono_twin_sync {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_cst_feature_chrono_twin_sync;

typedef struct wire_cst_struct_with_comments_twin_rust_async {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_rust_async;

typedef struct wire_cst_struct_with_comments_twin_sync {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_sync;

typedef struct wire_cst_EnumDartOpaqueTwinRustAsync_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinRustAsync_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinRustAsync_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinRustAsync_Opaque;

typedef union EnumDartOpaqueTwinRustAsyncKind {
  struct wire_cst_EnumDartOpaqueTwinRustAsync_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinRustAsync_Opaque *Opaque;
} EnumDartOpaqueTwinRustAsyncKind;

typedef struct wire_cst_enum_dart_opaque_twin_rust_async {
  int32_t tag;
  union EnumDartOpaqueTwinRustAsyncKind *kind;
} wire_cst_enum_dart_opaque_twin_rust_async;

typedef struct wire_cst_dart_opaque_nested_twin_rust_async {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_rust_async;

typedef struct wire_cst_EnumDartOpaqueTwinSync_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinSync_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinSync_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinSync_Opaque;

typedef union EnumDartOpaqueTwinSyncKind {
  struct wire_cst_EnumDartOpaqueTwinSync_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinSync_Opaque *Opaque;
} EnumDartOpaqueTwinSyncKind;

typedef struct wire_cst_enum_dart_opaque_twin_sync {
  int32_t tag;
  union EnumDartOpaqueTwinSyncKind *kind;
} wire_cst_enum_dart_opaque_twin_sync;

typedef struct wire_cst_dart_opaque_nested_twin_sync {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_sync;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsync_A {

} wire_cst_EnumWithItemMixedTwinRustAsync_A;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsync_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinRustAsync_B;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsync_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinRustAsync_C;

typedef union EnumWithItemMixedTwinRustAsyncKind {
  struct wire_cst_EnumWithItemMixedTwinRustAsync_A *A;
  struct wire_cst_EnumWithItemMixedTwinRustAsync_B *B;
  struct wire_cst_EnumWithItemMixedTwinRustAsync_C *C;
} EnumWithItemMixedTwinRustAsyncKind;

typedef struct wire_cst_enum_with_item_mixed_twin_rust_async {
  int32_t tag;
  union EnumWithItemMixedTwinRustAsyncKind *kind;
} wire_cst_enum_with_item_mixed_twin_rust_async;

typedef struct wire_cst_EnumWithItemStructTwinRustAsync_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinRustAsync_A;

typedef struct wire_cst_EnumWithItemStructTwinRustAsync_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinRustAsync_B;

typedef union EnumWithItemStructTwinRustAsyncKind {
  struct wire_cst_EnumWithItemStructTwinRustAsync_A *A;
  struct wire_cst_EnumWithItemStructTwinRustAsync_B *B;
} EnumWithItemStructTwinRustAsyncKind;

typedef struct wire_cst_enum_with_item_struct_twin_rust_async {
  int32_t tag;
  union EnumWithItemStructTwinRustAsyncKind *kind;
} wire_cst_enum_with_item_struct_twin_rust_async;

typedef struct wire_cst_EnumWithItemTupleTwinRustAsync_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinRustAsync_A;

typedef struct wire_cst_EnumWithItemTupleTwinRustAsync_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinRustAsync_B;

typedef union EnumWithItemTupleTwinRustAsyncKind {
  struct wire_cst_EnumWithItemTupleTwinRustAsync_A *A;
  struct wire_cst_EnumWithItemTupleTwinRustAsync_B *B;
} EnumWithItemTupleTwinRustAsyncKind;

typedef struct wire_cst_enum_with_item_tuple_twin_rust_async {
  int32_t tag;
  union EnumWithItemTupleTwinRustAsyncKind *kind;
} wire_cst_enum_with_item_tuple_twin_rust_async;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Empty {

} wire_cst_KitchenSinkTwinRustAsync_Empty;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinRustAsync_Primitives;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_rust_async *field1;
} wire_cst_KitchenSinkTwinRustAsync_Nested;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinRustAsync_Optional;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinRustAsync_Buffer;

typedef struct wire_cst_KitchenSinkTwinRustAsync_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinRustAsync_Enums;

typedef union KitchenSinkTwinRustAsyncKind {
  struct wire_cst_KitchenSinkTwinRustAsync_Empty *Empty;
  struct wire_cst_KitchenSinkTwinRustAsync_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinRustAsync_Nested *Nested;
  struct wire_cst_KitchenSinkTwinRustAsync_Optional *Optional;
  struct wire_cst_KitchenSinkTwinRustAsync_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinRustAsync_Enums *Enums;
} KitchenSinkTwinRustAsyncKind;

typedef struct wire_cst_kitchen_sink_twin_rust_async {
  int32_t tag;
  union KitchenSinkTwinRustAsyncKind *kind;
} wire_cst_kitchen_sink_twin_rust_async;

typedef struct wire_cst_SpeedTwinRustAsync_Unknown {

} wire_cst_SpeedTwinRustAsync_Unknown;

typedef struct wire_cst_SpeedTwinRustAsync_GPS {
  double field0;
} wire_cst_SpeedTwinRustAsync_GPS;

typedef union SpeedTwinRustAsyncKind {
  struct wire_cst_SpeedTwinRustAsync_Unknown *Unknown;
  struct wire_cst_SpeedTwinRustAsync_GPS *GPS;
} SpeedTwinRustAsyncKind;

typedef struct wire_cst_speed_twin_rust_async {
  int32_t tag;
  union SpeedTwinRustAsyncKind *kind;
} wire_cst_speed_twin_rust_async;

typedef struct wire_cst_MeasureTwinRustAsync_Speed {
  struct wire_cst_speed_twin_rust_async *field0;
} wire_cst_MeasureTwinRustAsync_Speed;

typedef struct wire_cst_DistanceTwinRustAsync_Unknown {

} wire_cst_DistanceTwinRustAsync_Unknown;

typedef struct wire_cst_DistanceTwinRustAsync_Map {
  double field0;
} wire_cst_DistanceTwinRustAsync_Map;

typedef union DistanceTwinRustAsyncKind {
  struct wire_cst_DistanceTwinRustAsync_Unknown *Unknown;
  struct wire_cst_DistanceTwinRustAsync_Map *Map;
} DistanceTwinRustAsyncKind;

typedef struct wire_cst_distance_twin_rust_async {
  int32_t tag;
  union DistanceTwinRustAsyncKind *kind;
} wire_cst_distance_twin_rust_async;

typedef struct wire_cst_MeasureTwinRustAsync_Distance {
  struct wire_cst_distance_twin_rust_async *field0;
} wire_cst_MeasureTwinRustAsync_Distance;

typedef union MeasureTwinRustAsyncKind {
  struct wire_cst_MeasureTwinRustAsync_Speed *Speed;
  struct wire_cst_MeasureTwinRustAsync_Distance *Distance;
} MeasureTwinRustAsyncKind;

typedef struct wire_cst_measure_twin_rust_async {
  int32_t tag;
  union MeasureTwinRustAsyncKind *kind;
} wire_cst_measure_twin_rust_async;

typedef struct wire_cst_note_twin_rust_async {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_rust_async;

typedef struct wire_cst_EnumWithItemMixedTwinSync_A {

} wire_cst_EnumWithItemMixedTwinSync_A;

typedef struct wire_cst_EnumWithItemMixedTwinSync_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinSync_B;

typedef struct wire_cst_EnumWithItemMixedTwinSync_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinSync_C;

typedef union EnumWithItemMixedTwinSyncKind {
  struct wire_cst_EnumWithItemMixedTwinSync_A *A;
  struct wire_cst_EnumWithItemMixedTwinSync_B *B;
  struct wire_cst_EnumWithItemMixedTwinSync_C *C;
} EnumWithItemMixedTwinSyncKind;

typedef struct wire_cst_enum_with_item_mixed_twin_sync {
  int32_t tag;
  union EnumWithItemMixedTwinSyncKind *kind;
} wire_cst_enum_with_item_mixed_twin_sync;

typedef struct wire_cst_EnumWithItemStructTwinSync_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinSync_A;

typedef struct wire_cst_EnumWithItemStructTwinSync_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinSync_B;

typedef union EnumWithItemStructTwinSyncKind {
  struct wire_cst_EnumWithItemStructTwinSync_A *A;
  struct wire_cst_EnumWithItemStructTwinSync_B *B;
} EnumWithItemStructTwinSyncKind;

typedef struct wire_cst_enum_with_item_struct_twin_sync {
  int32_t tag;
  union EnumWithItemStructTwinSyncKind *kind;
} wire_cst_enum_with_item_struct_twin_sync;

typedef struct wire_cst_EnumWithItemTupleTwinSync_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinSync_A;

typedef struct wire_cst_EnumWithItemTupleTwinSync_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinSync_B;

typedef union EnumWithItemTupleTwinSyncKind {
  struct wire_cst_EnumWithItemTupleTwinSync_A *A;
  struct wire_cst_EnumWithItemTupleTwinSync_B *B;
} EnumWithItemTupleTwinSyncKind;

typedef struct wire_cst_enum_with_item_tuple_twin_sync {
  int32_t tag;
  union EnumWithItemTupleTwinSyncKind *kind;
} wire_cst_enum_with_item_tuple_twin_sync;

typedef struct wire_cst_KitchenSinkTwinSync_Empty {

} wire_cst_KitchenSinkTwinSync_Empty;

typedef struct wire_cst_KitchenSinkTwinSync_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinSync_Primitives;

typedef struct wire_cst_KitchenSinkTwinSync_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_sync *field1;
} wire_cst_KitchenSinkTwinSync_Nested;

typedef struct wire_cst_KitchenSinkTwinSync_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinSync_Optional;

typedef struct wire_cst_KitchenSinkTwinSync_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinSync_Buffer;

typedef struct wire_cst_KitchenSinkTwinSync_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinSync_Enums;

typedef union KitchenSinkTwinSyncKind {
  struct wire_cst_KitchenSinkTwinSync_Empty *Empty;
  struct wire_cst_KitchenSinkTwinSync_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinSync_Nested *Nested;
  struct wire_cst_KitchenSinkTwinSync_Optional *Optional;
  struct wire_cst_KitchenSinkTwinSync_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinSync_Enums *Enums;
} KitchenSinkTwinSyncKind;

typedef struct wire_cst_kitchen_sink_twin_sync {
  int32_t tag;
  union KitchenSinkTwinSyncKind *kind;
} wire_cst_kitchen_sink_twin_sync;

typedef struct wire_cst_SpeedTwinSync_Unknown {

} wire_cst_SpeedTwinSync_Unknown;

typedef struct wire_cst_SpeedTwinSync_GPS {
  double field0;
} wire_cst_SpeedTwinSync_GPS;

typedef union SpeedTwinSyncKind {
  struct wire_cst_SpeedTwinSync_Unknown *Unknown;
  struct wire_cst_SpeedTwinSync_GPS *GPS;
} SpeedTwinSyncKind;

typedef struct wire_cst_speed_twin_sync {
  int32_t tag;
  union SpeedTwinSyncKind *kind;
} wire_cst_speed_twin_sync;

typedef struct wire_cst_MeasureTwinSync_Speed {
  struct wire_cst_speed_twin_sync *field0;
} wire_cst_MeasureTwinSync_Speed;

typedef struct wire_cst_DistanceTwinSync_Unknown {

} wire_cst_DistanceTwinSync_Unknown;

typedef struct wire_cst_DistanceTwinSync_Map {
  double field0;
} wire_cst_DistanceTwinSync_Map;

typedef union DistanceTwinSyncKind {
  struct wire_cst_DistanceTwinSync_Unknown *Unknown;
  struct wire_cst_DistanceTwinSync_Map *Map;
} DistanceTwinSyncKind;

typedef struct wire_cst_distance_twin_sync {
  int32_t tag;
  union DistanceTwinSyncKind *kind;
} wire_cst_distance_twin_sync;

typedef struct wire_cst_MeasureTwinSync_Distance {
  struct wire_cst_distance_twin_sync *field0;
} wire_cst_MeasureTwinSync_Distance;

typedef union MeasureTwinSyncKind {
  struct wire_cst_MeasureTwinSync_Speed *Speed;
  struct wire_cst_MeasureTwinSync_Distance *Distance;
} MeasureTwinSyncKind;

typedef struct wire_cst_measure_twin_sync {
  int32_t tag;
  union MeasureTwinSyncKind *kind;
} wire_cst_measure_twin_sync;

typedef struct wire_cst_note_twin_sync {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_sync;

typedef struct wire_cst_event_twin_rust_async {
  struct wire_cst_list_prim_u_8 *address;
  struct wire_cst_list_prim_u_8 *payload;
} wire_cst_event_twin_rust_async;

typedef struct wire_cst_custom_struct_twin_rust_async {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_rust_async;

typedef struct wire_cst_some_struct_twin_rust_async {
  uint32_t value;
} wire_cst_some_struct_twin_rust_async;

typedef struct wire_cst_CustomNestedErrorOuterTwinRustAsync_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinRustAsync_One;

typedef struct wire_cst_CustomNestedErrorInnerTwinRustAsync_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinRustAsync_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinRustAsync_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinRustAsync_Four;

typedef union CustomNestedErrorInnerTwinRustAsyncKind {
  struct wire_cst_CustomNestedErrorInnerTwinRustAsync_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinRustAsync_Four *Four;
} CustomNestedErrorInnerTwinRustAsyncKind;

typedef struct wire_cst_custom_nested_error_inner_twin_rust_async {
  int32_t tag;
  union CustomNestedErrorInnerTwinRustAsyncKind *kind;
} wire_cst_custom_nested_error_inner_twin_rust_async;

typedef struct wire_cst_CustomNestedErrorOuterTwinRustAsync_Two {
  struct wire_cst_custom_nested_error_inner_twin_rust_async *field0;
} wire_cst_CustomNestedErrorOuterTwinRustAsync_Two;

typedef union CustomNestedErrorOuterTwinRustAsyncKind {
  struct wire_cst_CustomNestedErrorOuterTwinRustAsync_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinRustAsync_Two *Two;
} CustomNestedErrorOuterTwinRustAsyncKind;

typedef struct wire_cst_custom_nested_error_outer_twin_rust_async {
  int32_t tag;
  union CustomNestedErrorOuterTwinRustAsyncKind *kind;
} wire_cst_custom_nested_error_outer_twin_rust_async;

typedef struct wire_cst_custom_struct_error_twin_rust_async {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_rust_async;

typedef struct wire_cst_custom_struct_twin_sync {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_sync;

typedef struct wire_cst_some_struct_twin_sync {
  uint32_t value;
} wire_cst_some_struct_twin_sync;

typedef struct wire_cst_CustomNestedErrorOuterTwinSync_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinSync_One;

typedef struct wire_cst_CustomNestedErrorInnerTwinSync_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinSync_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinSync_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinSync_Four;

typedef union CustomNestedErrorInnerTwinSyncKind {
  struct wire_cst_CustomNestedErrorInnerTwinSync_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinSync_Four *Four;
} CustomNestedErrorInnerTwinSyncKind;

typedef struct wire_cst_custom_nested_error_inner_twin_sync {
  int32_t tag;
  union CustomNestedErrorInnerTwinSyncKind *kind;
} wire_cst_custom_nested_error_inner_twin_sync;

typedef struct wire_cst_CustomNestedErrorOuterTwinSync_Two {
  struct wire_cst_custom_nested_error_inner_twin_sync *field0;
} wire_cst_CustomNestedErrorOuterTwinSync_Two;

typedef union CustomNestedErrorOuterTwinSyncKind {
  struct wire_cst_CustomNestedErrorOuterTwinSync_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinSync_Two *Two;
} CustomNestedErrorOuterTwinSyncKind;

typedef struct wire_cst_custom_nested_error_outer_twin_sync {
  int32_t tag;
  union CustomNestedErrorOuterTwinSyncKind *kind;
} wire_cst_custom_nested_error_outer_twin_sync;

typedef struct wire_cst_custom_struct_error_twin_sync {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_sync;

typedef struct wire_cst_concatenate_with_twin_rust_async {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_rust_async;

typedef struct wire_cst_sum_with_twin_rust_async {
  uint32_t x;
} wire_cst_sum_with_twin_rust_async;

typedef struct wire_cst_concatenate_with_twin_sync {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_sync;

typedef struct wire_cst_sum_with_twin_sync {
  uint32_t x;
} wire_cst_sum_with_twin_sync;

typedef struct wire_cst_list_my_tree_node_twin_rust_async {
  struct wire_cst_my_tree_node_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_rust_async;

typedef struct wire_cst_my_tree_node_twin_rust_async {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_rust_async *children;
} wire_cst_my_tree_node_twin_rust_async;

typedef struct wire_cst_my_nested_struct_twin_rust_async {
  struct wire_cst_my_tree_node_twin_rust_async tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_rust_async;

typedef struct wire_cst_list_weekdays_twin_rust_async {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_rust_async;

typedef struct wire_cst_a_twin_rust_async {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_rust_async;

typedef struct wire_cst_AbcTwinRustAsync_A {
  struct wire_cst_a_twin_rust_async *field0;
} wire_cst_AbcTwinRustAsync_A;

typedef struct wire_cst_b_twin_rust_async {
  int32_t b;
} wire_cst_b_twin_rust_async;

typedef struct wire_cst_AbcTwinRustAsync_B {
  struct wire_cst_b_twin_rust_async *field0;
} wire_cst_AbcTwinRustAsync_B;

typedef struct wire_cst_c_twin_rust_async {
  bool c;
} wire_cst_c_twin_rust_async;

typedef struct wire_cst_AbcTwinRustAsync_C {
  struct wire_cst_c_twin_rust_async *field0;
} wire_cst_AbcTwinRustAsync_C;

typedef struct wire_cst_AbcTwinRustAsync_JustInt {
  int32_t field0;
} wire_cst_AbcTwinRustAsync_JustInt;

typedef union AbcTwinRustAsyncKind {
  struct wire_cst_AbcTwinRustAsync_A *A;
  struct wire_cst_AbcTwinRustAsync_B *B;
  struct wire_cst_AbcTwinRustAsync_C *C;
  struct wire_cst_AbcTwinRustAsync_JustInt *JustInt;
} AbcTwinRustAsyncKind;

typedef struct wire_cst_abc_twin_rust_async {
  int32_t tag;
  union AbcTwinRustAsyncKind *kind;
} wire_cst_abc_twin_rust_async;

typedef struct wire_cst_struct_with_enum_twin_rust_async {
  struct wire_cst_abc_twin_rust_async abc1;
  struct wire_cst_abc_twin_rust_async abc2;
} wire_cst_struct_with_enum_twin_rust_async;

typedef struct wire_cst_list_my_tree_node_twin_sync {
  struct wire_cst_my_tree_node_twin_sync *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_sync;

typedef struct wire_cst_my_tree_node_twin_sync {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_sync *children;
} wire_cst_my_tree_node_twin_sync;

typedef struct wire_cst_my_nested_struct_twin_sync {
  struct wire_cst_my_tree_node_twin_sync tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_sync;

typedef struct wire_cst_list_weekdays_twin_sync {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_sync;

typedef struct wire_cst_a_twin_sync {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_sync;

typedef struct wire_cst_AbcTwinSync_A {
  struct wire_cst_a_twin_sync *field0;
} wire_cst_AbcTwinSync_A;

typedef struct wire_cst_b_twin_sync {
  int32_t b;
} wire_cst_b_twin_sync;

typedef struct wire_cst_AbcTwinSync_B {
  struct wire_cst_b_twin_sync *field0;
} wire_cst_AbcTwinSync_B;

typedef struct wire_cst_c_twin_sync {
  bool c;
} wire_cst_c_twin_sync;

typedef struct wire_cst_AbcTwinSync_C {
  struct wire_cst_c_twin_sync *field0;
} wire_cst_AbcTwinSync_C;

typedef struct wire_cst_AbcTwinSync_JustInt {
  int32_t field0;
} wire_cst_AbcTwinSync_JustInt;

typedef union AbcTwinSyncKind {
  struct wire_cst_AbcTwinSync_A *A;
  struct wire_cst_AbcTwinSync_B *B;
  struct wire_cst_AbcTwinSync_C *C;
  struct wire_cst_AbcTwinSync_JustInt *JustInt;
} AbcTwinSyncKind;

typedef struct wire_cst_abc_twin_sync {
  int32_t tag;
  union AbcTwinSyncKind *kind;
} wire_cst_abc_twin_sync;

typedef struct wire_cst_struct_with_enum_twin_sync {
  struct wire_cst_abc_twin_sync abc1;
  struct wire_cst_abc_twin_sync abc2;
} wire_cst_struct_with_enum_twin_sync;

typedef struct wire_cst_empty_twin_rust_async {

} wire_cst_empty_twin_rust_async;

typedef struct wire_cst_empty_twin_sync {

} wire_cst_empty_twin_sync;

typedef struct wire_cst_new_type_int_twin_rust_async {
  int64_t field0;
} wire_cst_new_type_int_twin_rust_async;

typedef struct wire_cst_new_type_int_twin_sync {
  int64_t field0;
} wire_cst_new_type_int_twin_sync;

typedef struct wire_cst_attribute_twin_rust_async {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_rust_async;

typedef struct wire_cst_list_attribute_twin_rust_async {
  struct wire_cst_attribute_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_rust_async;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async {
  struct wire_cst_attribute_twin_rust_async **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_rust_async;

typedef struct wire_cst_exotic_optionals_twin_rust_async {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_rust_async *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async *nullable_attributes;
  struct wire_cst_new_type_int_twin_rust_async *newtypeint;
} wire_cst_exotic_optionals_twin_rust_async;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async;

typedef struct wire_cst_opt_vecs_twin_rust_async {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_rust_async;

typedef struct wire_cst_attribute_twin_sync {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_sync;

typedef struct wire_cst_list_attribute_twin_sync {
  struct wire_cst_attribute_twin_sync *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_sync;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_sync {
  struct wire_cst_attribute_twin_sync **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_sync;

typedef struct wire_cst_exotic_optionals_twin_sync {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_sync *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sync *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sync *nullable_attributes;
  struct wire_cst_new_type_int_twin_sync *newtypeint;
} wire_cst_exotic_optionals_twin_sync;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_sync;

typedef struct wire_cst_opt_vecs_twin_sync {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_sync;

typedef struct wire_cst_list_bool {
  bool *ptr;
  int32_t len;
} wire_cst_list_bool;

typedef struct wire_cst_list_prim_i_16 {
  int16_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_16;

typedef struct wire_cst_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_64;

typedef struct wire_cst_list_prim_u_16 {
  uint16_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_16;

typedef struct wire_cst_list_prim_u_32 {
  uint32_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_32;

typedef struct wire_cst_list_prim_u_64 {
  uint64_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_64;

typedef struct wire_cst_list_RustOpaque_hide_data {
  const void **ptr;
  int32_t len;
} wire_cst_list_RustOpaque_hide_data;

typedef struct wire_cst_EnumOpaqueTwinRustAsync_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsync_Struct;

typedef struct wire_cst_EnumOpaqueTwinRustAsync_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsync_Primitive;

typedef struct wire_cst_EnumOpaqueTwinRustAsync_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsync_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinRustAsync_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsync_Mutex;

typedef struct wire_cst_EnumOpaqueTwinRustAsync_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsync_RwLock;

typedef union EnumOpaqueTwinRustAsyncKind {
  struct wire_cst_EnumOpaqueTwinRustAsync_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinRustAsync_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinRustAsync_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinRustAsync_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinRustAsync_RwLock *RwLock;
} EnumOpaqueTwinRustAsyncKind;

typedef struct wire_cst_enum_opaque_twin_rust_async {
  int32_t tag;
  union EnumOpaqueTwinRustAsyncKind *kind;
} wire_cst_enum_opaque_twin_rust_async;

typedef struct wire_cst_opaque_nested_twin_rust_async {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_rust_async;

typedef struct wire_cst_EnumOpaqueTwinSync_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinSync_Struct;

typedef struct wire_cst_EnumOpaqueTwinSync_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinSync_Primitive;

typedef struct wire_cst_EnumOpaqueTwinSync_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinSync_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinSync_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinSync_Mutex;

typedef struct wire_cst_EnumOpaqueTwinSync_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinSync_RwLock;

typedef union EnumOpaqueTwinSyncKind {
  struct wire_cst_EnumOpaqueTwinSync_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinSync_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinSync_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinSync_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinSync_RwLock *RwLock;
} EnumOpaqueTwinSyncKind;

typedef struct wire_cst_enum_opaque_twin_sync {
  int32_t tag;
  union EnumOpaqueTwinSyncKind *kind;
} wire_cst_enum_opaque_twin_sync;

typedef struct wire_cst_opaque_nested_twin_sync {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_sync;

typedef struct wire_cst_struct_with_one_field_twin_rust_async {
  int32_t a;
} wire_cst_struct_with_one_field_twin_rust_async;

typedef struct wire_cst_struct_with_two_field_twin_rust_async {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_rust_async;

typedef struct wire_cst_struct_with_zero_field_twin_rust_async {

} wire_cst_struct_with_zero_field_twin_rust_async;

typedef struct wire_cst_tuple_struct_with_one_field_twin_rust_async {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_rust_async;

typedef struct wire_cst_tuple_struct_with_two_field_twin_rust_async {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_rust_async;

typedef struct wire_cst_struct_with_one_field_twin_sync {
  int32_t a;
} wire_cst_struct_with_one_field_twin_sync;

typedef struct wire_cst_struct_with_two_field_twin_sync {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_sync;

typedef struct wire_cst_struct_with_zero_field_twin_sync {

} wire_cst_struct_with_zero_field_twin_sync;

typedef struct wire_cst_tuple_struct_with_one_field_twin_sync {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_sync;

typedef struct wire_cst_tuple_struct_with_two_field_twin_sync {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_sync;

typedef struct wire_cst_record_string_i_32 {
  struct wire_cst_list_prim_u_8 *field0;
  int32_t field1;
} wire_cst_record_string_i_32;

typedef struct wire_cst_list_record_string_i_32 {
  struct wire_cst_record_string_i_32 *ptr;
  int32_t len;
} wire_cst_list_record_string_i_32;

typedef struct wire_cst_feature_uuid_twin_rust_async {
  struct wire_cst_list_prim_u_8 *one;
} wire_cst_feature_uuid_twin_rust_async;

typedef struct wire_cst_feature_uuid_twin_sync {
  struct wire_cst_list_prim_u_8 *one;
} wire_cst_feature_uuid_twin_sync;

typedef struct wire_cst_EnumOpaqueTwinNormal_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinNormal_Struct;

typedef struct wire_cst_EnumOpaqueTwinNormal_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinNormal_Primitive;

typedef struct wire_cst_EnumOpaqueTwinNormal_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinNormal_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinNormal_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinNormal_Mutex;

typedef struct wire_cst_EnumOpaqueTwinNormal_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinNormal_RwLock;

typedef union EnumOpaqueTwinNormalKind {
  struct wire_cst_EnumOpaqueTwinNormal_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinNormal_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinNormal_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinNormal_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinNormal_RwLock *RwLock;
} EnumOpaqueTwinNormalKind;

typedef struct wire_cst_enum_opaque_twin_normal {
  int32_t tag;
  union EnumOpaqueTwinNormalKind *kind;
} wire_cst_enum_opaque_twin_normal;

typedef struct wire_cst_opaque_nested_twin_normal {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_normal;

typedef struct wire_cst_struct_with_one_field_twin_normal {
  int32_t a;
} wire_cst_struct_with_one_field_twin_normal;

typedef struct wire_cst_struct_with_two_field_twin_normal {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_normal;

typedef struct wire_cst_struct_with_zero_field_twin_normal {

} wire_cst_struct_with_zero_field_twin_normal;

typedef struct wire_cst_tuple_struct_with_one_field_twin_normal {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_normal;

typedef struct wire_cst_tuple_struct_with_two_field_twin_normal {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_normal;

typedef struct wire_cst_feature_uuid_twin_normal {
  struct wire_cst_list_prim_u_8 *one;
} wire_cst_feature_uuid_twin_normal;

typedef struct wire_cst_a_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_rust_async_sse;

typedef struct wire_cst_a_twin_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_sse;

typedef struct wire_cst_a_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_a_twin_sync_sse;

typedef struct wire_cst_AbcTwinRustAsyncSse_A {
  struct wire_cst_a_twin_rust_async_sse *field0;
} wire_cst_AbcTwinRustAsyncSse_A;

typedef struct wire_cst_b_twin_rust_async_sse {
  int32_t b;
} wire_cst_b_twin_rust_async_sse;

typedef struct wire_cst_AbcTwinRustAsyncSse_B {
  struct wire_cst_b_twin_rust_async_sse *field0;
} wire_cst_AbcTwinRustAsyncSse_B;

typedef struct wire_cst_c_twin_rust_async_sse {
  bool c;
} wire_cst_c_twin_rust_async_sse;

typedef struct wire_cst_AbcTwinRustAsyncSse_C {
  struct wire_cst_c_twin_rust_async_sse *field0;
} wire_cst_AbcTwinRustAsyncSse_C;

typedef struct wire_cst_AbcTwinRustAsyncSse_JustInt {
  int32_t field0;
} wire_cst_AbcTwinRustAsyncSse_JustInt;

typedef union AbcTwinRustAsyncSseKind {
  struct wire_cst_AbcTwinRustAsyncSse_A *A;
  struct wire_cst_AbcTwinRustAsyncSse_B *B;
  struct wire_cst_AbcTwinRustAsyncSse_C *C;
  struct wire_cst_AbcTwinRustAsyncSse_JustInt *JustInt;
} AbcTwinRustAsyncSseKind;

typedef struct wire_cst_abc_twin_rust_async_sse {
  int32_t tag;
  union AbcTwinRustAsyncSseKind *kind;
} wire_cst_abc_twin_rust_async_sse;

typedef struct wire_cst_AbcTwinSse_A {
  struct wire_cst_a_twin_sse *field0;
} wire_cst_AbcTwinSse_A;

typedef struct wire_cst_b_twin_sse {
  int32_t b;
} wire_cst_b_twin_sse;

typedef struct wire_cst_AbcTwinSse_B {
  struct wire_cst_b_twin_sse *field0;
} wire_cst_AbcTwinSse_B;

typedef struct wire_cst_c_twin_sse {
  bool c;
} wire_cst_c_twin_sse;

typedef struct wire_cst_AbcTwinSse_C {
  struct wire_cst_c_twin_sse *field0;
} wire_cst_AbcTwinSse_C;

typedef struct wire_cst_AbcTwinSse_JustInt {
  int32_t field0;
} wire_cst_AbcTwinSse_JustInt;

typedef union AbcTwinSseKind {
  struct wire_cst_AbcTwinSse_A *A;
  struct wire_cst_AbcTwinSse_B *B;
  struct wire_cst_AbcTwinSse_C *C;
  struct wire_cst_AbcTwinSse_JustInt *JustInt;
} AbcTwinSseKind;

typedef struct wire_cst_abc_twin_sse {
  int32_t tag;
  union AbcTwinSseKind *kind;
} wire_cst_abc_twin_sse;

typedef struct wire_cst_AbcTwinSyncSse_A {
  struct wire_cst_a_twin_sync_sse *field0;
} wire_cst_AbcTwinSyncSse_A;

typedef struct wire_cst_b_twin_sync_sse {
  int32_t b;
} wire_cst_b_twin_sync_sse;

typedef struct wire_cst_AbcTwinSyncSse_B {
  struct wire_cst_b_twin_sync_sse *field0;
} wire_cst_AbcTwinSyncSse_B;

typedef struct wire_cst_c_twin_sync_sse {
  bool c;
} wire_cst_c_twin_sync_sse;

typedef struct wire_cst_AbcTwinSyncSse_C {
  struct wire_cst_c_twin_sync_sse *field0;
} wire_cst_AbcTwinSyncSse_C;

typedef struct wire_cst_AbcTwinSyncSse_JustInt {
  int32_t field0;
} wire_cst_AbcTwinSyncSse_JustInt;

typedef union AbcTwinSyncSseKind {
  struct wire_cst_AbcTwinSyncSse_A *A;
  struct wire_cst_AbcTwinSyncSse_B *B;
  struct wire_cst_AbcTwinSyncSse_C *C;
  struct wire_cst_AbcTwinSyncSse_JustInt *JustInt;
} AbcTwinSyncSseKind;

typedef struct wire_cst_abc_twin_sync_sse {
  int32_t tag;
  union AbcTwinSyncSseKind *kind;
} wire_cst_abc_twin_sync_sse;

typedef struct wire_cst_attribute_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_rust_async_sse;

typedef struct wire_cst_attribute_twin_sse {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_sse;

typedef struct wire_cst_attribute_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_attribute_twin_sync_sse;

typedef struct wire_cst_concatenate_with_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_rust_async_sse;

typedef struct wire_cst_concatenate_with_twin_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_sse;

typedef struct wire_cst_concatenate_with_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_concatenate_with_twin_sync_sse;

typedef struct wire_cst_CustomNestedError2TwinNormal_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinNormal_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinNormal_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinNormal_CustomNested2Number;

typedef union CustomNestedError2TwinNormalKind {
  struct wire_cst_CustomNestedError2TwinNormal_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinNormal_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinNormalKind;

typedef struct wire_cst_custom_nested_error_2_twin_normal {
  int32_t tag;
  union CustomNestedError2TwinNormalKind *kind;
} wire_cst_custom_nested_error_2_twin_normal;

typedef struct wire_cst_CustomNestedError2TwinRustAsync_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinRustAsync_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinRustAsync_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinRustAsync_CustomNested2Number;

typedef union CustomNestedError2TwinRustAsyncKind {
  struct wire_cst_CustomNestedError2TwinRustAsync_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinRustAsync_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinRustAsyncKind;

typedef struct wire_cst_custom_nested_error_2_twin_rust_async {
  int32_t tag;
  union CustomNestedError2TwinRustAsyncKind *kind;
} wire_cst_custom_nested_error_2_twin_rust_async;

typedef struct wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2Number;

typedef union CustomNestedError2TwinRustAsyncSseKind {
  struct wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinRustAsyncSse_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinRustAsyncSseKind;

typedef struct wire_cst_custom_nested_error_2_twin_rust_async_sse {
  int32_t tag;
  union CustomNestedError2TwinRustAsyncSseKind *kind;
} wire_cst_custom_nested_error_2_twin_rust_async_sse;

typedef struct wire_cst_CustomNestedError2TwinSse_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinSse_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinSse_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinSse_CustomNested2Number;

typedef union CustomNestedError2TwinSseKind {
  struct wire_cst_CustomNestedError2TwinSse_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinSse_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinSseKind;

typedef struct wire_cst_custom_nested_error_2_twin_sse {
  int32_t tag;
  union CustomNestedError2TwinSseKind *kind;
} wire_cst_custom_nested_error_2_twin_sse;

typedef struct wire_cst_CustomNestedError2TwinSync_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinSync_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinSync_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinSync_CustomNested2Number;

typedef union CustomNestedError2TwinSyncKind {
  struct wire_cst_CustomNestedError2TwinSync_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinSync_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinSyncKind;

typedef struct wire_cst_custom_nested_error_2_twin_sync {
  int32_t tag;
  union CustomNestedError2TwinSyncKind *kind;
} wire_cst_custom_nested_error_2_twin_sync;

typedef struct wire_cst_CustomNestedError2TwinSyncSse_CustomNested2 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError2TwinSyncSse_CustomNested2;

typedef struct wire_cst_CustomNestedError2TwinSyncSse_CustomNested2Number {
  uint32_t field0;
} wire_cst_CustomNestedError2TwinSyncSse_CustomNested2Number;

typedef union CustomNestedError2TwinSyncSseKind {
  struct wire_cst_CustomNestedError2TwinSyncSse_CustomNested2 *CustomNested2;
  struct wire_cst_CustomNestedError2TwinSyncSse_CustomNested2Number *CustomNested2Number;
} CustomNestedError2TwinSyncSseKind;

typedef struct wire_cst_custom_nested_error_2_twin_sync_sse {
  int32_t tag;
  union CustomNestedError2TwinSyncSseKind *kind;
} wire_cst_custom_nested_error_2_twin_sync_sse;

typedef struct wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Four;

typedef union CustomNestedErrorInnerTwinRustAsyncSseKind {
  struct wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinRustAsyncSse_Four *Four;
} CustomNestedErrorInnerTwinRustAsyncSseKind;

typedef struct wire_cst_custom_nested_error_inner_twin_rust_async_sse {
  int32_t tag;
  union CustomNestedErrorInnerTwinRustAsyncSseKind *kind;
} wire_cst_custom_nested_error_inner_twin_rust_async_sse;

typedef struct wire_cst_CustomNestedErrorInnerTwinSse_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinSse_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinSse_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinSse_Four;

typedef union CustomNestedErrorInnerTwinSseKind {
  struct wire_cst_CustomNestedErrorInnerTwinSse_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinSse_Four *Four;
} CustomNestedErrorInnerTwinSseKind;

typedef struct wire_cst_custom_nested_error_inner_twin_sse {
  int32_t tag;
  union CustomNestedErrorInnerTwinSseKind *kind;
} wire_cst_custom_nested_error_inner_twin_sse;

typedef struct wire_cst_CustomNestedErrorInnerTwinSyncSse_Three {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorInnerTwinSyncSse_Three;

typedef struct wire_cst_CustomNestedErrorInnerTwinSyncSse_Four {
  uint32_t field0;
} wire_cst_CustomNestedErrorInnerTwinSyncSse_Four;

typedef union CustomNestedErrorInnerTwinSyncSseKind {
  struct wire_cst_CustomNestedErrorInnerTwinSyncSse_Three *Three;
  struct wire_cst_CustomNestedErrorInnerTwinSyncSse_Four *Four;
} CustomNestedErrorInnerTwinSyncSseKind;

typedef struct wire_cst_custom_nested_error_inner_twin_sync_sse {
  int32_t tag;
  union CustomNestedErrorInnerTwinSyncSseKind *kind;
} wire_cst_custom_nested_error_inner_twin_sync_sse;

typedef struct wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_One;

typedef struct wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_Two {
  struct wire_cst_custom_nested_error_inner_twin_rust_async_sse *field0;
} wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_Two;

typedef union CustomNestedErrorOuterTwinRustAsyncSseKind {
  struct wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinRustAsyncSse_Two *Two;
} CustomNestedErrorOuterTwinRustAsyncSseKind;

typedef struct wire_cst_custom_nested_error_outer_twin_rust_async_sse {
  int32_t tag;
  union CustomNestedErrorOuterTwinRustAsyncSseKind *kind;
} wire_cst_custom_nested_error_outer_twin_rust_async_sse;

typedef struct wire_cst_CustomNestedErrorOuterTwinSse_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinSse_One;

typedef struct wire_cst_CustomNestedErrorOuterTwinSse_Two {
  struct wire_cst_custom_nested_error_inner_twin_sse *field0;
} wire_cst_CustomNestedErrorOuterTwinSse_Two;

typedef union CustomNestedErrorOuterTwinSseKind {
  struct wire_cst_CustomNestedErrorOuterTwinSse_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinSse_Two *Two;
} CustomNestedErrorOuterTwinSseKind;

typedef struct wire_cst_custom_nested_error_outer_twin_sse {
  int32_t tag;
  union CustomNestedErrorOuterTwinSseKind *kind;
} wire_cst_custom_nested_error_outer_twin_sse;

typedef struct wire_cst_CustomNestedErrorOuterTwinSyncSse_One {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedErrorOuterTwinSyncSse_One;

typedef struct wire_cst_CustomNestedErrorOuterTwinSyncSse_Two {
  struct wire_cst_custom_nested_error_inner_twin_sync_sse *field0;
} wire_cst_CustomNestedErrorOuterTwinSyncSse_Two;

typedef union CustomNestedErrorOuterTwinSyncSseKind {
  struct wire_cst_CustomNestedErrorOuterTwinSyncSse_One *One;
  struct wire_cst_CustomNestedErrorOuterTwinSyncSse_Two *Two;
} CustomNestedErrorOuterTwinSyncSseKind;

typedef struct wire_cst_custom_nested_error_outer_twin_sync_sse {
  int32_t tag;
  union CustomNestedErrorOuterTwinSyncSseKind *kind;
} wire_cst_custom_nested_error_outer_twin_sync_sse;

typedef struct wire_cst_custom_struct_error_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_rust_async_sse;

typedef struct wire_cst_custom_struct_error_twin_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_sse;

typedef struct wire_cst_custom_struct_error_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_custom_struct_error_twin_sync_sse;

typedef struct wire_cst_custom_struct_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_rust_async_sse;

typedef struct wire_cst_custom_struct_twin_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_sse;

typedef struct wire_cst_custom_struct_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_twin_sync_sse;

typedef struct wire_cst_customized_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_rust_async_sse;

typedef struct wire_cst_customized_twin_sse {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_sse;

typedef struct wire_cst_customized_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *final_field;
  struct wire_cst_list_prim_u_8 *non_final_field;
} wire_cst_customized_twin_sync_sse;

typedef struct wire_cst_dart_opaque_nested_twin_rust_async_sse {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_rust_async_sse;

typedef struct wire_cst_dart_opaque_nested_twin_sse {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_sse;

typedef struct wire_cst_dart_opaque_nested_twin_sync_sse {
  const void *first;
  const void *second;
} wire_cst_dart_opaque_nested_twin_sync_sse;

typedef struct wire_cst_list_element_twin_normal {
  struct wire_cst_element_twin_normal *ptr;
  int32_t len;
} wire_cst_list_element_twin_normal;

typedef struct wire_cst_element_twin_normal {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_normal *attributes;
  struct wire_cst_list_element_twin_normal *children;
} wire_cst_element_twin_normal;

typedef struct wire_cst_list_element_twin_rust_async {
  struct wire_cst_element_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_element_twin_rust_async;

typedef struct wire_cst_element_twin_rust_async {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_rust_async *attributes;
  struct wire_cst_list_element_twin_rust_async *children;
} wire_cst_element_twin_rust_async;

typedef struct wire_cst_list_attribute_twin_rust_async_sse {
  struct wire_cst_attribute_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_rust_async_sse;

typedef struct wire_cst_list_element_twin_rust_async_sse {
  struct wire_cst_element_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_element_twin_rust_async_sse;

typedef struct wire_cst_element_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_rust_async_sse *attributes;
  struct wire_cst_list_element_twin_rust_async_sse *children;
} wire_cst_element_twin_rust_async_sse;

typedef struct wire_cst_list_attribute_twin_sse {
  struct wire_cst_attribute_twin_sse *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_sse;

typedef struct wire_cst_list_element_twin_sse {
  struct wire_cst_element_twin_sse *ptr;
  int32_t len;
} wire_cst_list_element_twin_sse;

typedef struct wire_cst_element_twin_sse {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_sse *attributes;
  struct wire_cst_list_element_twin_sse *children;
} wire_cst_element_twin_sse;

typedef struct wire_cst_list_element_twin_sync {
  struct wire_cst_element_twin_sync *ptr;
  int32_t len;
} wire_cst_list_element_twin_sync;

typedef struct wire_cst_element_twin_sync {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_sync *attributes;
  struct wire_cst_list_element_twin_sync *children;
} wire_cst_element_twin_sync;

typedef struct wire_cst_list_attribute_twin_sync_sse {
  struct wire_cst_attribute_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_attribute_twin_sync_sse;

typedef struct wire_cst_list_element_twin_sync_sse {
  struct wire_cst_element_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_element_twin_sync_sse;

typedef struct wire_cst_element_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *tag;
  struct wire_cst_list_prim_u_8 *text;
  struct wire_cst_list_attribute_twin_sync_sse *attributes;
  struct wire_cst_list_element_twin_sync_sse *children;
} wire_cst_element_twin_sync_sse;

typedef struct wire_cst_empty_twin_rust_async_sse {

} wire_cst_empty_twin_rust_async_sse;

typedef struct wire_cst_empty_twin_sse {

} wire_cst_empty_twin_sse;

typedef struct wire_cst_empty_twin_sync_sse {

} wire_cst_empty_twin_sync_sse;

typedef struct wire_cst_EnumDartOpaqueTwinRustAsyncSse_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinRustAsyncSse_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinRustAsyncSse_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinRustAsyncSse_Opaque;

typedef union EnumDartOpaqueTwinRustAsyncSseKind {
  struct wire_cst_EnumDartOpaqueTwinRustAsyncSse_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinRustAsyncSse_Opaque *Opaque;
} EnumDartOpaqueTwinRustAsyncSseKind;

typedef struct wire_cst_enum_dart_opaque_twin_rust_async_sse {
  int32_t tag;
  union EnumDartOpaqueTwinRustAsyncSseKind *kind;
} wire_cst_enum_dart_opaque_twin_rust_async_sse;

typedef struct wire_cst_EnumDartOpaqueTwinSse_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinSse_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinSse_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinSse_Opaque;

typedef union EnumDartOpaqueTwinSseKind {
  struct wire_cst_EnumDartOpaqueTwinSse_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinSse_Opaque *Opaque;
} EnumDartOpaqueTwinSseKind;

typedef struct wire_cst_enum_dart_opaque_twin_sse {
  int32_t tag;
  union EnumDartOpaqueTwinSseKind *kind;
} wire_cst_enum_dart_opaque_twin_sse;

typedef struct wire_cst_EnumDartOpaqueTwinSyncSse_Primitive {
  int32_t field0;
} wire_cst_EnumDartOpaqueTwinSyncSse_Primitive;

typedef struct wire_cst_EnumDartOpaqueTwinSyncSse_Opaque {
  const void *field0;
} wire_cst_EnumDartOpaqueTwinSyncSse_Opaque;

typedef union EnumDartOpaqueTwinSyncSseKind {
  struct wire_cst_EnumDartOpaqueTwinSyncSse_Primitive *Primitive;
  struct wire_cst_EnumDartOpaqueTwinSyncSse_Opaque *Opaque;
} EnumDartOpaqueTwinSyncSseKind;

typedef struct wire_cst_enum_dart_opaque_twin_sync_sse {
  int32_t tag;
  union EnumDartOpaqueTwinSyncSseKind *kind;
} wire_cst_enum_dart_opaque_twin_sync_sse;

typedef struct wire_cst_EnumOpaqueTwinRustAsyncSse_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsyncSse_Struct;

typedef struct wire_cst_EnumOpaqueTwinRustAsyncSse_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsyncSse_Primitive;

typedef struct wire_cst_EnumOpaqueTwinRustAsyncSse_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsyncSse_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinRustAsyncSse_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsyncSse_Mutex;

typedef struct wire_cst_EnumOpaqueTwinRustAsyncSse_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinRustAsyncSse_RwLock;

typedef union EnumOpaqueTwinRustAsyncSseKind {
  struct wire_cst_EnumOpaqueTwinRustAsyncSse_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinRustAsyncSse_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinRustAsyncSse_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinRustAsyncSse_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinRustAsyncSse_RwLock *RwLock;
} EnumOpaqueTwinRustAsyncSseKind;

typedef struct wire_cst_enum_opaque_twin_rust_async_sse {
  int32_t tag;
  union EnumOpaqueTwinRustAsyncSseKind *kind;
} wire_cst_enum_opaque_twin_rust_async_sse;

typedef struct wire_cst_EnumOpaqueTwinSse_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinSse_Struct;

typedef struct wire_cst_EnumOpaqueTwinSse_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinSse_Primitive;

typedef struct wire_cst_EnumOpaqueTwinSse_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinSse_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinSse_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinSse_Mutex;

typedef struct wire_cst_EnumOpaqueTwinSse_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinSse_RwLock;

typedef union EnumOpaqueTwinSseKind {
  struct wire_cst_EnumOpaqueTwinSse_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinSse_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinSse_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinSse_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinSse_RwLock *RwLock;
} EnumOpaqueTwinSseKind;

typedef struct wire_cst_enum_opaque_twin_sse {
  int32_t tag;
  union EnumOpaqueTwinSseKind *kind;
} wire_cst_enum_opaque_twin_sse;

typedef struct wire_cst_EnumOpaqueTwinSyncSse_Struct {
  const void *field0;
} wire_cst_EnumOpaqueTwinSyncSse_Struct;

typedef struct wire_cst_EnumOpaqueTwinSyncSse_Primitive {
  const void *field0;
} wire_cst_EnumOpaqueTwinSyncSse_Primitive;

typedef struct wire_cst_EnumOpaqueTwinSyncSse_TraitObj {
  const void *field0;
} wire_cst_EnumOpaqueTwinSyncSse_TraitObj;

typedef struct wire_cst_EnumOpaqueTwinSyncSse_Mutex {
  const void *field0;
} wire_cst_EnumOpaqueTwinSyncSse_Mutex;

typedef struct wire_cst_EnumOpaqueTwinSyncSse_RwLock {
  const void *field0;
} wire_cst_EnumOpaqueTwinSyncSse_RwLock;

typedef union EnumOpaqueTwinSyncSseKind {
  struct wire_cst_EnumOpaqueTwinSyncSse_Struct *Struct;
  struct wire_cst_EnumOpaqueTwinSyncSse_Primitive *Primitive;
  struct wire_cst_EnumOpaqueTwinSyncSse_TraitObj *TraitObj;
  struct wire_cst_EnumOpaqueTwinSyncSse_Mutex *Mutex;
  struct wire_cst_EnumOpaqueTwinSyncSse_RwLock *RwLock;
} EnumOpaqueTwinSyncSseKind;

typedef struct wire_cst_enum_opaque_twin_sync_sse {
  int32_t tag;
  union EnumOpaqueTwinSyncSseKind *kind;
} wire_cst_enum_opaque_twin_sync_sse;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_A {

} wire_cst_EnumWithItemMixedTwinRustAsyncSse_A;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinRustAsyncSse_B;

typedef struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinRustAsyncSse_C;

typedef union EnumWithItemMixedTwinRustAsyncSseKind {
  struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_A *A;
  struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_B *B;
  struct wire_cst_EnumWithItemMixedTwinRustAsyncSse_C *C;
} EnumWithItemMixedTwinRustAsyncSseKind;

typedef struct wire_cst_enum_with_item_mixed_twin_rust_async_sse {
  int32_t tag;
  union EnumWithItemMixedTwinRustAsyncSseKind *kind;
} wire_cst_enum_with_item_mixed_twin_rust_async_sse;

typedef struct wire_cst_EnumWithItemMixedTwinSse_A {

} wire_cst_EnumWithItemMixedTwinSse_A;

typedef struct wire_cst_EnumWithItemMixedTwinSse_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinSse_B;

typedef struct wire_cst_EnumWithItemMixedTwinSse_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinSse_C;

typedef union EnumWithItemMixedTwinSseKind {
  struct wire_cst_EnumWithItemMixedTwinSse_A *A;
  struct wire_cst_EnumWithItemMixedTwinSse_B *B;
  struct wire_cst_EnumWithItemMixedTwinSse_C *C;
} EnumWithItemMixedTwinSseKind;

typedef struct wire_cst_enum_with_item_mixed_twin_sse {
  int32_t tag;
  union EnumWithItemMixedTwinSseKind *kind;
} wire_cst_enum_with_item_mixed_twin_sse;

typedef struct wire_cst_EnumWithItemMixedTwinSyncSse_A {

} wire_cst_EnumWithItemMixedTwinSyncSse_A;

typedef struct wire_cst_EnumWithItemMixedTwinSyncSse_B {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemMixedTwinSyncSse_B;

typedef struct wire_cst_EnumWithItemMixedTwinSyncSse_C {
  struct wire_cst_list_prim_u_8 *c_field;
} wire_cst_EnumWithItemMixedTwinSyncSse_C;

typedef union EnumWithItemMixedTwinSyncSseKind {
  struct wire_cst_EnumWithItemMixedTwinSyncSse_A *A;
  struct wire_cst_EnumWithItemMixedTwinSyncSse_B *B;
  struct wire_cst_EnumWithItemMixedTwinSyncSse_C *C;
} EnumWithItemMixedTwinSyncSseKind;

typedef struct wire_cst_enum_with_item_mixed_twin_sync_sse {
  int32_t tag;
  union EnumWithItemMixedTwinSyncSseKind *kind;
} wire_cst_enum_with_item_mixed_twin_sync_sse;

typedef struct wire_cst_EnumWithItemStructTwinRustAsyncSse_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinRustAsyncSse_A;

typedef struct wire_cst_EnumWithItemStructTwinRustAsyncSse_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinRustAsyncSse_B;

typedef union EnumWithItemStructTwinRustAsyncSseKind {
  struct wire_cst_EnumWithItemStructTwinRustAsyncSse_A *A;
  struct wire_cst_EnumWithItemStructTwinRustAsyncSse_B *B;
} EnumWithItemStructTwinRustAsyncSseKind;

typedef struct wire_cst_enum_with_item_struct_twin_rust_async_sse {
  int32_t tag;
  union EnumWithItemStructTwinRustAsyncSseKind *kind;
} wire_cst_enum_with_item_struct_twin_rust_async_sse;

typedef struct wire_cst_EnumWithItemStructTwinSse_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinSse_A;

typedef struct wire_cst_EnumWithItemStructTwinSse_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinSse_B;

typedef union EnumWithItemStructTwinSseKind {
  struct wire_cst_EnumWithItemStructTwinSse_A *A;
  struct wire_cst_EnumWithItemStructTwinSse_B *B;
} EnumWithItemStructTwinSseKind;

typedef struct wire_cst_enum_with_item_struct_twin_sse {
  int32_t tag;
  union EnumWithItemStructTwinSseKind *kind;
} wire_cst_enum_with_item_struct_twin_sse;

typedef struct wire_cst_EnumWithItemStructTwinSyncSse_A {
  struct wire_cst_list_prim_u_8 *a_field;
} wire_cst_EnumWithItemStructTwinSyncSse_A;

typedef struct wire_cst_EnumWithItemStructTwinSyncSse_B {
  struct wire_cst_list_prim_i_32 *b_field;
} wire_cst_EnumWithItemStructTwinSyncSse_B;

typedef union EnumWithItemStructTwinSyncSseKind {
  struct wire_cst_EnumWithItemStructTwinSyncSse_A *A;
  struct wire_cst_EnumWithItemStructTwinSyncSse_B *B;
} EnumWithItemStructTwinSyncSseKind;

typedef struct wire_cst_enum_with_item_struct_twin_sync_sse {
  int32_t tag;
  union EnumWithItemStructTwinSyncSseKind *kind;
} wire_cst_enum_with_item_struct_twin_sync_sse;

typedef struct wire_cst_EnumWithItemTupleTwinRustAsyncSse_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinRustAsyncSse_A;

typedef struct wire_cst_EnumWithItemTupleTwinRustAsyncSse_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinRustAsyncSse_B;

typedef union EnumWithItemTupleTwinRustAsyncSseKind {
  struct wire_cst_EnumWithItemTupleTwinRustAsyncSse_A *A;
  struct wire_cst_EnumWithItemTupleTwinRustAsyncSse_B *B;
} EnumWithItemTupleTwinRustAsyncSseKind;

typedef struct wire_cst_enum_with_item_tuple_twin_rust_async_sse {
  int32_t tag;
  union EnumWithItemTupleTwinRustAsyncSseKind *kind;
} wire_cst_enum_with_item_tuple_twin_rust_async_sse;

typedef struct wire_cst_EnumWithItemTupleTwinSse_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinSse_A;

typedef struct wire_cst_EnumWithItemTupleTwinSse_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinSse_B;

typedef union EnumWithItemTupleTwinSseKind {
  struct wire_cst_EnumWithItemTupleTwinSse_A *A;
  struct wire_cst_EnumWithItemTupleTwinSse_B *B;
} EnumWithItemTupleTwinSseKind;

typedef struct wire_cst_enum_with_item_tuple_twin_sse {
  int32_t tag;
  union EnumWithItemTupleTwinSseKind *kind;
} wire_cst_enum_with_item_tuple_twin_sse;

typedef struct wire_cst_EnumWithItemTupleTwinSyncSse_A {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_EnumWithItemTupleTwinSyncSse_A;

typedef struct wire_cst_EnumWithItemTupleTwinSyncSse_B {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_EnumWithItemTupleTwinSyncSse_B;

typedef union EnumWithItemTupleTwinSyncSseKind {
  struct wire_cst_EnumWithItemTupleTwinSyncSse_A *A;
  struct wire_cst_EnumWithItemTupleTwinSyncSse_B *B;
} EnumWithItemTupleTwinSyncSseKind;

typedef struct wire_cst_enum_with_item_tuple_twin_sync_sse {
  int32_t tag;
  union EnumWithItemTupleTwinSyncSseKind *kind;
} wire_cst_enum_with_item_tuple_twin_sync_sse;

typedef struct wire_cst_event_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *address;
  struct wire_cst_list_prim_u_8 *payload;
} wire_cst_event_twin_rust_async_sse;

typedef struct wire_cst_event_twin_sse {
  struct wire_cst_list_prim_u_8 *address;
  struct wire_cst_list_prim_u_8 *payload;
} wire_cst_event_twin_sse;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async_sse {
  struct wire_cst_attribute_twin_rust_async_sse **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_rust_async_sse;

typedef struct wire_cst_new_type_int_twin_rust_async_sse {
  int64_t field0;
} wire_cst_new_type_int_twin_rust_async_sse;

typedef struct wire_cst_exotic_optionals_twin_rust_async_sse {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_rust_async_sse *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async_sse *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async_sse *nullable_attributes;
  struct wire_cst_new_type_int_twin_rust_async_sse *newtypeint;
} wire_cst_exotic_optionals_twin_rust_async_sse;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_sse {
  struct wire_cst_attribute_twin_sse **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_sse;

typedef struct wire_cst_new_type_int_twin_sse {
  int64_t field0;
} wire_cst_new_type_int_twin_sse;

typedef struct wire_cst_exotic_optionals_twin_sse {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_sse *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sse *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sse *nullable_attributes;
  struct wire_cst_new_type_int_twin_sse *newtypeint;
} wire_cst_exotic_optionals_twin_sse;

typedef struct wire_cst_list_opt_box_autoadd_attribute_twin_sync_sse {
  struct wire_cst_attribute_twin_sync_sse **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_attribute_twin_sync_sse;

typedef struct wire_cst_new_type_int_twin_sync_sse {
  int64_t field0;
} wire_cst_new_type_int_twin_sync_sse;

typedef struct wire_cst_exotic_optionals_twin_sync_sse {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_cst_list_prim_u_8 *zerocopy;
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_attribute_twin_sync_sse *attributes;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sync_sse *attributes_nullable;
  struct wire_cst_list_opt_box_autoadd_attribute_twin_sync_sse *nullable_attributes;
  struct wire_cst_new_type_int_twin_sync_sse *newtypeint;
} wire_cst_exotic_optionals_twin_sync_sse;

typedef struct wire_cst_feed_id_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_rust_async_sse;

typedef struct wire_cst_feed_id_twin_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_sse;

typedef struct wire_cst_feed_id_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_feed_id_twin_sync_sse;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Empty {

} wire_cst_KitchenSinkTwinRustAsyncSse_Empty;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinRustAsyncSse_Primitives;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_rust_async_sse *field1;
} wire_cst_KitchenSinkTwinRustAsyncSse_Nested;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinRustAsyncSse_Optional;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinRustAsyncSse_Buffer;

typedef struct wire_cst_KitchenSinkTwinRustAsyncSse_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinRustAsyncSse_Enums;

typedef union KitchenSinkTwinRustAsyncSseKind {
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Empty *Empty;
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Nested *Nested;
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Optional *Optional;
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinRustAsyncSse_Enums *Enums;
} KitchenSinkTwinRustAsyncSseKind;

typedef struct wire_cst_kitchen_sink_twin_rust_async_sse {
  int32_t tag;
  union KitchenSinkTwinRustAsyncSseKind *kind;
} wire_cst_kitchen_sink_twin_rust_async_sse;

typedef struct wire_cst_KitchenSinkTwinSse_Empty {

} wire_cst_KitchenSinkTwinSse_Empty;

typedef struct wire_cst_KitchenSinkTwinSse_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinSse_Primitives;

typedef struct wire_cst_KitchenSinkTwinSse_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_sse *field1;
} wire_cst_KitchenSinkTwinSse_Nested;

typedef struct wire_cst_KitchenSinkTwinSse_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinSse_Optional;

typedef struct wire_cst_KitchenSinkTwinSse_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinSse_Buffer;

typedef struct wire_cst_KitchenSinkTwinSse_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinSse_Enums;

typedef union KitchenSinkTwinSseKind {
  struct wire_cst_KitchenSinkTwinSse_Empty *Empty;
  struct wire_cst_KitchenSinkTwinSse_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinSse_Nested *Nested;
  struct wire_cst_KitchenSinkTwinSse_Optional *Optional;
  struct wire_cst_KitchenSinkTwinSse_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinSse_Enums *Enums;
} KitchenSinkTwinSseKind;

typedef struct wire_cst_kitchen_sink_twin_sse {
  int32_t tag;
  union KitchenSinkTwinSseKind *kind;
} wire_cst_kitchen_sink_twin_sse;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Empty {

} wire_cst_KitchenSinkTwinSyncSse_Empty;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_cst_KitchenSinkTwinSyncSse_Primitives;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Nested {
  int32_t field0;
  struct wire_cst_kitchen_sink_twin_sync_sse *field1;
} wire_cst_KitchenSinkTwinSyncSse_Nested;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_cst_KitchenSinkTwinSyncSse_Optional;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Buffer {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_KitchenSinkTwinSyncSse_Buffer;

typedef struct wire_cst_KitchenSinkTwinSyncSse_Enums {
  int32_t field0;
} wire_cst_KitchenSinkTwinSyncSse_Enums;

typedef union KitchenSinkTwinSyncSseKind {
  struct wire_cst_KitchenSinkTwinSyncSse_Empty *Empty;
  struct wire_cst_KitchenSinkTwinSyncSse_Primitives *Primitives;
  struct wire_cst_KitchenSinkTwinSyncSse_Nested *Nested;
  struct wire_cst_KitchenSinkTwinSyncSse_Optional *Optional;
  struct wire_cst_KitchenSinkTwinSyncSse_Buffer *Buffer;
  struct wire_cst_KitchenSinkTwinSyncSse_Enums *Enums;
} KitchenSinkTwinSyncSseKind;

typedef struct wire_cst_kitchen_sink_twin_sync_sse {
  int32_t tag;
  union KitchenSinkTwinSyncSseKind *kind;
} wire_cst_kitchen_sink_twin_sync_sse;

typedef struct wire_cst_raw_string_mirrored {
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_raw_string_mirrored;

typedef struct wire_cst_nested_raw_string_mirrored {
  struct wire_cst_raw_string_mirrored raw;
} wire_cst_nested_raw_string_mirrored;

typedef struct wire_cst_list_nested_raw_string_mirrored {
  struct wire_cst_nested_raw_string_mirrored *ptr;
  int32_t len;
} wire_cst_list_nested_raw_string_mirrored;

typedef struct wire_cst_list_of_nested_raw_string_mirrored {
  struct wire_cst_list_nested_raw_string_mirrored *raw;
} wire_cst_list_of_nested_raw_string_mirrored;

typedef struct wire_cst_SpeedTwinRustAsyncSse_Unknown {

} wire_cst_SpeedTwinRustAsyncSse_Unknown;

typedef struct wire_cst_SpeedTwinRustAsyncSse_GPS {
  double field0;
} wire_cst_SpeedTwinRustAsyncSse_GPS;

typedef union SpeedTwinRustAsyncSseKind {
  struct wire_cst_SpeedTwinRustAsyncSse_Unknown *Unknown;
  struct wire_cst_SpeedTwinRustAsyncSse_GPS *GPS;
} SpeedTwinRustAsyncSseKind;

typedef struct wire_cst_speed_twin_rust_async_sse {
  int32_t tag;
  union SpeedTwinRustAsyncSseKind *kind;
} wire_cst_speed_twin_rust_async_sse;

typedef struct wire_cst_MeasureTwinRustAsyncSse_Speed {
  struct wire_cst_speed_twin_rust_async_sse *field0;
} wire_cst_MeasureTwinRustAsyncSse_Speed;

typedef struct wire_cst_DistanceTwinRustAsyncSse_Unknown {

} wire_cst_DistanceTwinRustAsyncSse_Unknown;

typedef struct wire_cst_DistanceTwinRustAsyncSse_Map {
  double field0;
} wire_cst_DistanceTwinRustAsyncSse_Map;

typedef union DistanceTwinRustAsyncSseKind {
  struct wire_cst_DistanceTwinRustAsyncSse_Unknown *Unknown;
  struct wire_cst_DistanceTwinRustAsyncSse_Map *Map;
} DistanceTwinRustAsyncSseKind;

typedef struct wire_cst_distance_twin_rust_async_sse {
  int32_t tag;
  union DistanceTwinRustAsyncSseKind *kind;
} wire_cst_distance_twin_rust_async_sse;

typedef struct wire_cst_MeasureTwinRustAsyncSse_Distance {
  struct wire_cst_distance_twin_rust_async_sse *field0;
} wire_cst_MeasureTwinRustAsyncSse_Distance;

typedef union MeasureTwinRustAsyncSseKind {
  struct wire_cst_MeasureTwinRustAsyncSse_Speed *Speed;
  struct wire_cst_MeasureTwinRustAsyncSse_Distance *Distance;
} MeasureTwinRustAsyncSseKind;

typedef struct wire_cst_measure_twin_rust_async_sse {
  int32_t tag;
  union MeasureTwinRustAsyncSseKind *kind;
} wire_cst_measure_twin_rust_async_sse;

typedef struct wire_cst_SpeedTwinSse_Unknown {

} wire_cst_SpeedTwinSse_Unknown;

typedef struct wire_cst_SpeedTwinSse_GPS {
  double field0;
} wire_cst_SpeedTwinSse_GPS;

typedef union SpeedTwinSseKind {
  struct wire_cst_SpeedTwinSse_Unknown *Unknown;
  struct wire_cst_SpeedTwinSse_GPS *GPS;
} SpeedTwinSseKind;

typedef struct wire_cst_speed_twin_sse {
  int32_t tag;
  union SpeedTwinSseKind *kind;
} wire_cst_speed_twin_sse;

typedef struct wire_cst_MeasureTwinSse_Speed {
  struct wire_cst_speed_twin_sse *field0;
} wire_cst_MeasureTwinSse_Speed;

typedef struct wire_cst_DistanceTwinSse_Unknown {

} wire_cst_DistanceTwinSse_Unknown;

typedef struct wire_cst_DistanceTwinSse_Map {
  double field0;
} wire_cst_DistanceTwinSse_Map;

typedef union DistanceTwinSseKind {
  struct wire_cst_DistanceTwinSse_Unknown *Unknown;
  struct wire_cst_DistanceTwinSse_Map *Map;
} DistanceTwinSseKind;

typedef struct wire_cst_distance_twin_sse {
  int32_t tag;
  union DistanceTwinSseKind *kind;
} wire_cst_distance_twin_sse;

typedef struct wire_cst_MeasureTwinSse_Distance {
  struct wire_cst_distance_twin_sse *field0;
} wire_cst_MeasureTwinSse_Distance;

typedef union MeasureTwinSseKind {
  struct wire_cst_MeasureTwinSse_Speed *Speed;
  struct wire_cst_MeasureTwinSse_Distance *Distance;
} MeasureTwinSseKind;

typedef struct wire_cst_measure_twin_sse {
  int32_t tag;
  union MeasureTwinSseKind *kind;
} wire_cst_measure_twin_sse;

typedef struct wire_cst_SpeedTwinSyncSse_Unknown {

} wire_cst_SpeedTwinSyncSse_Unknown;

typedef struct wire_cst_SpeedTwinSyncSse_GPS {
  double field0;
} wire_cst_SpeedTwinSyncSse_GPS;

typedef union SpeedTwinSyncSseKind {
  struct wire_cst_SpeedTwinSyncSse_Unknown *Unknown;
  struct wire_cst_SpeedTwinSyncSse_GPS *GPS;
} SpeedTwinSyncSseKind;

typedef struct wire_cst_speed_twin_sync_sse {
  int32_t tag;
  union SpeedTwinSyncSseKind *kind;
} wire_cst_speed_twin_sync_sse;

typedef struct wire_cst_MeasureTwinSyncSse_Speed {
  struct wire_cst_speed_twin_sync_sse *field0;
} wire_cst_MeasureTwinSyncSse_Speed;

typedef struct wire_cst_DistanceTwinSyncSse_Unknown {

} wire_cst_DistanceTwinSyncSse_Unknown;

typedef struct wire_cst_DistanceTwinSyncSse_Map {
  double field0;
} wire_cst_DistanceTwinSyncSse_Map;

typedef union DistanceTwinSyncSseKind {
  struct wire_cst_DistanceTwinSyncSse_Unknown *Unknown;
  struct wire_cst_DistanceTwinSyncSse_Map *Map;
} DistanceTwinSyncSseKind;

typedef struct wire_cst_distance_twin_sync_sse {
  int32_t tag;
  union DistanceTwinSyncSseKind *kind;
} wire_cst_distance_twin_sync_sse;

typedef struct wire_cst_MeasureTwinSyncSse_Distance {
  struct wire_cst_distance_twin_sync_sse *field0;
} wire_cst_MeasureTwinSyncSse_Distance;

typedef union MeasureTwinSyncSseKind {
  struct wire_cst_MeasureTwinSyncSse_Speed *Speed;
  struct wire_cst_MeasureTwinSyncSse_Distance *Distance;
} MeasureTwinSyncSseKind;

typedef struct wire_cst_measure_twin_sync_sse {
  int32_t tag;
  union MeasureTwinSyncSseKind *kind;
} wire_cst_measure_twin_sync_sse;

typedef struct wire_cst_message_id_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_rust_async_sse;

typedef struct wire_cst_message_id_twin_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_sse;

typedef struct wire_cst_message_id_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_message_id_twin_sync_sse;

typedef struct wire_cst_list_my_tree_node_twin_rust_async_sse {
  struct wire_cst_my_tree_node_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_rust_async_sse;

typedef struct wire_cst_my_tree_node_twin_rust_async_sse {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_rust_async_sse *children;
} wire_cst_my_tree_node_twin_rust_async_sse;

typedef struct wire_cst_my_nested_struct_twin_rust_async_sse {
  struct wire_cst_my_tree_node_twin_rust_async_sse tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_rust_async_sse;

typedef struct wire_cst_list_my_tree_node_twin_sse {
  struct wire_cst_my_tree_node_twin_sse *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_sse;

typedef struct wire_cst_my_tree_node_twin_sse {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_sse *children;
} wire_cst_my_tree_node_twin_sse;

typedef struct wire_cst_my_nested_struct_twin_sse {
  struct wire_cst_my_tree_node_twin_sse tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_sse;

typedef struct wire_cst_list_my_tree_node_twin_sync_sse {
  struct wire_cst_my_tree_node_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_my_tree_node_twin_sync_sse;

typedef struct wire_cst_my_tree_node_twin_sync_sse {
  int32_t value_i32;
  struct wire_cst_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_cst_list_my_tree_node_twin_sync_sse *children;
} wire_cst_my_tree_node_twin_sync_sse;

typedef struct wire_cst_my_nested_struct_twin_sync_sse {
  struct wire_cst_my_tree_node_twin_sync_sse tree_node;
  int32_t weekday;
} wire_cst_my_nested_struct_twin_sync_sse;

typedef struct wire_cst_note_twin_rust_async_sse {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_rust_async_sse;

typedef struct wire_cst_note_twin_sse {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_sse;

typedef struct wire_cst_note_twin_sync_sse {
  int32_t *day;
  struct wire_cst_list_prim_u_8 *body;
} wire_cst_note_twin_sync_sse;

typedef struct wire_cst_opaque_nested_twin_rust_async_sse {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_rust_async_sse;

typedef struct wire_cst_opaque_nested_twin_sse {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_sse;

typedef struct wire_cst_opaque_nested_twin_sync_sse {
  const void *first;
  const void *second;
} wire_cst_opaque_nested_twin_sync_sse;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async_sse {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async_sse;

typedef struct wire_cst_opt_vecs_twin_rust_async_sse {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async_sse *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_rust_async_sse;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_sse {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_sse;

typedef struct wire_cst_opt_vecs_twin_sse {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_sse *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_sse;

typedef struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync_sse {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_weekdays_twin_sync_sse;

typedef struct wire_cst_opt_vecs_twin_sync_sse {
  struct wire_cst_list_opt_box_autoadd_i_32 *i32;
  struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync_sse *enums;
  struct wire_cst_list_opt_String *strings;
  struct wire_cst_list_opt_list_prim_i_32 *buffers;
} wire_cst_opt_vecs_twin_sync_sse;

typedef struct wire_cst_some_struct_twin_rust_async_sse {
  uint32_t value;
} wire_cst_some_struct_twin_rust_async_sse;

typedef struct wire_cst_some_struct_twin_sse {
  uint32_t value;
} wire_cst_some_struct_twin_sse;

typedef struct wire_cst_some_struct_twin_sync_sse {
  uint32_t value;
} wire_cst_some_struct_twin_sync_sse;

typedef struct wire_cst_struct_with_comments_twin_rust_async_sse {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_rust_async_sse;

typedef struct wire_cst_struct_with_comments_twin_sse {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_sse;

typedef struct wire_cst_struct_with_comments_twin_sync_sse {
  int32_t field_with_comments;
} wire_cst_struct_with_comments_twin_sync_sse;

typedef struct wire_cst_struct_with_enum_twin_rust_async_sse {
  struct wire_cst_abc_twin_rust_async_sse abc1;
  struct wire_cst_abc_twin_rust_async_sse abc2;
} wire_cst_struct_with_enum_twin_rust_async_sse;

typedef struct wire_cst_struct_with_enum_twin_sse {
  struct wire_cst_abc_twin_sse abc1;
  struct wire_cst_abc_twin_sse abc2;
} wire_cst_struct_with_enum_twin_sse;

typedef struct wire_cst_struct_with_enum_twin_sync_sse {
  struct wire_cst_abc_twin_sync_sse abc1;
  struct wire_cst_abc_twin_sync_sse abc2;
} wire_cst_struct_with_enum_twin_sync_sse;

typedef struct wire_cst_struct_with_one_field_twin_rust_async_sse {
  int32_t a;
} wire_cst_struct_with_one_field_twin_rust_async_sse;

typedef struct wire_cst_struct_with_one_field_twin_sse {
  int32_t a;
} wire_cst_struct_with_one_field_twin_sse;

typedef struct wire_cst_struct_with_one_field_twin_sync_sse {
  int32_t a;
} wire_cst_struct_with_one_field_twin_sync_sse;

typedef struct wire_cst_struct_with_two_field_twin_rust_async_sse {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_rust_async_sse;

typedef struct wire_cst_struct_with_two_field_twin_sse {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_sse;

typedef struct wire_cst_struct_with_two_field_twin_sync_sse {
  int32_t a;
  int32_t b;
} wire_cst_struct_with_two_field_twin_sync_sse;

typedef struct wire_cst_struct_with_zero_field_twin_rust_async_sse {

} wire_cst_struct_with_zero_field_twin_rust_async_sse;

typedef struct wire_cst_struct_with_zero_field_twin_sse {

} wire_cst_struct_with_zero_field_twin_sse;

typedef struct wire_cst_struct_with_zero_field_twin_sync_sse {

} wire_cst_struct_with_zero_field_twin_sync_sse;

typedef struct wire_cst_sum_with_twin_rust_async_sse {
  uint32_t x;
} wire_cst_sum_with_twin_rust_async_sse;

typedef struct wire_cst_sum_with_twin_sse {
  uint32_t x;
} wire_cst_sum_with_twin_sse;

typedef struct wire_cst_sum_with_twin_sync_sse {
  uint32_t x;
} wire_cst_sum_with_twin_sync_sse;

typedef struct wire_cst_test_id_twin_rust_async_sse {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_rust_async_sse;

typedef struct wire_cst_test_id_twin_sse {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_sse;

typedef struct wire_cst_test_id_twin_sync_sse {
  struct wire_cst_list_prim_i_32 *field0;
} wire_cst_test_id_twin_sync_sse;

typedef struct wire_cst_tuple_struct_with_one_field_twin_rust_async_sse {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_rust_async_sse;

typedef struct wire_cst_tuple_struct_with_one_field_twin_sse {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_sse;

typedef struct wire_cst_tuple_struct_with_one_field_twin_sync_sse {
  int32_t field0;
} wire_cst_tuple_struct_with_one_field_twin_sync_sse;

typedef struct wire_cst_tuple_struct_with_two_field_twin_rust_async_sse {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_rust_async_sse;

typedef struct wire_cst_tuple_struct_with_two_field_twin_sse {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_sse;

typedef struct wire_cst_tuple_struct_with_two_field_twin_sync_sse {
  int32_t field0;
  int32_t field1;
} wire_cst_tuple_struct_with_two_field_twin_sync_sse;

typedef struct wire_cst_user_id_twin_rust_async_sse {
  uint32_t value;
} wire_cst_user_id_twin_rust_async_sse;

typedef struct wire_cst_user_id_twin_sse {
  uint32_t value;
} wire_cst_user_id_twin_sse;

typedef struct wire_cst_user_id_twin_sync_sse {
  uint32_t value;
} wire_cst_user_id_twin_sync_sse;

typedef struct wire_cst_blob_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_rust_async_sse;

typedef struct wire_cst_blob_twin_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_sse;

typedef struct wire_cst_blob_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_blob_twin_sync_sse;

typedef struct wire_cst_list_Chrono_Local {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_Chrono_Local;

typedef struct wire_cst_list_application_settings {
  struct wire_cst_application_settings *ptr;
  int32_t len;
} wire_cst_list_application_settings;

typedef struct wire_cst_list_enum_opaque_twin_normal {
  struct wire_cst_enum_opaque_twin_normal *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_normal;

typedef struct wire_cst_list_enum_opaque_twin_rust_async {
  struct wire_cst_enum_opaque_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_rust_async;

typedef struct wire_cst_list_enum_opaque_twin_rust_async_sse {
  struct wire_cst_enum_opaque_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_rust_async_sse;

typedef struct wire_cst_list_enum_opaque_twin_sse {
  struct wire_cst_enum_opaque_twin_sse *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_sse;

typedef struct wire_cst_list_enum_opaque_twin_sync {
  struct wire_cst_enum_opaque_twin_sync *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_sync;

typedef struct wire_cst_list_enum_opaque_twin_sync_sse {
  struct wire_cst_enum_opaque_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_enum_opaque_twin_sync_sse;

typedef struct wire_cst_list_my_enum {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_my_enum;

typedef struct wire_cst_point_twin_normal {
  float x;
  float y;
} wire_cst_point_twin_normal;

typedef struct wire_cst_list_point_twin_normal {
  struct wire_cst_point_twin_normal *ptr;
  int32_t len;
} wire_cst_list_point_twin_normal;

typedef struct wire_cst_point_twin_rust_async {
  float x;
  float y;
} wire_cst_point_twin_rust_async;

typedef struct wire_cst_list_point_twin_rust_async {
  struct wire_cst_point_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_point_twin_rust_async;

typedef struct wire_cst_point_twin_rust_async_sse {
  float x;
  float y;
} wire_cst_point_twin_rust_async_sse;

typedef struct wire_cst_list_point_twin_rust_async_sse {
  struct wire_cst_point_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_point_twin_rust_async_sse;

typedef struct wire_cst_point_twin_sse {
  float x;
  float y;
} wire_cst_point_twin_sse;

typedef struct wire_cst_list_point_twin_sse {
  struct wire_cst_point_twin_sse *ptr;
  int32_t len;
} wire_cst_list_point_twin_sse;

typedef struct wire_cst_point_twin_sync {
  float x;
  float y;
} wire_cst_point_twin_sync;

typedef struct wire_cst_list_point_twin_sync {
  struct wire_cst_point_twin_sync *ptr;
  int32_t len;
} wire_cst_list_point_twin_sync;

typedef struct wire_cst_point_twin_sync_sse {
  float x;
  float y;
} wire_cst_point_twin_sync_sse;

typedef struct wire_cst_list_point_twin_sync_sse {
  struct wire_cst_point_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_point_twin_sync_sse;

typedef struct wire_cst_RawStringEnumMirrored_Raw {
  struct wire_cst_raw_string_mirrored *field0;
} wire_cst_RawStringEnumMirrored_Raw;

typedef struct wire_cst_RawStringEnumMirrored_Nested {
  struct wire_cst_nested_raw_string_mirrored *field0;
} wire_cst_RawStringEnumMirrored_Nested;

typedef struct wire_cst_RawStringEnumMirrored_ListOfNested {
  struct wire_cst_list_of_nested_raw_string_mirrored *field0;
} wire_cst_RawStringEnumMirrored_ListOfNested;

typedef union RawStringEnumMirroredKind {
  struct wire_cst_RawStringEnumMirrored_Raw *Raw;
  struct wire_cst_RawStringEnumMirrored_Nested *Nested;
  struct wire_cst_RawStringEnumMirrored_ListOfNested *ListOfNested;
} RawStringEnumMirroredKind;

typedef struct wire_cst_raw_string_enum_mirrored {
  int32_t tag;
  union RawStringEnumMirroredKind *kind;
} wire_cst_raw_string_enum_mirrored;

typedef struct wire_cst_list_raw_string_enum_mirrored {
  struct wire_cst_raw_string_enum_mirrored *ptr;
  int32_t len;
} wire_cst_list_raw_string_enum_mirrored;

typedef struct wire_cst_list_raw_string_mirrored {
  struct wire_cst_raw_string_mirrored *ptr;
  int32_t len;
} wire_cst_list_raw_string_mirrored;

typedef struct wire_cst_list_sum_with_twin_normal {
  struct wire_cst_sum_with_twin_normal *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_normal;

typedef struct wire_cst_list_sum_with_twin_rust_async {
  struct wire_cst_sum_with_twin_rust_async *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_rust_async;

typedef struct wire_cst_list_sum_with_twin_rust_async_sse {
  struct wire_cst_sum_with_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_rust_async_sse;

typedef struct wire_cst_list_sum_with_twin_sse {
  struct wire_cst_sum_with_twin_sse *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_sse;

typedef struct wire_cst_list_sum_with_twin_sync {
  struct wire_cst_sum_with_twin_sync *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_sync;

typedef struct wire_cst_list_sum_with_twin_sync_sse {
  struct wire_cst_sum_with_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_sum_with_twin_sync_sse;

typedef struct wire_cst_list_test_id_twin_rust_async_sse {
  struct wire_cst_test_id_twin_rust_async_sse *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_rust_async_sse;

typedef struct wire_cst_list_test_id_twin_sse {
  struct wire_cst_test_id_twin_sse *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_sse;

typedef struct wire_cst_list_test_id_twin_sync_sse {
  struct wire_cst_test_id_twin_sync_sse *ptr;
  int32_t len;
} wire_cst_list_test_id_twin_sync_sse;

typedef struct wire_cst_list_weekdays_twin_rust_async_sse {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_rust_async_sse;

typedef struct wire_cst_list_weekdays_twin_sse {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_sse;

typedef struct wire_cst_list_weekdays_twin_sync_sse {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_weekdays_twin_sync_sse;

typedef struct wire_cst_ApplicationMessage_DisplayMessage {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_ApplicationMessage_DisplayMessage;

typedef struct wire_cst_ApplicationMessage_RenderPixel {
  int32_t x;
  int32_t y;
} wire_cst_ApplicationMessage_RenderPixel;

typedef struct wire_cst_ApplicationMessage_Exit {

} wire_cst_ApplicationMessage_Exit;

typedef union ApplicationMessageKind {
  struct wire_cst_ApplicationMessage_DisplayMessage *DisplayMessage;
  struct wire_cst_ApplicationMessage_RenderPixel *RenderPixel;
  struct wire_cst_ApplicationMessage_Exit *Exit;
} ApplicationMessageKind;

typedef struct wire_cst_CustomEnumErrorTwinNormal_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinNormal_One;

typedef struct wire_cst_CustomEnumErrorTwinNormal_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinNormal_Two;

typedef union CustomEnumErrorTwinNormalKind {
  struct wire_cst_CustomEnumErrorTwinNormal_One *One;
  struct wire_cst_CustomEnumErrorTwinNormal_Two *Two;
} CustomEnumErrorTwinNormalKind;

typedef struct wire_cst_CustomEnumErrorTwinRustAsync_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinRustAsync_One;

typedef struct wire_cst_CustomEnumErrorTwinRustAsync_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinRustAsync_Two;

typedef union CustomEnumErrorTwinRustAsyncKind {
  struct wire_cst_CustomEnumErrorTwinRustAsync_One *One;
  struct wire_cst_CustomEnumErrorTwinRustAsync_Two *Two;
} CustomEnumErrorTwinRustAsyncKind;

typedef struct wire_cst_CustomEnumErrorTwinRustAsyncSse_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinRustAsyncSse_One;

typedef struct wire_cst_CustomEnumErrorTwinRustAsyncSse_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinRustAsyncSse_Two;

typedef union CustomEnumErrorTwinRustAsyncSseKind {
  struct wire_cst_CustomEnumErrorTwinRustAsyncSse_One *One;
  struct wire_cst_CustomEnumErrorTwinRustAsyncSse_Two *Two;
} CustomEnumErrorTwinRustAsyncSseKind;

typedef struct wire_cst_CustomEnumErrorTwinSse_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSse_One;

typedef struct wire_cst_CustomEnumErrorTwinSse_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSse_Two;

typedef union CustomEnumErrorTwinSseKind {
  struct wire_cst_CustomEnumErrorTwinSse_One *One;
  struct wire_cst_CustomEnumErrorTwinSse_Two *Two;
} CustomEnumErrorTwinSseKind;

typedef struct wire_cst_CustomEnumErrorTwinSync_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSync_One;

typedef struct wire_cst_CustomEnumErrorTwinSync_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSync_Two;

typedef union CustomEnumErrorTwinSyncKind {
  struct wire_cst_CustomEnumErrorTwinSync_One *One;
  struct wire_cst_CustomEnumErrorTwinSync_Two *Two;
} CustomEnumErrorTwinSyncKind;

typedef struct wire_cst_CustomEnumErrorTwinSyncSse_One {
  struct wire_cst_list_prim_u_8 *message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSyncSse_One;

typedef struct wire_cst_CustomEnumErrorTwinSyncSse_Two {
  uint32_t message;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomEnumErrorTwinSyncSse_Two;

typedef union CustomEnumErrorTwinSyncSseKind {
  struct wire_cst_CustomEnumErrorTwinSyncSse_One *One;
  struct wire_cst_CustomEnumErrorTwinSyncSse_Two *Two;
} CustomEnumErrorTwinSyncSseKind;

typedef struct wire_cst_CustomErrorTwinNormal_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinNormal_Error0;

typedef struct wire_cst_CustomErrorTwinNormal_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinNormal_Error1;

typedef union CustomErrorTwinNormalKind {
  struct wire_cst_CustomErrorTwinNormal_Error0 *Error0;
  struct wire_cst_CustomErrorTwinNormal_Error1 *Error1;
} CustomErrorTwinNormalKind;

typedef struct wire_cst_CustomErrorTwinRustAsync_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinRustAsync_Error0;

typedef struct wire_cst_CustomErrorTwinRustAsync_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinRustAsync_Error1;

typedef union CustomErrorTwinRustAsyncKind {
  struct wire_cst_CustomErrorTwinRustAsync_Error0 *Error0;
  struct wire_cst_CustomErrorTwinRustAsync_Error1 *Error1;
} CustomErrorTwinRustAsyncKind;

typedef struct wire_cst_CustomErrorTwinRustAsyncSse_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinRustAsyncSse_Error0;

typedef struct wire_cst_CustomErrorTwinRustAsyncSse_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinRustAsyncSse_Error1;

typedef union CustomErrorTwinRustAsyncSseKind {
  struct wire_cst_CustomErrorTwinRustAsyncSse_Error0 *Error0;
  struct wire_cst_CustomErrorTwinRustAsyncSse_Error1 *Error1;
} CustomErrorTwinRustAsyncSseKind;

typedef struct wire_cst_CustomErrorTwinSse_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSse_Error0;

typedef struct wire_cst_CustomErrorTwinSse_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSse_Error1;

typedef union CustomErrorTwinSseKind {
  struct wire_cst_CustomErrorTwinSse_Error0 *Error0;
  struct wire_cst_CustomErrorTwinSse_Error1 *Error1;
} CustomErrorTwinSseKind;

typedef struct wire_cst_CustomErrorTwinSync_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSync_Error0;

typedef struct wire_cst_CustomErrorTwinSync_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSync_Error1;

typedef union CustomErrorTwinSyncKind {
  struct wire_cst_CustomErrorTwinSync_Error0 *Error0;
  struct wire_cst_CustomErrorTwinSync_Error1 *Error1;
} CustomErrorTwinSyncKind;

typedef struct wire_cst_CustomErrorTwinSyncSse_Error0 {
  struct wire_cst_list_prim_u_8 *e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSyncSse_Error0;

typedef struct wire_cst_CustomErrorTwinSyncSse_Error1 {
  uint32_t e;
  struct wire_cst_list_prim_u_8 *backtrace;
} wire_cst_CustomErrorTwinSyncSse_Error1;

typedef union CustomErrorTwinSyncSseKind {
  struct wire_cst_CustomErrorTwinSyncSse_Error0 *Error0;
  struct wire_cst_CustomErrorTwinSyncSse_Error1 *Error1;
} CustomErrorTwinSyncSseKind;

typedef struct wire_cst_CustomNestedError1TwinNormal_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinNormal_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinNormal_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_normal *field0;
} wire_cst_CustomNestedError1TwinNormal_ErrorNested;

typedef union CustomNestedError1TwinNormalKind {
  struct wire_cst_CustomNestedError1TwinNormal_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinNormal_ErrorNested *ErrorNested;
} CustomNestedError1TwinNormalKind;

typedef struct wire_cst_CustomNestedError1TwinRustAsync_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinRustAsync_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinRustAsync_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_rust_async *field0;
} wire_cst_CustomNestedError1TwinRustAsync_ErrorNested;

typedef union CustomNestedError1TwinRustAsyncKind {
  struct wire_cst_CustomNestedError1TwinRustAsync_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinRustAsync_ErrorNested *ErrorNested;
} CustomNestedError1TwinRustAsyncKind;

typedef struct wire_cst_CustomNestedError1TwinRustAsyncSse_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinRustAsyncSse_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinRustAsyncSse_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_rust_async_sse *field0;
} wire_cst_CustomNestedError1TwinRustAsyncSse_ErrorNested;

typedef union CustomNestedError1TwinRustAsyncSseKind {
  struct wire_cst_CustomNestedError1TwinRustAsyncSse_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinRustAsyncSse_ErrorNested *ErrorNested;
} CustomNestedError1TwinRustAsyncSseKind;

typedef struct wire_cst_CustomNestedError1TwinSse_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinSse_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinSse_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_sse *field0;
} wire_cst_CustomNestedError1TwinSse_ErrorNested;

typedef union CustomNestedError1TwinSseKind {
  struct wire_cst_CustomNestedError1TwinSse_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinSse_ErrorNested *ErrorNested;
} CustomNestedError1TwinSseKind;

typedef struct wire_cst_CustomNestedError1TwinSync_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinSync_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinSync_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_sync *field0;
} wire_cst_CustomNestedError1TwinSync_ErrorNested;

typedef union CustomNestedError1TwinSyncKind {
  struct wire_cst_CustomNestedError1TwinSync_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinSync_ErrorNested *ErrorNested;
} CustomNestedError1TwinSyncKind;

typedef struct wire_cst_CustomNestedError1TwinSyncSse_CustomNested1 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_CustomNestedError1TwinSyncSse_CustomNested1;

typedef struct wire_cst_CustomNestedError1TwinSyncSse_ErrorNested {
  struct wire_cst_custom_nested_error_2_twin_sync_sse *field0;
} wire_cst_CustomNestedError1TwinSyncSse_ErrorNested;

typedef union CustomNestedError1TwinSyncSseKind {
  struct wire_cst_CustomNestedError1TwinSyncSse_CustomNested1 *CustomNested1;
  struct wire_cst_CustomNestedError1TwinSyncSse_ErrorNested *ErrorNested;
} CustomNestedError1TwinSyncSseKind;

typedef struct wire_cst_another_macro_struct_twin_normal {
  int32_t data;
  int32_t non_final_data;
} wire_cst_another_macro_struct_twin_normal;

typedef struct wire_cst_another_twin_normal {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_normal;

typedef struct wire_cst_another_twin_rust_async {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_rust_async;

typedef struct wire_cst_another_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_rust_async_sse;

typedef struct wire_cst_another_twin_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_sse;

typedef struct wire_cst_another_twin_sync {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_sync;

typedef struct wire_cst_another_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *a;
} wire_cst_another_twin_sync_sse;

typedef struct wire_cst_application_message {
  int32_t tag;
  union ApplicationMessageKind *kind;
} wire_cst_application_message;

typedef struct wire_cst_big_buffers_twin_normal {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_normal;

typedef struct wire_cst_big_buffers_twin_rust_async {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_rust_async;

typedef struct wire_cst_big_buffers_twin_rust_async_sse {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_rust_async_sse;

typedef struct wire_cst_big_buffers_twin_sse {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_sse;

typedef struct wire_cst_big_buffers_twin_sync {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_sync;

typedef struct wire_cst_big_buffers_twin_sync_sse {
  struct wire_cst_list_prim_i_64 *int64;
  struct wire_cst_list_prim_u_64 *uint64;
} wire_cst_big_buffers_twin_sync_sse;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_normal {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_normal test2;
} wire_cst_contains_mirrored_sub_struct_twin_normal;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_rust_async {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_rust_async test2;
} wire_cst_contains_mirrored_sub_struct_twin_rust_async;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_rust_async_sse {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_rust_async_sse test2;
} wire_cst_contains_mirrored_sub_struct_twin_rust_async_sse;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_sse {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_sse test2;
} wire_cst_contains_mirrored_sub_struct_twin_sse;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_sync {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_sync test2;
} wire_cst_contains_mirrored_sub_struct_twin_sync;

typedef struct wire_cst_contains_mirrored_sub_struct_twin_sync_sse {
  struct wire_cst_raw_string_mirrored test;
  struct wire_cst_another_twin_sync_sse test2;
} wire_cst_contains_mirrored_sub_struct_twin_sync_sse;

typedef struct wire_cst_custom_enum_error_twin_normal {
  int32_t tag;
  union CustomEnumErrorTwinNormalKind *kind;
} wire_cst_custom_enum_error_twin_normal;

typedef struct wire_cst_custom_enum_error_twin_rust_async {
  int32_t tag;
  union CustomEnumErrorTwinRustAsyncKind *kind;
} wire_cst_custom_enum_error_twin_rust_async;

typedef struct wire_cst_custom_enum_error_twin_rust_async_sse {
  int32_t tag;
  union CustomEnumErrorTwinRustAsyncSseKind *kind;
} wire_cst_custom_enum_error_twin_rust_async_sse;

typedef struct wire_cst_custom_enum_error_twin_sse {
  int32_t tag;
  union CustomEnumErrorTwinSseKind *kind;
} wire_cst_custom_enum_error_twin_sse;

typedef struct wire_cst_custom_enum_error_twin_sync {
  int32_t tag;
  union CustomEnumErrorTwinSyncKind *kind;
} wire_cst_custom_enum_error_twin_sync;

typedef struct wire_cst_custom_enum_error_twin_sync_sse {
  int32_t tag;
  union CustomEnumErrorTwinSyncSseKind *kind;
} wire_cst_custom_enum_error_twin_sync_sse;

typedef struct wire_cst_custom_error_twin_normal {
  int32_t tag;
  union CustomErrorTwinNormalKind *kind;
} wire_cst_custom_error_twin_normal;

typedef struct wire_cst_custom_error_twin_rust_async {
  int32_t tag;
  union CustomErrorTwinRustAsyncKind *kind;
} wire_cst_custom_error_twin_rust_async;

typedef struct wire_cst_custom_error_twin_rust_async_sse {
  int32_t tag;
  union CustomErrorTwinRustAsyncSseKind *kind;
} wire_cst_custom_error_twin_rust_async_sse;

typedef struct wire_cst_custom_error_twin_sse {
  int32_t tag;
  union CustomErrorTwinSseKind *kind;
} wire_cst_custom_error_twin_sse;

typedef struct wire_cst_custom_error_twin_sync {
  int32_t tag;
  union CustomErrorTwinSyncKind *kind;
} wire_cst_custom_error_twin_sync;

typedef struct wire_cst_custom_error_twin_sync_sse {
  int32_t tag;
  union CustomErrorTwinSyncSseKind *kind;
} wire_cst_custom_error_twin_sync_sse;

typedef struct wire_cst_custom_nested_error_1_twin_normal {
  int32_t tag;
  union CustomNestedError1TwinNormalKind *kind;
} wire_cst_custom_nested_error_1_twin_normal;

typedef struct wire_cst_custom_nested_error_1_twin_rust_async {
  int32_t tag;
  union CustomNestedError1TwinRustAsyncKind *kind;
} wire_cst_custom_nested_error_1_twin_rust_async;

typedef struct wire_cst_custom_nested_error_1_twin_rust_async_sse {
  int32_t tag;
  union CustomNestedError1TwinRustAsyncSseKind *kind;
} wire_cst_custom_nested_error_1_twin_rust_async_sse;

typedef struct wire_cst_custom_nested_error_1_twin_sse {
  int32_t tag;
  union CustomNestedError1TwinSseKind *kind;
} wire_cst_custom_nested_error_1_twin_sse;

typedef struct wire_cst_custom_nested_error_1_twin_sync {
  int32_t tag;
  union CustomNestedError1TwinSyncKind *kind;
} wire_cst_custom_nested_error_1_twin_sync;

typedef struct wire_cst_custom_nested_error_1_twin_sync_sse {
  int32_t tag;
  union CustomNestedError1TwinSyncSseKind *kind;
} wire_cst_custom_nested_error_1_twin_sync_sse;

typedef struct wire_cst_custom_struct_error_another_twin_normal {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_normal;

typedef struct wire_cst_custom_struct_error_another_twin_rust_async {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_rust_async;

typedef struct wire_cst_custom_struct_error_another_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_rust_async_sse;

typedef struct wire_cst_custom_struct_error_another_twin_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_sse;

typedef struct wire_cst_custom_struct_error_another_twin_sync {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_sync;

typedef struct wire_cst_custom_struct_error_another_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *message;
} wire_cst_custom_struct_error_another_twin_sync_sse;

typedef struct wire_cst_demo_struct_for_rust_call_dart {
  struct wire_cst_list_prim_u_8 *name;
} wire_cst_demo_struct_for_rust_call_dart;

typedef struct wire_cst_log_2_twin_normal {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_normal;

typedef struct wire_cst_log_2_twin_rust_async {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_rust_async;

typedef struct wire_cst_log_2_twin_rust_async_sse {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_rust_async_sse;

typedef struct wire_cst_log_2_twin_sse {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_sse;

typedef struct wire_cst_log_2_twin_sync {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_sync;

typedef struct wire_cst_log_2_twin_sync_sse {
  uint32_t key;
  struct wire_cst_list_prim_u_8 *value;
} wire_cst_log_2_twin_sync_sse;

typedef struct wire_cst_log_twin_normal {
  uint32_t key;
  uint32_t value;
} wire_cst_log_twin_normal;

typedef struct wire_cst_log_twin_rust_async {
  uint32_t key;
  uint32_t value;
} wire_cst_log_twin_rust_async;

typedef struct wire_cst_log_twin_rust_async_sse {
  uint32_t key;
  uint32_t value;
} wire_cst_log_twin_rust_async_sse;

typedef struct wire_cst_log_twin_sse {
  uint32_t key;
  uint32_t value;
} wire_cst_log_twin_sse;

typedef struct wire_cst_mirror_struct_twin_normal {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_normal;

typedef struct wire_cst_mirror_struct_twin_rust_async {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_rust_async;

typedef struct wire_cst_mirror_struct_twin_rust_async_sse {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_rust_async_sse;

typedef struct wire_cst_mirror_struct_twin_sse {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_sse;

typedef struct wire_cst_mirror_struct_twin_sync {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_sync;

typedef struct wire_cst_mirror_struct_twin_sync_sse {
  struct wire_cst_application_settings a;
  struct wire_cst_my_struct b;
  struct wire_cst_list_my_enum *c;
  struct wire_cst_list_application_settings *d;
} wire_cst_mirror_struct_twin_sync_sse;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_normal {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_normal;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_rust_async {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_rust_async;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_rust_async_sse;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_sse {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_sse;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_sync {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_sync;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *regular;
  struct wire_cst_list_prim_u_8 *type;
  bool async;
  struct wire_cst_list_prim_u_8 *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_sync_sse;

typedef struct wire_cst_my_stream_entry_twin_normal {
  struct wire_cst_list_prim_u_8 *hello;
} wire_cst_my_stream_entry_twin_normal;

typedef struct wire_cst_my_stream_entry_twin_rust_async {
  struct wire_cst_list_prim_u_8 *hello;
} wire_cst_my_stream_entry_twin_rust_async;

typedef struct wire_cst_my_stream_entry_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *hello;
} wire_cst_my_stream_entry_twin_rust_async_sse;

typedef struct wire_cst_my_stream_entry_twin_sse {
  struct wire_cst_list_prim_u_8 *hello;
} wire_cst_my_stream_entry_twin_sse;

typedef struct wire_cst_new_simple_struct {
  int32_t field;
} wire_cst_new_simple_struct;

typedef struct wire_cst_old_simple_struct {
  int32_t field;
} wire_cst_old_simple_struct;

typedef struct wire_cst_raw_string_item_struct_twin_normal {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_normal;

typedef struct wire_cst_raw_string_item_struct_twin_rust_async {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_rust_async;

typedef struct wire_cst_raw_string_item_struct_twin_rust_async_sse {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_rust_async_sse;

typedef struct wire_cst_raw_string_item_struct_twin_sse {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_sse;

typedef struct wire_cst_raw_string_item_struct_twin_sync {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_sync;

typedef struct wire_cst_raw_string_item_struct_twin_sync_sse {
  struct wire_cst_list_prim_u_8 *type;
} wire_cst_raw_string_item_struct_twin_sync_sse;

typedef struct wire_cst_record_application_settings_raw_string_enum_mirrored {
  struct wire_cst_application_settings field0;
  struct wire_cst_raw_string_enum_mirrored field1;
} wire_cst_record_application_settings_raw_string_enum_mirrored;

typedef struct wire_cst_test_chrono_twin_normal {
  int64_t *dt;
  int64_t *dt2;
  int64_t *du;
} wire_cst_test_chrono_twin_normal;

typedef struct wire_cst_test_chrono_twin_rust_async {
  int64_t *dt;
  int64_t *dt2;
  int64_t *du;
} wire_cst_test_chrono_twin_rust_async;

typedef struct wire_cst_test_chrono_twin_sync {
  int64_t *dt;
  int64_t *dt2;
  int64_t *du;
} wire_cst_test_chrono_twin_sync;

typedef struct wire_cst_test_model_twin_normal {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_normal;

typedef struct wire_cst_test_model_twin_rust_async {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_rust_async;

typedef struct wire_cst_test_model_twin_rust_async_sse {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_rust_async_sse;

typedef struct wire_cst_test_model_twin_sse {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_sse;

typedef struct wire_cst_test_model_twin_sync {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_sync;

typedef struct wire_cst_test_model_twin_sync_sse {
  uint64_t id;
  struct wire_cst_list_prim_u_8 *name;
  int32_t alias_enum;
  struct wire_cst_my_struct alias_struct;
} wire_cst_test_model_twin_sync_sse;

typedef struct wire_cst_vec_of_primitive_pack_twin_normal {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_normal;

typedef struct wire_cst_vec_of_primitive_pack_twin_rust_async {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_rust_async;

typedef struct wire_cst_vec_of_primitive_pack_twin_rust_async_sse {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_rust_async_sse;

typedef struct wire_cst_vec_of_primitive_pack_twin_sse {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_sse;

typedef struct wire_cst_vec_of_primitive_pack_twin_sync {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_sync;

typedef struct wire_cst_vec_of_primitive_pack_twin_sync_sse {
  struct wire_cst_list_prim_i_8 *int8list;
  struct wire_cst_list_prim_u_8 *uint8list;
  struct wire_cst_list_prim_i_16 *int16list;
  struct wire_cst_list_prim_u_16 *uint16list;
  struct wire_cst_list_prim_u_32 *uint32list;
  struct wire_cst_list_prim_i_32 *int32list;
  struct wire_cst_list_prim_u_64 *uint64list;
  struct wire_cst_list_prim_i_64 *int64list;
  struct wire_cst_list_prim_f_32 *float32list;
  struct wire_cst_list_prim_f_64 *float64list;
  struct wire_cst_list_bool *bool_list;
} wire_cst_vec_of_primitive_pack_twin_sync_sse;

void benchmark_raw_void_sync(void);

struct benchmark_raw_list_prim_u_8 benchmark_raw_new_list_prim_u_8(int32_t len);

int32_t benchmark_raw_input_bytes(struct benchmark_raw_list_prim_u_8 bytes);

void benchmark_raw_output_bytes(MessagePort port, int32_t message_id, int32_t size);

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

void dart_fn_deliver_output(int32_t call_id,
                            uint8_t *ptr_,
                            int32_t rust_vec_len_,
                            int32_t data_len_);

void wire_boxed_blob_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *blob);

void wire_func_test_id_twin_normal(int64_t port_, struct wire_cst_test_id_twin_normal *id);

void wire_get_array_twin_normal(int64_t port_);

void wire_get_complex_array_twin_normal(int64_t port_);

void wire_last_number_twin_normal(int64_t port_, struct wire_cst_list_prim_f_64 *array);

void wire_nested_id_twin_normal(int64_t port_, struct wire_cst_list_test_id_twin_normal *id);

void wire_new_msgid_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *id);

void wire_return_boxed_feed_id_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *id);

void wire_return_boxed_raw_feed_id_twin_normal(int64_t port_,
                                               struct wire_cst_feed_id_twin_normal *id);

void wire_use_boxed_blob_twin_normal(int64_t port_, struct wire_cst_blob_twin_normal *blob);

void wire_use_msgid_twin_normal(int64_t port_, struct wire_cst_message_id_twin_normal *id);

void wire_func_async_simple_add_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_async_void_twin_normal(int64_t port_);

void wire_handle_customized_struct_twin_normal(int64_t port_,
                                               struct wire_cst_customized_twin_normal *val);

void wire_next_user_id_twin_normal(int64_t port_, struct wire_cst_user_id_twin_normal *user_id);

void wire_benchmark_input_bytes_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *bytes);

void wire_benchmark_output_bytes_twin_normal(int64_t port_, int32_t size);

void wire_benchmark_void_twin_normal(int64_t port_);

void wire_datetime_local_twin_normal(int64_t port_, int64_t d);

void wire_datetime_utc_twin_normal(int64_t port_, int64_t d);

void wire_duration_twin_normal(int64_t port_, int64_t d);

void wire_handle_durations_twin_normal(int64_t port_,
                                       struct wire_cst_list_Chrono_Duration *durations,
                                       int64_t since);

void wire_handle_timestamps_twin_normal(int64_t port_,
                                        struct wire_cst_list_Chrono_Naive *timestamps,
                                        int64_t epoch);

void wire_how_long_does_it_take_twin_normal(int64_t port_,
                                            struct wire_cst_feature_chrono_twin_normal *mine);

void wire_naivedatetime_twin_normal(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc_twin_normal(int64_t port_, int64_t *d);

void wire_test_chrono_twin_normal(int64_t port_);

void wire_test_precise_chrono_twin_normal(int64_t port_);

void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(int64_t port_,
                                                                   struct wire_cst_struct_with_comments_twin_normal *that);

void wire_StructWithCommentsTwinNormal_static_method_twin_normal(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_normal(int64_t port_);

void wire_return_dart_dynamic_twin_normal(int64_t port_);

void wire_rust_call_dart_loopback(int64_t port_, const void *callback);

void wire_rust_call_dart_multi_times(int64_t port_, const void *callback, int32_t num_times);

void wire_rust_call_dart_one_arg(int64_t port_, const void *callback);

void wire_rust_call_dart_return(int64_t port_, const void *callback);

void wire_rust_call_dart_simple(int64_t port_, const void *callback);

void wire_rust_call_dart_two_args(int64_t port_, const void *callback);

void wire_rust_call_dart_with_dart_opaque_arg(int64_t port_,
                                              const void *input,
                                              const void *callback);

void wire_rust_call_dart_with_dart_opaque_result(int64_t port_, const void *callback);

void wire_async_accept_dart_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_clone_dart_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_create_enum_dart_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_create_nested_dart_opaque_twin_normal(int64_t port_,
                                                const void *opaque1,
                                                const void *opaque2);

void wire_drop_static_dart_opaque_twin_normal(int64_t port_, int32_t id);

void wire_get_enum_dart_opaque_twin_normal(int64_t port_,
                                           struct wire_cst_enum_dart_opaque_twin_normal *opaque);

void wire_get_nested_dart_opaque_twin_normal(int64_t port_,
                                             struct wire_cst_dart_opaque_nested_twin_normal *opaque);

void wire_loop_back_array_get_twin_normal(int64_t port_, struct wire_cst_list_DartOpaque *opaque);

void wire_loop_back_array_twin_normal(int64_t port_, const void *opaque);

void wire_loop_back_option_get_twin_normal(int64_t port_, const void **opaque);

void wire_loop_back_option_twin_normal(int64_t port_, const void *opaque);

void wire_loop_back_twin_normal(int64_t port_, const void *opaque);

void wire_loop_back_vec_get_twin_normal(int64_t port_, struct wire_cst_list_DartOpaque *opaque);

void wire_loop_back_vec_twin_normal(int64_t port_, const void *opaque);

void wire_panic_unwrap_dart_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_set_static_dart_opaque_twin_normal(int64_t port_, int32_t id, const void *opaque);

WireSyncRust2DartDco wire_sync_accept_dart_opaque_twin_normal(const void *opaque);

WireSyncRust2DartDco wire_sync_loopback_twin_normal(const void *opaque);

WireSyncRust2DartDco wire_sync_option_dart_opaque_twin_normal(const void *opaque);

WireSyncRust2DartDco wire_sync_option_loopback_twin_normal(const void **opaque);

WireSyncRust2DartDco wire_unwrap_dart_opaque_twin_normal(const void *opaque);

void wire_func_enum_simple_twin_normal(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_normal(int64_t port_,
                                                struct wire_cst_enum_with_item_mixed_twin_normal *arg);

void wire_func_enum_with_item_struct_twin_normal(int64_t port_,
                                                 struct wire_cst_enum_with_item_struct_twin_normal *arg);

void wire_func_enum_with_item_tuple_twin_normal(int64_t port_,
                                                struct wire_cst_enum_with_item_tuple_twin_normal *arg);

void wire_handle_enum_parameter_twin_normal(int64_t port_, int32_t weekday);

void wire_handle_enum_struct_twin_normal(int64_t port_,
                                         struct wire_cst_kitchen_sink_twin_normal *val);

void wire_handle_return_enum_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *input);

void wire_multiply_by_ten_twin_normal(int64_t port_, struct wire_cst_measure_twin_normal *measure);

void wire_print_note_twin_normal(int64_t port_, struct wire_cst_note_twin_normal *note);

void wire_EventTwinNormal_as_string_twin_normal(int64_t port_,
                                                struct wire_cst_event_twin_normal *that);

void wire_close_event_listener_twin_normal(int64_t port_);

void wire_create_event_twin_normal(int64_t port_,
                                   struct wire_cst_list_prim_u_8 *address,
                                   struct wire_cst_list_prim_u_8 *payload);

void wire_register_event_listener_twin_normal(int64_t port_);

void wire_CustomStructTwinNormal_new_twin_normal(int64_t port_,
                                                 struct wire_cst_list_prim_u_8 *message);

void wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal(int64_t port_,
                                                                                  struct wire_cst_custom_struct_twin_normal *that);

void wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal(int64_t port_,
                                                                               struct wire_cst_custom_struct_twin_normal *that);

void wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal(int64_t port_);

void wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal(int64_t port_);

void wire_SomeStructTwinNormal_new_twin_normal(int64_t port_, uint32_t value);

void wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal(int64_t port_,
                                                                              struct wire_cst_some_struct_twin_normal *that);

void wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal(int64_t port_,
                                                                             struct wire_cst_some_struct_twin_normal *that);

void wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal(int64_t port_);

void wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal(int64_t port_);

void wire_custom_enum_error_panic_twin_normal(int64_t port_);

void wire_custom_enum_error_return_error_twin_normal(int64_t port_);

void wire_custom_enum_error_return_ok_twin_normal(int64_t port_, uint32_t arg);

void wire_custom_nested_error_return_error_twin_normal(int64_t port_,
                                                       struct wire_cst_custom_nested_error_outer_twin_normal *arg);

void wire_custom_struct_error_return_error_twin_normal(int64_t port_,
                                                       struct wire_cst_custom_struct_error_twin_normal *arg);

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

void wire_throw_anyhow_twin_normal(int64_t port_);

void wire_call_new_module_system_twin_normal(int64_t port_);

void wire_call_old_module_system_twin_normal(int64_t port_);

void wire_use_imported_enum_twin_normal(int64_t port_, int32_t my_enum);

void wire_use_imported_struct_twin_normal(int64_t port_, struct wire_cst_my_struct *my_struct);

void wire_another_macro_struct_twin_normal(int64_t port_);

void wire_func_macro_struct_twin_normal(int64_t port_, struct wire_cst_macro_struct *arg);

void wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal(int64_t port_,
                                                                   struct wire_cst_list_prim_u_8 *a,
                                                                   struct wire_cst_list_prim_u_8 *b);

void wire_ConcatenateWithTwinNormal_concatenate_twin_normal(int64_t port_,
                                                            struct wire_cst_concatenate_with_twin_normal *that,
                                                            struct wire_cst_list_prim_u_8 *b);

void wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal(int64_t port_);

void wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal(int64_t port_,
                                                                               uint32_t key,
                                                                               uint32_t max);

void wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal(int64_t port_,
                                                                             struct wire_cst_concatenate_with_twin_normal *that);

void wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal(int64_t port_,
                                                                        struct wire_cst_concatenate_with_twin_normal *that,
                                                                        uint32_t key,
                                                                        uint32_t max);

void wire_ConcatenateWithTwinNormal_new_twin_normal(int64_t port_,
                                                    struct wire_cst_list_prim_u_8 *a);

void wire_SumWithTwinNormal_sum_twin_normal(int64_t port_,
                                            struct wire_cst_sum_with_twin_normal *that,
                                            uint32_t y,
                                            uint32_t z);

void wire_get_sum_array_twin_normal(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_get_sum_struct_twin_normal(int64_t port_);

void wire_app_settings_stream_twin_normal(int64_t port_);

void wire_app_settings_vec_stream_twin_normal(int64_t port_);

void wire_first_number_twin_normal(int64_t port_, struct wire_cst_numbers *nums);

void wire_first_sequence_twin_normal(int64_t port_, struct wire_cst_sequences *seqs);

void wire_get_app_settings_twin_normal(int64_t port_);

void wire_get_fallible_app_settings_twin_normal(int64_t port_);

void wire_get_message_twin_normal(int64_t port_);

void wire_is_app_embedded_twin_normal(int64_t port_,
                                      struct wire_cst_application_settings *app_settings);

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

void wire_handle_complex_struct_twin_normal(int64_t port_,
                                            struct wire_cst_my_tree_node_twin_normal *s);

void wire_handle_nested_struct_twin_normal(int64_t port_,
                                           struct wire_cst_my_nested_struct_twin_normal *s);

void wire_handle_string_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *s);

void wire_handle_struct_twin_normal(int64_t port_,
                                    struct wire_cst_my_size *arg,
                                    struct wire_cst_my_size *boxed);

void wire_handle_vec_u8_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *v);

void wire_list_of_primitive_enums_twin_normal(int64_t port_,
                                              struct wire_cst_list_weekdays_twin_normal *weekdays);

void wire_test_abc_enum_twin_normal(int64_t port_, struct wire_cst_abc_twin_normal *abc);

void wire_test_struct_with_enum_twin_normal(int64_t port_,
                                            struct wire_cst_struct_with_enum_twin_normal *se);

void wire_empty_struct_twin_normal(int64_t port_, struct wire_cst_empty_twin_normal *empty);

void wire_func_return_unit_twin_normal(int64_t port_);

void wire_func_string_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *arg);

void wire_handle_list_of_struct_twin_normal(int64_t port_, struct wire_cst_list_my_size *l);

void wire_handle_string_list_twin_normal(int64_t port_, struct wire_cst_list_String *names);

void wire_handle_newtype_twin_normal(int64_t port_, struct wire_cst_new_type_int_twin_normal *arg);

void wire_handle_increment_boxed_optional_twin_normal(int64_t port_, double *opt);

void wire_handle_option_box_arguments_twin_normal(int64_t port_,
                                                  int8_t *i8box,
                                                  uint8_t *u8box,
                                                  int32_t *i32box,
                                                  int64_t *i64box,
                                                  double *f64box,
                                                  bool *boolbox,
                                                  struct wire_cst_exotic_optionals_twin_normal *structbox);

void wire_handle_optional_increment_twin_normal(int64_t port_,
                                                struct wire_cst_exotic_optionals_twin_normal *opt);

void wire_handle_optional_return_twin_normal(int64_t port_, double left, double right);

void wire_handle_optional_struct_twin_normal(int64_t port_,
                                             struct wire_cst_list_prim_u_8 *document);

void wire_handle_vec_of_opts_twin_normal(int64_t port_, struct wire_cst_opt_vecs_twin_normal *opt);

void wire_primitive_optional_types_twin_normal(int64_t port_,
                                               int32_t *my_i32,
                                               int64_t *my_i64,
                                               double *my_f64,
                                               bool *my_bool);

void wire_handle_vec_of_primitive_twin_normal(int64_t port_, int32_t n);

void wire_primitive_types_twin_normal(int64_t port_,
                                      int32_t my_i32,
                                      int64_t my_i64,
                                      double my_f64,
                                      bool my_bool);

void wire_primitive_u32_twin_normal(int64_t port_, uint32_t my_u32);

void wire_boxed_blob_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *blob);

void wire_func_test_id_twin_rust_async(int64_t port_, struct wire_cst_test_id_twin_rust_async *id);

void wire_get_array_twin_rust_async(int64_t port_);

void wire_get_complex_array_twin_rust_async(int64_t port_);

void wire_last_number_twin_rust_async(int64_t port_, struct wire_cst_list_prim_f_64 *array);

void wire_nested_id_twin_rust_async(int64_t port_,
                                    struct wire_cst_list_test_id_twin_rust_async *id);

void wire_new_msgid_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *id);

void wire_return_boxed_feed_id_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *id);

void wire_return_boxed_raw_feed_id_twin_rust_async(int64_t port_,
                                                   struct wire_cst_feed_id_twin_rust_async *id);

void wire_use_boxed_blob_twin_rust_async(int64_t port_, struct wire_cst_blob_twin_rust_async *blob);

void wire_use_msgid_twin_rust_async(int64_t port_, struct wire_cst_message_id_twin_rust_async *id);

void wire_boxed_blob_twin_rust_async_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_func_test_id_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_get_array_twin_rust_async_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_get_complex_array_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_last_number_twin_rust_async_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_nested_id_twin_rust_async_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_new_msgid_twin_rust_async_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_return_boxed_feed_id_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_return_boxed_raw_feed_id_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_use_boxed_blob_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_use_msgid_twin_rust_async_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_boxed_blob_twin_sse(int64_t port_,
                              uint8_t *ptr_,
                              int32_t rust_vec_len_,
                              int32_t data_len_);

void wire_func_test_id_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_get_array_twin_sse(int64_t port_,
                             uint8_t *ptr_,
                             int32_t rust_vec_len_,
                             int32_t data_len_);

void wire_get_complex_array_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_last_number_twin_sse(int64_t port_,
                               uint8_t *ptr_,
                               int32_t rust_vec_len_,
                               int32_t data_len_);

void wire_nested_id_twin_sse(int64_t port_,
                             uint8_t *ptr_,
                             int32_t rust_vec_len_,
                             int32_t data_len_);

void wire_new_msgid_twin_sse(int64_t port_,
                             uint8_t *ptr_,
                             int32_t rust_vec_len_,
                             int32_t data_len_);

void wire_return_boxed_feed_id_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_return_boxed_raw_feed_id_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_use_boxed_blob_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

void wire_use_msgid_twin_sse(int64_t port_,
                             uint8_t *ptr_,
                             int32_t rust_vec_len_,
                             int32_t data_len_);

WireSyncRust2DartDco wire_boxed_blob_twin_sync(struct wire_cst_list_prim_u_8 *blob);

WireSyncRust2DartDco wire_func_test_id_twin_sync(struct wire_cst_test_id_twin_sync *id);

WireSyncRust2DartDco wire_get_array_twin_sync(void);

WireSyncRust2DartDco wire_get_complex_array_twin_sync(void);

WireSyncRust2DartDco wire_last_number_twin_sync(struct wire_cst_list_prim_f_64 *array);

WireSyncRust2DartDco wire_nested_id_twin_sync(struct wire_cst_list_test_id_twin_sync *id);

WireSyncRust2DartDco wire_new_msgid_twin_sync(struct wire_cst_list_prim_u_8 *id);

WireSyncRust2DartDco wire_return_boxed_feed_id_twin_sync(struct wire_cst_list_prim_u_8 *id);

WireSyncRust2DartDco wire_return_boxed_raw_feed_id_twin_sync(struct wire_cst_feed_id_twin_sync *id);

WireSyncRust2DartDco wire_use_boxed_blob_twin_sync(struct wire_cst_blob_twin_sync *blob);

WireSyncRust2DartDco wire_use_msgid_twin_sync(struct wire_cst_message_id_twin_sync *id);

WireSyncRust2DartSse wire_boxed_blob_twin_sync_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

WireSyncRust2DartSse wire_func_test_id_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

WireSyncRust2DartSse wire_get_array_twin_sync_sse(uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

WireSyncRust2DartSse wire_get_complex_array_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_last_number_twin_sync_sse(uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

WireSyncRust2DartSse wire_nested_id_twin_sync_sse(uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

WireSyncRust2DartSse wire_new_msgid_twin_sync_sse(uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

WireSyncRust2DartSse wire_return_boxed_feed_id_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_return_boxed_raw_feed_id_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_use_boxed_blob_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

WireSyncRust2DartSse wire_use_msgid_twin_sync_sse(uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_func_async_simple_add_twin_sse(int64_t port_, int32_t a, int32_t b);

void wire_func_async_void_twin_sse(int64_t port_);

void wire_handle_customized_struct_twin_rust_async(int64_t port_,
                                                   struct wire_cst_customized_twin_rust_async *val);

void wire_next_user_id_twin_rust_async(int64_t port_,
                                       struct wire_cst_user_id_twin_rust_async *user_id);

void wire_handle_customized_struct_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_next_user_id_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_handle_customized_struct_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_next_user_id_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

WireSyncRust2DartDco wire_handle_customized_struct_twin_sync(struct wire_cst_customized_twin_sync *val);

WireSyncRust2DartDco wire_next_user_id_twin_sync(struct wire_cst_user_id_twin_sync *user_id);

WireSyncRust2DartSse wire_handle_customized_struct_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_next_user_id_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_benchmark_input_bytes_twin_rust_async(int64_t port_,
                                                struct wire_cst_list_prim_u_8 *bytes);

void wire_benchmark_output_bytes_twin_rust_async(int64_t port_, int32_t size);

void wire_benchmark_void_twin_rust_async(int64_t port_);

void wire_benchmark_input_bytes_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_benchmark_output_bytes_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_benchmark_void_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_benchmark_input_bytes_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_benchmark_output_bytes_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_benchmark_void_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

WireSyncRust2DartDco wire_benchmark_input_bytes_twin_sync(struct wire_cst_list_prim_u_8 *bytes);

WireSyncRust2DartDco wire_benchmark_output_bytes_twin_sync(int32_t size);

WireSyncRust2DartDco wire_benchmark_void_twin_sync(void);

WireSyncRust2DartSse wire_benchmark_input_bytes_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

WireSyncRust2DartSse wire_benchmark_output_bytes_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_benchmark_void_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_datetime_local_twin_rust_async(int64_t port_, int64_t d);

void wire_datetime_utc_twin_rust_async(int64_t port_, int64_t d);

void wire_duration_twin_rust_async(int64_t port_, int64_t d);

void wire_handle_durations_twin_rust_async(int64_t port_,
                                           struct wire_cst_list_Chrono_Duration *durations,
                                           int64_t since);

void wire_handle_timestamps_twin_rust_async(int64_t port_,
                                            struct wire_cst_list_Chrono_Naive *timestamps,
                                            int64_t epoch);

void wire_how_long_does_it_take_twin_rust_async(int64_t port_,
                                                struct wire_cst_feature_chrono_twin_rust_async *mine);

void wire_naivedatetime_twin_rust_async(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc_twin_rust_async(int64_t port_, int64_t *d);

void wire_test_chrono_twin_rust_async(int64_t port_);

void wire_test_precise_chrono_twin_rust_async(int64_t port_);

WireSyncRust2DartDco wire_datetime_local_twin_sync(int64_t d);

WireSyncRust2DartDco wire_datetime_utc_twin_sync(int64_t d);

WireSyncRust2DartDco wire_duration_twin_sync(int64_t d);

WireSyncRust2DartDco wire_handle_durations_twin_sync(struct wire_cst_list_Chrono_Duration *durations,
                                                     int64_t since);

WireSyncRust2DartDco wire_handle_timestamps_twin_sync(struct wire_cst_list_Chrono_Naive *timestamps,
                                                      int64_t epoch);

WireSyncRust2DartDco wire_how_long_does_it_take_twin_sync(struct wire_cst_feature_chrono_twin_sync *mine);

WireSyncRust2DartDco wire_naivedatetime_twin_sync(int64_t d);

WireSyncRust2DartDco wire_optional_empty_datetime_utc_twin_sync(int64_t *d);

WireSyncRust2DartDco wire_test_chrono_twin_sync(void);

WireSyncRust2DartDco wire_test_precise_chrono_twin_sync(void);

void wire_StructWithCommentsTwinRustAsync_instance_method_twin_rust_async(int64_t port_,
                                                                          struct wire_cst_struct_with_comments_twin_rust_async *that);

void wire_StructWithCommentsTwinRustAsync_static_method_twin_rust_async(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_rust_async(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_rust_async(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_rust_async(int64_t port_);

void wire_StructWithCommentsTwinRustAsyncSse_instance_method_twin_rust_async_sse(int64_t port_,
                                                                                 uint8_t *ptr_,
                                                                                 int32_t rust_vec_len_,
                                                                                 int32_t data_len_);

void wire_StructWithCommentsTwinRustAsyncSse_static_method_twin_rust_async_sse(int64_t port_,
                                                                               uint8_t *ptr_,
                                                                               int32_t rust_vec_len_,
                                                                               int32_t data_len_);

void wire_function_with_comments_slash_star_star_twin_rust_async_sse(int64_t port_,
                                                                     uint8_t *ptr_,
                                                                     int32_t rust_vec_len_,
                                                                     int32_t data_len_);

void wire_function_with_comments_triple_slash_multi_line_twin_rust_async_sse(int64_t port_,
                                                                             uint8_t *ptr_,
                                                                             int32_t rust_vec_len_,
                                                                             int32_t data_len_);

void wire_function_with_comments_triple_slash_single_line_twin_rust_async_sse(int64_t port_,
                                                                              uint8_t *ptr_,
                                                                              int32_t rust_vec_len_,
                                                                              int32_t data_len_);

void wire_StructWithCommentsTwinSse_instance_method_twin_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_StructWithCommentsTwinSse_static_method_twin_sse(int64_t port_,
                                                           uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_function_with_comments_slash_star_star_twin_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_function_with_comments_triple_slash_multi_line_twin_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_function_with_comments_triple_slash_single_line_twin_sse(int64_t port_,
                                                                   uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartDco wire_StructWithCommentsTwinSync_instance_method_twin_sync(struct wire_cst_struct_with_comments_twin_sync *that);

WireSyncRust2DartDco wire_StructWithCommentsTwinSync_static_method_twin_sync(void);

WireSyncRust2DartDco wire_function_with_comments_slash_star_star_twin_sync(void);

WireSyncRust2DartDco wire_function_with_comments_triple_slash_multi_line_twin_sync(void);

WireSyncRust2DartDco wire_function_with_comments_triple_slash_single_line_twin_sync(void);

WireSyncRust2DartSse wire_StructWithCommentsTwinSyncSse_instance_method_twin_sync_sse(uint8_t *ptr_,
                                                                                      int32_t rust_vec_len_,
                                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_StructWithCommentsTwinSyncSse_static_method_twin_sync_sse(uint8_t *ptr_,
                                                                                    int32_t rust_vec_len_,
                                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_function_with_comments_slash_star_star_twin_sync_sse(uint8_t *ptr_,
                                                                               int32_t rust_vec_len_,
                                                                               int32_t data_len_);

WireSyncRust2DartSse wire_function_with_comments_triple_slash_multi_line_twin_sync_sse(uint8_t *ptr_,
                                                                                       int32_t rust_vec_len_,
                                                                                       int32_t data_len_);

WireSyncRust2DartSse wire_function_with_comments_triple_slash_single_line_twin_sync_sse(uint8_t *ptr_,
                                                                                        int32_t rust_vec_len_,
                                                                                        int32_t data_len_);

void wire_return_dart_dynamic_twin_rust_async(int64_t port_);

WireSyncRust2DartDco wire_return_dart_dynamic_twin_sync(void);

WireSyncRust2DartSse wire_sync_accept_dart_opaque_twin_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_sync_loopback_twin_sse(uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

WireSyncRust2DartSse wire_sync_option_dart_opaque_twin_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_sync_option_loopback_twin_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_unwrap_dart_opaque_twin_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_async_accept_dart_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_clone_dart_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_create_enum_dart_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_create_nested_dart_opaque_twin_rust_async(int64_t port_,
                                                    const void *opaque1,
                                                    const void *opaque2);

void wire_drop_static_dart_opaque_twin_rust_async(int64_t port_, int32_t id);

void wire_get_enum_dart_opaque_twin_rust_async(int64_t port_,
                                               struct wire_cst_enum_dart_opaque_twin_rust_async *opaque);

void wire_get_nested_dart_opaque_twin_rust_async(int64_t port_,
                                                 struct wire_cst_dart_opaque_nested_twin_rust_async *opaque);

void wire_loop_back_array_get_twin_rust_async(int64_t port_,
                                              struct wire_cst_list_DartOpaque *opaque);

void wire_loop_back_array_twin_rust_async(int64_t port_, const void *opaque);

void wire_loop_back_option_get_twin_rust_async(int64_t port_, const void **opaque);

void wire_loop_back_option_twin_rust_async(int64_t port_, const void *opaque);

void wire_loop_back_twin_rust_async(int64_t port_, const void *opaque);

void wire_loop_back_vec_get_twin_rust_async(int64_t port_, struct wire_cst_list_DartOpaque *opaque);

void wire_loop_back_vec_twin_rust_async(int64_t port_, const void *opaque);

void wire_panic_unwrap_dart_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_set_static_dart_opaque_twin_rust_async(int64_t port_, int32_t id, const void *opaque);

void wire_async_accept_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_clone_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_create_enum_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_create_nested_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_drop_static_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_get_enum_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_get_nested_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_loop_back_array_get_twin_rust_async_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_loop_back_array_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_loop_back_option_get_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_loop_back_option_twin_rust_async_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_loop_back_twin_rust_async_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_loop_back_vec_get_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_loop_back_vec_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_panic_unwrap_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_set_static_dart_opaque_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_async_accept_dart_opaque_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_clone_dart_opaque_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_create_enum_dart_opaque_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_create_nested_dart_opaque_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_drop_static_dart_opaque_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_get_enum_dart_opaque_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_get_nested_dart_opaque_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_loop_back_array_get_twin_sse(int64_t port_,
                                       uint8_t *ptr_,
                                       int32_t rust_vec_len_,
                                       int32_t data_len_);

void wire_loop_back_array_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_loop_back_option_get_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_loop_back_option_twin_sse(int64_t port_,
                                    uint8_t *ptr_,
                                    int32_t rust_vec_len_,
                                    int32_t data_len_);

void wire_loop_back_twin_sse(int64_t port_,
                             uint8_t *ptr_,
                             int32_t rust_vec_len_,
                             int32_t data_len_);

void wire_loop_back_vec_get_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_loop_back_vec_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_panic_unwrap_dart_opaque_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_set_static_dart_opaque_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

WireSyncRust2DartDco wire_async_accept_dart_opaque_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_clone_dart_opaque_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_create_enum_dart_opaque_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_create_nested_dart_opaque_twin_sync(const void *opaque1,
                                                              const void *opaque2);

WireSyncRust2DartDco wire_drop_static_dart_opaque_twin_sync(int32_t id);

WireSyncRust2DartDco wire_get_enum_dart_opaque_twin_sync(struct wire_cst_enum_dart_opaque_twin_sync *opaque);

WireSyncRust2DartDco wire_get_nested_dart_opaque_twin_sync(struct wire_cst_dart_opaque_nested_twin_sync *opaque);

WireSyncRust2DartDco wire_loop_back_array_get_twin_sync(struct wire_cst_list_DartOpaque *opaque);

WireSyncRust2DartDco wire_loop_back_array_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_loop_back_option_get_twin_sync(const void **opaque);

WireSyncRust2DartDco wire_loop_back_option_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_loop_back_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_loop_back_vec_get_twin_sync(struct wire_cst_list_DartOpaque *opaque);

WireSyncRust2DartDco wire_loop_back_vec_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_panic_unwrap_dart_opaque_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_set_static_dart_opaque_twin_sync(int32_t id, const void *opaque);

WireSyncRust2DartSse wire_async_accept_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_clone_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_create_enum_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_create_nested_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_drop_static_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_get_enum_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_get_nested_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_array_get_twin_sync_sse(uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_array_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_option_get_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_option_twin_sync_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_twin_sync_sse(uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_vec_get_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_loop_back_vec_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_panic_unwrap_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_set_static_dart_opaque_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_func_enum_simple_twin_rust_async(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_rust_async(int64_t port_,
                                                    struct wire_cst_enum_with_item_mixed_twin_rust_async *arg);

void wire_func_enum_with_item_struct_twin_rust_async(int64_t port_,
                                                     struct wire_cst_enum_with_item_struct_twin_rust_async *arg);

void wire_func_enum_with_item_tuple_twin_rust_async(int64_t port_,
                                                    struct wire_cst_enum_with_item_tuple_twin_rust_async *arg);

void wire_handle_enum_parameter_twin_rust_async(int64_t port_, int32_t weekday);

void wire_handle_enum_struct_twin_rust_async(int64_t port_,
                                             struct wire_cst_kitchen_sink_twin_rust_async *val);

void wire_handle_return_enum_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *input);

void wire_multiply_by_ten_twin_rust_async(int64_t port_,
                                          struct wire_cst_measure_twin_rust_async *measure);

void wire_print_note_twin_rust_async(int64_t port_, struct wire_cst_note_twin_rust_async *note);

void wire_func_enum_simple_twin_rust_async_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_func_enum_with_item_mixed_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_func_enum_with_item_struct_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_func_enum_with_item_tuple_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_handle_enum_parameter_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_handle_enum_struct_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_handle_return_enum_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_multiply_by_ten_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_print_note_twin_rust_async_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_func_enum_simple_twin_sse(int64_t port_,
                                    uint8_t *ptr_,
                                    int32_t rust_vec_len_,
                                    int32_t data_len_);

void wire_func_enum_with_item_mixed_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_func_enum_with_item_struct_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_func_enum_with_item_tuple_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_handle_enum_parameter_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_handle_enum_struct_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_handle_return_enum_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_multiply_by_ten_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_print_note_twin_sse(int64_t port_,
                              uint8_t *ptr_,
                              int32_t rust_vec_len_,
                              int32_t data_len_);

WireSyncRust2DartDco wire_func_enum_simple_twin_sync(int32_t arg);

WireSyncRust2DartDco wire_func_enum_with_item_mixed_twin_sync(struct wire_cst_enum_with_item_mixed_twin_sync *arg);

WireSyncRust2DartDco wire_func_enum_with_item_struct_twin_sync(struct wire_cst_enum_with_item_struct_twin_sync *arg);

WireSyncRust2DartDco wire_func_enum_with_item_tuple_twin_sync(struct wire_cst_enum_with_item_tuple_twin_sync *arg);

WireSyncRust2DartDco wire_handle_enum_parameter_twin_sync(int32_t weekday);

WireSyncRust2DartDco wire_handle_enum_struct_twin_sync(struct wire_cst_kitchen_sink_twin_sync *val);

WireSyncRust2DartDco wire_handle_return_enum_twin_sync(struct wire_cst_list_prim_u_8 *input);

WireSyncRust2DartDco wire_multiply_by_ten_twin_sync(struct wire_cst_measure_twin_sync *measure);

WireSyncRust2DartDco wire_print_note_twin_sync(struct wire_cst_note_twin_sync *note);

WireSyncRust2DartSse wire_func_enum_simple_twin_sync_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_func_enum_with_item_mixed_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_func_enum_with_item_struct_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_func_enum_with_item_tuple_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_handle_enum_parameter_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

WireSyncRust2DartSse wire_handle_enum_struct_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_handle_return_enum_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_multiply_by_ten_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_print_note_twin_sync_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_EventTwinRustAsync_as_string_twin_rust_async(int64_t port_,
                                                       struct wire_cst_event_twin_rust_async *that);

void wire_close_event_listener_twin_rust_async(int64_t port_);

void wire_create_event_twin_rust_async(int64_t port_,
                                       struct wire_cst_list_prim_u_8 *address,
                                       struct wire_cst_list_prim_u_8 *payload);

void wire_register_event_listener_twin_rust_async(int64_t port_);

void wire_EventTwinRustAsyncSse_as_string_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_close_event_listener_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_create_event_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_register_event_listener_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_EventTwinSse_as_string_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_close_event_listener_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_create_event_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_register_event_listener_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_CustomStructTwinRustAsync_new_twin_rust_async(int64_t port_,
                                                        struct wire_cst_list_prim_u_8 *message);

void wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_error_twin_rust_async(int64_t port_,
                                                                                         struct wire_cst_custom_struct_twin_rust_async *that);

void wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_ok_twin_rust_async(int64_t port_,
                                                                                      struct wire_cst_custom_struct_twin_rust_async *that);

void wire_CustomStructTwinRustAsync_static_return_custom_struct_error_twin_rust_async(int64_t port_);

void wire_CustomStructTwinRustAsync_static_return_custom_struct_ok_twin_rust_async(int64_t port_);

void wire_SomeStructTwinRustAsync_new_twin_rust_async(int64_t port_, uint32_t value);

void wire_SomeStructTwinRustAsync_non_static_return_err_custom_error_twin_rust_async(int64_t port_,
                                                                                     struct wire_cst_some_struct_twin_rust_async *that);

void wire_SomeStructTwinRustAsync_non_static_return_ok_custom_error_twin_rust_async(int64_t port_,
                                                                                    struct wire_cst_some_struct_twin_rust_async *that);

void wire_SomeStructTwinRustAsync_static_return_err_custom_error_twin_rust_async(int64_t port_);

void wire_SomeStructTwinRustAsync_static_return_ok_custom_error_twin_rust_async(int64_t port_);

void wire_custom_enum_error_panic_twin_rust_async(int64_t port_);

void wire_custom_enum_error_return_error_twin_rust_async(int64_t port_);

void wire_custom_enum_error_return_ok_twin_rust_async(int64_t port_, uint32_t arg);

void wire_custom_nested_error_return_error_twin_rust_async(int64_t port_,
                                                           struct wire_cst_custom_nested_error_outer_twin_rust_async *arg);

void wire_custom_struct_error_return_error_twin_rust_async(int64_t port_,
                                                           struct wire_cst_custom_struct_error_twin_rust_async *arg);

void wire_func_return_error_twin_rust_async(int64_t port_);

void wire_func_type_fallible_panic_twin_rust_async(int64_t port_);

void wire_func_type_infallible_panic_twin_rust_async(int64_t port_);

void wire_panic_with_custom_result_twin_rust_async(int64_t port_);

void wire_return_custom_nested_error_1_twin_rust_async(int64_t port_);

void wire_return_custom_nested_error_1_variant1_twin_rust_async(int64_t port_);

void wire_return_custom_nested_error_2_twin_rust_async(int64_t port_);

void wire_return_custom_struct_error_twin_rust_async(int64_t port_);

void wire_return_custom_struct_ok_twin_rust_async(int64_t port_);

void wire_return_err_custom_error_twin_rust_async(int64_t port_);

void wire_return_error_variant_twin_rust_async(int64_t port_, uint32_t variant);

void wire_return_ok_custom_error_twin_rust_async(int64_t port_);

void wire_stream_sink_throw_anyhow_twin_rust_async(int64_t port_);

void wire_throw_anyhow_twin_rust_async(int64_t port_);

void wire_CustomStructTwinRustAsyncSse_new_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_CustomStructTwinRustAsyncSse_nonstatic_return_custom_struct_error_twin_rust_async_sse(int64_t port_,
                                                                                                uint8_t *ptr_,
                                                                                                int32_t rust_vec_len_,
                                                                                                int32_t data_len_);

void wire_CustomStructTwinRustAsyncSse_nonstatic_return_custom_struct_ok_twin_rust_async_sse(int64_t port_,
                                                                                             uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

void wire_CustomStructTwinRustAsyncSse_static_return_custom_struct_error_twin_rust_async_sse(int64_t port_,
                                                                                             uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

void wire_CustomStructTwinRustAsyncSse_static_return_custom_struct_ok_twin_rust_async_sse(int64_t port_,
                                                                                          uint8_t *ptr_,
                                                                                          int32_t rust_vec_len_,
                                                                                          int32_t data_len_);

void wire_SomeStructTwinRustAsyncSse_new_twin_rust_async_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_SomeStructTwinRustAsyncSse_non_static_return_err_custom_error_twin_rust_async_sse(int64_t port_,
                                                                                            uint8_t *ptr_,
                                                                                            int32_t rust_vec_len_,
                                                                                            int32_t data_len_);

void wire_SomeStructTwinRustAsyncSse_non_static_return_ok_custom_error_twin_rust_async_sse(int64_t port_,
                                                                                           uint8_t *ptr_,
                                                                                           int32_t rust_vec_len_,
                                                                                           int32_t data_len_);

void wire_SomeStructTwinRustAsyncSse_static_return_err_custom_error_twin_rust_async_sse(int64_t port_,
                                                                                        uint8_t *ptr_,
                                                                                        int32_t rust_vec_len_,
                                                                                        int32_t data_len_);

void wire_SomeStructTwinRustAsyncSse_static_return_ok_custom_error_twin_rust_async_sse(int64_t port_,
                                                                                       uint8_t *ptr_,
                                                                                       int32_t rust_vec_len_,
                                                                                       int32_t data_len_);

void wire_custom_enum_error_panic_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_custom_enum_error_return_error_twin_rust_async_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_custom_enum_error_return_ok_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_custom_nested_error_return_error_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_custom_struct_error_return_error_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_func_return_error_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_func_type_fallible_panic_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_func_type_infallible_panic_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_panic_with_custom_result_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_return_custom_nested_error_1_twin_rust_async_sse(int64_t port_,
                                                           uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_return_custom_nested_error_1_variant1_twin_rust_async_sse(int64_t port_,
                                                                    uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_return_custom_nested_error_2_twin_rust_async_sse(int64_t port_,
                                                           uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_return_custom_struct_error_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_return_custom_struct_ok_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_return_err_custom_error_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_return_error_variant_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_return_ok_custom_error_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_stream_sink_throw_anyhow_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_throw_anyhow_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_CustomStructTwinSse_new_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_CustomStructTwinSse_nonstatic_return_custom_struct_error_twin_sse(int64_t port_,
                                                                            uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

void wire_CustomStructTwinSse_nonstatic_return_custom_struct_ok_twin_sse(int64_t port_,
                                                                         uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

void wire_CustomStructTwinSse_static_return_custom_struct_error_twin_sse(int64_t port_,
                                                                         uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

void wire_CustomStructTwinSse_static_return_custom_struct_ok_twin_sse(int64_t port_,
                                                                      uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

void wire_SomeStructTwinSse_new_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_SomeStructTwinSse_non_static_return_err_custom_error_twin_sse(int64_t port_,
                                                                        uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

void wire_SomeStructTwinSse_non_static_return_ok_custom_error_twin_sse(int64_t port_,
                                                                       uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

void wire_SomeStructTwinSse_static_return_err_custom_error_twin_sse(int64_t port_,
                                                                    uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_SomeStructTwinSse_static_return_ok_custom_error_twin_sse(int64_t port_,
                                                                   uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

void wire_custom_enum_error_panic_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_custom_enum_error_return_error_twin_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_custom_enum_error_return_ok_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_custom_nested_error_return_error_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_custom_struct_error_return_error_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_func_return_error_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_func_type_fallible_panic_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_func_type_infallible_panic_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_panic_with_custom_result_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_return_custom_nested_error_1_twin_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_return_custom_nested_error_1_variant1_twin_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_return_custom_nested_error_2_twin_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_return_custom_struct_error_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_return_custom_struct_ok_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_return_err_custom_error_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_return_error_variant_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_return_ok_custom_error_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_stream_sink_throw_anyhow_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_throw_anyhow_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

WireSyncRust2DartDco wire_CustomStructTwinSync_new_twin_sync(struct wire_cst_list_prim_u_8 *message);

WireSyncRust2DartDco wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync(struct wire_cst_custom_struct_twin_sync *that);

WireSyncRust2DartDco wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync(struct wire_cst_custom_struct_twin_sync *that);

WireSyncRust2DartDco wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync(void);

WireSyncRust2DartDco wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync(void);

WireSyncRust2DartDco wire_SomeStructTwinSync_new_twin_sync(uint32_t value);

WireSyncRust2DartDco wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync(struct wire_cst_some_struct_twin_sync *that);

WireSyncRust2DartDco wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync(struct wire_cst_some_struct_twin_sync *that);

WireSyncRust2DartDco wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync(void);

WireSyncRust2DartDco wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync(void);

WireSyncRust2DartDco wire_custom_enum_error_panic_twin_sync(void);

WireSyncRust2DartDco wire_custom_enum_error_return_error_twin_sync(void);

WireSyncRust2DartDco wire_custom_enum_error_return_ok_twin_sync(uint32_t arg);

WireSyncRust2DartDco wire_custom_nested_error_return_error_twin_sync(struct wire_cst_custom_nested_error_outer_twin_sync *arg);

WireSyncRust2DartDco wire_custom_struct_error_return_error_twin_sync(struct wire_cst_custom_struct_error_twin_sync *arg);

WireSyncRust2DartDco wire_func_return_error_twin_sync(void);

WireSyncRust2DartDco wire_func_type_fallible_panic_twin_sync(void);

WireSyncRust2DartDco wire_func_type_infallible_panic_twin_sync(void);

WireSyncRust2DartDco wire_panic_with_custom_result_twin_sync(void);

WireSyncRust2DartDco wire_return_custom_nested_error_1_twin_sync(void);

WireSyncRust2DartDco wire_return_custom_nested_error_1_variant1_twin_sync(void);

WireSyncRust2DartDco wire_return_custom_nested_error_2_twin_sync(void);

WireSyncRust2DartDco wire_return_custom_struct_error_twin_sync(void);

WireSyncRust2DartDco wire_return_custom_struct_ok_twin_sync(void);

WireSyncRust2DartDco wire_return_err_custom_error_twin_sync(void);

WireSyncRust2DartDco wire_return_error_variant_twin_sync(uint32_t variant);

WireSyncRust2DartDco wire_return_ok_custom_error_twin_sync(void);

void wire_stream_sink_throw_anyhow_twin_sync(int64_t port_);

WireSyncRust2DartDco wire_throw_anyhow_twin_sync(void);

WireSyncRust2DartSse wire_CustomStructTwinSyncSse_new_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_CustomStructTwinSyncSse_nonstatic_return_custom_struct_error_twin_sync_sse(uint8_t *ptr_,
                                                                                                     int32_t rust_vec_len_,
                                                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_CustomStructTwinSyncSse_nonstatic_return_custom_struct_ok_twin_sync_sse(uint8_t *ptr_,
                                                                                                  int32_t rust_vec_len_,
                                                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_CustomStructTwinSyncSse_static_return_custom_struct_error_twin_sync_sse(uint8_t *ptr_,
                                                                                                  int32_t rust_vec_len_,
                                                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_CustomStructTwinSyncSse_static_return_custom_struct_ok_twin_sync_sse(uint8_t *ptr_,
                                                                                               int32_t rust_vec_len_,
                                                                                               int32_t data_len_);

WireSyncRust2DartSse wire_SomeStructTwinSyncSse_new_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_SomeStructTwinSyncSse_non_static_return_err_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                                                                 int32_t rust_vec_len_,
                                                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_SomeStructTwinSyncSse_non_static_return_ok_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                                                                int32_t rust_vec_len_,
                                                                                                int32_t data_len_);

WireSyncRust2DartSse wire_SomeStructTwinSyncSse_static_return_err_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

WireSyncRust2DartSse wire_SomeStructTwinSyncSse_static_return_ok_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                                                            int32_t rust_vec_len_,
                                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_custom_enum_error_panic_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_custom_enum_error_return_error_twin_sync_sse(uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

WireSyncRust2DartSse wire_custom_enum_error_return_ok_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_custom_nested_error_return_error_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

WireSyncRust2DartSse wire_custom_struct_error_return_error_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

WireSyncRust2DartSse wire_func_return_error_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_func_type_fallible_panic_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_func_type_infallible_panic_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_panic_with_custom_result_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_return_custom_nested_error_1_twin_sync_sse(uint8_t *ptr_,
                                                                     int32_t rust_vec_len_,
                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_return_custom_nested_error_1_variant1_twin_sync_sse(uint8_t *ptr_,
                                                                              int32_t rust_vec_len_,
                                                                              int32_t data_len_);

WireSyncRust2DartSse wire_return_custom_nested_error_2_twin_sync_sse(uint8_t *ptr_,
                                                                     int32_t rust_vec_len_,
                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_return_custom_struct_error_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_return_custom_struct_ok_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_return_err_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_return_error_variant_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_return_ok_custom_error_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_stream_sink_throw_anyhow_twin_sync_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

WireSyncRust2DartSse wire_throw_anyhow_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_call_new_module_system_twin_rust_async(int64_t port_);

void wire_call_old_module_system_twin_rust_async(int64_t port_);

void wire_use_imported_enum_twin_rust_async(int64_t port_, int32_t my_enum);

void wire_use_imported_struct_twin_rust_async(int64_t port_, struct wire_cst_my_struct *my_struct);

void wire_call_new_module_system_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_call_old_module_system_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_use_imported_enum_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_use_imported_struct_twin_rust_async_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_call_new_module_system_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_call_old_module_system_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_use_imported_enum_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_use_imported_struct_twin_sse(int64_t port_,
                                       uint8_t *ptr_,
                                       int32_t rust_vec_len_,
                                       int32_t data_len_);

WireSyncRust2DartDco wire_call_new_module_system_twin_sync(void);

WireSyncRust2DartDco wire_call_old_module_system_twin_sync(void);

WireSyncRust2DartDco wire_use_imported_enum_twin_sync(int32_t my_enum);

WireSyncRust2DartDco wire_use_imported_struct_twin_sync(struct wire_cst_my_struct *my_struct);

WireSyncRust2DartSse wire_call_new_module_system_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_call_old_module_system_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_use_imported_enum_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_use_imported_struct_twin_sync_sse(uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

void wire_ConcatenateWithTwinRustAsync_concatenate_static_twin_rust_async(int64_t port_,
                                                                          struct wire_cst_list_prim_u_8 *a,
                                                                          struct wire_cst_list_prim_u_8 *b);

void wire_ConcatenateWithTwinRustAsync_concatenate_twin_rust_async(int64_t port_,
                                                                   struct wire_cst_concatenate_with_twin_rust_async *that,
                                                                   struct wire_cst_list_prim_u_8 *b);

void wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_single_arg_twin_rust_async(int64_t port_);

void wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_twin_rust_async(int64_t port_,
                                                                                      uint32_t key,
                                                                                      uint32_t max);

void wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_at_1_twin_rust_async(int64_t port_,
                                                                                    struct wire_cst_concatenate_with_twin_rust_async *that);

void wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_twin_rust_async(int64_t port_,
                                                                               struct wire_cst_concatenate_with_twin_rust_async *that,
                                                                               uint32_t key,
                                                                               uint32_t max);

void wire_ConcatenateWithTwinRustAsync_new_twin_rust_async(int64_t port_,
                                                           struct wire_cst_list_prim_u_8 *a);

void wire_SumWithTwinRustAsync_sum_twin_rust_async(int64_t port_,
                                                   struct wire_cst_sum_with_twin_rust_async *that,
                                                   uint32_t y,
                                                   uint32_t z);

void wire_get_sum_array_twin_rust_async(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_get_sum_struct_twin_rust_async(int64_t port_);

void wire_ConcatenateWithTwinRustAsyncSse_concatenate_static_twin_rust_async_sse(int64_t port_,
                                                                                 uint8_t *ptr_,
                                                                                 int32_t rust_vec_len_,
                                                                                 int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_concatenate_twin_rust_async_sse(int64_t port_,
                                                                          uint8_t *ptr_,
                                                                          int32_t rust_vec_len_,
                                                                          int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_handle_some_static_stream_sink_single_arg_twin_rust_async_sse(int64_t port_,
                                                                                                        uint8_t *ptr_,
                                                                                                        int32_t rust_vec_len_,
                                                                                                        int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_handle_some_static_stream_sink_twin_rust_async_sse(int64_t port_,
                                                                                             uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_handle_some_stream_sink_at_1_twin_rust_async_sse(int64_t port_,
                                                                                           uint8_t *ptr_,
                                                                                           int32_t rust_vec_len_,
                                                                                           int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_handle_some_stream_sink_twin_rust_async_sse(int64_t port_,
                                                                                      uint8_t *ptr_,
                                                                                      int32_t rust_vec_len_,
                                                                                      int32_t data_len_);

void wire_ConcatenateWithTwinRustAsyncSse_new_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_SumWithTwinRustAsyncSse_sum_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_get_sum_array_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_get_sum_struct_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_ConcatenateWithTwinSse_concatenate_static_twin_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_ConcatenateWithTwinSse_concatenate_twin_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_ConcatenateWithTwinSse_handle_some_static_stream_sink_single_arg_twin_sse(int64_t port_,
                                                                                    uint8_t *ptr_,
                                                                                    int32_t rust_vec_len_,
                                                                                    int32_t data_len_);

void wire_ConcatenateWithTwinSse_handle_some_static_stream_sink_twin_sse(int64_t port_,
                                                                         uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

void wire_ConcatenateWithTwinSse_handle_some_stream_sink_at_1_twin_sse(int64_t port_,
                                                                       uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

void wire_ConcatenateWithTwinSse_handle_some_stream_sink_twin_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_ConcatenateWithTwinSse_new_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_SumWithTwinSse_sum_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_get_sum_array_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_get_sum_struct_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

WireSyncRust2DartDco wire_ConcatenateWithTwinSync_concatenate_static_twin_sync(struct wire_cst_list_prim_u_8 *a,
                                                                               struct wire_cst_list_prim_u_8 *b);

WireSyncRust2DartDco wire_ConcatenateWithTwinSync_concatenate_twin_sync(struct wire_cst_concatenate_with_twin_sync *that,
                                                                        struct wire_cst_list_prim_u_8 *b);

void wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync(int64_t port_);

void wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync(int64_t port_,
                                                                           uint32_t key,
                                                                           uint32_t max);

void wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync(int64_t port_,
                                                                         struct wire_cst_concatenate_with_twin_sync *that);

void wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync(int64_t port_,
                                                                    struct wire_cst_concatenate_with_twin_sync *that,
                                                                    uint32_t key,
                                                                    uint32_t max);

WireSyncRust2DartDco wire_ConcatenateWithTwinSync_new_twin_sync(struct wire_cst_list_prim_u_8 *a);

WireSyncRust2DartDco wire_SumWithTwinSync_sum_twin_sync(struct wire_cst_sum_with_twin_sync *that,
                                                        uint32_t y,
                                                        uint32_t z);

WireSyncRust2DartDco wire_get_sum_array_twin_sync(uint32_t a, uint32_t b, uint32_t c);

WireSyncRust2DartDco wire_get_sum_struct_twin_sync(void);

WireSyncRust2DartSse wire_ConcatenateWithTwinSyncSse_concatenate_static_twin_sync_sse(uint8_t *ptr_,
                                                                                      int32_t rust_vec_len_,
                                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_ConcatenateWithTwinSyncSse_concatenate_twin_sync_sse(uint8_t *ptr_,
                                                                               int32_t rust_vec_len_,
                                                                               int32_t data_len_);

void wire_ConcatenateWithTwinSyncSse_handle_some_static_stream_sink_single_arg_twin_sync_sse(int64_t port_,
                                                                                             uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

void wire_ConcatenateWithTwinSyncSse_handle_some_static_stream_sink_twin_sync_sse(int64_t port_,
                                                                                  uint8_t *ptr_,
                                                                                  int32_t rust_vec_len_,
                                                                                  int32_t data_len_);

void wire_ConcatenateWithTwinSyncSse_handle_some_stream_sink_at_1_twin_sync_sse(int64_t port_,
                                                                                uint8_t *ptr_,
                                                                                int32_t rust_vec_len_,
                                                                                int32_t data_len_);

void wire_ConcatenateWithTwinSyncSse_handle_some_stream_sink_twin_sync_sse(int64_t port_,
                                                                           uint8_t *ptr_,
                                                                           int32_t rust_vec_len_,
                                                                           int32_t data_len_);

WireSyncRust2DartSse wire_ConcatenateWithTwinSyncSse_new_twin_sync_sse(uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

WireSyncRust2DartSse wire_SumWithTwinSyncSse_sum_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_get_sum_array_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_get_sum_struct_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_app_settings_stream_twin_rust_async(int64_t port_);

void wire_app_settings_vec_stream_twin_rust_async(int64_t port_);

void wire_first_number_twin_rust_async(int64_t port_, struct wire_cst_numbers *nums);

void wire_first_sequence_twin_rust_async(int64_t port_, struct wire_cst_sequences *seqs);

void wire_get_app_settings_twin_rust_async(int64_t port_);

void wire_get_fallible_app_settings_twin_rust_async(int64_t port_);

void wire_get_message_twin_rust_async(int64_t port_);

void wire_is_app_embedded_twin_rust_async(int64_t port_,
                                          struct wire_cst_application_settings *app_settings);

void wire_mirror_struct_stream_twin_rust_async(int64_t port_);

void wire_mirror_tuple_stream_twin_rust_async(int64_t port_);

void wire_repeat_number_twin_rust_async(int64_t port_, int32_t num, uintptr_t times);

void wire_repeat_sequence_twin_rust_async(int64_t port_, int32_t seq, uintptr_t times);

void wire_test_contains_mirrored_sub_struct_twin_rust_async(int64_t port_);

void wire_test_fallible_of_raw_string_mirrored_twin_rust_async(int64_t port_);

void wire_test_list_of_nested_enums_mirrored_twin_rust_async(int64_t port_);

void wire_test_list_of_raw_nested_string_mirrored_twin_rust_async(int64_t port_);

void wire_test_nested_raw_string_mirrored_twin_rust_async(int64_t port_);

void wire_test_raw_string_enum_mirrored_twin_rust_async(int64_t port_, bool nested);

void wire_test_raw_string_mirrored_twin_rust_async(int64_t port_);

void wire_app_settings_stream_twin_rust_async_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_app_settings_vec_stream_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_first_number_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_first_sequence_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_get_app_settings_twin_rust_async_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_get_fallible_app_settings_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_get_message_twin_rust_async_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_is_app_embedded_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_mirror_struct_stream_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_mirror_tuple_stream_twin_rust_async_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_repeat_number_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_repeat_sequence_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_test_contains_mirrored_sub_struct_twin_rust_async_sse(int64_t port_,
                                                                uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

void wire_test_fallible_of_raw_string_mirrored_twin_rust_async_sse(int64_t port_,
                                                                   uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

void wire_test_list_of_nested_enums_mirrored_twin_rust_async_sse(int64_t port_,
                                                                 uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_test_list_of_raw_nested_string_mirrored_twin_rust_async_sse(int64_t port_,
                                                                      uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

void wire_test_nested_raw_string_mirrored_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_test_raw_string_enum_mirrored_twin_rust_async_sse(int64_t port_,
                                                            uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

void wire_test_raw_string_mirrored_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_app_settings_stream_twin_sse(int64_t port_,
                                       uint8_t *ptr_,
                                       int32_t rust_vec_len_,
                                       int32_t data_len_);

void wire_app_settings_vec_stream_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_first_number_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_first_sequence_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

void wire_get_app_settings_twin_sse(int64_t port_,
                                    uint8_t *ptr_,
                                    int32_t rust_vec_len_,
                                    int32_t data_len_);

void wire_get_fallible_app_settings_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_get_message_twin_sse(int64_t port_,
                               uint8_t *ptr_,
                               int32_t rust_vec_len_,
                               int32_t data_len_);

void wire_is_app_embedded_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_mirror_struct_stream_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_mirror_tuple_stream_twin_sse(int64_t port_,
                                       uint8_t *ptr_,
                                       int32_t rust_vec_len_,
                                       int32_t data_len_);

void wire_repeat_number_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_repeat_sequence_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_test_contains_mirrored_sub_struct_twin_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_test_fallible_of_raw_string_mirrored_twin_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_test_list_of_nested_enums_mirrored_twin_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_test_list_of_raw_nested_string_mirrored_twin_sse(int64_t port_,
                                                           uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_test_nested_raw_string_mirrored_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_test_raw_string_enum_mirrored_twin_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_test_raw_string_mirrored_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_app_settings_stream_twin_sync(int64_t port_);

void wire_app_settings_vec_stream_twin_sync(int64_t port_);

WireSyncRust2DartDco wire_first_number_twin_sync(struct wire_cst_numbers *nums);

WireSyncRust2DartDco wire_first_sequence_twin_sync(struct wire_cst_sequences *seqs);

WireSyncRust2DartDco wire_get_app_settings_twin_sync(void);

WireSyncRust2DartDco wire_get_fallible_app_settings_twin_sync(void);

WireSyncRust2DartDco wire_get_message_twin_sync(void);

WireSyncRust2DartDco wire_is_app_embedded_twin_sync(struct wire_cst_application_settings *app_settings);

void wire_mirror_struct_stream_twin_sync(int64_t port_);

void wire_mirror_tuple_stream_twin_sync(int64_t port_);

WireSyncRust2DartDco wire_repeat_number_twin_sync(int32_t num, uintptr_t times);

WireSyncRust2DartDco wire_repeat_sequence_twin_sync(int32_t seq, uintptr_t times);

WireSyncRust2DartDco wire_test_contains_mirrored_sub_struct_twin_sync(void);

WireSyncRust2DartDco wire_test_fallible_of_raw_string_mirrored_twin_sync(void);

WireSyncRust2DartDco wire_test_list_of_nested_enums_mirrored_twin_sync(void);

WireSyncRust2DartDco wire_test_list_of_raw_nested_string_mirrored_twin_sync(void);

WireSyncRust2DartDco wire_test_nested_raw_string_mirrored_twin_sync(void);

WireSyncRust2DartDco wire_test_raw_string_enum_mirrored_twin_sync(bool nested);

WireSyncRust2DartDco wire_test_raw_string_mirrored_twin_sync(void);

void wire_app_settings_stream_twin_sync_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_app_settings_vec_stream_twin_sync_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

WireSyncRust2DartSse wire_first_number_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

WireSyncRust2DartSse wire_first_sequence_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

WireSyncRust2DartSse wire_get_app_settings_twin_sync_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_get_fallible_app_settings_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_get_message_twin_sync_sse(uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

WireSyncRust2DartSse wire_is_app_embedded_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_mirror_struct_stream_twin_sync_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_mirror_tuple_stream_twin_sync_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

WireSyncRust2DartSse wire_repeat_number_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_repeat_sequence_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_test_contains_mirrored_sub_struct_twin_sync_sse(uint8_t *ptr_,
                                                                          int32_t rust_vec_len_,
                                                                          int32_t data_len_);

WireSyncRust2DartSse wire_test_fallible_of_raw_string_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                             int32_t rust_vec_len_,
                                                                             int32_t data_len_);

WireSyncRust2DartSse wire_test_list_of_nested_enums_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                           int32_t rust_vec_len_,
                                                                           int32_t data_len_);

WireSyncRust2DartSse wire_test_list_of_raw_nested_string_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                                int32_t rust_vec_len_,
                                                                                int32_t data_len_);

WireSyncRust2DartSse wire_test_nested_raw_string_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_test_raw_string_enum_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_test_raw_string_mirrored_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_handle_big_buffers_twin_rust_async(int64_t port_);

void wire_handle_complex_struct_twin_rust_async(int64_t port_,
                                                struct wire_cst_my_tree_node_twin_rust_async *s);

void wire_handle_nested_struct_twin_rust_async(int64_t port_,
                                               struct wire_cst_my_nested_struct_twin_rust_async *s);

void wire_handle_string_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *s);

void wire_handle_struct_twin_rust_async(int64_t port_,
                                        struct wire_cst_my_size *arg,
                                        struct wire_cst_my_size *boxed);

void wire_handle_vec_u8_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *v);

void wire_list_of_primitive_enums_twin_rust_async(int64_t port_,
                                                  struct wire_cst_list_weekdays_twin_rust_async *weekdays);

void wire_test_abc_enum_twin_rust_async(int64_t port_, struct wire_cst_abc_twin_rust_async *abc);

void wire_test_struct_with_enum_twin_rust_async(int64_t port_,
                                                struct wire_cst_struct_with_enum_twin_rust_async *se);

void wire_handle_big_buffers_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_handle_complex_struct_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_handle_nested_struct_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_handle_string_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_handle_struct_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_handle_vec_u8_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_list_of_primitive_enums_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_test_abc_enum_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_test_struct_with_enum_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_handle_big_buffers_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_handle_complex_struct_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_handle_nested_struct_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_handle_string_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_handle_struct_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_handle_vec_u8_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_list_of_primitive_enums_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_test_abc_enum_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_test_struct_with_enum_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

WireSyncRust2DartDco wire_handle_big_buffers_twin_sync(void);

WireSyncRust2DartDco wire_handle_complex_struct_twin_sync(struct wire_cst_my_tree_node_twin_sync *s);

WireSyncRust2DartDco wire_handle_nested_struct_twin_sync(struct wire_cst_my_nested_struct_twin_sync *s);

WireSyncRust2DartDco wire_handle_string_twin_sync(struct wire_cst_list_prim_u_8 *s);

WireSyncRust2DartDco wire_handle_struct_twin_sync(struct wire_cst_my_size *arg,
                                                  struct wire_cst_my_size *boxed);

WireSyncRust2DartDco wire_handle_vec_u8_twin_sync(struct wire_cst_list_prim_u_8 *v);

WireSyncRust2DartDco wire_list_of_primitive_enums_twin_sync(struct wire_cst_list_weekdays_twin_sync *weekdays);

WireSyncRust2DartDco wire_test_abc_enum_twin_sync(struct wire_cst_abc_twin_sync *abc);

WireSyncRust2DartDco wire_test_struct_with_enum_twin_sync(struct wire_cst_struct_with_enum_twin_sync *se);

WireSyncRust2DartSse wire_handle_big_buffers_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_handle_complex_struct_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

WireSyncRust2DartSse wire_handle_nested_struct_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_handle_string_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_handle_struct_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_handle_vec_u8_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_list_of_primitive_enums_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_test_abc_enum_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_test_struct_with_enum_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_empty_struct_twin_rust_async(int64_t port_, struct wire_cst_empty_twin_rust_async *empty);

void wire_func_return_unit_twin_rust_async(int64_t port_);

void wire_func_string_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *arg);

void wire_handle_list_of_struct_twin_rust_async(int64_t port_, struct wire_cst_list_my_size *l);

void wire_handle_string_list_twin_rust_async(int64_t port_, struct wire_cst_list_String *names);

void wire_empty_struct_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_func_return_unit_twin_rust_async_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_func_string_twin_rust_async_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_handle_list_of_struct_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_handle_string_list_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_empty_struct_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_func_return_unit_twin_sse(int64_t port_,
                                    uint8_t *ptr_,
                                    int32_t rust_vec_len_,
                                    int32_t data_len_);

void wire_func_string_twin_sse(int64_t port_,
                               uint8_t *ptr_,
                               int32_t rust_vec_len_,
                               int32_t data_len_);

void wire_handle_list_of_struct_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_handle_string_list_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

WireSyncRust2DartDco wire_empty_struct_twin_sync(struct wire_cst_empty_twin_sync *empty);

WireSyncRust2DartDco wire_func_return_unit_twin_sync(void);

WireSyncRust2DartDco wire_func_string_twin_sync(struct wire_cst_list_prim_u_8 *arg);

WireSyncRust2DartDco wire_handle_list_of_struct_twin_sync(struct wire_cst_list_my_size *l);

WireSyncRust2DartDco wire_handle_string_list_twin_sync(struct wire_cst_list_String *names);

WireSyncRust2DartSse wire_empty_struct_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

WireSyncRust2DartSse wire_func_return_unit_twin_sync_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_func_string_twin_sync_sse(uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

WireSyncRust2DartSse wire_handle_list_of_struct_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

WireSyncRust2DartSse wire_handle_string_list_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_handle_newtype_twin_rust_async(int64_t port_,
                                         struct wire_cst_new_type_int_twin_rust_async *arg);

void wire_handle_newtype_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_handle_newtype_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

WireSyncRust2DartDco wire_handle_newtype_twin_sync(struct wire_cst_new_type_int_twin_sync *arg);

WireSyncRust2DartSse wire_handle_newtype_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_bool_twin_normal(int64_t port_, bool *arg);

void wire_example_optional_primitive_type_f32_twin_normal(int64_t port_, float *arg);

void wire_example_optional_primitive_type_f64_twin_normal(int64_t port_, double *arg);

void wire_example_optional_primitive_type_i16_twin_normal(int64_t port_, int16_t *arg);

void wire_example_optional_primitive_type_i32_twin_normal(int64_t port_, int32_t *arg);

void wire_example_optional_primitive_type_i64_twin_normal(int64_t port_, int64_t *arg);

void wire_example_optional_primitive_type_i8_twin_normal(int64_t port_, int8_t *arg);

void wire_example_optional_primitive_type_u16_twin_normal(int64_t port_, uint16_t *arg);

void wire_example_optional_primitive_type_u32_twin_normal(int64_t port_, uint32_t *arg);

void wire_example_optional_primitive_type_u64_twin_normal(int64_t port_, uint64_t *arg);

void wire_example_optional_primitive_type_u8_twin_normal(int64_t port_, uint8_t *arg);

void wire_primitive_optional_types_twin_rust_async(int64_t port_,
                                                   int32_t *my_i32,
                                                   int64_t *my_i64,
                                                   double *my_f64,
                                                   bool *my_bool);

void wire_primitive_optional_types_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_primitive_optional_types_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

WireSyncRust2DartDco wire_primitive_optional_types_twin_sync(int32_t *my_i32,
                                                             int64_t *my_i64,
                                                             double *my_f64,
                                                             bool *my_bool);

WireSyncRust2DartSse wire_primitive_optional_types_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_example_optional_primitive_type_bool_twin_rust_async(int64_t port_, bool *arg);

void wire_example_optional_primitive_type_f32_twin_rust_async(int64_t port_, float *arg);

void wire_example_optional_primitive_type_f64_twin_rust_async(int64_t port_, double *arg);

void wire_example_optional_primitive_type_i16_twin_rust_async(int64_t port_, int16_t *arg);

void wire_example_optional_primitive_type_i32_twin_rust_async(int64_t port_, int32_t *arg);

void wire_example_optional_primitive_type_i64_twin_rust_async(int64_t port_, int64_t *arg);

void wire_example_optional_primitive_type_i8_twin_rust_async(int64_t port_, int8_t *arg);

void wire_example_optional_primitive_type_u16_twin_rust_async(int64_t port_, uint16_t *arg);

void wire_example_optional_primitive_type_u32_twin_rust_async(int64_t port_, uint32_t *arg);

void wire_example_optional_primitive_type_u64_twin_rust_async(int64_t port_, uint64_t *arg);

void wire_example_optional_primitive_type_u8_twin_rust_async(int64_t port_, uint8_t *arg);

void wire_example_optional_primitive_type_bool_twin_rust_async_sse(int64_t port_,
                                                                   uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

void wire_example_optional_primitive_type_f32_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_f64_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_i16_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_i32_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_i64_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_i8_twin_rust_async_sse(int64_t port_,
                                                                 uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_example_optional_primitive_type_u16_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_u32_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_u64_twin_rust_async_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_example_optional_primitive_type_u8_twin_rust_async_sse(int64_t port_,
                                                                 uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_example_optional_primitive_type_bool_twin_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_example_optional_primitive_type_f32_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_f64_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_i16_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_i32_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_i64_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_i8_twin_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_example_optional_primitive_type_u16_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_u32_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_u64_twin_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_example_optional_primitive_type_u8_twin_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartDco wire_example_optional_primitive_type_bool_twin_sync(bool *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_f32_twin_sync(float *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_f64_twin_sync(double *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_i16_twin_sync(int16_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_i32_twin_sync(int32_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_i64_twin_sync(int64_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_i8_twin_sync(int8_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_u16_twin_sync(uint16_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_u32_twin_sync(uint32_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_u64_twin_sync(uint64_t *arg);

WireSyncRust2DartDco wire_example_optional_primitive_type_u8_twin_sync(uint8_t *arg);

WireSyncRust2DartSse wire_example_optional_primitive_type_bool_twin_sync_sse(uint8_t *ptr_,
                                                                             int32_t rust_vec_len_,
                                                                             int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_f32_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_f64_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_i16_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_i32_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_i64_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_i8_twin_sync_sse(uint8_t *ptr_,
                                                                           int32_t rust_vec_len_,
                                                                           int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_u16_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_u32_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_u64_twin_sync_sse(uint8_t *ptr_,
                                                                            int32_t rust_vec_len_,
                                                                            int32_t data_len_);

WireSyncRust2DartSse wire_example_optional_primitive_type_u8_twin_sync_sse(uint8_t *ptr_,
                                                                           int32_t rust_vec_len_,
                                                                           int32_t data_len_);

void wire_handle_increment_boxed_optional_twin_rust_async(int64_t port_, double *opt);

void wire_handle_option_box_arguments_twin_rust_async(int64_t port_,
                                                      int8_t *i8box,
                                                      uint8_t *u8box,
                                                      int32_t *i32box,
                                                      int64_t *i64box,
                                                      double *f64box,
                                                      bool *boolbox,
                                                      struct wire_cst_exotic_optionals_twin_rust_async *structbox);

void wire_handle_optional_increment_twin_rust_async(int64_t port_,
                                                    struct wire_cst_exotic_optionals_twin_rust_async *opt);

void wire_handle_optional_return_twin_rust_async(int64_t port_, double left, double right);

void wire_handle_optional_struct_twin_rust_async(int64_t port_,
                                                 struct wire_cst_list_prim_u_8 *document);

void wire_handle_vec_of_opts_twin_rust_async(int64_t port_,
                                             struct wire_cst_opt_vecs_twin_rust_async *opt);

void wire_handle_increment_boxed_optional_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_handle_option_box_arguments_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_handle_optional_increment_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_handle_optional_return_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_handle_optional_struct_twin_rust_async_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_handle_vec_of_opts_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_handle_increment_boxed_optional_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_handle_option_box_arguments_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_handle_optional_increment_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_handle_optional_return_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_handle_optional_struct_twin_sse(int64_t port_,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

void wire_handle_vec_of_opts_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

WireSyncRust2DartDco wire_handle_increment_boxed_optional_twin_sync(double *opt);

WireSyncRust2DartDco wire_handle_option_box_arguments_twin_sync(int8_t *i8box,
                                                                uint8_t *u8box,
                                                                int32_t *i32box,
                                                                int64_t *i64box,
                                                                double *f64box,
                                                                bool *boolbox,
                                                                struct wire_cst_exotic_optionals_twin_sync *structbox);

WireSyncRust2DartDco wire_handle_optional_increment_twin_sync(struct wire_cst_exotic_optionals_twin_sync *opt);

WireSyncRust2DartDco wire_handle_optional_return_twin_sync(double left, double right);

WireSyncRust2DartDco wire_handle_optional_struct_twin_sync(struct wire_cst_list_prim_u_8 *document);

WireSyncRust2DartDco wire_handle_vec_of_opts_twin_sync(struct wire_cst_opt_vecs_twin_sync *opt);

WireSyncRust2DartSse wire_handle_increment_boxed_optional_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_handle_option_box_arguments_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_handle_optional_increment_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_handle_optional_return_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_handle_optional_struct_twin_sync_sse(uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

WireSyncRust2DartSse wire_handle_vec_of_opts_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_example_primitive_type_bool_twin_normal(int64_t port_, bool arg);

void wire_example_primitive_type_f32_twin_normal(int64_t port_, float arg);

void wire_example_primitive_type_f64_twin_normal(int64_t port_, double arg);

void wire_example_primitive_type_i16_twin_normal(int64_t port_, int16_t arg);

void wire_example_primitive_type_i32_twin_normal(int64_t port_, int32_t arg);

void wire_example_primitive_type_i64_twin_normal(int64_t port_, int64_t arg);

void wire_example_primitive_type_i8_twin_normal(int64_t port_, int8_t arg);

void wire_example_primitive_type_u16_twin_normal(int64_t port_, uint16_t arg);

void wire_example_primitive_type_u32_twin_normal(int64_t port_, uint32_t arg);

void wire_example_primitive_type_u64_twin_normal(int64_t port_, uint64_t arg);

void wire_example_primitive_type_u8_twin_normal(int64_t port_, uint8_t arg);

void wire_example_primitive_list_type_bool_twin_normal(int64_t port_,
                                                       struct wire_cst_list_bool *arg);

void wire_example_primitive_list_type_f32_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_f_32 *arg);

void wire_example_primitive_list_type_f64_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_f_64 *arg);

void wire_example_primitive_list_type_i16_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_i_16 *arg);

void wire_example_primitive_list_type_i32_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_i_32 *arg);

void wire_example_primitive_list_type_i64_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_i_64 *arg);

void wire_example_primitive_list_type_i8_twin_normal(int64_t port_,
                                                     struct wire_cst_list_prim_i_8 *arg);

void wire_example_primitive_list_type_u16_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_u_16 *arg);

void wire_example_primitive_list_type_u32_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_u_32 *arg);

void wire_example_primitive_list_type_u64_twin_normal(int64_t port_,
                                                      struct wire_cst_list_prim_u_64 *arg);

void wire_example_primitive_list_type_u8_twin_normal(int64_t port_,
                                                     struct wire_cst_list_prim_u_8 *arg);

void wire_handle_vec_of_primitive_twin_rust_async(int64_t port_, int32_t n);

void wire_handle_vec_of_primitive_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_handle_vec_of_primitive_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

WireSyncRust2DartDco wire_handle_vec_of_primitive_twin_sync(int32_t n);

WireSyncRust2DartSse wire_handle_vec_of_primitive_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

void wire_example_primitive_list_type_bool_twin_rust_async(int64_t port_,
                                                           struct wire_cst_list_bool *arg);

void wire_example_primitive_list_type_f32_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_f_32 *arg);

void wire_example_primitive_list_type_f64_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_f_64 *arg);

void wire_example_primitive_list_type_i16_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_i_16 *arg);

void wire_example_primitive_list_type_i32_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_i_32 *arg);

void wire_example_primitive_list_type_i64_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_i_64 *arg);

void wire_example_primitive_list_type_i8_twin_rust_async(int64_t port_,
                                                         struct wire_cst_list_prim_i_8 *arg);

void wire_example_primitive_list_type_u16_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_u_16 *arg);

void wire_example_primitive_list_type_u32_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_u_32 *arg);

void wire_example_primitive_list_type_u64_twin_rust_async(int64_t port_,
                                                          struct wire_cst_list_prim_u_64 *arg);

void wire_example_primitive_list_type_u8_twin_rust_async(int64_t port_,
                                                         struct wire_cst_list_prim_u_8 *arg);

void wire_example_primitive_list_type_bool_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_example_primitive_list_type_f32_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_f64_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_i16_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_i32_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_i64_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_i8_twin_rust_async_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_example_primitive_list_type_u16_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_u32_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_u64_twin_rust_async_sse(int64_t port_,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void wire_example_primitive_list_type_u8_twin_rust_async_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_example_primitive_list_type_bool_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_example_primitive_list_type_f32_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_f64_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_i16_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_i32_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_i64_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_i8_twin_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_example_primitive_list_type_u16_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_u32_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_u64_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_example_primitive_list_type_u8_twin_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

WireSyncRust2DartDco wire_example_primitive_list_type_bool_twin_sync(struct wire_cst_list_bool *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_f32_twin_sync(struct wire_cst_list_prim_f_32 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_f64_twin_sync(struct wire_cst_list_prim_f_64 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_i16_twin_sync(struct wire_cst_list_prim_i_16 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_i32_twin_sync(struct wire_cst_list_prim_i_32 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_i64_twin_sync(struct wire_cst_list_prim_i_64 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_i8_twin_sync(struct wire_cst_list_prim_i_8 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_u16_twin_sync(struct wire_cst_list_prim_u_16 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_u32_twin_sync(struct wire_cst_list_prim_u_32 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_u64_twin_sync(struct wire_cst_list_prim_u_64 *arg);

WireSyncRust2DartDco wire_example_primitive_list_type_u8_twin_sync(struct wire_cst_list_prim_u_8 *arg);

WireSyncRust2DartSse wire_example_primitive_list_type_bool_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_f32_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_f64_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_i16_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_i32_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_i64_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_i8_twin_sync_sse(uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_u16_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_u32_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_u64_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_list_type_u8_twin_sync_sse(uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

void wire_primitive_types_twin_rust_async(int64_t port_,
                                          int32_t my_i32,
                                          int64_t my_i64,
                                          double my_f64,
                                          bool my_bool);

void wire_primitive_u32_twin_rust_async(int64_t port_, uint32_t my_u32);

void wire_primitive_types_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_primitive_u32_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_primitive_types_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_primitive_u32_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

WireSyncRust2DartDco wire_primitive_types_twin_sync(int32_t my_i32,
                                                    int64_t my_i64,
                                                    double my_f64,
                                                    bool my_bool);

WireSyncRust2DartDco wire_primitive_u32_twin_sync(uint32_t my_u32);

WireSyncRust2DartSse wire_primitive_types_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_primitive_u32_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_example_primitive_type_bool_twin_rust_async(int64_t port_, bool arg);

void wire_example_primitive_type_f32_twin_rust_async(int64_t port_, float arg);

void wire_example_primitive_type_f64_twin_rust_async(int64_t port_, double arg);

void wire_example_primitive_type_i16_twin_rust_async(int64_t port_, int16_t arg);

void wire_example_primitive_type_i32_twin_rust_async(int64_t port_, int32_t arg);

void wire_example_primitive_type_i64_twin_rust_async(int64_t port_, int64_t arg);

void wire_example_primitive_type_i8_twin_rust_async(int64_t port_, int8_t arg);

void wire_example_primitive_type_u16_twin_rust_async(int64_t port_, uint16_t arg);

void wire_example_primitive_type_u32_twin_rust_async(int64_t port_, uint32_t arg);

void wire_example_primitive_type_u64_twin_rust_async(int64_t port_, uint64_t arg);

void wire_example_primitive_type_u8_twin_rust_async(int64_t port_, uint8_t arg);

void wire_example_primitive_type_bool_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_example_primitive_type_f32_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_f64_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_i16_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_i32_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_i64_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_i8_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_example_primitive_type_u16_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_u32_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_u64_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_example_primitive_type_u8_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_example_primitive_type_bool_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_example_primitive_type_f32_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_f64_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_i16_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_i32_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_i64_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_i8_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_example_primitive_type_u16_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_u32_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_u64_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_example_primitive_type_u8_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

WireSyncRust2DartDco wire_example_primitive_type_bool_twin_sync(bool arg);

WireSyncRust2DartDco wire_example_primitive_type_f32_twin_sync(float arg);

WireSyncRust2DartDco wire_example_primitive_type_f64_twin_sync(double arg);

WireSyncRust2DartDco wire_example_primitive_type_i16_twin_sync(int16_t arg);

WireSyncRust2DartDco wire_example_primitive_type_i32_twin_sync(int32_t arg);

WireSyncRust2DartDco wire_example_primitive_type_i64_twin_sync(int64_t arg);

WireSyncRust2DartDco wire_example_primitive_type_i8_twin_sync(int8_t arg);

WireSyncRust2DartDco wire_example_primitive_type_u16_twin_sync(uint16_t arg);

WireSyncRust2DartDco wire_example_primitive_type_u32_twin_sync(uint32_t arg);

WireSyncRust2DartDco wire_example_primitive_type_u64_twin_sync(uint64_t arg);

WireSyncRust2DartDco wire_example_primitive_type_u8_twin_sync(uint8_t arg);

WireSyncRust2DartSse wire_example_primitive_type_bool_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_f32_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_f64_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_i16_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_i32_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_i64_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_i8_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_u16_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_u32_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_u64_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_example_primitive_type_u8_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_test_more_than_just_one_raw_string_struct_twin_rust_async(int64_t port_);

void wire_test_raw_string_item_struct_twin_rust_async(int64_t port_);

void wire_test_more_than_just_one_raw_string_struct_twin_rust_async_sse(int64_t port_,
                                                                        uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

void wire_test_raw_string_item_struct_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_test_more_than_just_one_raw_string_struct_twin_sse(int64_t port_,
                                                             uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

void wire_test_raw_string_item_struct_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

WireSyncRust2DartDco wire_test_more_than_just_one_raw_string_struct_twin_sync(void);

WireSyncRust2DartDco wire_test_raw_string_item_struct_twin_sync(void);

WireSyncRust2DartSse wire_test_more_than_just_one_raw_string_struct_twin_sync_sse(uint8_t *ptr_,
                                                                                  int32_t rust_vec_len_,
                                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_test_raw_string_item_struct_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_NonCloneSimpleTwinSse_instance_method_arg_borrow_twin_sse(int64_t port_,
                                                                    uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_NonCloneSimpleTwinSse_instance_method_arg_mut_borrow_twin_sse(int64_t port_,
                                                                        uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

void wire_NonCloneSimpleTwinSse_instance_method_arg_own_twin_sse(int64_t port_,
                                                                 uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

void wire_NonCloneSimpleTwinSse_instance_method_return_own_twin_sse(int64_t port_,
                                                                    uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_NonCloneSimpleTwinSse_new_custom_name_twin_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_NonCloneSimpleTwinSse_new_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_NonCloneSimpleTwinSse_static_method_arg_borrow_twin_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_NonCloneSimpleTwinSse_static_method_arg_mut_borrow_twin_sse(int64_t port_,
                                                                      uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

void wire_NonCloneSimpleTwinSse_static_method_arg_own_twin_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_NonCloneSimpleTwinSse_static_method_return_own_twin_sse(int64_t port_,
                                                                  uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_rust_auto_opaque_arg_borrow_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_rust_auto_opaque_arg_mut_borrow_twin_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_rust_auto_opaque_arg_own_and_return_own_twin_sse(int64_t port_,
                                                           uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_rust_auto_opaque_arg_own_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_rust_auto_opaque_callable_arg_twin_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_rust_auto_opaque_callable_return_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_rust_auto_opaque_normal_and_opaque_arg_twin_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_rust_auto_opaque_plus_sign_arg_twin_sse(int64_t port_,
                                                  uint8_t *ptr_,
                                                  int32_t rust_vec_len_,
                                                  int32_t data_len_);

void wire_rust_auto_opaque_plus_sign_return_twin_sse(int64_t port_,
                                                     uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_rust_auto_opaque_return_own_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sse(int64_t port_,
                                                                                 uint8_t *ptr_,
                                                                                 int32_t rust_vec_len_,
                                                                                 int32_t data_len_);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sse(int64_t port_,
                                                                                     uint8_t *ptr_,
                                                                                     int32_t rust_vec_len_,
                                                                                     int32_t data_len_);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sse(int64_t port_,
                                                                              uint8_t *ptr_,
                                                                              int32_t rust_vec_len_,
                                                                              int32_t data_len_);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sse(int64_t port_,
                                                                                 uint8_t *ptr_,
                                                                                 int32_t rust_vec_len_,
                                                                                 int32_t data_len_);

void wire_rust_auto_opaque_trait_object_arg_borrow_twin_sse(int64_t port_,
                                                            uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

void wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sse(int64_t port_,
                                                                uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

void wire_rust_auto_opaque_trait_object_arg_own_twin_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_rust_auto_opaque_trait_object_return_own_one_twin_sse(int64_t port_,
                                                                uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

void wire_rust_auto_opaque_trait_object_return_own_two_twin_sse(int64_t port_,
                                                                uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

void wire_rust_auto_opaque_two_args_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_instance_method_arg_borrow_twin_sync(const void *that);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_instance_method_arg_mut_borrow_twin_sync(const void *that);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_instance_method_arg_own_twin_sync(const void *that);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_instance_method_return_own_twin_sync(const void *that);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_new_custom_name_twin_sync(void);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_new_twin_sync(void);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_static_method_arg_borrow_twin_sync(const void *arg);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_static_method_arg_mut_borrow_twin_sync(const void *arg);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_static_method_arg_own_twin_sync(const void *arg);

WireSyncRust2DartDco wire_NonCloneSimpleTwinSync_static_method_return_own_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_arg_borrow_twin_sync(const void *arg, int32_t expect);

WireSyncRust2DartDco wire_rust_auto_opaque_arg_mut_borrow_twin_sync(const void *arg,
                                                                    int32_t expect,
                                                                    int32_t adder);

WireSyncRust2DartDco wire_rust_auto_opaque_arg_own_and_return_own_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_arg_own_twin_sync(const void *arg, int32_t expect);

WireSyncRust2DartDco wire_rust_auto_opaque_callable_arg_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_callable_return_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync(const void *a,
                                                                           struct wire_cst_list_prim_u_8 *b);

WireSyncRust2DartDco wire_rust_auto_opaque_plus_sign_arg_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_plus_sign_return_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_return_own_twin_sync(int32_t initial);

WireSyncRust2DartDco wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync(const void *arg);

WireSyncRust2DartDco wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync(const void *arg,
                                                                             struct wire_cst_list_prim_u_8 *expect);

WireSyncRust2DartDco wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync(const void *arg,
                                                                                 struct wire_cst_list_prim_u_8 *expect);

WireSyncRust2DartDco wire_rust_auto_opaque_trait_object_arg_own_twin_sync(const void *arg,
                                                                          struct wire_cst_list_prim_u_8 *expect);

WireSyncRust2DartDco wire_rust_auto_opaque_trait_object_return_own_one_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_trait_object_return_own_two_twin_sync(void);

WireSyncRust2DartDco wire_rust_auto_opaque_two_args_twin_sync(const void *a, const void *b);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_instance_method_arg_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_instance_method_arg_mut_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                                 int32_t rust_vec_len_,
                                                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_instance_method_arg_own_twin_sync_sse(uint8_t *ptr_,
                                                                                          int32_t rust_vec_len_,
                                                                                          int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_instance_method_return_own_twin_sync_sse(uint8_t *ptr_,
                                                                                             int32_t rust_vec_len_,
                                                                                             int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_new_custom_name_twin_sync_sse(uint8_t *ptr_,
                                                                                  int32_t rust_vec_len_,
                                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_new_twin_sync_sse(uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_static_method_arg_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                           int32_t rust_vec_len_,
                                                                                           int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_static_method_arg_mut_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                               int32_t rust_vec_len_,
                                                                                               int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_static_method_arg_own_twin_sync_sse(uint8_t *ptr_,
                                                                                        int32_t rust_vec_len_,
                                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_NonCloneSimpleTwinSyncSse_static_method_return_own_twin_sync_sse(uint8_t *ptr_,
                                                                                           int32_t rust_vec_len_,
                                                                                           int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_arg_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_arg_mut_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                        int32_t rust_vec_len_,
                                                                        int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_arg_own_and_return_own_twin_sync_sse(uint8_t *ptr_,
                                                                                int32_t rust_vec_len_,
                                                                                int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_arg_own_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_callable_arg_twin_sync_sse(uint8_t *ptr_,
                                                                      int32_t rust_vec_len_,
                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_callable_return_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync_sse(uint8_t *ptr_,
                                                                               int32_t rust_vec_len_,
                                                                               int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_plus_sign_arg_twin_sync_sse(uint8_t *ptr_,
                                                                       int32_t rust_vec_len_,
                                                                       int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_plus_sign_return_twin_sync_sse(uint8_t *ptr_,
                                                                          int32_t rust_vec_len_,
                                                                          int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_return_own_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                                      int32_t rust_vec_len_,
                                                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                                          int32_t rust_vec_len_,
                                                                                                          int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync_sse(uint8_t *ptr_,
                                                                                                   int32_t rust_vec_len_,
                                                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync_sse(uint8_t *ptr_,
                                                                                                      int32_t rust_vec_len_,
                                                                                                      int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                 int32_t rust_vec_len_,
                                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync_sse(uint8_t *ptr_,
                                                                                     int32_t rust_vec_len_,
                                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_trait_object_arg_own_twin_sync_sse(uint8_t *ptr_,
                                                                              int32_t rust_vec_len_,
                                                                              int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_trait_object_return_own_one_twin_sync_sse(uint8_t *ptr_,
                                                                                     int32_t rust_vec_len_,
                                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_trait_object_return_own_two_twin_sync_sse(uint8_t *ptr_,
                                                                                     int32_t rust_vec_len_,
                                                                                     int32_t data_len_);

WireSyncRust2DartSse wire_rust_auto_opaque_two_args_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

WireSyncRust2DartSse wire_frb_sync_generator_test_twin_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_sync_create_non_clone_twin_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_sync_create_opaque_twin_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_sync_create_sync_opaque_twin_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_sync_option_rust_opaque_twin_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_sync_run_opaque_twin_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_create_array_opaque_enum_twin_rust_async(int64_t port_);

void wire_create_nested_opaque_twin_rust_async(int64_t port_);

void wire_create_opaque_twin_rust_async(int64_t port_);

void wire_create_option_opaque_twin_rust_async(int64_t port_, const void **opaque);

void wire_create_sync_opaque_twin_rust_async(int64_t port_);

void wire_frb_generator_test_twin_rust_async(int64_t port_);

void wire_opaque_array_run_twin_rust_async(int64_t port_,
                                           struct wire_cst_list_RustOpaque_hide_data *data);

void wire_opaque_array_twin_rust_async(int64_t port_);

void wire_opaque_vec_run_twin_rust_async(int64_t port_,
                                         struct wire_cst_list_RustOpaque_hide_data *data);

void wire_opaque_vec_twin_rust_async(int64_t port_);

void wire_run_enum_opaque_twin_rust_async(int64_t port_,
                                          struct wire_cst_enum_opaque_twin_rust_async *opaque);

void wire_run_nested_opaque_twin_rust_async(int64_t port_,
                                            struct wire_cst_opaque_nested_twin_rust_async *opaque);

void wire_run_non_clone_twin_rust_async(int64_t port_, const void *clone);

void wire_run_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_run_opaque_with_delay_twin_rust_async(int64_t port_, const void *opaque);

void wire_unwrap_rust_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_create_array_opaque_enum_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_create_nested_opaque_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_create_opaque_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_create_option_opaque_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_create_sync_opaque_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_frb_generator_test_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_opaque_array_run_twin_rust_async_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_opaque_array_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_opaque_vec_run_twin_rust_async_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

void wire_opaque_vec_twin_rust_async_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_run_enum_opaque_twin_rust_async_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_run_nested_opaque_twin_rust_async_sse(int64_t port_,
                                                uint8_t *ptr_,
                                                int32_t rust_vec_len_,
                                                int32_t data_len_);

void wire_run_non_clone_twin_rust_async_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_run_opaque_twin_rust_async_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_run_opaque_with_delay_twin_rust_async_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_unwrap_rust_opaque_twin_rust_async_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_create_array_opaque_enum_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_create_nested_opaque_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_create_opaque_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_create_option_opaque_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_create_sync_opaque_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_frb_generator_test_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

void wire_opaque_array_run_twin_sse(int64_t port_,
                                    uint8_t *ptr_,
                                    int32_t rust_vec_len_,
                                    int32_t data_len_);

void wire_opaque_array_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_opaque_vec_run_twin_sse(int64_t port_,
                                  uint8_t *ptr_,
                                  int32_t rust_vec_len_,
                                  int32_t data_len_);

void wire_opaque_vec_twin_sse(int64_t port_,
                              uint8_t *ptr_,
                              int32_t rust_vec_len_,
                              int32_t data_len_);

void wire_run_enum_opaque_twin_sse(int64_t port_,
                                   uint8_t *ptr_,
                                   int32_t rust_vec_len_,
                                   int32_t data_len_);

void wire_run_nested_opaque_twin_sse(int64_t port_,
                                     uint8_t *ptr_,
                                     int32_t rust_vec_len_,
                                     int32_t data_len_);

void wire_run_non_clone_twin_sse(int64_t port_,
                                 uint8_t *ptr_,
                                 int32_t rust_vec_len_,
                                 int32_t data_len_);

void wire_run_opaque_twin_sse(int64_t port_,
                              uint8_t *ptr_,
                              int32_t rust_vec_len_,
                              int32_t data_len_);

void wire_run_opaque_with_delay_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_unwrap_rust_opaque_twin_sse(int64_t port_,
                                      uint8_t *ptr_,
                                      int32_t rust_vec_len_,
                                      int32_t data_len_);

WireSyncRust2DartDco wire_create_array_opaque_enum_twin_sync(void);

WireSyncRust2DartDco wire_create_nested_opaque_twin_sync(void);

WireSyncRust2DartDco wire_create_opaque_twin_sync(void);

WireSyncRust2DartDco wire_create_option_opaque_twin_sync(const void **opaque);

WireSyncRust2DartDco wire_create_sync_opaque_twin_sync(void);

WireSyncRust2DartDco wire_frb_generator_test_twin_sync(void);

WireSyncRust2DartDco wire_opaque_array_run_twin_sync(struct wire_cst_list_RustOpaque_hide_data *data);

WireSyncRust2DartDco wire_opaque_array_twin_sync(void);

WireSyncRust2DartDco wire_opaque_vec_run_twin_sync(struct wire_cst_list_RustOpaque_hide_data *data);

WireSyncRust2DartDco wire_opaque_vec_twin_sync(void);

WireSyncRust2DartDco wire_run_enum_opaque_twin_sync(struct wire_cst_enum_opaque_twin_sync *opaque);

WireSyncRust2DartDco wire_run_nested_opaque_twin_sync(struct wire_cst_opaque_nested_twin_sync *opaque);

WireSyncRust2DartDco wire_run_non_clone_twin_sync(const void *clone);

WireSyncRust2DartDco wire_run_opaque_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_run_opaque_with_delay_twin_sync(const void *opaque);

WireSyncRust2DartDco wire_unwrap_rust_opaque_twin_sync(const void *opaque);

WireSyncRust2DartSse wire_create_array_opaque_enum_twin_sync_sse(uint8_t *ptr_,
                                                                 int32_t rust_vec_len_,
                                                                 int32_t data_len_);

WireSyncRust2DartSse wire_create_nested_opaque_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_create_opaque_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_create_option_opaque_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_create_sync_opaque_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_frb_generator_test_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

WireSyncRust2DartSse wire_opaque_array_run_twin_sync_sse(uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

WireSyncRust2DartSse wire_opaque_array_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

WireSyncRust2DartSse wire_opaque_vec_run_twin_sync_sse(uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

WireSyncRust2DartSse wire_opaque_vec_twin_sync_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

WireSyncRust2DartSse wire_run_enum_opaque_twin_sync_sse(uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

WireSyncRust2DartSse wire_run_nested_opaque_twin_sync_sse(uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

WireSyncRust2DartSse wire_run_non_clone_twin_sync_sse(uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

WireSyncRust2DartSse wire_run_opaque_twin_sync_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

WireSyncRust2DartSse wire_run_opaque_with_delay_twin_sync_sse(uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

WireSyncRust2DartSse wire_unwrap_rust_opaque_twin_sync_sse(uint8_t *ptr_,
                                                           int32_t rust_vec_len_,
                                                           int32_t data_len_);

void wire_simple_adder_twin_rust_async(int64_t port_, int32_t a, int32_t b);

void wire_simple_adder_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_simple_adder_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

WireSyncRust2DartDco wire_simple_adder_twin_sync(int32_t a, int32_t b);

WireSyncRust2DartSse wire_simple_adder_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

void wire_func_stream_realistic_twin_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_func_stream_return_error_twin_rust_async(int64_t port_);

void wire_func_stream_return_panic_twin_rust_async(int64_t port_);

void wire_func_stream_sink_arg_position_twin_rust_async(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct_twin_rust_async(int64_t port_);

void wire_handle_stream_sink_at_1_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_func_stream_return_error_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_func_stream_return_panic_twin_rust_async_sse(int64_t port_,
                                                       uint8_t *ptr_,
                                                       int32_t rust_vec_len_,
                                                       int32_t data_len_);

void wire_func_stream_sink_arg_position_twin_rust_async_sse(int64_t port_,
                                                            uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

void wire_handle_stream_of_struct_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_handle_stream_sink_at_1_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_handle_stream_sink_at_2_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_handle_stream_sink_at_3_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_func_stream_return_error_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_func_stream_return_panic_twin_sse(int64_t port_,
                                            uint8_t *ptr_,
                                            int32_t rust_vec_len_,
                                            int32_t data_len_);

void wire_func_stream_sink_arg_position_twin_sse(int64_t port_,
                                                 uint8_t *ptr_,
                                                 int32_t rust_vec_len_,
                                                 int32_t data_len_);

void wire_handle_stream_of_struct_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_handle_stream_sink_at_1_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_handle_stream_sink_at_2_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_handle_stream_sink_at_3_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_func_struct_with_one_field_twin_rust_async(int64_t port_,
                                                     struct wire_cst_struct_with_one_field_twin_rust_async *arg);

void wire_func_struct_with_two_field_twin_rust_async(int64_t port_,
                                                     struct wire_cst_struct_with_two_field_twin_rust_async *arg);

void wire_func_struct_with_zero_field_twin_rust_async(int64_t port_,
                                                      struct wire_cst_struct_with_zero_field_twin_rust_async *arg);

void wire_func_tuple_struct_with_one_field_twin_rust_async(int64_t port_,
                                                           struct wire_cst_tuple_struct_with_one_field_twin_rust_async *arg);

void wire_func_tuple_struct_with_two_field_twin_rust_async(int64_t port_,
                                                           struct wire_cst_tuple_struct_with_two_field_twin_rust_async *arg);

void wire_func_struct_with_one_field_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_func_struct_with_two_field_twin_rust_async_sse(int64_t port_,
                                                         uint8_t *ptr_,
                                                         int32_t rust_vec_len_,
                                                         int32_t data_len_);

void wire_func_struct_with_zero_field_twin_rust_async_sse(int64_t port_,
                                                          uint8_t *ptr_,
                                                          int32_t rust_vec_len_,
                                                          int32_t data_len_);

void wire_func_tuple_struct_with_one_field_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_func_tuple_struct_with_two_field_twin_rust_async_sse(int64_t port_,
                                                               uint8_t *ptr_,
                                                               int32_t rust_vec_len_,
                                                               int32_t data_len_);

void wire_func_struct_with_one_field_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_func_struct_with_two_field_twin_sse(int64_t port_,
                                              uint8_t *ptr_,
                                              int32_t rust_vec_len_,
                                              int32_t data_len_);

void wire_func_struct_with_zero_field_twin_sse(int64_t port_,
                                               uint8_t *ptr_,
                                               int32_t rust_vec_len_,
                                               int32_t data_len_);

void wire_func_tuple_struct_with_one_field_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

void wire_func_tuple_struct_with_two_field_twin_sse(int64_t port_,
                                                    uint8_t *ptr_,
                                                    int32_t rust_vec_len_,
                                                    int32_t data_len_);

WireSyncRust2DartDco wire_func_struct_with_one_field_twin_sync(struct wire_cst_struct_with_one_field_twin_sync *arg);

WireSyncRust2DartDco wire_func_struct_with_two_field_twin_sync(struct wire_cst_struct_with_two_field_twin_sync *arg);

WireSyncRust2DartDco wire_func_struct_with_zero_field_twin_sync(struct wire_cst_struct_with_zero_field_twin_sync *arg);

WireSyncRust2DartDco wire_func_tuple_struct_with_one_field_twin_sync(struct wire_cst_tuple_struct_with_one_field_twin_sync *arg);

WireSyncRust2DartDco wire_func_tuple_struct_with_two_field_twin_sync(struct wire_cst_tuple_struct_with_two_field_twin_sync *arg);

WireSyncRust2DartSse wire_func_struct_with_one_field_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_func_struct_with_two_field_twin_sync_sse(uint8_t *ptr_,
                                                                   int32_t rust_vec_len_,
                                                                   int32_t data_len_);

WireSyncRust2DartSse wire_func_struct_with_zero_field_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

WireSyncRust2DartSse wire_func_tuple_struct_with_one_field_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

WireSyncRust2DartSse wire_func_tuple_struct_with_two_field_twin_sync_sse(uint8_t *ptr_,
                                                                         int32_t rust_vec_len_,
                                                                         int32_t data_len_);

void wire_test_tuple_2_twin_rust_async(int64_t port_,
                                       struct wire_cst_list_record_string_i_32 *value);

void wire_test_tuple_twin_rust_async(int64_t port_, struct wire_cst_record_string_i_32 *value);

void wire_test_tuple_2_twin_rust_async_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_test_tuple_twin_rust_async_sse(int64_t port_,
                                         uint8_t *ptr_,
                                         int32_t rust_vec_len_,
                                         int32_t data_len_);

void wire_test_tuple_2_twin_sse(int64_t port_,
                                uint8_t *ptr_,
                                int32_t rust_vec_len_,
                                int32_t data_len_);

void wire_test_tuple_twin_sse(int64_t port_,
                              uint8_t *ptr_,
                              int32_t rust_vec_len_,
                              int32_t data_len_);

WireSyncRust2DartDco wire_test_tuple_2_twin_sync(struct wire_cst_list_record_string_i_32 *value);

WireSyncRust2DartDco wire_test_tuple_twin_sync(struct wire_cst_record_string_i_32 *value);

WireSyncRust2DartSse wire_test_tuple_2_twin_sync_sse(uint8_t *ptr_,
                                                     int32_t rust_vec_len_,
                                                     int32_t data_len_);

WireSyncRust2DartSse wire_test_tuple_twin_sync_sse(uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_handle_type_alias_id_twin_rust_async(int64_t port_, uint64_t input);

void wire_handle_type_alias_model_twin_rust_async(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id_twin_rust_async(int64_t port_, uint64_t input);

void wire_handle_type_alias_id_twin_rust_async_sse(int64_t port_,
                                                   uint8_t *ptr_,
                                                   int32_t rust_vec_len_,
                                                   int32_t data_len_);

void wire_handle_type_alias_model_twin_rust_async_sse(int64_t port_,
                                                      uint8_t *ptr_,
                                                      int32_t rust_vec_len_,
                                                      int32_t data_len_);

void wire_handle_type_nest_alias_id_twin_rust_async_sse(int64_t port_,
                                                        uint8_t *ptr_,
                                                        int32_t rust_vec_len_,
                                                        int32_t data_len_);

void wire_handle_type_alias_id_twin_sse(int64_t port_,
                                        uint8_t *ptr_,
                                        int32_t rust_vec_len_,
                                        int32_t data_len_);

void wire_handle_type_alias_model_twin_sse(int64_t port_,
                                           uint8_t *ptr_,
                                           int32_t rust_vec_len_,
                                           int32_t data_len_);

void wire_handle_type_nest_alias_id_twin_sse(int64_t port_,
                                             uint8_t *ptr_,
                                             int32_t rust_vec_len_,
                                             int32_t data_len_);

WireSyncRust2DartDco wire_handle_type_alias_id_twin_sync(uint64_t input);

WireSyncRust2DartDco wire_handle_type_alias_model_twin_sync(uint64_t input);

WireSyncRust2DartDco wire_handle_type_nest_alias_id_twin_sync(uint64_t input);

WireSyncRust2DartSse wire_handle_type_alias_id_twin_sync_sse(uint8_t *ptr_,
                                                             int32_t rust_vec_len_,
                                                             int32_t data_len_);

WireSyncRust2DartSse wire_handle_type_alias_model_twin_sync_sse(uint8_t *ptr_,
                                                                int32_t rust_vec_len_,
                                                                int32_t data_len_);

WireSyncRust2DartSse wire_handle_type_nest_alias_id_twin_sync_sse(uint8_t *ptr_,
                                                                  int32_t rust_vec_len_,
                                                                  int32_t data_len_);

void wire_handle_nested_uuids_twin_rust_async(int64_t port_,
                                              struct wire_cst_feature_uuid_twin_rust_async *ids);

void wire_handle_uuid_twin_rust_async(int64_t port_, struct wire_cst_list_prim_u_8 *id);

WireSyncRust2DartDco wire_handle_nested_uuids_twin_sync(struct wire_cst_feature_uuid_twin_sync *ids);

WireSyncRust2DartDco wire_handle_uuid_twin_sync(struct wire_cst_list_prim_u_8 *id);

void wire_test_more_than_just_one_raw_string_struct_twin_normal(int64_t port_);

void wire_test_raw_string_item_struct_twin_normal(int64_t port_);

void wire_NonCloneSimpleTwinNormal_instance_method_arg_borrow_twin_normal(int64_t port_,
                                                                          const void *that);

void wire_NonCloneSimpleTwinNormal_instance_method_arg_mut_borrow_twin_normal(int64_t port_,
                                                                              const void *that);

void wire_NonCloneSimpleTwinNormal_instance_method_arg_own_twin_normal(int64_t port_,
                                                                       const void *that);

void wire_NonCloneSimpleTwinNormal_instance_method_return_own_twin_normal(int64_t port_,
                                                                          const void *that);

void wire_NonCloneSimpleTwinNormal_new_custom_name_twin_normal(int64_t port_);

void wire_NonCloneSimpleTwinNormal_new_twin_normal(int64_t port_);

void wire_NonCloneSimpleTwinNormal_static_method_arg_borrow_twin_normal(int64_t port_,
                                                                        const void *arg);

void wire_NonCloneSimpleTwinNormal_static_method_arg_mut_borrow_twin_normal(int64_t port_,
                                                                            const void *arg);

void wire_NonCloneSimpleTwinNormal_static_method_arg_own_twin_normal(int64_t port_,
                                                                     const void *arg);

void wire_NonCloneSimpleTwinNormal_static_method_return_own_twin_normal(int64_t port_);

void wire_rust_auto_opaque_arg_borrow_twin_normal(int64_t port_, const void *arg, int32_t expect);

void wire_rust_auto_opaque_arg_mut_borrow_twin_normal(int64_t port_,
                                                      const void *arg,
                                                      int32_t expect,
                                                      int32_t adder);

void wire_rust_auto_opaque_arg_own_and_return_own_twin_normal(int64_t port_, const void *arg);

void wire_rust_auto_opaque_arg_own_twin_normal(int64_t port_, const void *arg, int32_t expect);

void wire_rust_auto_opaque_callable_arg_twin_normal(int64_t port_, const void *arg);

void wire_rust_auto_opaque_callable_return_twin_normal(int64_t port_);

void wire_rust_auto_opaque_normal_and_opaque_arg_twin_normal(int64_t port_,
                                                             const void *a,
                                                             struct wire_cst_list_prim_u_8 *b);

void wire_rust_auto_opaque_plus_sign_arg_twin_normal(int64_t port_, const void *arg);

void wire_rust_auto_opaque_plus_sign_return_twin_normal(int64_t port_);

void wire_rust_auto_opaque_return_own_twin_normal(int64_t port_, int32_t initial);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_normal(int64_t port_,
                                                                                    const void *arg);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_normal(int64_t port_,
                                                                                        const void *arg);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_normal(int64_t port_,
                                                                                 const void *arg);

void wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_normal(int64_t port_);

void wire_rust_auto_opaque_trait_object_arg_borrow_twin_normal(int64_t port_,
                                                               const void *arg,
                                                               struct wire_cst_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_normal(int64_t port_,
                                                                   const void *arg,
                                                                   struct wire_cst_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_arg_own_twin_normal(int64_t port_,
                                                            const void *arg,
                                                            struct wire_cst_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_return_own_one_twin_normal(int64_t port_);

void wire_rust_auto_opaque_trait_object_return_own_two_twin_normal(int64_t port_);

void wire_rust_auto_opaque_two_args_twin_normal(int64_t port_, const void *a, const void *b);

void wire_create_array_opaque_enum_twin_normal(int64_t port_);

void wire_create_nested_opaque_twin_normal(int64_t port_);

void wire_create_opaque_twin_normal(int64_t port_);

void wire_create_option_opaque_twin_normal(int64_t port_, const void **opaque);

void wire_create_sync_opaque_twin_normal(int64_t port_);

void wire_frb_generator_test_twin_normal(int64_t port_);

void wire_opaque_array_run_twin_normal(int64_t port_,
                                       struct wire_cst_list_RustOpaque_hide_data *data);

void wire_opaque_array_twin_normal(int64_t port_);

void wire_opaque_vec_run_twin_normal(int64_t port_,
                                     struct wire_cst_list_RustOpaque_hide_data *data);

void wire_opaque_vec_twin_normal(int64_t port_);

void wire_run_enum_opaque_twin_normal(int64_t port_,
                                      struct wire_cst_enum_opaque_twin_normal *opaque);

void wire_run_nested_opaque_twin_normal(int64_t port_,
                                        struct wire_cst_opaque_nested_twin_normal *opaque);

void wire_run_non_clone_twin_normal(int64_t port_, const void *clone);

void wire_run_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_run_opaque_with_delay_twin_normal(int64_t port_, const void *opaque);

void wire_unwrap_rust_opaque_twin_normal(int64_t port_, const void *opaque);

WireSyncRust2DartDco wire_frb_sync_generator_test_twin_normal(void);

WireSyncRust2DartDco wire_sync_create_non_clone_twin_normal(void);

WireSyncRust2DartDco wire_sync_create_opaque_twin_normal(void);

WireSyncRust2DartDco wire_sync_create_sync_opaque_twin_normal(void);

WireSyncRust2DartDco wire_sync_option_rust_opaque_twin_normal(void);

WireSyncRust2DartDco wire_sync_run_opaque_twin_normal(const void *opaque);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_stream_return_error_twin_normal(int64_t port_);

void wire_func_stream_return_panic_twin_normal(int64_t port_);

void wire_func_stream_sink_arg_position_twin_normal(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct_twin_normal(int64_t port_);

void wire_handle_stream_sink_at_1_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_func_stream_realistic_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *arg);

void wire_func_struct_with_one_field_twin_normal(int64_t port_,
                                                 struct wire_cst_struct_with_one_field_twin_normal *arg);

void wire_func_struct_with_two_field_twin_normal(int64_t port_,
                                                 struct wire_cst_struct_with_two_field_twin_normal *arg);

void wire_func_struct_with_zero_field_twin_normal(int64_t port_,
                                                  struct wire_cst_struct_with_zero_field_twin_normal *arg);

void wire_func_tuple_struct_with_one_field_twin_normal(int64_t port_,
                                                       struct wire_cst_tuple_struct_with_one_field_twin_normal *arg);

void wire_func_tuple_struct_with_two_field_twin_normal(int64_t port_,
                                                       struct wire_cst_tuple_struct_with_two_field_twin_normal *arg);

void wire_test_tuple_2_twin_normal(int64_t port_, struct wire_cst_list_record_string_i_32 *value);

void wire_test_tuple_twin_normal(int64_t port_, struct wire_cst_record_string_i_32 *value);

void wire_handle_type_alias_id_twin_normal(int64_t port_, uint64_t input);

void wire_handle_type_alias_model_twin_normal(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id_twin_normal(int64_t port_, uint64_t input);

void wire_handle_nested_uuids_twin_normal(int64_t port_,
                                          struct wire_cst_feature_uuid_twin_normal *ids);

void wire_handle_uuid_twin_normal(int64_t port_, struct wire_cst_list_prim_u_8 *id);

const void *dart_opaque_dart2rust_encode(Dart_Handle handle);

void rust_arc_increment_strong_count_RustOpaque_MutexHideData(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_MutexHideData(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_RwLockHideData(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_RwLockHideData(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsyncSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsyncSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSyncSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSyncSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_frb_opaque_return(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_frb_opaque_return(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_frb_opaque_sync_return(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_frb_opaque_sync_return(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_hide_data(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_hide_data(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_i_32(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_i_32(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_non_clone_data(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_non_clone_data(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_non_send_hide_data(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_non_send_hide_data(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSyncSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSyncSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSseSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSseSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSseSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSseSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSyncSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSyncSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSse(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSyncSse(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSyncSse(const void *ptr);

struct wire_cst_application_env *cst_new_box_application_env(void);

int64_t *cst_new_box_autoadd_Chrono_Duration(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Naive(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Utc(int64_t value);

const void **cst_new_box_autoadd_DartOpaque(const void *value);

const void **cst_new_box_autoadd_RustOpaque_hide_data(const void *value);

struct wire_cst_a_twin_normal *cst_new_box_autoadd_a_twin_normal(void);

struct wire_cst_a_twin_rust_async *cst_new_box_autoadd_a_twin_rust_async(void);

struct wire_cst_a_twin_rust_async_sse *cst_new_box_autoadd_a_twin_rust_async_sse(void);

struct wire_cst_a_twin_sse *cst_new_box_autoadd_a_twin_sse(void);

struct wire_cst_a_twin_sync *cst_new_box_autoadd_a_twin_sync(void);

struct wire_cst_a_twin_sync_sse *cst_new_box_autoadd_a_twin_sync_sse(void);

struct wire_cst_abc_twin_normal *cst_new_box_autoadd_abc_twin_normal(void);

struct wire_cst_abc_twin_rust_async *cst_new_box_autoadd_abc_twin_rust_async(void);

struct wire_cst_abc_twin_rust_async_sse *cst_new_box_autoadd_abc_twin_rust_async_sse(void);

struct wire_cst_abc_twin_sse *cst_new_box_autoadd_abc_twin_sse(void);

struct wire_cst_abc_twin_sync *cst_new_box_autoadd_abc_twin_sync(void);

struct wire_cst_abc_twin_sync_sse *cst_new_box_autoadd_abc_twin_sync_sse(void);

struct wire_cst_application_env *cst_new_box_autoadd_application_env(void);

struct wire_cst_application_settings *cst_new_box_autoadd_application_settings(void);

struct wire_cst_attribute_twin_normal *cst_new_box_autoadd_attribute_twin_normal(void);

struct wire_cst_attribute_twin_rust_async *cst_new_box_autoadd_attribute_twin_rust_async(void);

struct wire_cst_attribute_twin_rust_async_sse *cst_new_box_autoadd_attribute_twin_rust_async_sse(void);

struct wire_cst_attribute_twin_sse *cst_new_box_autoadd_attribute_twin_sse(void);

struct wire_cst_attribute_twin_sync *cst_new_box_autoadd_attribute_twin_sync(void);

struct wire_cst_attribute_twin_sync_sse *cst_new_box_autoadd_attribute_twin_sync_sse(void);

struct wire_cst_b_twin_normal *cst_new_box_autoadd_b_twin_normal(void);

struct wire_cst_b_twin_rust_async *cst_new_box_autoadd_b_twin_rust_async(void);

struct wire_cst_b_twin_rust_async_sse *cst_new_box_autoadd_b_twin_rust_async_sse(void);

struct wire_cst_b_twin_sse *cst_new_box_autoadd_b_twin_sse(void);

struct wire_cst_b_twin_sync *cst_new_box_autoadd_b_twin_sync(void);

struct wire_cst_b_twin_sync_sse *cst_new_box_autoadd_b_twin_sync_sse(void);

bool *cst_new_box_autoadd_bool(bool value);

struct wire_cst_c_twin_normal *cst_new_box_autoadd_c_twin_normal(void);

struct wire_cst_c_twin_rust_async *cst_new_box_autoadd_c_twin_rust_async(void);

struct wire_cst_c_twin_rust_async_sse *cst_new_box_autoadd_c_twin_rust_async_sse(void);

struct wire_cst_c_twin_sse *cst_new_box_autoadd_c_twin_sse(void);

struct wire_cst_c_twin_sync *cst_new_box_autoadd_c_twin_sync(void);

struct wire_cst_c_twin_sync_sse *cst_new_box_autoadd_c_twin_sync_sse(void);

struct wire_cst_concatenate_with_twin_normal *cst_new_box_autoadd_concatenate_with_twin_normal(void);

struct wire_cst_concatenate_with_twin_rust_async *cst_new_box_autoadd_concatenate_with_twin_rust_async(void);

struct wire_cst_concatenate_with_twin_rust_async_sse *cst_new_box_autoadd_concatenate_with_twin_rust_async_sse(void);

struct wire_cst_concatenate_with_twin_sse *cst_new_box_autoadd_concatenate_with_twin_sse(void);

struct wire_cst_concatenate_with_twin_sync *cst_new_box_autoadd_concatenate_with_twin_sync(void);

struct wire_cst_concatenate_with_twin_sync_sse *cst_new_box_autoadd_concatenate_with_twin_sync_sse(void);

struct wire_cst_custom_nested_error_2_twin_normal *cst_new_box_autoadd_custom_nested_error_2_twin_normal(void);

struct wire_cst_custom_nested_error_2_twin_rust_async *cst_new_box_autoadd_custom_nested_error_2_twin_rust_async(void);

struct wire_cst_custom_nested_error_2_twin_rust_async_sse *cst_new_box_autoadd_custom_nested_error_2_twin_rust_async_sse(void);

struct wire_cst_custom_nested_error_2_twin_sse *cst_new_box_autoadd_custom_nested_error_2_twin_sse(void);

struct wire_cst_custom_nested_error_2_twin_sync *cst_new_box_autoadd_custom_nested_error_2_twin_sync(void);

struct wire_cst_custom_nested_error_2_twin_sync_sse *cst_new_box_autoadd_custom_nested_error_2_twin_sync_sse(void);

struct wire_cst_custom_nested_error_inner_twin_normal *cst_new_box_autoadd_custom_nested_error_inner_twin_normal(void);

struct wire_cst_custom_nested_error_inner_twin_rust_async *cst_new_box_autoadd_custom_nested_error_inner_twin_rust_async(void);

struct wire_cst_custom_nested_error_inner_twin_rust_async_sse *cst_new_box_autoadd_custom_nested_error_inner_twin_rust_async_sse(void);

struct wire_cst_custom_nested_error_inner_twin_sse *cst_new_box_autoadd_custom_nested_error_inner_twin_sse(void);

struct wire_cst_custom_nested_error_inner_twin_sync *cst_new_box_autoadd_custom_nested_error_inner_twin_sync(void);

struct wire_cst_custom_nested_error_inner_twin_sync_sse *cst_new_box_autoadd_custom_nested_error_inner_twin_sync_sse(void);

struct wire_cst_custom_nested_error_outer_twin_normal *cst_new_box_autoadd_custom_nested_error_outer_twin_normal(void);

struct wire_cst_custom_nested_error_outer_twin_rust_async *cst_new_box_autoadd_custom_nested_error_outer_twin_rust_async(void);

struct wire_cst_custom_nested_error_outer_twin_rust_async_sse *cst_new_box_autoadd_custom_nested_error_outer_twin_rust_async_sse(void);

struct wire_cst_custom_nested_error_outer_twin_sse *cst_new_box_autoadd_custom_nested_error_outer_twin_sse(void);

struct wire_cst_custom_nested_error_outer_twin_sync *cst_new_box_autoadd_custom_nested_error_outer_twin_sync(void);

struct wire_cst_custom_nested_error_outer_twin_sync_sse *cst_new_box_autoadd_custom_nested_error_outer_twin_sync_sse(void);

struct wire_cst_custom_struct_error_twin_normal *cst_new_box_autoadd_custom_struct_error_twin_normal(void);

struct wire_cst_custom_struct_error_twin_rust_async *cst_new_box_autoadd_custom_struct_error_twin_rust_async(void);

struct wire_cst_custom_struct_error_twin_rust_async_sse *cst_new_box_autoadd_custom_struct_error_twin_rust_async_sse(void);

struct wire_cst_custom_struct_error_twin_sse *cst_new_box_autoadd_custom_struct_error_twin_sse(void);

struct wire_cst_custom_struct_error_twin_sync *cst_new_box_autoadd_custom_struct_error_twin_sync(void);

struct wire_cst_custom_struct_error_twin_sync_sse *cst_new_box_autoadd_custom_struct_error_twin_sync_sse(void);

struct wire_cst_custom_struct_twin_normal *cst_new_box_autoadd_custom_struct_twin_normal(void);

struct wire_cst_custom_struct_twin_rust_async *cst_new_box_autoadd_custom_struct_twin_rust_async(void);

struct wire_cst_custom_struct_twin_rust_async_sse *cst_new_box_autoadd_custom_struct_twin_rust_async_sse(void);

struct wire_cst_custom_struct_twin_sse *cst_new_box_autoadd_custom_struct_twin_sse(void);

struct wire_cst_custom_struct_twin_sync *cst_new_box_autoadd_custom_struct_twin_sync(void);

struct wire_cst_custom_struct_twin_sync_sse *cst_new_box_autoadd_custom_struct_twin_sync_sse(void);

struct wire_cst_customized_twin_normal *cst_new_box_autoadd_customized_twin_normal(void);

struct wire_cst_customized_twin_rust_async *cst_new_box_autoadd_customized_twin_rust_async(void);

struct wire_cst_customized_twin_rust_async_sse *cst_new_box_autoadd_customized_twin_rust_async_sse(void);

struct wire_cst_customized_twin_sse *cst_new_box_autoadd_customized_twin_sse(void);

struct wire_cst_customized_twin_sync *cst_new_box_autoadd_customized_twin_sync(void);

struct wire_cst_customized_twin_sync_sse *cst_new_box_autoadd_customized_twin_sync_sse(void);

struct wire_cst_dart_opaque_nested_twin_normal *cst_new_box_autoadd_dart_opaque_nested_twin_normal(void);

struct wire_cst_dart_opaque_nested_twin_rust_async *cst_new_box_autoadd_dart_opaque_nested_twin_rust_async(void);

struct wire_cst_dart_opaque_nested_twin_rust_async_sse *cst_new_box_autoadd_dart_opaque_nested_twin_rust_async_sse(void);

struct wire_cst_dart_opaque_nested_twin_sse *cst_new_box_autoadd_dart_opaque_nested_twin_sse(void);

struct wire_cst_dart_opaque_nested_twin_sync *cst_new_box_autoadd_dart_opaque_nested_twin_sync(void);

struct wire_cst_dart_opaque_nested_twin_sync_sse *cst_new_box_autoadd_dart_opaque_nested_twin_sync_sse(void);

struct wire_cst_element_twin_normal *cst_new_box_autoadd_element_twin_normal(void);

struct wire_cst_element_twin_rust_async *cst_new_box_autoadd_element_twin_rust_async(void);

struct wire_cst_element_twin_rust_async_sse *cst_new_box_autoadd_element_twin_rust_async_sse(void);

struct wire_cst_element_twin_sse *cst_new_box_autoadd_element_twin_sse(void);

struct wire_cst_element_twin_sync *cst_new_box_autoadd_element_twin_sync(void);

struct wire_cst_element_twin_sync_sse *cst_new_box_autoadd_element_twin_sync_sse(void);

struct wire_cst_empty_twin_normal *cst_new_box_autoadd_empty_twin_normal(void);

struct wire_cst_empty_twin_rust_async *cst_new_box_autoadd_empty_twin_rust_async(void);

struct wire_cst_empty_twin_rust_async_sse *cst_new_box_autoadd_empty_twin_rust_async_sse(void);

struct wire_cst_empty_twin_sse *cst_new_box_autoadd_empty_twin_sse(void);

struct wire_cst_empty_twin_sync *cst_new_box_autoadd_empty_twin_sync(void);

struct wire_cst_empty_twin_sync_sse *cst_new_box_autoadd_empty_twin_sync_sse(void);

struct wire_cst_enum_dart_opaque_twin_normal *cst_new_box_autoadd_enum_dart_opaque_twin_normal(void);

struct wire_cst_enum_dart_opaque_twin_rust_async *cst_new_box_autoadd_enum_dart_opaque_twin_rust_async(void);

struct wire_cst_enum_dart_opaque_twin_rust_async_sse *cst_new_box_autoadd_enum_dart_opaque_twin_rust_async_sse(void);

struct wire_cst_enum_dart_opaque_twin_sse *cst_new_box_autoadd_enum_dart_opaque_twin_sse(void);

struct wire_cst_enum_dart_opaque_twin_sync *cst_new_box_autoadd_enum_dart_opaque_twin_sync(void);

struct wire_cst_enum_dart_opaque_twin_sync_sse *cst_new_box_autoadd_enum_dart_opaque_twin_sync_sse(void);

struct wire_cst_enum_opaque_twin_normal *cst_new_box_autoadd_enum_opaque_twin_normal(void);

struct wire_cst_enum_opaque_twin_rust_async *cst_new_box_autoadd_enum_opaque_twin_rust_async(void);

struct wire_cst_enum_opaque_twin_rust_async_sse *cst_new_box_autoadd_enum_opaque_twin_rust_async_sse(void);

struct wire_cst_enum_opaque_twin_sse *cst_new_box_autoadd_enum_opaque_twin_sse(void);

struct wire_cst_enum_opaque_twin_sync *cst_new_box_autoadd_enum_opaque_twin_sync(void);

struct wire_cst_enum_opaque_twin_sync_sse *cst_new_box_autoadd_enum_opaque_twin_sync_sse(void);

struct wire_cst_enum_with_item_mixed_twin_normal *cst_new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_cst_enum_with_item_mixed_twin_rust_async *cst_new_box_autoadd_enum_with_item_mixed_twin_rust_async(void);

struct wire_cst_enum_with_item_mixed_twin_rust_async_sse *cst_new_box_autoadd_enum_with_item_mixed_twin_rust_async_sse(void);

struct wire_cst_enum_with_item_mixed_twin_sse *cst_new_box_autoadd_enum_with_item_mixed_twin_sse(void);

struct wire_cst_enum_with_item_mixed_twin_sync *cst_new_box_autoadd_enum_with_item_mixed_twin_sync(void);

struct wire_cst_enum_with_item_mixed_twin_sync_sse *cst_new_box_autoadd_enum_with_item_mixed_twin_sync_sse(void);

struct wire_cst_enum_with_item_struct_twin_normal *cst_new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_cst_enum_with_item_struct_twin_rust_async *cst_new_box_autoadd_enum_with_item_struct_twin_rust_async(void);

struct wire_cst_enum_with_item_struct_twin_rust_async_sse *cst_new_box_autoadd_enum_with_item_struct_twin_rust_async_sse(void);

struct wire_cst_enum_with_item_struct_twin_sse *cst_new_box_autoadd_enum_with_item_struct_twin_sse(void);

struct wire_cst_enum_with_item_struct_twin_sync *cst_new_box_autoadd_enum_with_item_struct_twin_sync(void);

struct wire_cst_enum_with_item_struct_twin_sync_sse *cst_new_box_autoadd_enum_with_item_struct_twin_sync_sse(void);

struct wire_cst_enum_with_item_tuple_twin_normal *cst_new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_cst_enum_with_item_tuple_twin_rust_async *cst_new_box_autoadd_enum_with_item_tuple_twin_rust_async(void);

struct wire_cst_enum_with_item_tuple_twin_rust_async_sse *cst_new_box_autoadd_enum_with_item_tuple_twin_rust_async_sse(void);

struct wire_cst_enum_with_item_tuple_twin_sse *cst_new_box_autoadd_enum_with_item_tuple_twin_sse(void);

struct wire_cst_enum_with_item_tuple_twin_sync *cst_new_box_autoadd_enum_with_item_tuple_twin_sync(void);

struct wire_cst_enum_with_item_tuple_twin_sync_sse *cst_new_box_autoadd_enum_with_item_tuple_twin_sync_sse(void);

struct wire_cst_event_twin_normal *cst_new_box_autoadd_event_twin_normal(void);

struct wire_cst_event_twin_rust_async *cst_new_box_autoadd_event_twin_rust_async(void);

struct wire_cst_event_twin_rust_async_sse *cst_new_box_autoadd_event_twin_rust_async_sse(void);

struct wire_cst_event_twin_sse *cst_new_box_autoadd_event_twin_sse(void);

struct wire_cst_exotic_optionals_twin_normal *cst_new_box_autoadd_exotic_optionals_twin_normal(void);

struct wire_cst_exotic_optionals_twin_rust_async *cst_new_box_autoadd_exotic_optionals_twin_rust_async(void);

struct wire_cst_exotic_optionals_twin_rust_async_sse *cst_new_box_autoadd_exotic_optionals_twin_rust_async_sse(void);

struct wire_cst_exotic_optionals_twin_sse *cst_new_box_autoadd_exotic_optionals_twin_sse(void);

struct wire_cst_exotic_optionals_twin_sync *cst_new_box_autoadd_exotic_optionals_twin_sync(void);

struct wire_cst_exotic_optionals_twin_sync_sse *cst_new_box_autoadd_exotic_optionals_twin_sync_sse(void);

float *cst_new_box_autoadd_f_32(float value);

double *cst_new_box_autoadd_f_64(double value);

struct wire_cst_feature_chrono_twin_normal *cst_new_box_autoadd_feature_chrono_twin_normal(void);

struct wire_cst_feature_chrono_twin_rust_async *cst_new_box_autoadd_feature_chrono_twin_rust_async(void);

struct wire_cst_feature_chrono_twin_sync *cst_new_box_autoadd_feature_chrono_twin_sync(void);

struct wire_cst_feature_uuid_twin_normal *cst_new_box_autoadd_feature_uuid_twin_normal(void);

struct wire_cst_feature_uuid_twin_rust_async *cst_new_box_autoadd_feature_uuid_twin_rust_async(void);

struct wire_cst_feature_uuid_twin_sync *cst_new_box_autoadd_feature_uuid_twin_sync(void);

struct wire_cst_feed_id_twin_normal *cst_new_box_autoadd_feed_id_twin_normal(void);

struct wire_cst_feed_id_twin_rust_async *cst_new_box_autoadd_feed_id_twin_rust_async(void);

struct wire_cst_feed_id_twin_rust_async_sse *cst_new_box_autoadd_feed_id_twin_rust_async_sse(void);

struct wire_cst_feed_id_twin_sse *cst_new_box_autoadd_feed_id_twin_sse(void);

struct wire_cst_feed_id_twin_sync *cst_new_box_autoadd_feed_id_twin_sync(void);

struct wire_cst_feed_id_twin_sync_sse *cst_new_box_autoadd_feed_id_twin_sync_sse(void);

int16_t *cst_new_box_autoadd_i_16(int16_t value);

int32_t *cst_new_box_autoadd_i_32(int32_t value);

int64_t *cst_new_box_autoadd_i_64(int64_t value);

int8_t *cst_new_box_autoadd_i_8(int8_t value);

struct wire_cst_kitchen_sink_twin_normal *cst_new_box_autoadd_kitchen_sink_twin_normal(void);

struct wire_cst_kitchen_sink_twin_rust_async *cst_new_box_autoadd_kitchen_sink_twin_rust_async(void);

struct wire_cst_kitchen_sink_twin_rust_async_sse *cst_new_box_autoadd_kitchen_sink_twin_rust_async_sse(void);

struct wire_cst_kitchen_sink_twin_sse *cst_new_box_autoadd_kitchen_sink_twin_sse(void);

struct wire_cst_kitchen_sink_twin_sync *cst_new_box_autoadd_kitchen_sink_twin_sync(void);

struct wire_cst_kitchen_sink_twin_sync_sse *cst_new_box_autoadd_kitchen_sink_twin_sync_sse(void);

struct wire_cst_list_of_nested_raw_string_mirrored *cst_new_box_autoadd_list_of_nested_raw_string_mirrored(void);

struct wire_cst_macro_struct *cst_new_box_autoadd_macro_struct(void);

struct wire_cst_measure_twin_normal *cst_new_box_autoadd_measure_twin_normal(void);

struct wire_cst_measure_twin_rust_async *cst_new_box_autoadd_measure_twin_rust_async(void);

struct wire_cst_measure_twin_rust_async_sse *cst_new_box_autoadd_measure_twin_rust_async_sse(void);

struct wire_cst_measure_twin_sse *cst_new_box_autoadd_measure_twin_sse(void);

struct wire_cst_measure_twin_sync *cst_new_box_autoadd_measure_twin_sync(void);

struct wire_cst_measure_twin_sync_sse *cst_new_box_autoadd_measure_twin_sync_sse(void);

struct wire_cst_message_id_twin_normal *cst_new_box_autoadd_message_id_twin_normal(void);

struct wire_cst_message_id_twin_rust_async *cst_new_box_autoadd_message_id_twin_rust_async(void);

struct wire_cst_message_id_twin_rust_async_sse *cst_new_box_autoadd_message_id_twin_rust_async_sse(void);

struct wire_cst_message_id_twin_sse *cst_new_box_autoadd_message_id_twin_sse(void);

struct wire_cst_message_id_twin_sync *cst_new_box_autoadd_message_id_twin_sync(void);

struct wire_cst_message_id_twin_sync_sse *cst_new_box_autoadd_message_id_twin_sync_sse(void);

struct wire_cst_my_nested_struct_twin_normal *cst_new_box_autoadd_my_nested_struct_twin_normal(void);

struct wire_cst_my_nested_struct_twin_rust_async *cst_new_box_autoadd_my_nested_struct_twin_rust_async(void);

struct wire_cst_my_nested_struct_twin_rust_async_sse *cst_new_box_autoadd_my_nested_struct_twin_rust_async_sse(void);

struct wire_cst_my_nested_struct_twin_sse *cst_new_box_autoadd_my_nested_struct_twin_sse(void);

struct wire_cst_my_nested_struct_twin_sync *cst_new_box_autoadd_my_nested_struct_twin_sync(void);

struct wire_cst_my_nested_struct_twin_sync_sse *cst_new_box_autoadd_my_nested_struct_twin_sync_sse(void);

struct wire_cst_my_size *cst_new_box_autoadd_my_size(void);

struct wire_cst_my_struct *cst_new_box_autoadd_my_struct(void);

struct wire_cst_my_tree_node_twin_normal *cst_new_box_autoadd_my_tree_node_twin_normal(void);

struct wire_cst_my_tree_node_twin_rust_async *cst_new_box_autoadd_my_tree_node_twin_rust_async(void);

struct wire_cst_my_tree_node_twin_rust_async_sse *cst_new_box_autoadd_my_tree_node_twin_rust_async_sse(void);

struct wire_cst_my_tree_node_twin_sse *cst_new_box_autoadd_my_tree_node_twin_sse(void);

struct wire_cst_my_tree_node_twin_sync *cst_new_box_autoadd_my_tree_node_twin_sync(void);

struct wire_cst_my_tree_node_twin_sync_sse *cst_new_box_autoadd_my_tree_node_twin_sync_sse(void);

struct wire_cst_nested_raw_string_mirrored *cst_new_box_autoadd_nested_raw_string_mirrored(void);

struct wire_cst_new_type_int_twin_normal *cst_new_box_autoadd_new_type_int_twin_normal(void);

struct wire_cst_new_type_int_twin_rust_async *cst_new_box_autoadd_new_type_int_twin_rust_async(void);

struct wire_cst_new_type_int_twin_rust_async_sse *cst_new_box_autoadd_new_type_int_twin_rust_async_sse(void);

struct wire_cst_new_type_int_twin_sse *cst_new_box_autoadd_new_type_int_twin_sse(void);

struct wire_cst_new_type_int_twin_sync *cst_new_box_autoadd_new_type_int_twin_sync(void);

struct wire_cst_new_type_int_twin_sync_sse *cst_new_box_autoadd_new_type_int_twin_sync_sse(void);

struct wire_cst_note_twin_normal *cst_new_box_autoadd_note_twin_normal(void);

struct wire_cst_note_twin_rust_async *cst_new_box_autoadd_note_twin_rust_async(void);

struct wire_cst_note_twin_rust_async_sse *cst_new_box_autoadd_note_twin_rust_async_sse(void);

struct wire_cst_note_twin_sse *cst_new_box_autoadd_note_twin_sse(void);

struct wire_cst_note_twin_sync *cst_new_box_autoadd_note_twin_sync(void);

struct wire_cst_note_twin_sync_sse *cst_new_box_autoadd_note_twin_sync_sse(void);

struct wire_cst_numbers *cst_new_box_autoadd_numbers(void);

struct wire_cst_opaque_nested_twin_normal *cst_new_box_autoadd_opaque_nested_twin_normal(void);

struct wire_cst_opaque_nested_twin_rust_async *cst_new_box_autoadd_opaque_nested_twin_rust_async(void);

struct wire_cst_opaque_nested_twin_rust_async_sse *cst_new_box_autoadd_opaque_nested_twin_rust_async_sse(void);

struct wire_cst_opaque_nested_twin_sse *cst_new_box_autoadd_opaque_nested_twin_sse(void);

struct wire_cst_opaque_nested_twin_sync *cst_new_box_autoadd_opaque_nested_twin_sync(void);

struct wire_cst_opaque_nested_twin_sync_sse *cst_new_box_autoadd_opaque_nested_twin_sync_sse(void);

struct wire_cst_opt_vecs_twin_normal *cst_new_box_autoadd_opt_vecs_twin_normal(void);

struct wire_cst_opt_vecs_twin_rust_async *cst_new_box_autoadd_opt_vecs_twin_rust_async(void);

struct wire_cst_opt_vecs_twin_rust_async_sse *cst_new_box_autoadd_opt_vecs_twin_rust_async_sse(void);

struct wire_cst_opt_vecs_twin_sse *cst_new_box_autoadd_opt_vecs_twin_sse(void);

struct wire_cst_opt_vecs_twin_sync *cst_new_box_autoadd_opt_vecs_twin_sync(void);

struct wire_cst_opt_vecs_twin_sync_sse *cst_new_box_autoadd_opt_vecs_twin_sync_sse(void);

struct wire_cst_raw_string_mirrored *cst_new_box_autoadd_raw_string_mirrored(void);

struct wire_cst_record_string_i_32 *cst_new_box_autoadd_record_string_i_32(void);

struct wire_cst_sequences *cst_new_box_autoadd_sequences(void);

struct wire_cst_some_struct_twin_normal *cst_new_box_autoadd_some_struct_twin_normal(void);

struct wire_cst_some_struct_twin_rust_async *cst_new_box_autoadd_some_struct_twin_rust_async(void);

struct wire_cst_some_struct_twin_rust_async_sse *cst_new_box_autoadd_some_struct_twin_rust_async_sse(void);

struct wire_cst_some_struct_twin_sse *cst_new_box_autoadd_some_struct_twin_sse(void);

struct wire_cst_some_struct_twin_sync *cst_new_box_autoadd_some_struct_twin_sync(void);

struct wire_cst_some_struct_twin_sync_sse *cst_new_box_autoadd_some_struct_twin_sync_sse(void);

struct wire_cst_struct_with_comments_twin_normal *cst_new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_cst_struct_with_comments_twin_rust_async *cst_new_box_autoadd_struct_with_comments_twin_rust_async(void);

struct wire_cst_struct_with_comments_twin_rust_async_sse *cst_new_box_autoadd_struct_with_comments_twin_rust_async_sse(void);

struct wire_cst_struct_with_comments_twin_sse *cst_new_box_autoadd_struct_with_comments_twin_sse(void);

struct wire_cst_struct_with_comments_twin_sync *cst_new_box_autoadd_struct_with_comments_twin_sync(void);

struct wire_cst_struct_with_comments_twin_sync_sse *cst_new_box_autoadd_struct_with_comments_twin_sync_sse(void);

struct wire_cst_struct_with_enum_twin_normal *cst_new_box_autoadd_struct_with_enum_twin_normal(void);

struct wire_cst_struct_with_enum_twin_rust_async *cst_new_box_autoadd_struct_with_enum_twin_rust_async(void);

struct wire_cst_struct_with_enum_twin_rust_async_sse *cst_new_box_autoadd_struct_with_enum_twin_rust_async_sse(void);

struct wire_cst_struct_with_enum_twin_sse *cst_new_box_autoadd_struct_with_enum_twin_sse(void);

struct wire_cst_struct_with_enum_twin_sync *cst_new_box_autoadd_struct_with_enum_twin_sync(void);

struct wire_cst_struct_with_enum_twin_sync_sse *cst_new_box_autoadd_struct_with_enum_twin_sync_sse(void);

struct wire_cst_struct_with_one_field_twin_normal *cst_new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_cst_struct_with_one_field_twin_rust_async *cst_new_box_autoadd_struct_with_one_field_twin_rust_async(void);

struct wire_cst_struct_with_one_field_twin_rust_async_sse *cst_new_box_autoadd_struct_with_one_field_twin_rust_async_sse(void);

struct wire_cst_struct_with_one_field_twin_sse *cst_new_box_autoadd_struct_with_one_field_twin_sse(void);

struct wire_cst_struct_with_one_field_twin_sync *cst_new_box_autoadd_struct_with_one_field_twin_sync(void);

struct wire_cst_struct_with_one_field_twin_sync_sse *cst_new_box_autoadd_struct_with_one_field_twin_sync_sse(void);

struct wire_cst_struct_with_two_field_twin_normal *cst_new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_cst_struct_with_two_field_twin_rust_async *cst_new_box_autoadd_struct_with_two_field_twin_rust_async(void);

struct wire_cst_struct_with_two_field_twin_rust_async_sse *cst_new_box_autoadd_struct_with_two_field_twin_rust_async_sse(void);

struct wire_cst_struct_with_two_field_twin_sse *cst_new_box_autoadd_struct_with_two_field_twin_sse(void);

struct wire_cst_struct_with_two_field_twin_sync *cst_new_box_autoadd_struct_with_two_field_twin_sync(void);

struct wire_cst_struct_with_two_field_twin_sync_sse *cst_new_box_autoadd_struct_with_two_field_twin_sync_sse(void);

struct wire_cst_struct_with_zero_field_twin_normal *cst_new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_cst_struct_with_zero_field_twin_rust_async *cst_new_box_autoadd_struct_with_zero_field_twin_rust_async(void);

struct wire_cst_struct_with_zero_field_twin_rust_async_sse *cst_new_box_autoadd_struct_with_zero_field_twin_rust_async_sse(void);

struct wire_cst_struct_with_zero_field_twin_sse *cst_new_box_autoadd_struct_with_zero_field_twin_sse(void);

struct wire_cst_struct_with_zero_field_twin_sync *cst_new_box_autoadd_struct_with_zero_field_twin_sync(void);

struct wire_cst_struct_with_zero_field_twin_sync_sse *cst_new_box_autoadd_struct_with_zero_field_twin_sync_sse(void);

struct wire_cst_sum_with_twin_normal *cst_new_box_autoadd_sum_with_twin_normal(void);

struct wire_cst_sum_with_twin_rust_async *cst_new_box_autoadd_sum_with_twin_rust_async(void);

struct wire_cst_sum_with_twin_rust_async_sse *cst_new_box_autoadd_sum_with_twin_rust_async_sse(void);

struct wire_cst_sum_with_twin_sse *cst_new_box_autoadd_sum_with_twin_sse(void);

struct wire_cst_sum_with_twin_sync *cst_new_box_autoadd_sum_with_twin_sync(void);

struct wire_cst_sum_with_twin_sync_sse *cst_new_box_autoadd_sum_with_twin_sync_sse(void);

struct wire_cst_test_id_twin_normal *cst_new_box_autoadd_test_id_twin_normal(void);

struct wire_cst_test_id_twin_rust_async *cst_new_box_autoadd_test_id_twin_rust_async(void);

struct wire_cst_test_id_twin_rust_async_sse *cst_new_box_autoadd_test_id_twin_rust_async_sse(void);

struct wire_cst_test_id_twin_sse *cst_new_box_autoadd_test_id_twin_sse(void);

struct wire_cst_test_id_twin_sync *cst_new_box_autoadd_test_id_twin_sync(void);

struct wire_cst_test_id_twin_sync_sse *cst_new_box_autoadd_test_id_twin_sync_sse(void);

struct wire_cst_tuple_struct_with_one_field_twin_normal *cst_new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_cst_tuple_struct_with_one_field_twin_rust_async *cst_new_box_autoadd_tuple_struct_with_one_field_twin_rust_async(void);

struct wire_cst_tuple_struct_with_one_field_twin_rust_async_sse *cst_new_box_autoadd_tuple_struct_with_one_field_twin_rust_async_sse(void);

struct wire_cst_tuple_struct_with_one_field_twin_sse *cst_new_box_autoadd_tuple_struct_with_one_field_twin_sse(void);

struct wire_cst_tuple_struct_with_one_field_twin_sync *cst_new_box_autoadd_tuple_struct_with_one_field_twin_sync(void);

struct wire_cst_tuple_struct_with_one_field_twin_sync_sse *cst_new_box_autoadd_tuple_struct_with_one_field_twin_sync_sse(void);

struct wire_cst_tuple_struct_with_two_field_twin_normal *cst_new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_cst_tuple_struct_with_two_field_twin_rust_async *cst_new_box_autoadd_tuple_struct_with_two_field_twin_rust_async(void);

struct wire_cst_tuple_struct_with_two_field_twin_rust_async_sse *cst_new_box_autoadd_tuple_struct_with_two_field_twin_rust_async_sse(void);

struct wire_cst_tuple_struct_with_two_field_twin_sse *cst_new_box_autoadd_tuple_struct_with_two_field_twin_sse(void);

struct wire_cst_tuple_struct_with_two_field_twin_sync *cst_new_box_autoadd_tuple_struct_with_two_field_twin_sync(void);

struct wire_cst_tuple_struct_with_two_field_twin_sync_sse *cst_new_box_autoadd_tuple_struct_with_two_field_twin_sync_sse(void);

uint16_t *cst_new_box_autoadd_u_16(uint16_t value);

uint32_t *cst_new_box_autoadd_u_32(uint32_t value);

uint64_t *cst_new_box_autoadd_u_64(uint64_t value);

uint8_t *cst_new_box_autoadd_u_8(uint8_t value);

struct wire_cst_user_id_twin_normal *cst_new_box_autoadd_user_id_twin_normal(void);

struct wire_cst_user_id_twin_rust_async *cst_new_box_autoadd_user_id_twin_rust_async(void);

struct wire_cst_user_id_twin_rust_async_sse *cst_new_box_autoadd_user_id_twin_rust_async_sse(void);

struct wire_cst_user_id_twin_sse *cst_new_box_autoadd_user_id_twin_sse(void);

struct wire_cst_user_id_twin_sync *cst_new_box_autoadd_user_id_twin_sync(void);

struct wire_cst_user_id_twin_sync_sse *cst_new_box_autoadd_user_id_twin_sync_sse(void);

int32_t *cst_new_box_autoadd_weekdays_twin_normal(int32_t value);

int32_t *cst_new_box_autoadd_weekdays_twin_rust_async(int32_t value);

int32_t *cst_new_box_autoadd_weekdays_twin_rust_async_sse(int32_t value);

int32_t *cst_new_box_autoadd_weekdays_twin_sse(int32_t value);

int32_t *cst_new_box_autoadd_weekdays_twin_sync(int32_t value);

int32_t *cst_new_box_autoadd_weekdays_twin_sync_sse(int32_t value);

struct wire_cst_blob_twin_normal *cst_new_box_blob_twin_normal(void);

struct wire_cst_blob_twin_rust_async *cst_new_box_blob_twin_rust_async(void);

struct wire_cst_blob_twin_rust_async_sse *cst_new_box_blob_twin_rust_async_sse(void);

struct wire_cst_blob_twin_sse *cst_new_box_blob_twin_sse(void);

struct wire_cst_blob_twin_sync *cst_new_box_blob_twin_sync(void);

struct wire_cst_blob_twin_sync_sse *cst_new_box_blob_twin_sync_sse(void);

bool *cst_new_box_bool(bool value);

struct wire_cst_distance_twin_normal *cst_new_box_distance_twin_normal(void);

struct wire_cst_distance_twin_rust_async *cst_new_box_distance_twin_rust_async(void);

struct wire_cst_distance_twin_rust_async_sse *cst_new_box_distance_twin_rust_async_sse(void);

struct wire_cst_distance_twin_sse *cst_new_box_distance_twin_sse(void);

struct wire_cst_distance_twin_sync *cst_new_box_distance_twin_sync(void);

struct wire_cst_distance_twin_sync_sse *cst_new_box_distance_twin_sync_sse(void);

struct wire_cst_exotic_optionals_twin_normal *cst_new_box_exotic_optionals_twin_normal(void);

struct wire_cst_exotic_optionals_twin_rust_async *cst_new_box_exotic_optionals_twin_rust_async(void);

struct wire_cst_exotic_optionals_twin_rust_async_sse *cst_new_box_exotic_optionals_twin_rust_async_sse(void);

struct wire_cst_exotic_optionals_twin_sse *cst_new_box_exotic_optionals_twin_sse(void);

struct wire_cst_exotic_optionals_twin_sync *cst_new_box_exotic_optionals_twin_sync(void);

struct wire_cst_exotic_optionals_twin_sync_sse *cst_new_box_exotic_optionals_twin_sync_sse(void);

double *cst_new_box_f_64(double value);

struct wire_cst_feed_id_twin_normal *cst_new_box_feed_id_twin_normal(void);

struct wire_cst_feed_id_twin_rust_async *cst_new_box_feed_id_twin_rust_async(void);

struct wire_cst_feed_id_twin_rust_async_sse *cst_new_box_feed_id_twin_rust_async_sse(void);

struct wire_cst_feed_id_twin_sse *cst_new_box_feed_id_twin_sse(void);

struct wire_cst_feed_id_twin_sync *cst_new_box_feed_id_twin_sync(void);

struct wire_cst_feed_id_twin_sync_sse *cst_new_box_feed_id_twin_sync_sse(void);

int32_t *cst_new_box_i_32(int32_t value);

int64_t *cst_new_box_i_64(int64_t value);

int8_t *cst_new_box_i_8(int8_t value);

struct wire_cst_kitchen_sink_twin_normal *cst_new_box_kitchen_sink_twin_normal(void);

struct wire_cst_kitchen_sink_twin_rust_async *cst_new_box_kitchen_sink_twin_rust_async(void);

struct wire_cst_kitchen_sink_twin_rust_async_sse *cst_new_box_kitchen_sink_twin_rust_async_sse(void);

struct wire_cst_kitchen_sink_twin_sse *cst_new_box_kitchen_sink_twin_sse(void);

struct wire_cst_kitchen_sink_twin_sync *cst_new_box_kitchen_sink_twin_sync(void);

struct wire_cst_kitchen_sink_twin_sync_sse *cst_new_box_kitchen_sink_twin_sync_sse(void);

struct wire_cst_my_size *cst_new_box_my_size(void);

struct wire_cst_speed_twin_normal *cst_new_box_speed_twin_normal(void);

struct wire_cst_speed_twin_rust_async *cst_new_box_speed_twin_rust_async(void);

struct wire_cst_speed_twin_rust_async_sse *cst_new_box_speed_twin_rust_async_sse(void);

struct wire_cst_speed_twin_sse *cst_new_box_speed_twin_sse(void);

struct wire_cst_speed_twin_sync *cst_new_box_speed_twin_sync(void);

struct wire_cst_speed_twin_sync_sse *cst_new_box_speed_twin_sync_sse(void);

uint8_t *cst_new_box_u_8(uint8_t value);

int32_t *cst_new_box_weekdays_twin_normal(int32_t value);

int32_t *cst_new_box_weekdays_twin_rust_async(int32_t value);

int32_t *cst_new_box_weekdays_twin_rust_async_sse(int32_t value);

int32_t *cst_new_box_weekdays_twin_sse(int32_t value);

int32_t *cst_new_box_weekdays_twin_sync(int32_t value);

int32_t *cst_new_box_weekdays_twin_sync_sse(int32_t value);

struct wire_cst_list_Chrono_Duration *cst_new_list_Chrono_Duration(int32_t len);

struct wire_cst_list_Chrono_Local *cst_new_list_Chrono_Local(int32_t len);

struct wire_cst_list_Chrono_Naive *cst_new_list_Chrono_Naive(int32_t len);

struct wire_cst_list_DartOpaque *cst_new_list_DartOpaque(int32_t len);

struct wire_cst_list_RustOpaque_hide_data *cst_new_list_RustOpaque_hide_data(int32_t len);

struct wire_cst_list_String *cst_new_list_String(int32_t len);

struct wire_cst_list_application_env_var *cst_new_list_application_env_var(int32_t len);

struct wire_cst_list_application_settings *cst_new_list_application_settings(int32_t len);

struct wire_cst_list_attribute_twin_normal *cst_new_list_attribute_twin_normal(int32_t len);

struct wire_cst_list_attribute_twin_rust_async *cst_new_list_attribute_twin_rust_async(int32_t len);

struct wire_cst_list_attribute_twin_rust_async_sse *cst_new_list_attribute_twin_rust_async_sse(int32_t len);

struct wire_cst_list_attribute_twin_sse *cst_new_list_attribute_twin_sse(int32_t len);

struct wire_cst_list_attribute_twin_sync *cst_new_list_attribute_twin_sync(int32_t len);

struct wire_cst_list_attribute_twin_sync_sse *cst_new_list_attribute_twin_sync_sse(int32_t len);

struct wire_cst_list_bool *cst_new_list_bool(int32_t len);

struct wire_cst_list_element_twin_normal *cst_new_list_element_twin_normal(int32_t len);

struct wire_cst_list_element_twin_rust_async *cst_new_list_element_twin_rust_async(int32_t len);

struct wire_cst_list_element_twin_rust_async_sse *cst_new_list_element_twin_rust_async_sse(int32_t len);

struct wire_cst_list_element_twin_sse *cst_new_list_element_twin_sse(int32_t len);

struct wire_cst_list_element_twin_sync *cst_new_list_element_twin_sync(int32_t len);

struct wire_cst_list_element_twin_sync_sse *cst_new_list_element_twin_sync_sse(int32_t len);

struct wire_cst_list_enum_opaque_twin_normal *cst_new_list_enum_opaque_twin_normal(int32_t len);

struct wire_cst_list_enum_opaque_twin_rust_async *cst_new_list_enum_opaque_twin_rust_async(int32_t len);

struct wire_cst_list_enum_opaque_twin_rust_async_sse *cst_new_list_enum_opaque_twin_rust_async_sse(int32_t len);

struct wire_cst_list_enum_opaque_twin_sse *cst_new_list_enum_opaque_twin_sse(int32_t len);

struct wire_cst_list_enum_opaque_twin_sync *cst_new_list_enum_opaque_twin_sync(int32_t len);

struct wire_cst_list_enum_opaque_twin_sync_sse *cst_new_list_enum_opaque_twin_sync_sse(int32_t len);

struct wire_cst_list_my_enum *cst_new_list_my_enum(int32_t len);

struct wire_cst_list_my_size *cst_new_list_my_size(int32_t len);

struct wire_cst_list_my_tree_node_twin_normal *cst_new_list_my_tree_node_twin_normal(int32_t len);

struct wire_cst_list_my_tree_node_twin_rust_async *cst_new_list_my_tree_node_twin_rust_async(int32_t len);

struct wire_cst_list_my_tree_node_twin_rust_async_sse *cst_new_list_my_tree_node_twin_rust_async_sse(int32_t len);

struct wire_cst_list_my_tree_node_twin_sse *cst_new_list_my_tree_node_twin_sse(int32_t len);

struct wire_cst_list_my_tree_node_twin_sync *cst_new_list_my_tree_node_twin_sync(int32_t len);

struct wire_cst_list_my_tree_node_twin_sync_sse *cst_new_list_my_tree_node_twin_sync_sse(int32_t len);

struct wire_cst_list_nested_raw_string_mirrored *cst_new_list_nested_raw_string_mirrored(int32_t len);

struct wire_cst_list_opt_String *cst_new_list_opt_String(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_normal *cst_new_list_opt_box_autoadd_attribute_twin_normal(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async *cst_new_list_opt_box_autoadd_attribute_twin_rust_async(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_rust_async_sse *cst_new_list_opt_box_autoadd_attribute_twin_rust_async_sse(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_sse *cst_new_list_opt_box_autoadd_attribute_twin_sse(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_sync *cst_new_list_opt_box_autoadd_attribute_twin_sync(int32_t len);

struct wire_cst_list_opt_box_autoadd_attribute_twin_sync_sse *cst_new_list_opt_box_autoadd_attribute_twin_sync_sse(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_32 *cst_new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_normal *cst_new_list_opt_box_autoadd_weekdays_twin_normal(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async *cst_new_list_opt_box_autoadd_weekdays_twin_rust_async(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_rust_async_sse *cst_new_list_opt_box_autoadd_weekdays_twin_rust_async_sse(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_sse *cst_new_list_opt_box_autoadd_weekdays_twin_sse(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync *cst_new_list_opt_box_autoadd_weekdays_twin_sync(int32_t len);

struct wire_cst_list_opt_box_autoadd_weekdays_twin_sync_sse *cst_new_list_opt_box_autoadd_weekdays_twin_sync_sse(int32_t len);

struct wire_cst_list_opt_list_prim_i_32 *cst_new_list_opt_list_prim_i_32(int32_t len);

struct wire_cst_list_point_twin_normal *cst_new_list_point_twin_normal(int32_t len);

struct wire_cst_list_point_twin_rust_async *cst_new_list_point_twin_rust_async(int32_t len);

struct wire_cst_list_point_twin_rust_async_sse *cst_new_list_point_twin_rust_async_sse(int32_t len);

struct wire_cst_list_point_twin_sse *cst_new_list_point_twin_sse(int32_t len);

struct wire_cst_list_point_twin_sync *cst_new_list_point_twin_sync(int32_t len);

struct wire_cst_list_point_twin_sync_sse *cst_new_list_point_twin_sync_sse(int32_t len);

struct wire_cst_list_prim_f_32 *cst_new_list_prim_f_32(int32_t len);

struct wire_cst_list_prim_f_64 *cst_new_list_prim_f_64(int32_t len);

struct wire_cst_list_prim_i_16 *cst_new_list_prim_i_16(int32_t len);

struct wire_cst_list_prim_i_32 *cst_new_list_prim_i_32(int32_t len);

struct wire_cst_list_prim_i_64 *cst_new_list_prim_i_64(int32_t len);

struct wire_cst_list_prim_i_8 *cst_new_list_prim_i_8(int32_t len);

struct wire_cst_list_prim_u_16 *cst_new_list_prim_u_16(int32_t len);

struct wire_cst_list_prim_u_32 *cst_new_list_prim_u_32(int32_t len);

struct wire_cst_list_prim_u_64 *cst_new_list_prim_u_64(int32_t len);

struct wire_cst_list_prim_u_8 *cst_new_list_prim_u_8(int32_t len);

struct wire_cst_list_raw_string_enum_mirrored *cst_new_list_raw_string_enum_mirrored(int32_t len);

struct wire_cst_list_raw_string_mirrored *cst_new_list_raw_string_mirrored(int32_t len);

struct wire_cst_list_record_string_i_32 *cst_new_list_record_string_i_32(int32_t len);

struct wire_cst_list_sum_with_twin_normal *cst_new_list_sum_with_twin_normal(int32_t len);

struct wire_cst_list_sum_with_twin_rust_async *cst_new_list_sum_with_twin_rust_async(int32_t len);

struct wire_cst_list_sum_with_twin_rust_async_sse *cst_new_list_sum_with_twin_rust_async_sse(int32_t len);

struct wire_cst_list_sum_with_twin_sse *cst_new_list_sum_with_twin_sse(int32_t len);

struct wire_cst_list_sum_with_twin_sync *cst_new_list_sum_with_twin_sync(int32_t len);

struct wire_cst_list_sum_with_twin_sync_sse *cst_new_list_sum_with_twin_sync_sse(int32_t len);

struct wire_cst_list_test_id_twin_normal *cst_new_list_test_id_twin_normal(int32_t len);

struct wire_cst_list_test_id_twin_rust_async *cst_new_list_test_id_twin_rust_async(int32_t len);

struct wire_cst_list_test_id_twin_rust_async_sse *cst_new_list_test_id_twin_rust_async_sse(int32_t len);

struct wire_cst_list_test_id_twin_sse *cst_new_list_test_id_twin_sse(int32_t len);

struct wire_cst_list_test_id_twin_sync *cst_new_list_test_id_twin_sync(int32_t len);

struct wire_cst_list_test_id_twin_sync_sse *cst_new_list_test_id_twin_sync_sse(int32_t len);

struct wire_cst_list_weekdays_twin_normal *cst_new_list_weekdays_twin_normal(int32_t len);

struct wire_cst_list_weekdays_twin_rust_async *cst_new_list_weekdays_twin_rust_async(int32_t len);

struct wire_cst_list_weekdays_twin_rust_async_sse *cst_new_list_weekdays_twin_rust_async_sse(int32_t len);

struct wire_cst_list_weekdays_twin_sse *cst_new_list_weekdays_twin_sse(int32_t len);

struct wire_cst_list_weekdays_twin_sync *cst_new_list_weekdays_twin_sync(int32_t len);

struct wire_cst_list_weekdays_twin_sync_sse *cst_new_list_weekdays_twin_sync_sse(int32_t len);

union AbcTwinNormalKind *cst_inflate_AbcTwinNormal_A(void);

union AbcTwinNormalKind *cst_inflate_AbcTwinNormal_B(void);

union AbcTwinNormalKind *cst_inflate_AbcTwinNormal_C(void);

union AbcTwinNormalKind *cst_inflate_AbcTwinNormal_JustInt(void);

union AbcTwinRustAsyncKind *cst_inflate_AbcTwinRustAsync_A(void);

union AbcTwinRustAsyncKind *cst_inflate_AbcTwinRustAsync_B(void);

union AbcTwinRustAsyncKind *cst_inflate_AbcTwinRustAsync_C(void);

union AbcTwinRustAsyncKind *cst_inflate_AbcTwinRustAsync_JustInt(void);

union AbcTwinRustAsyncSseKind *cst_inflate_AbcTwinRustAsyncSse_A(void);

union AbcTwinRustAsyncSseKind *cst_inflate_AbcTwinRustAsyncSse_B(void);

union AbcTwinRustAsyncSseKind *cst_inflate_AbcTwinRustAsyncSse_C(void);

union AbcTwinRustAsyncSseKind *cst_inflate_AbcTwinRustAsyncSse_JustInt(void);

union AbcTwinSseKind *cst_inflate_AbcTwinSse_A(void);

union AbcTwinSseKind *cst_inflate_AbcTwinSse_B(void);

union AbcTwinSseKind *cst_inflate_AbcTwinSse_C(void);

union AbcTwinSseKind *cst_inflate_AbcTwinSse_JustInt(void);

union AbcTwinSyncKind *cst_inflate_AbcTwinSync_A(void);

union AbcTwinSyncKind *cst_inflate_AbcTwinSync_B(void);

union AbcTwinSyncKind *cst_inflate_AbcTwinSync_C(void);

union AbcTwinSyncKind *cst_inflate_AbcTwinSync_JustInt(void);

union AbcTwinSyncSseKind *cst_inflate_AbcTwinSyncSse_A(void);

union AbcTwinSyncSseKind *cst_inflate_AbcTwinSyncSse_B(void);

union AbcTwinSyncSseKind *cst_inflate_AbcTwinSyncSse_C(void);

union AbcTwinSyncSseKind *cst_inflate_AbcTwinSyncSse_JustInt(void);

union ApplicationMessageKind *cst_inflate_ApplicationMessage_DisplayMessage(void);

union ApplicationMessageKind *cst_inflate_ApplicationMessage_RenderPixel(void);

union CustomEnumErrorTwinNormalKind *cst_inflate_CustomEnumErrorTwinNormal_One(void);

union CustomEnumErrorTwinNormalKind *cst_inflate_CustomEnumErrorTwinNormal_Two(void);

union CustomEnumErrorTwinRustAsyncKind *cst_inflate_CustomEnumErrorTwinRustAsync_One(void);

union CustomEnumErrorTwinRustAsyncKind *cst_inflate_CustomEnumErrorTwinRustAsync_Two(void);

union CustomEnumErrorTwinRustAsyncSseKind *cst_inflate_CustomEnumErrorTwinRustAsyncSse_One(void);

union CustomEnumErrorTwinRustAsyncSseKind *cst_inflate_CustomEnumErrorTwinRustAsyncSse_Two(void);

union CustomEnumErrorTwinSseKind *cst_inflate_CustomEnumErrorTwinSse_One(void);

union CustomEnumErrorTwinSseKind *cst_inflate_CustomEnumErrorTwinSse_Two(void);

union CustomEnumErrorTwinSyncKind *cst_inflate_CustomEnumErrorTwinSync_One(void);

union CustomEnumErrorTwinSyncKind *cst_inflate_CustomEnumErrorTwinSync_Two(void);

union CustomEnumErrorTwinSyncSseKind *cst_inflate_CustomEnumErrorTwinSyncSse_One(void);

union CustomEnumErrorTwinSyncSseKind *cst_inflate_CustomEnumErrorTwinSyncSse_Two(void);

union CustomErrorTwinNormalKind *cst_inflate_CustomErrorTwinNormal_Error0(void);

union CustomErrorTwinNormalKind *cst_inflate_CustomErrorTwinNormal_Error1(void);

union CustomErrorTwinRustAsyncKind *cst_inflate_CustomErrorTwinRustAsync_Error0(void);

union CustomErrorTwinRustAsyncKind *cst_inflate_CustomErrorTwinRustAsync_Error1(void);

union CustomErrorTwinRustAsyncSseKind *cst_inflate_CustomErrorTwinRustAsyncSse_Error0(void);

union CustomErrorTwinRustAsyncSseKind *cst_inflate_CustomErrorTwinRustAsyncSse_Error1(void);

union CustomErrorTwinSseKind *cst_inflate_CustomErrorTwinSse_Error0(void);

union CustomErrorTwinSseKind *cst_inflate_CustomErrorTwinSse_Error1(void);

union CustomErrorTwinSyncKind *cst_inflate_CustomErrorTwinSync_Error0(void);

union CustomErrorTwinSyncKind *cst_inflate_CustomErrorTwinSync_Error1(void);

union CustomErrorTwinSyncSseKind *cst_inflate_CustomErrorTwinSyncSse_Error0(void);

union CustomErrorTwinSyncSseKind *cst_inflate_CustomErrorTwinSyncSse_Error1(void);

union CustomNestedError1TwinNormalKind *cst_inflate_CustomNestedError1TwinNormal_CustomNested1(void);

union CustomNestedError1TwinNormalKind *cst_inflate_CustomNestedError1TwinNormal_ErrorNested(void);

union CustomNestedError1TwinRustAsyncKind *cst_inflate_CustomNestedError1TwinRustAsync_CustomNested1(void);

union CustomNestedError1TwinRustAsyncKind *cst_inflate_CustomNestedError1TwinRustAsync_ErrorNested(void);

union CustomNestedError1TwinRustAsyncSseKind *cst_inflate_CustomNestedError1TwinRustAsyncSse_CustomNested1(void);

union CustomNestedError1TwinRustAsyncSseKind *cst_inflate_CustomNestedError1TwinRustAsyncSse_ErrorNested(void);

union CustomNestedError1TwinSseKind *cst_inflate_CustomNestedError1TwinSse_CustomNested1(void);

union CustomNestedError1TwinSseKind *cst_inflate_CustomNestedError1TwinSse_ErrorNested(void);

union CustomNestedError1TwinSyncKind *cst_inflate_CustomNestedError1TwinSync_CustomNested1(void);

union CustomNestedError1TwinSyncKind *cst_inflate_CustomNestedError1TwinSync_ErrorNested(void);

union CustomNestedError1TwinSyncSseKind *cst_inflate_CustomNestedError1TwinSyncSse_CustomNested1(void);

union CustomNestedError1TwinSyncSseKind *cst_inflate_CustomNestedError1TwinSyncSse_ErrorNested(void);

union CustomNestedError2TwinNormalKind *cst_inflate_CustomNestedError2TwinNormal_CustomNested2(void);

union CustomNestedError2TwinNormalKind *cst_inflate_CustomNestedError2TwinNormal_CustomNested2Number(void);

union CustomNestedError2TwinRustAsyncKind *cst_inflate_CustomNestedError2TwinRustAsync_CustomNested2(void);

union CustomNestedError2TwinRustAsyncKind *cst_inflate_CustomNestedError2TwinRustAsync_CustomNested2Number(void);

union CustomNestedError2TwinRustAsyncSseKind *cst_inflate_CustomNestedError2TwinRustAsyncSse_CustomNested2(void);

union CustomNestedError2TwinRustAsyncSseKind *cst_inflate_CustomNestedError2TwinRustAsyncSse_CustomNested2Number(void);

union CustomNestedError2TwinSseKind *cst_inflate_CustomNestedError2TwinSse_CustomNested2(void);

union CustomNestedError2TwinSseKind *cst_inflate_CustomNestedError2TwinSse_CustomNested2Number(void);

union CustomNestedError2TwinSyncKind *cst_inflate_CustomNestedError2TwinSync_CustomNested2(void);

union CustomNestedError2TwinSyncKind *cst_inflate_CustomNestedError2TwinSync_CustomNested2Number(void);

union CustomNestedError2TwinSyncSseKind *cst_inflate_CustomNestedError2TwinSyncSse_CustomNested2(void);

union CustomNestedError2TwinSyncSseKind *cst_inflate_CustomNestedError2TwinSyncSse_CustomNested2Number(void);

union CustomNestedErrorInnerTwinNormalKind *cst_inflate_CustomNestedErrorInnerTwinNormal_Three(void);

union CustomNestedErrorInnerTwinNormalKind *cst_inflate_CustomNestedErrorInnerTwinNormal_Four(void);

union CustomNestedErrorInnerTwinRustAsyncKind *cst_inflate_CustomNestedErrorInnerTwinRustAsync_Three(void);

union CustomNestedErrorInnerTwinRustAsyncKind *cst_inflate_CustomNestedErrorInnerTwinRustAsync_Four(void);

union CustomNestedErrorInnerTwinRustAsyncSseKind *cst_inflate_CustomNestedErrorInnerTwinRustAsyncSse_Three(void);

union CustomNestedErrorInnerTwinRustAsyncSseKind *cst_inflate_CustomNestedErrorInnerTwinRustAsyncSse_Four(void);

union CustomNestedErrorInnerTwinSseKind *cst_inflate_CustomNestedErrorInnerTwinSse_Three(void);

union CustomNestedErrorInnerTwinSseKind *cst_inflate_CustomNestedErrorInnerTwinSse_Four(void);

union CustomNestedErrorInnerTwinSyncKind *cst_inflate_CustomNestedErrorInnerTwinSync_Three(void);

union CustomNestedErrorInnerTwinSyncKind *cst_inflate_CustomNestedErrorInnerTwinSync_Four(void);

union CustomNestedErrorInnerTwinSyncSseKind *cst_inflate_CustomNestedErrorInnerTwinSyncSse_Three(void);

union CustomNestedErrorInnerTwinSyncSseKind *cst_inflate_CustomNestedErrorInnerTwinSyncSse_Four(void);

union CustomNestedErrorOuterTwinNormalKind *cst_inflate_CustomNestedErrorOuterTwinNormal_One(void);

union CustomNestedErrorOuterTwinNormalKind *cst_inflate_CustomNestedErrorOuterTwinNormal_Two(void);

union CustomNestedErrorOuterTwinRustAsyncKind *cst_inflate_CustomNestedErrorOuterTwinRustAsync_One(void);

union CustomNestedErrorOuterTwinRustAsyncKind *cst_inflate_CustomNestedErrorOuterTwinRustAsync_Two(void);

union CustomNestedErrorOuterTwinRustAsyncSseKind *cst_inflate_CustomNestedErrorOuterTwinRustAsyncSse_One(void);

union CustomNestedErrorOuterTwinRustAsyncSseKind *cst_inflate_CustomNestedErrorOuterTwinRustAsyncSse_Two(void);

union CustomNestedErrorOuterTwinSseKind *cst_inflate_CustomNestedErrorOuterTwinSse_One(void);

union CustomNestedErrorOuterTwinSseKind *cst_inflate_CustomNestedErrorOuterTwinSse_Two(void);

union CustomNestedErrorOuterTwinSyncKind *cst_inflate_CustomNestedErrorOuterTwinSync_One(void);

union CustomNestedErrorOuterTwinSyncKind *cst_inflate_CustomNestedErrorOuterTwinSync_Two(void);

union CustomNestedErrorOuterTwinSyncSseKind *cst_inflate_CustomNestedErrorOuterTwinSyncSse_One(void);

union CustomNestedErrorOuterTwinSyncSseKind *cst_inflate_CustomNestedErrorOuterTwinSyncSse_Two(void);

union DistanceTwinNormalKind *cst_inflate_DistanceTwinNormal_Map(void);

union DistanceTwinRustAsyncKind *cst_inflate_DistanceTwinRustAsync_Map(void);

union DistanceTwinRustAsyncSseKind *cst_inflate_DistanceTwinRustAsyncSse_Map(void);

union DistanceTwinSseKind *cst_inflate_DistanceTwinSse_Map(void);

union DistanceTwinSyncKind *cst_inflate_DistanceTwinSync_Map(void);

union DistanceTwinSyncSseKind *cst_inflate_DistanceTwinSyncSse_Map(void);

union EnumDartOpaqueTwinNormalKind *cst_inflate_EnumDartOpaqueTwinNormal_Primitive(void);

union EnumDartOpaqueTwinNormalKind *cst_inflate_EnumDartOpaqueTwinNormal_Opaque(void);

union EnumDartOpaqueTwinRustAsyncKind *cst_inflate_EnumDartOpaqueTwinRustAsync_Primitive(void);

union EnumDartOpaqueTwinRustAsyncKind *cst_inflate_EnumDartOpaqueTwinRustAsync_Opaque(void);

union EnumDartOpaqueTwinRustAsyncSseKind *cst_inflate_EnumDartOpaqueTwinRustAsyncSse_Primitive(void);

union EnumDartOpaqueTwinRustAsyncSseKind *cst_inflate_EnumDartOpaqueTwinRustAsyncSse_Opaque(void);

union EnumDartOpaqueTwinSseKind *cst_inflate_EnumDartOpaqueTwinSse_Primitive(void);

union EnumDartOpaqueTwinSseKind *cst_inflate_EnumDartOpaqueTwinSse_Opaque(void);

union EnumDartOpaqueTwinSyncKind *cst_inflate_EnumDartOpaqueTwinSync_Primitive(void);

union EnumDartOpaqueTwinSyncKind *cst_inflate_EnumDartOpaqueTwinSync_Opaque(void);

union EnumDartOpaqueTwinSyncSseKind *cst_inflate_EnumDartOpaqueTwinSyncSse_Primitive(void);

union EnumDartOpaqueTwinSyncSseKind *cst_inflate_EnumDartOpaqueTwinSyncSse_Opaque(void);

union EnumOpaqueTwinNormalKind *cst_inflate_EnumOpaqueTwinNormal_Struct(void);

union EnumOpaqueTwinNormalKind *cst_inflate_EnumOpaqueTwinNormal_Primitive(void);

union EnumOpaqueTwinNormalKind *cst_inflate_EnumOpaqueTwinNormal_TraitObj(void);

union EnumOpaqueTwinNormalKind *cst_inflate_EnumOpaqueTwinNormal_Mutex(void);

union EnumOpaqueTwinNormalKind *cst_inflate_EnumOpaqueTwinNormal_RwLock(void);

union EnumOpaqueTwinRustAsyncKind *cst_inflate_EnumOpaqueTwinRustAsync_Struct(void);

union EnumOpaqueTwinRustAsyncKind *cst_inflate_EnumOpaqueTwinRustAsync_Primitive(void);

union EnumOpaqueTwinRustAsyncKind *cst_inflate_EnumOpaqueTwinRustAsync_TraitObj(void);

union EnumOpaqueTwinRustAsyncKind *cst_inflate_EnumOpaqueTwinRustAsync_Mutex(void);

union EnumOpaqueTwinRustAsyncKind *cst_inflate_EnumOpaqueTwinRustAsync_RwLock(void);

union EnumOpaqueTwinRustAsyncSseKind *cst_inflate_EnumOpaqueTwinRustAsyncSse_Struct(void);

union EnumOpaqueTwinRustAsyncSseKind *cst_inflate_EnumOpaqueTwinRustAsyncSse_Primitive(void);

union EnumOpaqueTwinRustAsyncSseKind *cst_inflate_EnumOpaqueTwinRustAsyncSse_TraitObj(void);

union EnumOpaqueTwinRustAsyncSseKind *cst_inflate_EnumOpaqueTwinRustAsyncSse_Mutex(void);

union EnumOpaqueTwinRustAsyncSseKind *cst_inflate_EnumOpaqueTwinRustAsyncSse_RwLock(void);

union EnumOpaqueTwinSseKind *cst_inflate_EnumOpaqueTwinSse_Struct(void);

union EnumOpaqueTwinSseKind *cst_inflate_EnumOpaqueTwinSse_Primitive(void);

union EnumOpaqueTwinSseKind *cst_inflate_EnumOpaqueTwinSse_TraitObj(void);

union EnumOpaqueTwinSseKind *cst_inflate_EnumOpaqueTwinSse_Mutex(void);

union EnumOpaqueTwinSseKind *cst_inflate_EnumOpaqueTwinSse_RwLock(void);

union EnumOpaqueTwinSyncKind *cst_inflate_EnumOpaqueTwinSync_Struct(void);

union EnumOpaqueTwinSyncKind *cst_inflate_EnumOpaqueTwinSync_Primitive(void);

union EnumOpaqueTwinSyncKind *cst_inflate_EnumOpaqueTwinSync_TraitObj(void);

union EnumOpaqueTwinSyncKind *cst_inflate_EnumOpaqueTwinSync_Mutex(void);

union EnumOpaqueTwinSyncKind *cst_inflate_EnumOpaqueTwinSync_RwLock(void);

union EnumOpaqueTwinSyncSseKind *cst_inflate_EnumOpaqueTwinSyncSse_Struct(void);

union EnumOpaqueTwinSyncSseKind *cst_inflate_EnumOpaqueTwinSyncSse_Primitive(void);

union EnumOpaqueTwinSyncSseKind *cst_inflate_EnumOpaqueTwinSyncSse_TraitObj(void);

union EnumOpaqueTwinSyncSseKind *cst_inflate_EnumOpaqueTwinSyncSse_Mutex(void);

union EnumOpaqueTwinSyncSseKind *cst_inflate_EnumOpaqueTwinSyncSse_RwLock(void);

union EnumWithItemMixedTwinNormalKind *cst_inflate_EnumWithItemMixedTwinNormal_B(void);

union EnumWithItemMixedTwinNormalKind *cst_inflate_EnumWithItemMixedTwinNormal_C(void);

union EnumWithItemMixedTwinRustAsyncKind *cst_inflate_EnumWithItemMixedTwinRustAsync_B(void);

union EnumWithItemMixedTwinRustAsyncKind *cst_inflate_EnumWithItemMixedTwinRustAsync_C(void);

union EnumWithItemMixedTwinRustAsyncSseKind *cst_inflate_EnumWithItemMixedTwinRustAsyncSse_B(void);

union EnumWithItemMixedTwinRustAsyncSseKind *cst_inflate_EnumWithItemMixedTwinRustAsyncSse_C(void);

union EnumWithItemMixedTwinSseKind *cst_inflate_EnumWithItemMixedTwinSse_B(void);

union EnumWithItemMixedTwinSseKind *cst_inflate_EnumWithItemMixedTwinSse_C(void);

union EnumWithItemMixedTwinSyncKind *cst_inflate_EnumWithItemMixedTwinSync_B(void);

union EnumWithItemMixedTwinSyncKind *cst_inflate_EnumWithItemMixedTwinSync_C(void);

union EnumWithItemMixedTwinSyncSseKind *cst_inflate_EnumWithItemMixedTwinSyncSse_B(void);

union EnumWithItemMixedTwinSyncSseKind *cst_inflate_EnumWithItemMixedTwinSyncSse_C(void);

union EnumWithItemStructTwinNormalKind *cst_inflate_EnumWithItemStructTwinNormal_A(void);

union EnumWithItemStructTwinNormalKind *cst_inflate_EnumWithItemStructTwinNormal_B(void);

union EnumWithItemStructTwinRustAsyncKind *cst_inflate_EnumWithItemStructTwinRustAsync_A(void);

union EnumWithItemStructTwinRustAsyncKind *cst_inflate_EnumWithItemStructTwinRustAsync_B(void);

union EnumWithItemStructTwinRustAsyncSseKind *cst_inflate_EnumWithItemStructTwinRustAsyncSse_A(void);

union EnumWithItemStructTwinRustAsyncSseKind *cst_inflate_EnumWithItemStructTwinRustAsyncSse_B(void);

union EnumWithItemStructTwinSseKind *cst_inflate_EnumWithItemStructTwinSse_A(void);

union EnumWithItemStructTwinSseKind *cst_inflate_EnumWithItemStructTwinSse_B(void);

union EnumWithItemStructTwinSyncKind *cst_inflate_EnumWithItemStructTwinSync_A(void);

union EnumWithItemStructTwinSyncKind *cst_inflate_EnumWithItemStructTwinSync_B(void);

union EnumWithItemStructTwinSyncSseKind *cst_inflate_EnumWithItemStructTwinSyncSse_A(void);

union EnumWithItemStructTwinSyncSseKind *cst_inflate_EnumWithItemStructTwinSyncSse_B(void);

union EnumWithItemTupleTwinNormalKind *cst_inflate_EnumWithItemTupleTwinNormal_A(void);

union EnumWithItemTupleTwinNormalKind *cst_inflate_EnumWithItemTupleTwinNormal_B(void);

union EnumWithItemTupleTwinRustAsyncKind *cst_inflate_EnumWithItemTupleTwinRustAsync_A(void);

union EnumWithItemTupleTwinRustAsyncKind *cst_inflate_EnumWithItemTupleTwinRustAsync_B(void);

union EnumWithItemTupleTwinRustAsyncSseKind *cst_inflate_EnumWithItemTupleTwinRustAsyncSse_A(void);

union EnumWithItemTupleTwinRustAsyncSseKind *cst_inflate_EnumWithItemTupleTwinRustAsyncSse_B(void);

union EnumWithItemTupleTwinSseKind *cst_inflate_EnumWithItemTupleTwinSse_A(void);

union EnumWithItemTupleTwinSseKind *cst_inflate_EnumWithItemTupleTwinSse_B(void);

union EnumWithItemTupleTwinSyncKind *cst_inflate_EnumWithItemTupleTwinSync_A(void);

union EnumWithItemTupleTwinSyncKind *cst_inflate_EnumWithItemTupleTwinSync_B(void);

union EnumWithItemTupleTwinSyncSseKind *cst_inflate_EnumWithItemTupleTwinSyncSse_A(void);

union EnumWithItemTupleTwinSyncSseKind *cst_inflate_EnumWithItemTupleTwinSyncSse_B(void);

union KitchenSinkTwinNormalKind *cst_inflate_KitchenSinkTwinNormal_Primitives(void);

union KitchenSinkTwinNormalKind *cst_inflate_KitchenSinkTwinNormal_Nested(void);

union KitchenSinkTwinNormalKind *cst_inflate_KitchenSinkTwinNormal_Optional(void);

union KitchenSinkTwinNormalKind *cst_inflate_KitchenSinkTwinNormal_Buffer(void);

union KitchenSinkTwinNormalKind *cst_inflate_KitchenSinkTwinNormal_Enums(void);

union KitchenSinkTwinRustAsyncKind *cst_inflate_KitchenSinkTwinRustAsync_Primitives(void);

union KitchenSinkTwinRustAsyncKind *cst_inflate_KitchenSinkTwinRustAsync_Nested(void);

union KitchenSinkTwinRustAsyncKind *cst_inflate_KitchenSinkTwinRustAsync_Optional(void);

union KitchenSinkTwinRustAsyncKind *cst_inflate_KitchenSinkTwinRustAsync_Buffer(void);

union KitchenSinkTwinRustAsyncKind *cst_inflate_KitchenSinkTwinRustAsync_Enums(void);

union KitchenSinkTwinRustAsyncSseKind *cst_inflate_KitchenSinkTwinRustAsyncSse_Primitives(void);

union KitchenSinkTwinRustAsyncSseKind *cst_inflate_KitchenSinkTwinRustAsyncSse_Nested(void);

union KitchenSinkTwinRustAsyncSseKind *cst_inflate_KitchenSinkTwinRustAsyncSse_Optional(void);

union KitchenSinkTwinRustAsyncSseKind *cst_inflate_KitchenSinkTwinRustAsyncSse_Buffer(void);

union KitchenSinkTwinRustAsyncSseKind *cst_inflate_KitchenSinkTwinRustAsyncSse_Enums(void);

union KitchenSinkTwinSseKind *cst_inflate_KitchenSinkTwinSse_Primitives(void);

union KitchenSinkTwinSseKind *cst_inflate_KitchenSinkTwinSse_Nested(void);

union KitchenSinkTwinSseKind *cst_inflate_KitchenSinkTwinSse_Optional(void);

union KitchenSinkTwinSseKind *cst_inflate_KitchenSinkTwinSse_Buffer(void);

union KitchenSinkTwinSseKind *cst_inflate_KitchenSinkTwinSse_Enums(void);

union KitchenSinkTwinSyncKind *cst_inflate_KitchenSinkTwinSync_Primitives(void);

union KitchenSinkTwinSyncKind *cst_inflate_KitchenSinkTwinSync_Nested(void);

union KitchenSinkTwinSyncKind *cst_inflate_KitchenSinkTwinSync_Optional(void);

union KitchenSinkTwinSyncKind *cst_inflate_KitchenSinkTwinSync_Buffer(void);

union KitchenSinkTwinSyncKind *cst_inflate_KitchenSinkTwinSync_Enums(void);

union KitchenSinkTwinSyncSseKind *cst_inflate_KitchenSinkTwinSyncSse_Primitives(void);

union KitchenSinkTwinSyncSseKind *cst_inflate_KitchenSinkTwinSyncSse_Nested(void);

union KitchenSinkTwinSyncSseKind *cst_inflate_KitchenSinkTwinSyncSse_Optional(void);

union KitchenSinkTwinSyncSseKind *cst_inflate_KitchenSinkTwinSyncSse_Buffer(void);

union KitchenSinkTwinSyncSseKind *cst_inflate_KitchenSinkTwinSyncSse_Enums(void);

union MeasureTwinNormalKind *cst_inflate_MeasureTwinNormal_Speed(void);

union MeasureTwinNormalKind *cst_inflate_MeasureTwinNormal_Distance(void);

union MeasureTwinRustAsyncKind *cst_inflate_MeasureTwinRustAsync_Speed(void);

union MeasureTwinRustAsyncKind *cst_inflate_MeasureTwinRustAsync_Distance(void);

union MeasureTwinRustAsyncSseKind *cst_inflate_MeasureTwinRustAsyncSse_Speed(void);

union MeasureTwinRustAsyncSseKind *cst_inflate_MeasureTwinRustAsyncSse_Distance(void);

union MeasureTwinSseKind *cst_inflate_MeasureTwinSse_Speed(void);

union MeasureTwinSseKind *cst_inflate_MeasureTwinSse_Distance(void);

union MeasureTwinSyncKind *cst_inflate_MeasureTwinSync_Speed(void);

union MeasureTwinSyncKind *cst_inflate_MeasureTwinSync_Distance(void);

union MeasureTwinSyncSseKind *cst_inflate_MeasureTwinSyncSse_Speed(void);

union MeasureTwinSyncSseKind *cst_inflate_MeasureTwinSyncSse_Distance(void);

union RawStringEnumMirroredKind *cst_inflate_RawStringEnumMirrored_Raw(void);

union RawStringEnumMirroredKind *cst_inflate_RawStringEnumMirrored_Nested(void);

union RawStringEnumMirroredKind *cst_inflate_RawStringEnumMirrored_ListOfNested(void);

union SpeedTwinNormalKind *cst_inflate_SpeedTwinNormal_GPS(void);

union SpeedTwinRustAsyncKind *cst_inflate_SpeedTwinRustAsync_GPS(void);

union SpeedTwinRustAsyncSseKind *cst_inflate_SpeedTwinRustAsyncSse_GPS(void);

union SpeedTwinSseKind *cst_inflate_SpeedTwinSse_GPS(void);

union SpeedTwinSyncKind *cst_inflate_SpeedTwinSync_GPS(void);

union SpeedTwinSyncSseKind *cst_inflate_SpeedTwinSyncSse_GPS(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinNormal_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsyncSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsyncSse_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsync_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinRustAsync_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSse_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSyncSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSyncSse_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSync_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AbcTwinSync_JustInt);
    dummy_var ^= ((int64_t) (void*) cst_inflate_ApplicationMessage_DisplayMessage);
    dummy_var ^= ((int64_t) (void*) cst_inflate_ApplicationMessage_RenderPixel);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinNormal_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinNormal_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinRustAsyncSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinRustAsyncSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinRustAsync_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinRustAsync_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSyncSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSyncSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSync_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomEnumErrorTwinSync_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinNormal_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinNormal_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinRustAsyncSse_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinRustAsyncSse_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinRustAsync_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinRustAsync_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSse_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSse_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSyncSse_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSyncSse_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSync_Error0);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomErrorTwinSync_Error1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinNormal_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinNormal_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinRustAsyncSse_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinRustAsyncSse_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinRustAsync_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinRustAsync_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSse_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSse_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSyncSse_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSyncSse_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSync_CustomNested1);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError1TwinSync_ErrorNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinNormal_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinNormal_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinRustAsyncSse_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinRustAsyncSse_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinRustAsync_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinRustAsync_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSse_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSse_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSyncSse_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSyncSse_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSync_CustomNested2);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedError2TwinSync_CustomNested2Number);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinNormal_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinNormal_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinRustAsyncSse_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinRustAsyncSse_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinRustAsync_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinRustAsync_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSse_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSse_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSyncSse_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSyncSse_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSync_Four);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorInnerTwinSync_Three);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinNormal_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinNormal_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinRustAsyncSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinRustAsyncSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinRustAsync_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinRustAsync_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSyncSse_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSyncSse_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSync_One);
    dummy_var ^= ((int64_t) (void*) cst_inflate_CustomNestedErrorOuterTwinSync_Two);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinNormal_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinRustAsyncSse_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinRustAsync_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinSse_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinSyncSse_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DistanceTwinSync_Map);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinNormal_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinRustAsyncSse_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinRustAsyncSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinRustAsync_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinRustAsync_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSse_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSyncSse_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSyncSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSync_Opaque);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumDartOpaqueTwinSync_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinNormal_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinNormal_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinNormal_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinNormal_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsyncSse_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsyncSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsyncSse_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsyncSse_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsyncSse_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsync_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsync_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsync_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsync_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinRustAsync_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSse_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSse_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSse_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSse_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSyncSse_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSyncSse_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSyncSse_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSyncSse_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSyncSse_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSync_Mutex);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSync_Primitive);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSync_RwLock);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSync_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumOpaqueTwinSync_TraitObj);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinRustAsyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinRustAsyncSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinRustAsync_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSyncSse_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemMixedTwinSync_C);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinRustAsyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinRustAsyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemStructTwinSync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinRustAsyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinRustAsyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSyncSse_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSyncSse_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSync_A);
    dummy_var ^= ((int64_t) (void*) cst_inflate_EnumWithItemTupleTwinSync_B);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinNormal_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinNormal_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinNormal_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinNormal_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinNormal_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsyncSse_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsyncSse_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsyncSse_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsyncSse_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsyncSse_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsync_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsync_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsync_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsync_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinRustAsync_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSse_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSse_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSse_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSse_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSse_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSyncSse_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSyncSse_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSyncSse_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSyncSse_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSyncSse_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSync_Buffer);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSync_Enums);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSync_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSync_Optional);
    dummy_var ^= ((int64_t) (void*) cst_inflate_KitchenSinkTwinSync_Primitives);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinNormal_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinNormal_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinRustAsyncSse_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinRustAsyncSse_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinRustAsync_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinRustAsync_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSse_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSse_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSyncSse_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSyncSse_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSync_Distance);
    dummy_var ^= ((int64_t) (void*) cst_inflate_MeasureTwinSync_Speed);
    dummy_var ^= ((int64_t) (void*) cst_inflate_RawStringEnumMirrored_ListOfNested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_RawStringEnumMirrored_Nested);
    dummy_var ^= ((int64_t) (void*) cst_inflate_RawStringEnumMirrored_Raw);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinNormal_GPS);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinRustAsyncSse_GPS);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinRustAsync_GPS);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinSse_GPS);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinSyncSse_GPS);
    dummy_var ^= ((int64_t) (void*) cst_inflate_SpeedTwinSync_GPS);
    dummy_var ^= ((int64_t) (void*) cst_new_box_application_env);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_DartOpaque);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_a_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_abc_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_application_env);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_application_settings);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_attribute_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_b_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_c_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_concatenate_with_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_2_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_inner_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_nested_error_outer_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_custom_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_customized_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_dart_opaque_nested_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_element_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_empty_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_mixed_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_enum_with_item_tuple_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_event_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_event_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_event_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_exotic_optionals_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_f_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_uuid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feature_uuid_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_feed_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_16);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_8);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_kitchen_sink_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_list_of_nested_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_macro_struct);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_measure_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_message_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_nested_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_size);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_struct);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_my_tree_node_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_nested_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_new_type_int_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_note_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_numbers);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opaque_nested_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_opt_vecs_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sequences);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_some_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_comments_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_one_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_two_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_struct_with_zero_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sum_with_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_test_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_one_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_tuple_struct_with_two_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_16);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_8);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_user_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_weekdays_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_blob_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_distance_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_exotic_optionals_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_feed_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_i_8);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_kitchen_sink_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_my_size);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_speed_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_u_8);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_box_weekdays_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_list_Chrono_Local);
    dummy_var ^= ((int64_t) (void*) cst_new_list_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_list_DartOpaque);
    dummy_var ^= ((int64_t) (void*) cst_new_list_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_application_env_var);
    dummy_var ^= ((int64_t) (void*) cst_new_list_application_settings);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_attribute_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_element_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_enum_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_enum);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_size);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_my_tree_node_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_nested_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_attribute_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_weekdays_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_point_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_f_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_i_16);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_i_8);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_16);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) cst_new_list_raw_string_enum_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_list_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) cst_new_list_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_sum_with_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_test_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_sse);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) cst_new_list_weekdays_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) dart_opaque_dart2rust_encode);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSseSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSseSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSseSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSseSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSse);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSyncSse);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_concatenate_static_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_concatenate_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_handle_some_static_stream_sink_single_arg_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_handle_some_static_stream_sink_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_handle_some_stream_sink_at_1_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_handle_some_stream_sink_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsyncSse_new_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_concatenate_static_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_concatenate_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_single_arg_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_at_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_new_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_concatenate_static_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_concatenate_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_handle_some_static_stream_sink_single_arg_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_handle_some_static_stream_sink_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_handle_some_stream_sink_at_1_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_handle_some_stream_sink_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSse_new_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_concatenate_static_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_concatenate_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_handle_some_static_stream_sink_single_arg_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_handle_some_static_stream_sink_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_handle_some_stream_sink_at_1_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_handle_some_stream_sink_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSyncSse_new_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_concatenate_static_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_concatenate_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsyncSse_new_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsyncSse_nonstatic_return_custom_struct_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsyncSse_nonstatic_return_custom_struct_ok_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsyncSse_static_return_custom_struct_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsyncSse_static_return_custom_struct_ok_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_new_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_static_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_static_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSse_new_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSse_nonstatic_return_custom_struct_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSse_nonstatic_return_custom_struct_ok_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSse_static_return_custom_struct_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSse_static_return_custom_struct_ok_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSyncSse_new_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSyncSse_nonstatic_return_custom_struct_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSyncSse_nonstatic_return_custom_struct_ok_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSyncSse_static_return_custom_struct_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSyncSse_static_return_custom_struct_ok_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinNormal_as_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinRustAsyncSse_as_string_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinRustAsync_as_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinSse_as_string_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_instance_method_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_instance_method_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_instance_method_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_instance_method_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_new_custom_name_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_static_method_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_static_method_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_static_method_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinNormal_static_method_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_instance_method_arg_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_instance_method_arg_mut_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_instance_method_arg_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_instance_method_return_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_new_custom_name_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_new_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_static_method_arg_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_static_method_arg_mut_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_static_method_arg_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSse_static_method_return_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_instance_method_arg_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_instance_method_arg_mut_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_instance_method_arg_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_instance_method_return_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_new_custom_name_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_new_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_static_method_arg_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_static_method_arg_mut_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_static_method_arg_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSyncSse_static_method_return_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_instance_method_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_instance_method_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_instance_method_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_instance_method_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_new_custom_name_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_static_method_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_static_method_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_static_method_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_NonCloneSimpleTwinSync_static_method_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsyncSse_new_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsyncSse_non_static_return_err_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsyncSse_non_static_return_ok_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsyncSse_static_return_err_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsyncSse_static_return_ok_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_new_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_non_static_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_non_static_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_static_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_static_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSse_new_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSse_non_static_return_err_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSse_non_static_return_ok_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSse_static_return_err_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSse_static_return_ok_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSyncSse_new_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSyncSse_non_static_return_err_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSyncSse_non_static_return_ok_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSyncSse_static_return_err_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSyncSse_static_return_ok_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsyncSse_instance_method_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsyncSse_static_method_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsync_instance_method_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsync_static_method_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSse_instance_method_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSse_static_method_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSyncSse_instance_method_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSyncSse_static_method_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_instance_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_static_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinNormal_sum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinRustAsyncSse_sum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinRustAsync_sum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinSse_sum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinSyncSse_sum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinSync_sum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_another_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_clone_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_async_simple_add_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_async_simple_add_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_async_void_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_async_void_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_realistic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_realistic_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_loopback);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_multi_times);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_one_arg);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_return);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_simple);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_two_args);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_with_dart_opaque_arg);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_with_dart_opaque_result);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_rust_async_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_sse);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_sync_sse);
    return dummy_var;
}
