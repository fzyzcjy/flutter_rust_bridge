#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturn;
typedef struct _Dart_Handle* Dart_Handle;

typedef struct benchmark_raw_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} benchmark_raw_list_prim_u_8;

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

typedef struct wire_test_id_twin_rust_async {
  struct wire_list_prim_i_32 *field0;
} wire_test_id_twin_rust_async;

typedef struct wire_list_test_id_twin_rust_async {
  struct wire_test_id_twin_rust_async *ptr;
  int32_t len;
} wire_list_test_id_twin_rust_async;

typedef struct wire_feed_id_twin_rust_async {
  struct wire_list_prim_u_8 *field0;
} wire_feed_id_twin_rust_async;

typedef struct wire_blob_twin_rust_async {
  struct wire_list_prim_u_8 *field0;
} wire_blob_twin_rust_async;

typedef struct wire_message_id_twin_rust_async {
  struct wire_list_prim_u_8 *field0;
} wire_message_id_twin_rust_async;

typedef struct wire_test_id_twin_sync {
  struct wire_list_prim_i_32 *field0;
} wire_test_id_twin_sync;

typedef struct wire_list_test_id_twin_sync {
  struct wire_test_id_twin_sync *ptr;
  int32_t len;
} wire_list_test_id_twin_sync;

typedef struct wire_feed_id_twin_sync {
  struct wire_list_prim_u_8 *field0;
} wire_feed_id_twin_sync;

typedef struct wire_blob_twin_sync {
  struct wire_list_prim_u_8 *field0;
} wire_blob_twin_sync;

typedef struct wire_message_id_twin_sync {
  struct wire_list_prim_u_8 *field0;
} wire_message_id_twin_sync;

typedef struct wire_customized_twin_rust_async {
  struct wire_list_prim_u_8 *final_field;
  struct wire_list_prim_u_8 *non_final_field;
} wire_customized_twin_rust_async;

typedef struct wire_user_id_twin_rust_async {
  uint32_t value;
} wire_user_id_twin_rust_async;

typedef struct wire_customized_twin_sync {
  struct wire_list_prim_u_8 *final_field;
  struct wire_list_prim_u_8 *non_final_field;
} wire_customized_twin_sync;

typedef struct wire_user_id_twin_sync {
  uint32_t value;
} wire_user_id_twin_sync;

typedef struct wire_feature_chrono_twin_rust_async {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_feature_chrono_twin_rust_async;

typedef struct wire_feature_chrono_twin_sync {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_feature_chrono_twin_sync;

typedef struct wire_struct_with_comments_twin_rust_async {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_rust_async;

typedef struct wire_struct_with_comments_twin_sync {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_sync;

typedef struct wire_EnumDartOpaqueTwinRustAsync_Primitive {
  int32_t field0;
} wire_EnumDartOpaqueTwinRustAsync_Primitive;

typedef struct wire_EnumDartOpaqueTwinRustAsync_Opaque {
  struct wire_DartOpaque field0;
} wire_EnumDartOpaqueTwinRustAsync_Opaque;

typedef union EnumDartOpaqueTwinRustAsyncKind {
  struct wire_EnumDartOpaqueTwinRustAsync_Primitive *Primitive;
  struct wire_EnumDartOpaqueTwinRustAsync_Opaque *Opaque;
} EnumDartOpaqueTwinRustAsyncKind;

typedef struct wire_enum_dart_opaque_twin_rust_async {
  int32_t tag;
  union EnumDartOpaqueTwinRustAsyncKind *kind;
} wire_enum_dart_opaque_twin_rust_async;

typedef struct wire_dart_opaque_nested_twin_rust_async {
  struct wire_DartOpaque first;
  struct wire_DartOpaque second;
} wire_dart_opaque_nested_twin_rust_async;

typedef struct wire_EnumDartOpaqueTwinSync_Primitive {
  int32_t field0;
} wire_EnumDartOpaqueTwinSync_Primitive;

typedef struct wire_EnumDartOpaqueTwinSync_Opaque {
  struct wire_DartOpaque field0;
} wire_EnumDartOpaqueTwinSync_Opaque;

typedef union EnumDartOpaqueTwinSyncKind {
  struct wire_EnumDartOpaqueTwinSync_Primitive *Primitive;
  struct wire_EnumDartOpaqueTwinSync_Opaque *Opaque;
} EnumDartOpaqueTwinSyncKind;

typedef struct wire_enum_dart_opaque_twin_sync {
  int32_t tag;
  union EnumDartOpaqueTwinSyncKind *kind;
} wire_enum_dart_opaque_twin_sync;

typedef struct wire_dart_opaque_nested_twin_sync {
  struct wire_DartOpaque first;
  struct wire_DartOpaque second;
} wire_dart_opaque_nested_twin_sync;

typedef struct wire_EnumWithItemMixedTwinRustAsync_A {

} wire_EnumWithItemMixedTwinRustAsync_A;

typedef struct wire_EnumWithItemMixedTwinRustAsync_B {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemMixedTwinRustAsync_B;

typedef struct wire_EnumWithItemMixedTwinRustAsync_C {
  struct wire_list_prim_u_8 *c_field;
} wire_EnumWithItemMixedTwinRustAsync_C;

typedef union EnumWithItemMixedTwinRustAsyncKind {
  struct wire_EnumWithItemMixedTwinRustAsync_A *A;
  struct wire_EnumWithItemMixedTwinRustAsync_B *B;
  struct wire_EnumWithItemMixedTwinRustAsync_C *C;
} EnumWithItemMixedTwinRustAsyncKind;

typedef struct wire_enum_with_item_mixed_twin_rust_async {
  int32_t tag;
  union EnumWithItemMixedTwinRustAsyncKind *kind;
} wire_enum_with_item_mixed_twin_rust_async;

typedef struct wire_EnumWithItemStructTwinRustAsync_A {
  struct wire_list_prim_u_8 *a_field;
} wire_EnumWithItemStructTwinRustAsync_A;

typedef struct wire_EnumWithItemStructTwinRustAsync_B {
  struct wire_list_prim_i_32 *b_field;
} wire_EnumWithItemStructTwinRustAsync_B;

typedef union EnumWithItemStructTwinRustAsyncKind {
  struct wire_EnumWithItemStructTwinRustAsync_A *A;
  struct wire_EnumWithItemStructTwinRustAsync_B *B;
} EnumWithItemStructTwinRustAsyncKind;

typedef struct wire_enum_with_item_struct_twin_rust_async {
  int32_t tag;
  union EnumWithItemStructTwinRustAsyncKind *kind;
} wire_enum_with_item_struct_twin_rust_async;

typedef struct wire_EnumWithItemTupleTwinRustAsync_A {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemTupleTwinRustAsync_A;

typedef struct wire_EnumWithItemTupleTwinRustAsync_B {
  struct wire_list_prim_i_32 *field0;
} wire_EnumWithItemTupleTwinRustAsync_B;

typedef union EnumWithItemTupleTwinRustAsyncKind {
  struct wire_EnumWithItemTupleTwinRustAsync_A *A;
  struct wire_EnumWithItemTupleTwinRustAsync_B *B;
} EnumWithItemTupleTwinRustAsyncKind;

typedef struct wire_enum_with_item_tuple_twin_rust_async {
  int32_t tag;
  union EnumWithItemTupleTwinRustAsyncKind *kind;
} wire_enum_with_item_tuple_twin_rust_async;

typedef struct wire_KitchenSinkTwinRustAsync_Empty {

} wire_KitchenSinkTwinRustAsync_Empty;

typedef struct wire_KitchenSinkTwinRustAsync_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_KitchenSinkTwinRustAsync_Primitives;

typedef struct wire_KitchenSinkTwinRustAsync_Nested {
  int32_t field0;
  struct wire_kitchen_sink_twin_rust_async *field1;
} wire_KitchenSinkTwinRustAsync_Nested;

typedef struct wire_KitchenSinkTwinRustAsync_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_KitchenSinkTwinRustAsync_Optional;

typedef struct wire_KitchenSinkTwinRustAsync_Buffer {
  struct wire_list_prim_u_8 *field0;
} wire_KitchenSinkTwinRustAsync_Buffer;

typedef struct wire_KitchenSinkTwinRustAsync_Enums {
  int32_t field0;
} wire_KitchenSinkTwinRustAsync_Enums;

typedef union KitchenSinkTwinRustAsyncKind {
  struct wire_KitchenSinkTwinRustAsync_Empty *Empty;
  struct wire_KitchenSinkTwinRustAsync_Primitives *Primitives;
  struct wire_KitchenSinkTwinRustAsync_Nested *Nested;
  struct wire_KitchenSinkTwinRustAsync_Optional *Optional;
  struct wire_KitchenSinkTwinRustAsync_Buffer *Buffer;
  struct wire_KitchenSinkTwinRustAsync_Enums *Enums;
} KitchenSinkTwinRustAsyncKind;

typedef struct wire_kitchen_sink_twin_rust_async {
  int32_t tag;
  union KitchenSinkTwinRustAsyncKind *kind;
} wire_kitchen_sink_twin_rust_async;

typedef struct wire_SpeedTwinRustAsync_Unknown {

} wire_SpeedTwinRustAsync_Unknown;

typedef struct wire_SpeedTwinRustAsync_GPS {
  double field0;
} wire_SpeedTwinRustAsync_GPS;

typedef union SpeedTwinRustAsyncKind {
  struct wire_SpeedTwinRustAsync_Unknown *Unknown;
  struct wire_SpeedTwinRustAsync_GPS *GPS;
} SpeedTwinRustAsyncKind;

typedef struct wire_speed_twin_rust_async {
  int32_t tag;
  union SpeedTwinRustAsyncKind *kind;
} wire_speed_twin_rust_async;

typedef struct wire_MeasureTwinRustAsync_Speed {
  struct wire_speed_twin_rust_async *field0;
} wire_MeasureTwinRustAsync_Speed;

typedef struct wire_DistanceTwinRustAsync_Unknown {

} wire_DistanceTwinRustAsync_Unknown;

typedef struct wire_DistanceTwinRustAsync_Map {
  double field0;
} wire_DistanceTwinRustAsync_Map;

typedef union DistanceTwinRustAsyncKind {
  struct wire_DistanceTwinRustAsync_Unknown *Unknown;
  struct wire_DistanceTwinRustAsync_Map *Map;
} DistanceTwinRustAsyncKind;

typedef struct wire_distance_twin_rust_async {
  int32_t tag;
  union DistanceTwinRustAsyncKind *kind;
} wire_distance_twin_rust_async;

typedef struct wire_MeasureTwinRustAsync_Distance {
  struct wire_distance_twin_rust_async *field0;
} wire_MeasureTwinRustAsync_Distance;

typedef union MeasureTwinRustAsyncKind {
  struct wire_MeasureTwinRustAsync_Speed *Speed;
  struct wire_MeasureTwinRustAsync_Distance *Distance;
} MeasureTwinRustAsyncKind;

typedef struct wire_measure_twin_rust_async {
  int32_t tag;
  union MeasureTwinRustAsyncKind *kind;
} wire_measure_twin_rust_async;

typedef struct wire_note_twin_rust_async {
  int32_t *day;
  struct wire_list_prim_u_8 *body;
} wire_note_twin_rust_async;

typedef struct wire_EnumWithItemMixedTwinSync_A {

} wire_EnumWithItemMixedTwinSync_A;

typedef struct wire_EnumWithItemMixedTwinSync_B {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemMixedTwinSync_B;

typedef struct wire_EnumWithItemMixedTwinSync_C {
  struct wire_list_prim_u_8 *c_field;
} wire_EnumWithItemMixedTwinSync_C;

typedef union EnumWithItemMixedTwinSyncKind {
  struct wire_EnumWithItemMixedTwinSync_A *A;
  struct wire_EnumWithItemMixedTwinSync_B *B;
  struct wire_EnumWithItemMixedTwinSync_C *C;
} EnumWithItemMixedTwinSyncKind;

typedef struct wire_enum_with_item_mixed_twin_sync {
  int32_t tag;
  union EnumWithItemMixedTwinSyncKind *kind;
} wire_enum_with_item_mixed_twin_sync;

typedef struct wire_EnumWithItemStructTwinSync_A {
  struct wire_list_prim_u_8 *a_field;
} wire_EnumWithItemStructTwinSync_A;

typedef struct wire_EnumWithItemStructTwinSync_B {
  struct wire_list_prim_i_32 *b_field;
} wire_EnumWithItemStructTwinSync_B;

typedef union EnumWithItemStructTwinSyncKind {
  struct wire_EnumWithItemStructTwinSync_A *A;
  struct wire_EnumWithItemStructTwinSync_B *B;
} EnumWithItemStructTwinSyncKind;

typedef struct wire_enum_with_item_struct_twin_sync {
  int32_t tag;
  union EnumWithItemStructTwinSyncKind *kind;
} wire_enum_with_item_struct_twin_sync;

typedef struct wire_EnumWithItemTupleTwinSync_A {
  struct wire_list_prim_u_8 *field0;
} wire_EnumWithItemTupleTwinSync_A;

typedef struct wire_EnumWithItemTupleTwinSync_B {
  struct wire_list_prim_i_32 *field0;
} wire_EnumWithItemTupleTwinSync_B;

typedef union EnumWithItemTupleTwinSyncKind {
  struct wire_EnumWithItemTupleTwinSync_A *A;
  struct wire_EnumWithItemTupleTwinSync_B *B;
} EnumWithItemTupleTwinSyncKind;

typedef struct wire_enum_with_item_tuple_twin_sync {
  int32_t tag;
  union EnumWithItemTupleTwinSyncKind *kind;
} wire_enum_with_item_tuple_twin_sync;

typedef struct wire_KitchenSinkTwinSync_Empty {

} wire_KitchenSinkTwinSync_Empty;

typedef struct wire_KitchenSinkTwinSync_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_KitchenSinkTwinSync_Primitives;

typedef struct wire_KitchenSinkTwinSync_Nested {
  int32_t field0;
  struct wire_kitchen_sink_twin_sync *field1;
} wire_KitchenSinkTwinSync_Nested;

typedef struct wire_KitchenSinkTwinSync_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_KitchenSinkTwinSync_Optional;

typedef struct wire_KitchenSinkTwinSync_Buffer {
  struct wire_list_prim_u_8 *field0;
} wire_KitchenSinkTwinSync_Buffer;

typedef struct wire_KitchenSinkTwinSync_Enums {
  int32_t field0;
} wire_KitchenSinkTwinSync_Enums;

typedef union KitchenSinkTwinSyncKind {
  struct wire_KitchenSinkTwinSync_Empty *Empty;
  struct wire_KitchenSinkTwinSync_Primitives *Primitives;
  struct wire_KitchenSinkTwinSync_Nested *Nested;
  struct wire_KitchenSinkTwinSync_Optional *Optional;
  struct wire_KitchenSinkTwinSync_Buffer *Buffer;
  struct wire_KitchenSinkTwinSync_Enums *Enums;
} KitchenSinkTwinSyncKind;

typedef struct wire_kitchen_sink_twin_sync {
  int32_t tag;
  union KitchenSinkTwinSyncKind *kind;
} wire_kitchen_sink_twin_sync;

typedef struct wire_SpeedTwinSync_Unknown {

} wire_SpeedTwinSync_Unknown;

typedef struct wire_SpeedTwinSync_GPS {
  double field0;
} wire_SpeedTwinSync_GPS;

typedef union SpeedTwinSyncKind {
  struct wire_SpeedTwinSync_Unknown *Unknown;
  struct wire_SpeedTwinSync_GPS *GPS;
} SpeedTwinSyncKind;

typedef struct wire_speed_twin_sync {
  int32_t tag;
  union SpeedTwinSyncKind *kind;
} wire_speed_twin_sync;

typedef struct wire_MeasureTwinSync_Speed {
  struct wire_speed_twin_sync *field0;
} wire_MeasureTwinSync_Speed;

typedef struct wire_DistanceTwinSync_Unknown {

} wire_DistanceTwinSync_Unknown;

typedef struct wire_DistanceTwinSync_Map {
  double field0;
} wire_DistanceTwinSync_Map;

typedef union DistanceTwinSyncKind {
  struct wire_DistanceTwinSync_Unknown *Unknown;
  struct wire_DistanceTwinSync_Map *Map;
} DistanceTwinSyncKind;

typedef struct wire_distance_twin_sync {
  int32_t tag;
  union DistanceTwinSyncKind *kind;
} wire_distance_twin_sync;

typedef struct wire_MeasureTwinSync_Distance {
  struct wire_distance_twin_sync *field0;
} wire_MeasureTwinSync_Distance;

typedef union MeasureTwinSyncKind {
  struct wire_MeasureTwinSync_Speed *Speed;
  struct wire_MeasureTwinSync_Distance *Distance;
} MeasureTwinSyncKind;

typedef struct wire_measure_twin_sync {
  int32_t tag;
  union MeasureTwinSyncKind *kind;
} wire_measure_twin_sync;

typedef struct wire_note_twin_sync {
  int32_t *day;
  struct wire_list_prim_u_8 *body;
} wire_note_twin_sync;

typedef struct wire_event_twin_rust_async {
  struct wire_list_prim_u_8 *address;
  struct wire_list_prim_u_8 *payload;
} wire_event_twin_rust_async;

typedef struct wire_event_twin_sync {
  struct wire_list_prim_u_8 *address;
  struct wire_list_prim_u_8 *payload;
} wire_event_twin_sync;

typedef struct wire_custom_struct_twin_rust_async {
  struct wire_list_prim_u_8 *message;
} wire_custom_struct_twin_rust_async;

typedef struct wire_some_struct_twin_rust_async {
  uint32_t value;
} wire_some_struct_twin_rust_async;

typedef struct wire_CustomNestedErrorOuterTwinRustAsync_One {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorOuterTwinRustAsync_One;

typedef struct wire_CustomNestedErrorInnerTwinRustAsync_Three {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorInnerTwinRustAsync_Three;

typedef struct wire_CustomNestedErrorInnerTwinRustAsync_Four {
  uint32_t field0;
} wire_CustomNestedErrorInnerTwinRustAsync_Four;

typedef union CustomNestedErrorInnerTwinRustAsyncKind {
  struct wire_CustomNestedErrorInnerTwinRustAsync_Three *Three;
  struct wire_CustomNestedErrorInnerTwinRustAsync_Four *Four;
} CustomNestedErrorInnerTwinRustAsyncKind;

typedef struct wire_custom_nested_error_inner_twin_rust_async {
  int32_t tag;
  union CustomNestedErrorInnerTwinRustAsyncKind *kind;
} wire_custom_nested_error_inner_twin_rust_async;

typedef struct wire_CustomNestedErrorOuterTwinRustAsync_Two {
  struct wire_custom_nested_error_inner_twin_rust_async *field0;
} wire_CustomNestedErrorOuterTwinRustAsync_Two;

typedef union CustomNestedErrorOuterTwinRustAsyncKind {
  struct wire_CustomNestedErrorOuterTwinRustAsync_One *One;
  struct wire_CustomNestedErrorOuterTwinRustAsync_Two *Two;
} CustomNestedErrorOuterTwinRustAsyncKind;

typedef struct wire_custom_nested_error_outer_twin_rust_async {
  int32_t tag;
  union CustomNestedErrorOuterTwinRustAsyncKind *kind;
} wire_custom_nested_error_outer_twin_rust_async;

typedef struct wire_custom_struct_error_twin_rust_async {
  struct wire_list_prim_u_8 *a;
} wire_custom_struct_error_twin_rust_async;

typedef struct wire_custom_struct_twin_sync {
  struct wire_list_prim_u_8 *message;
} wire_custom_struct_twin_sync;

typedef struct wire_some_struct_twin_sync {
  uint32_t value;
} wire_some_struct_twin_sync;

typedef struct wire_CustomNestedErrorOuterTwinSync_One {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorOuterTwinSync_One;

typedef struct wire_CustomNestedErrorInnerTwinSync_Three {
  struct wire_list_prim_u_8 *field0;
} wire_CustomNestedErrorInnerTwinSync_Three;

typedef struct wire_CustomNestedErrorInnerTwinSync_Four {
  uint32_t field0;
} wire_CustomNestedErrorInnerTwinSync_Four;

typedef union CustomNestedErrorInnerTwinSyncKind {
  struct wire_CustomNestedErrorInnerTwinSync_Three *Three;
  struct wire_CustomNestedErrorInnerTwinSync_Four *Four;
} CustomNestedErrorInnerTwinSyncKind;

typedef struct wire_custom_nested_error_inner_twin_sync {
  int32_t tag;
  union CustomNestedErrorInnerTwinSyncKind *kind;
} wire_custom_nested_error_inner_twin_sync;

typedef struct wire_CustomNestedErrorOuterTwinSync_Two {
  struct wire_custom_nested_error_inner_twin_sync *field0;
} wire_CustomNestedErrorOuterTwinSync_Two;

typedef union CustomNestedErrorOuterTwinSyncKind {
  struct wire_CustomNestedErrorOuterTwinSync_One *One;
  struct wire_CustomNestedErrorOuterTwinSync_Two *Two;
} CustomNestedErrorOuterTwinSyncKind;

typedef struct wire_custom_nested_error_outer_twin_sync {
  int32_t tag;
  union CustomNestedErrorOuterTwinSyncKind *kind;
} wire_custom_nested_error_outer_twin_sync;

typedef struct wire_custom_struct_error_twin_sync {
  struct wire_list_prim_u_8 *a;
} wire_custom_struct_error_twin_sync;

typedef struct wire_concatenate_with_twin_rust_async {
  struct wire_list_prim_u_8 *a;
} wire_concatenate_with_twin_rust_async;

typedef struct wire_sum_with_twin_rust_async {
  uint32_t x;
} wire_sum_with_twin_rust_async;

typedef struct wire_concatenate_with_twin_sync {
  struct wire_list_prim_u_8 *a;
} wire_concatenate_with_twin_sync;

typedef struct wire_sum_with_twin_sync {
  uint32_t x;
} wire_sum_with_twin_sync;

typedef struct wire_list_my_tree_node_twin_rust_async {
  struct wire_my_tree_node_twin_rust_async *ptr;
  int32_t len;
} wire_list_my_tree_node_twin_rust_async;

typedef struct wire_my_tree_node_twin_rust_async {
  int32_t value_i32;
  struct wire_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_list_my_tree_node_twin_rust_async *children;
} wire_my_tree_node_twin_rust_async;

typedef struct wire_my_nested_struct_twin_rust_async {
  struct wire_my_tree_node_twin_rust_async tree_node;
  int32_t weekday;
} wire_my_nested_struct_twin_rust_async;

typedef struct wire_list_weekdays_twin_rust_async {
  int32_t *ptr;
  int32_t len;
} wire_list_weekdays_twin_rust_async;

typedef struct wire_a_twin_rust_async {
  struct wire_list_prim_u_8 *a;
} wire_a_twin_rust_async;

typedef struct wire_AbcTwinRustAsync_A {
  struct wire_a_twin_rust_async *field0;
} wire_AbcTwinRustAsync_A;

typedef struct wire_b_twin_rust_async {
  int32_t b;
} wire_b_twin_rust_async;

typedef struct wire_AbcTwinRustAsync_B {
  struct wire_b_twin_rust_async *field0;
} wire_AbcTwinRustAsync_B;

typedef struct wire_c_twin_rust_async {
  bool c;
} wire_c_twin_rust_async;

typedef struct wire_AbcTwinRustAsync_C {
  struct wire_c_twin_rust_async *field0;
} wire_AbcTwinRustAsync_C;

typedef struct wire_AbcTwinRustAsync_JustInt {
  int32_t field0;
} wire_AbcTwinRustAsync_JustInt;

typedef union AbcTwinRustAsyncKind {
  struct wire_AbcTwinRustAsync_A *A;
  struct wire_AbcTwinRustAsync_B *B;
  struct wire_AbcTwinRustAsync_C *C;
  struct wire_AbcTwinRustAsync_JustInt *JustInt;
} AbcTwinRustAsyncKind;

typedef struct wire_abc_twin_rust_async {
  int32_t tag;
  union AbcTwinRustAsyncKind *kind;
} wire_abc_twin_rust_async;

typedef struct wire_struct_with_enum_twin_rust_async {
  struct wire_abc_twin_rust_async abc1;
  struct wire_abc_twin_rust_async abc2;
} wire_struct_with_enum_twin_rust_async;

typedef struct wire_list_my_tree_node_twin_sync {
  struct wire_my_tree_node_twin_sync *ptr;
  int32_t len;
} wire_list_my_tree_node_twin_sync;

typedef struct wire_my_tree_node_twin_sync {
  int32_t value_i32;
  struct wire_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_list_my_tree_node_twin_sync *children;
} wire_my_tree_node_twin_sync;

typedef struct wire_my_nested_struct_twin_sync {
  struct wire_my_tree_node_twin_sync tree_node;
  int32_t weekday;
} wire_my_nested_struct_twin_sync;

typedef struct wire_list_weekdays_twin_sync {
  int32_t *ptr;
  int32_t len;
} wire_list_weekdays_twin_sync;

typedef struct wire_a_twin_sync {
  struct wire_list_prim_u_8 *a;
} wire_a_twin_sync;

typedef struct wire_AbcTwinSync_A {
  struct wire_a_twin_sync *field0;
} wire_AbcTwinSync_A;

typedef struct wire_b_twin_sync {
  int32_t b;
} wire_b_twin_sync;

typedef struct wire_AbcTwinSync_B {
  struct wire_b_twin_sync *field0;
} wire_AbcTwinSync_B;

typedef struct wire_c_twin_sync {
  bool c;
} wire_c_twin_sync;

typedef struct wire_AbcTwinSync_C {
  struct wire_c_twin_sync *field0;
} wire_AbcTwinSync_C;

typedef struct wire_AbcTwinSync_JustInt {
  int32_t field0;
} wire_AbcTwinSync_JustInt;

typedef union AbcTwinSyncKind {
  struct wire_AbcTwinSync_A *A;
  struct wire_AbcTwinSync_B *B;
  struct wire_AbcTwinSync_C *C;
  struct wire_AbcTwinSync_JustInt *JustInt;
} AbcTwinSyncKind;

typedef struct wire_abc_twin_sync {
  int32_t tag;
  union AbcTwinSyncKind *kind;
} wire_abc_twin_sync;

typedef struct wire_struct_with_enum_twin_sync {
  struct wire_abc_twin_sync abc1;
  struct wire_abc_twin_sync abc2;
} wire_struct_with_enum_twin_sync;

typedef struct wire_empty_twin_rust_async {

} wire_empty_twin_rust_async;

typedef struct wire_empty_twin_sync {

} wire_empty_twin_sync;

typedef struct wire_new_type_int_twin_rust_async {
  int64_t field0;
} wire_new_type_int_twin_rust_async;

typedef struct wire_new_type_int_twin_sync {
  int64_t field0;
} wire_new_type_int_twin_sync;

typedef struct wire_attribute_twin_rust_async {
  struct wire_list_prim_u_8 *key;
  struct wire_list_prim_u_8 *value;
} wire_attribute_twin_rust_async;

typedef struct wire_list_attribute_twin_rust_async {
  struct wire_attribute_twin_rust_async *ptr;
  int32_t len;
} wire_list_attribute_twin_rust_async;

typedef struct wire_list_opt_box_autoadd_attribute_twin_rust_async {
  struct wire_attribute_twin_rust_async **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_attribute_twin_rust_async;

typedef struct wire_exotic_optionals_twin_rust_async {
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
  struct wire_list_attribute_twin_rust_async *attributes;
  struct wire_list_opt_box_autoadd_attribute_twin_rust_async *attributes_nullable;
  struct wire_list_opt_box_autoadd_attribute_twin_rust_async *nullable_attributes;
  struct wire_new_type_int_twin_rust_async *newtypeint;
} wire_exotic_optionals_twin_rust_async;

typedef struct wire_list_opt_box_autoadd_weekdays_twin_rust_async {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_weekdays_twin_rust_async;

typedef struct wire_opt_vecs_twin_rust_async {
  struct wire_list_opt_box_autoadd_i_32 *i32;
  struct wire_list_opt_box_autoadd_weekdays_twin_rust_async *enums;
  struct wire_list_opt_String *strings;
  struct wire_list_opt_list_prim_i_32 *buffers;
} wire_opt_vecs_twin_rust_async;

typedef struct wire_attribute_twin_sync {
  struct wire_list_prim_u_8 *key;
  struct wire_list_prim_u_8 *value;
} wire_attribute_twin_sync;

typedef struct wire_list_attribute_twin_sync {
  struct wire_attribute_twin_sync *ptr;
  int32_t len;
} wire_list_attribute_twin_sync;

typedef struct wire_list_opt_box_autoadd_attribute_twin_sync {
  struct wire_attribute_twin_sync **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_attribute_twin_sync;

typedef struct wire_exotic_optionals_twin_sync {
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
  struct wire_list_attribute_twin_sync *attributes;
  struct wire_list_opt_box_autoadd_attribute_twin_sync *attributes_nullable;
  struct wire_list_opt_box_autoadd_attribute_twin_sync *nullable_attributes;
  struct wire_new_type_int_twin_sync *newtypeint;
} wire_exotic_optionals_twin_sync;

typedef struct wire_list_opt_box_autoadd_weekdays_twin_sync {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_weekdays_twin_sync;

typedef struct wire_opt_vecs_twin_sync {
  struct wire_list_opt_box_autoadd_i_32 *i32;
  struct wire_list_opt_box_autoadd_weekdays_twin_sync *enums;
  struct wire_list_opt_String *strings;
  struct wire_list_opt_list_prim_i_32 *buffers;
} wire_opt_vecs_twin_sync;

typedef struct wire_list_bool {
  bool *ptr;
  int32_t len;
} wire_list_bool;

typedef struct wire_list_prim_i_16 {
  int16_t *ptr;
  int32_t len;
} wire_list_prim_i_16;

typedef struct wire_list_prim_u_16 {
  uint16_t *ptr;
  int32_t len;
} wire_list_prim_u_16;

typedef struct wire_list_prim_u_32 {
  uint32_t *ptr;
  int32_t len;
} wire_list_prim_u_32;

typedef struct wire_list_prim_u_64 {
  uint64_t *ptr;
  int32_t len;
} wire_list_prim_u_64;

typedef struct wire_list_RustOpaque_hide_data {
  const void **ptr;
  int32_t len;
} wire_list_RustOpaque_hide_data;

typedef struct wire_EnumOpaqueTwinRustAsync_Struct {
  const void *field0;
} wire_EnumOpaqueTwinRustAsync_Struct;

typedef struct wire_EnumOpaqueTwinRustAsync_Primitive {
  const void *field0;
} wire_EnumOpaqueTwinRustAsync_Primitive;

typedef struct wire_EnumOpaqueTwinRustAsync_TraitObj {
  const void *field0;
} wire_EnumOpaqueTwinRustAsync_TraitObj;

typedef struct wire_EnumOpaqueTwinRustAsync_Mutex {
  const void *field0;
} wire_EnumOpaqueTwinRustAsync_Mutex;

typedef struct wire_EnumOpaqueTwinRustAsync_RwLock {
  const void *field0;
} wire_EnumOpaqueTwinRustAsync_RwLock;

typedef union EnumOpaqueTwinRustAsyncKind {
  struct wire_EnumOpaqueTwinRustAsync_Struct *Struct;
  struct wire_EnumOpaqueTwinRustAsync_Primitive *Primitive;
  struct wire_EnumOpaqueTwinRustAsync_TraitObj *TraitObj;
  struct wire_EnumOpaqueTwinRustAsync_Mutex *Mutex;
  struct wire_EnumOpaqueTwinRustAsync_RwLock *RwLock;
} EnumOpaqueTwinRustAsyncKind;

typedef struct wire_enum_opaque_twin_rust_async {
  int32_t tag;
  union EnumOpaqueTwinRustAsyncKind *kind;
} wire_enum_opaque_twin_rust_async;

typedef struct wire_opaque_nested_twin_rust_async {
  const void *first;
  const void *second;
} wire_opaque_nested_twin_rust_async;

typedef struct wire_EnumOpaqueTwinSync_Struct {
  const void *field0;
} wire_EnumOpaqueTwinSync_Struct;

typedef struct wire_EnumOpaqueTwinSync_Primitive {
  const void *field0;
} wire_EnumOpaqueTwinSync_Primitive;

typedef struct wire_EnumOpaqueTwinSync_TraitObj {
  const void *field0;
} wire_EnumOpaqueTwinSync_TraitObj;

typedef struct wire_EnumOpaqueTwinSync_Mutex {
  const void *field0;
} wire_EnumOpaqueTwinSync_Mutex;

typedef struct wire_EnumOpaqueTwinSync_RwLock {
  const void *field0;
} wire_EnumOpaqueTwinSync_RwLock;

typedef union EnumOpaqueTwinSyncKind {
  struct wire_EnumOpaqueTwinSync_Struct *Struct;
  struct wire_EnumOpaqueTwinSync_Primitive *Primitive;
  struct wire_EnumOpaqueTwinSync_TraitObj *TraitObj;
  struct wire_EnumOpaqueTwinSync_Mutex *Mutex;
  struct wire_EnumOpaqueTwinSync_RwLock *RwLock;
} EnumOpaqueTwinSyncKind;

typedef struct wire_enum_opaque_twin_sync {
  int32_t tag;
  union EnumOpaqueTwinSyncKind *kind;
} wire_enum_opaque_twin_sync;

typedef struct wire_opaque_nested_twin_sync {
  const void *first;
  const void *second;
} wire_opaque_nested_twin_sync;

typedef struct wire_struct_with_one_field_twin_rust_async {
  int32_t a;
} wire_struct_with_one_field_twin_rust_async;

typedef struct wire_struct_with_two_field_twin_rust_async {
  int32_t a;
  int32_t b;
} wire_struct_with_two_field_twin_rust_async;

typedef struct wire_struct_with_zero_field_twin_rust_async {

} wire_struct_with_zero_field_twin_rust_async;

typedef struct wire_tuple_struct_with_one_field_twin_rust_async {
  int32_t field0;
} wire_tuple_struct_with_one_field_twin_rust_async;

typedef struct wire_tuple_struct_with_two_field_twin_rust_async {
  int32_t field0;
  int32_t field1;
} wire_tuple_struct_with_two_field_twin_rust_async;

typedef struct wire_struct_with_one_field_twin_sync {
  int32_t a;
} wire_struct_with_one_field_twin_sync;

typedef struct wire_struct_with_two_field_twin_sync {
  int32_t a;
  int32_t b;
} wire_struct_with_two_field_twin_sync;

typedef struct wire_struct_with_zero_field_twin_sync {

} wire_struct_with_zero_field_twin_sync;

typedef struct wire_tuple_struct_with_one_field_twin_sync {
  int32_t field0;
} wire_tuple_struct_with_one_field_twin_sync;

typedef struct wire_tuple_struct_with_two_field_twin_sync {
  int32_t field0;
  int32_t field1;
} wire_tuple_struct_with_two_field_twin_sync;

typedef struct wire_record_string_i_32 {
  struct wire_list_prim_u_8 *field0;
  int32_t field1;
} wire_record_string_i_32;

typedef struct wire_list_record_string_i_32 {
  struct wire_record_string_i_32 *ptr;
  int32_t len;
} wire_list_record_string_i_32;

typedef struct wire_feature_uuid_twin_rust_async {
  struct wire_list_prim_u_8 *one;
  struct wire_list_prim_u_8 *many;
} wire_feature_uuid_twin_rust_async;

typedef struct wire_feature_uuid_twin_sync {
  struct wire_list_prim_u_8 *one;
  struct wire_list_prim_u_8 *many;
} wire_feature_uuid_twin_sync;

typedef struct wire_EnumOpaqueTwinNormal_Struct {
  const void *field0;
} wire_EnumOpaqueTwinNormal_Struct;

typedef struct wire_EnumOpaqueTwinNormal_Primitive {
  const void *field0;
} wire_EnumOpaqueTwinNormal_Primitive;

typedef struct wire_EnumOpaqueTwinNormal_TraitObj {
  const void *field0;
} wire_EnumOpaqueTwinNormal_TraitObj;

typedef struct wire_EnumOpaqueTwinNormal_Mutex {
  const void *field0;
} wire_EnumOpaqueTwinNormal_Mutex;

typedef struct wire_EnumOpaqueTwinNormal_RwLock {
  const void *field0;
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
  const void *first;
  const void *second;
} wire_opaque_nested_twin_normal;

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

typedef struct wire_feature_uuid_twin_normal {
  struct wire_list_prim_u_8 *one;
  struct wire_list_prim_u_8 *many;
} wire_feature_uuid_twin_normal;

void benchmark_raw_void_sync(void);

struct benchmark_raw_list_prim_u_8 benchmark_raw_new_list_prim_u_8(int32_t len);

int32_t benchmark_raw_input_bytes(struct benchmark_raw_list_prim_u_8 bytes);

void benchmark_raw_output_bytes(MessagePort port, int32_t message_id, int32_t size);

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

void wire_func_async_simple_add(int64_t port_, int32_t a, int32_t b);

void wire_func_async_void(int64_t port_);

void wire_handle_customized_struct_twin_normal(int64_t port_,
                                               struct wire_customized_twin_normal *val);

void wire_next_user_id_twin_normal(int64_t port_, struct wire_user_id_twin_normal *user_id);

void wire_benchmark_input_bytes_twin_normal(int64_t port_, struct wire_list_prim_u_8 *bytes);

void wire_benchmark_output_bytes_twin_normal(int64_t port_, int32_t size);

void wire_benchmark_void_twin_normal(int64_t port_);

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

void wire_rust_call_dart_loopback(int64_t port_, struct wire_DartOpaque callback);

void wire_rust_call_dart_one_arg(int64_t port_, struct wire_DartOpaque callback);

void wire_rust_call_dart_return(int64_t port_, struct wire_DartOpaque callback);

void wire_rust_call_dart_simple(int64_t port_, struct wire_DartOpaque callback);

void wire_rust_call_dart_two_args(int64_t port_, struct wire_DartOpaque callback);

void wire_rust_call_dart_with_dart_opaque_arg(int64_t port_,
                                              struct wire_DartOpaque input,
                                              struct wire_DartOpaque callback);

void wire_rust_call_dart_with_dart_opaque_result(int64_t port_, struct wire_DartOpaque callback);

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

void wire_primitive_optional_types_twin_normal(int64_t port_,
                                               int32_t *my_i32,
                                               int64_t *my_i64,
                                               double *my_f64,
                                               bool *my_bool);

void wire_handle_vec_of_primitive_twin_normal(int64_t port_, int32_t n);

void wire_handle_zero_copy_vec_of_primitive_twin_normal(int64_t port_, int32_t n);

void wire_primitive_types_twin_normal(int64_t port_,
                                      int32_t my_i32,
                                      int64_t my_i64,
                                      double my_f64,
                                      bool my_bool);

void wire_primitive_u32_twin_normal(int64_t port_, uint32_t my_u32);

void wire_boxed_blob_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *blob);

void wire_func_test_id_twin_rust_async(int64_t port_, struct wire_test_id_twin_rust_async *id);

void wire_get_array_twin_rust_async(int64_t port_);

void wire_get_complex_array_twin_rust_async(int64_t port_);

void wire_last_number_twin_rust_async(int64_t port_, struct wire_list_prim_f_64 *array);

void wire_nested_id_twin_rust_async(int64_t port_, struct wire_list_test_id_twin_rust_async *id);

void wire_new_msgid_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_feed_id_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_raw_feed_id_twin_rust_async(int64_t port_,
                                                   struct wire_feed_id_twin_rust_async *id);

void wire_use_boxed_blob_twin_rust_async(int64_t port_, struct wire_blob_twin_rust_async *blob);

void wire_use_msgid_twin_rust_async(int64_t port_, struct wire_message_id_twin_rust_async *id);

WireSyncReturn wire_boxed_blob_twin_sync(struct wire_list_prim_u_8 *blob);

WireSyncReturn wire_func_test_id_twin_sync(struct wire_test_id_twin_sync *id);

WireSyncReturn wire_get_array_twin_sync(void);

WireSyncReturn wire_get_complex_array_twin_sync(void);

WireSyncReturn wire_last_number_twin_sync(struct wire_list_prim_f_64 *array);

WireSyncReturn wire_nested_id_twin_sync(struct wire_list_test_id_twin_sync *id);

WireSyncReturn wire_new_msgid_twin_sync(struct wire_list_prim_u_8 *id);

WireSyncReturn wire_return_boxed_feed_id_twin_sync(struct wire_list_prim_u_8 *id);

WireSyncReturn wire_return_boxed_raw_feed_id_twin_sync(struct wire_feed_id_twin_sync *id);

WireSyncReturn wire_use_boxed_blob_twin_sync(struct wire_blob_twin_sync *blob);

WireSyncReturn wire_use_msgid_twin_sync(struct wire_message_id_twin_sync *id);

void wire_handle_customized_struct_twin_rust_async(int64_t port_,
                                                   struct wire_customized_twin_rust_async *val);

void wire_next_user_id_twin_rust_async(int64_t port_, struct wire_user_id_twin_rust_async *user_id);

WireSyncReturn wire_handle_customized_struct_twin_sync(struct wire_customized_twin_sync *val);

WireSyncReturn wire_next_user_id_twin_sync(struct wire_user_id_twin_sync *user_id);

void wire_benchmark_input_bytes_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *bytes);

void wire_benchmark_output_bytes_twin_rust_async(int64_t port_, int32_t size);

void wire_benchmark_void_twin_rust_async(int64_t port_);

WireSyncReturn wire_benchmark_input_bytes_twin_sync(struct wire_list_prim_u_8 *bytes);

WireSyncReturn wire_benchmark_output_bytes_twin_sync(int32_t size);

WireSyncReturn wire_benchmark_void_twin_sync(void);

void wire_datetime_local_twin_rust_async(int64_t port_, int64_t d);

void wire_datetime_utc_twin_rust_async(int64_t port_, int64_t d);

void wire_duration_twin_rust_async(int64_t port_, int64_t d);

void wire_handle_durations_twin_rust_async(int64_t port_,
                                           struct wire_list_prim_i_64 *durations,
                                           int64_t since);

void wire_handle_timestamps_twin_rust_async(int64_t port_,
                                            struct wire_list_prim_i_64 *timestamps,
                                            int64_t epoch);

void wire_how_long_does_it_take_twin_rust_async(int64_t port_,
                                                struct wire_feature_chrono_twin_rust_async *mine);

void wire_naivedatetime_twin_rust_async(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc_twin_rust_async(int64_t port_, int64_t *d);

void wire_test_chrono_twin_rust_async(int64_t port_);

void wire_test_precise_chrono_twin_rust_async(int64_t port_);

WireSyncReturn wire_datetime_local_twin_sync(int64_t d);

WireSyncReturn wire_datetime_utc_twin_sync(int64_t d);

WireSyncReturn wire_duration_twin_sync(int64_t d);

WireSyncReturn wire_handle_durations_twin_sync(struct wire_list_prim_i_64 *durations,
                                               int64_t since);

WireSyncReturn wire_handle_timestamps_twin_sync(struct wire_list_prim_i_64 *timestamps,
                                                int64_t epoch);

WireSyncReturn wire_how_long_does_it_take_twin_sync(struct wire_feature_chrono_twin_sync *mine);

WireSyncReturn wire_naivedatetime_twin_sync(int64_t d);

WireSyncReturn wire_optional_empty_datetime_utc_twin_sync(int64_t *d);

WireSyncReturn wire_test_chrono_twin_sync(void);

WireSyncReturn wire_test_precise_chrono_twin_sync(void);

void wire_StructWithCommentsTwinRustAsync_instance_method_twin_rust_async(int64_t port_,
                                                                          struct wire_struct_with_comments_twin_rust_async *that);

void wire_StructWithCommentsTwinRustAsync_static_method_twin_rust_async(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_rust_async(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_rust_async(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_rust_async(int64_t port_);

WireSyncReturn wire_StructWithCommentsTwinSync_instance_method_twin_sync(struct wire_struct_with_comments_twin_sync *that);

WireSyncReturn wire_StructWithCommentsTwinSync_static_method_twin_sync(void);

WireSyncReturn wire_function_with_comments_slash_star_star_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_multi_line_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_single_line_twin_sync(void);

void wire_return_dart_dynamic_twin_rust_async(int64_t port_);

WireSyncReturn wire_return_dart_dynamic_twin_sync(void);

void wire_async_accept_dart_opaque_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_enum_dart_opaque_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_nested_dart_opaque_twin_rust_async(int64_t port_,
                                                    struct wire_DartOpaque opaque1,
                                                    struct wire_DartOpaque opaque2);

void wire_drop_static_dart_opaque_twin_rust_async(int64_t port_);

void wire_get_enum_dart_opaque_twin_rust_async(int64_t port_,
                                               struct wire_enum_dart_opaque_twin_rust_async *opaque);

void wire_get_nested_dart_opaque_twin_rust_async(int64_t port_,
                                                 struct wire_dart_opaque_nested_twin_rust_async *opaque);

void wire_loop_back_array_get_twin_rust_async(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_array_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_option_get_twin_rust_async(int64_t port_, struct wire_DartOpaque *opaque);

void wire_loop_back_option_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_vec_get_twin_rust_async(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_vec_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_panic_unwrap_dart_opaque_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

void wire_set_static_dart_opaque_twin_rust_async(int64_t port_, struct wire_DartOpaque opaque);

WireSyncReturn wire_async_accept_dart_opaque_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_create_enum_dart_opaque_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_create_nested_dart_opaque_twin_sync(struct wire_DartOpaque opaque1,
                                                        struct wire_DartOpaque opaque2);

WireSyncReturn wire_drop_static_dart_opaque_twin_sync(void);

WireSyncReturn wire_get_enum_dart_opaque_twin_sync(struct wire_enum_dart_opaque_twin_sync *opaque);

WireSyncReturn wire_get_nested_dart_opaque_twin_sync(struct wire_dart_opaque_nested_twin_sync *opaque);

WireSyncReturn wire_loop_back_array_get_twin_sync(struct wire_list_DartOpaque *opaque);

WireSyncReturn wire_loop_back_array_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_loop_back_option_get_twin_sync(struct wire_DartOpaque *opaque);

WireSyncReturn wire_loop_back_option_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_loop_back_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_loop_back_vec_get_twin_sync(struct wire_list_DartOpaque *opaque);

WireSyncReturn wire_loop_back_vec_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_panic_unwrap_dart_opaque_twin_sync(struct wire_DartOpaque opaque);

WireSyncReturn wire_set_static_dart_opaque_twin_sync(struct wire_DartOpaque opaque);

void wire_func_enum_simple_twin_rust_async(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_rust_async(int64_t port_,
                                                    struct wire_enum_with_item_mixed_twin_rust_async *arg);

void wire_func_enum_with_item_struct_twin_rust_async(int64_t port_,
                                                     struct wire_enum_with_item_struct_twin_rust_async *arg);

void wire_func_enum_with_item_tuple_twin_rust_async(int64_t port_,
                                                    struct wire_enum_with_item_tuple_twin_rust_async *arg);

void wire_handle_enum_parameter_twin_rust_async(int64_t port_, int32_t weekday);

void wire_handle_enum_struct_twin_rust_async(int64_t port_,
                                             struct wire_kitchen_sink_twin_rust_async *val);

void wire_handle_return_enum_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *input);

void wire_multiply_by_ten_twin_rust_async(int64_t port_,
                                          struct wire_measure_twin_rust_async *measure);

void wire_print_note_twin_rust_async(int64_t port_, struct wire_note_twin_rust_async *note);

WireSyncReturn wire_func_enum_simple_twin_sync(int32_t arg);

WireSyncReturn wire_func_enum_with_item_mixed_twin_sync(struct wire_enum_with_item_mixed_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_struct_twin_sync(struct wire_enum_with_item_struct_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_tuple_twin_sync(struct wire_enum_with_item_tuple_twin_sync *arg);

WireSyncReturn wire_handle_enum_parameter_twin_sync(int32_t weekday);

WireSyncReturn wire_handle_enum_struct_twin_sync(struct wire_kitchen_sink_twin_sync *val);

WireSyncReturn wire_handle_return_enum_twin_sync(struct wire_list_prim_u_8 *input);

WireSyncReturn wire_multiply_by_ten_twin_sync(struct wire_measure_twin_sync *measure);

WireSyncReturn wire_print_note_twin_sync(struct wire_note_twin_sync *note);

void wire_EventTwinRustAsync_as_string_twin_rust_async(int64_t port_,
                                                       struct wire_event_twin_rust_async *that);

void wire_close_event_listener_twin_rust_async(int64_t port_);

void wire_create_event_twin_rust_async(int64_t port_,
                                       struct wire_list_prim_u_8 *address,
                                       struct wire_list_prim_u_8 *payload);

void wire_register_event_listener_twin_rust_async(int64_t port_);

WireSyncReturn wire_EventTwinSync_as_string_twin_sync(struct wire_event_twin_sync *that);

WireSyncReturn wire_close_event_listener_twin_sync(void);

WireSyncReturn wire_create_event_twin_sync(struct wire_list_prim_u_8 *address,
                                           struct wire_list_prim_u_8 *payload);

void wire_register_event_listener_twin_sync(int64_t port_);

void wire_CustomStructTwinRustAsync_new_twin_rust_async(int64_t port_,
                                                        struct wire_list_prim_u_8 *message);

void wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_error_twin_rust_async(int64_t port_,
                                                                                         struct wire_custom_struct_twin_rust_async *that);

void wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_ok_twin_rust_async(int64_t port_,
                                                                                      struct wire_custom_struct_twin_rust_async *that);

void wire_CustomStructTwinRustAsync_static_return_custom_struct_error_twin_rust_async(int64_t port_);

void wire_CustomStructTwinRustAsync_static_return_custom_struct_ok_twin_rust_async(int64_t port_);

void wire_SomeStructTwinRustAsync_new_twin_rust_async(int64_t port_, uint32_t value);

void wire_SomeStructTwinRustAsync_non_static_return_err_custom_error_twin_rust_async(int64_t port_,
                                                                                     struct wire_some_struct_twin_rust_async *that);

void wire_SomeStructTwinRustAsync_non_static_return_ok_custom_error_twin_rust_async(int64_t port_,
                                                                                    struct wire_some_struct_twin_rust_async *that);

void wire_SomeStructTwinRustAsync_static_return_err_custom_error_twin_rust_async(int64_t port_);

void wire_SomeStructTwinRustAsync_static_return_ok_custom_error_twin_rust_async(int64_t port_);

void wire_custom_enum_error_panic_twin_rust_async(int64_t port_);

void wire_custom_enum_error_return_error_twin_rust_async(int64_t port_);

void wire_custom_enum_error_return_ok_twin_rust_async(int64_t port_, uint32_t arg);

void wire_custom_nested_error_return_error_twin_rust_async(int64_t port_,
                                                           struct wire_custom_nested_error_outer_twin_rust_async *arg);

void wire_custom_struct_error_return_error_twin_rust_async(int64_t port_,
                                                           struct wire_custom_struct_error_twin_rust_async *arg);

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

WireSyncReturn wire_CustomStructTwinSync_new_twin_sync(struct wire_list_prim_u_8 *message);

WireSyncReturn wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync(struct wire_custom_struct_twin_sync *that);

WireSyncReturn wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync(struct wire_custom_struct_twin_sync *that);

WireSyncReturn wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync(void);

WireSyncReturn wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync(void);

WireSyncReturn wire_SomeStructTwinSync_new_twin_sync(uint32_t value);

WireSyncReturn wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync(struct wire_some_struct_twin_sync *that);

WireSyncReturn wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync(struct wire_some_struct_twin_sync *that);

WireSyncReturn wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync(void);

WireSyncReturn wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync(void);

WireSyncReturn wire_custom_enum_error_panic_twin_sync(void);

WireSyncReturn wire_custom_enum_error_return_error_twin_sync(void);

WireSyncReturn wire_custom_enum_error_return_ok_twin_sync(uint32_t arg);

WireSyncReturn wire_custom_nested_error_return_error_twin_sync(struct wire_custom_nested_error_outer_twin_sync *arg);

WireSyncReturn wire_custom_struct_error_return_error_twin_sync(struct wire_custom_struct_error_twin_sync *arg);

WireSyncReturn wire_func_return_error_twin_sync(void);

WireSyncReturn wire_func_type_fallible_panic_twin_sync(void);

WireSyncReturn wire_func_type_infallible_panic_twin_sync(void);

WireSyncReturn wire_panic_with_custom_result_twin_sync(void);

WireSyncReturn wire_return_custom_nested_error_1_twin_sync(void);

WireSyncReturn wire_return_custom_nested_error_1_variant1_twin_sync(void);

WireSyncReturn wire_return_custom_nested_error_2_twin_sync(void);

WireSyncReturn wire_return_custom_struct_error_twin_sync(void);

WireSyncReturn wire_return_custom_struct_ok_twin_sync(void);

WireSyncReturn wire_return_err_custom_error_twin_sync(void);

WireSyncReturn wire_return_error_variant_twin_sync(uint32_t variant);

WireSyncReturn wire_return_ok_custom_error_twin_sync(void);

void wire_stream_sink_throw_anyhow_twin_sync(int64_t port_);

WireSyncReturn wire_throw_anyhow_twin_sync(void);

void wire_call_new_module_system_twin_rust_async(int64_t port_);

void wire_call_old_module_system_twin_rust_async(int64_t port_);

void wire_use_imported_enum_twin_rust_async(int64_t port_, int32_t my_enum);

void wire_use_imported_struct_twin_rust_async(int64_t port_, struct wire_my_struct *my_struct);

WireSyncReturn wire_call_new_module_system_twin_sync(void);

WireSyncReturn wire_call_old_module_system_twin_sync(void);

WireSyncReturn wire_use_imported_enum_twin_sync(int32_t my_enum);

WireSyncReturn wire_use_imported_struct_twin_sync(struct wire_my_struct *my_struct);

void wire_ConcatenateWithTwinRustAsync_concatenate_static_twin_rust_async(int64_t port_,
                                                                          struct wire_list_prim_u_8 *a,
                                                                          struct wire_list_prim_u_8 *b);

void wire_ConcatenateWithTwinRustAsync_concatenate_twin_rust_async(int64_t port_,
                                                                   struct wire_concatenate_with_twin_rust_async *that,
                                                                   struct wire_list_prim_u_8 *b);

void wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_single_arg_twin_rust_async(int64_t port_);

void wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_twin_rust_async(int64_t port_,
                                                                                      uint32_t key,
                                                                                      uint32_t max);

void wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_at_1_twin_rust_async(int64_t port_,
                                                                                    struct wire_concatenate_with_twin_rust_async *that);

void wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_twin_rust_async(int64_t port_,
                                                                               struct wire_concatenate_with_twin_rust_async *that,
                                                                               uint32_t key,
                                                                               uint32_t max);

void wire_ConcatenateWithTwinRustAsync_new_twin_rust_async(int64_t port_,
                                                           struct wire_list_prim_u_8 *a);

void wire_SumWithTwinRustAsync_sum_twin_rust_async(int64_t port_,
                                                   struct wire_sum_with_twin_rust_async *that,
                                                   uint32_t y,
                                                   uint32_t z);

void wire_get_sum_array_twin_rust_async(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_get_sum_struct_twin_rust_async(int64_t port_);

WireSyncReturn wire_ConcatenateWithTwinSync_concatenate_static_twin_sync(struct wire_list_prim_u_8 *a,
                                                                         struct wire_list_prim_u_8 *b);

WireSyncReturn wire_ConcatenateWithTwinSync_concatenate_twin_sync(struct wire_concatenate_with_twin_sync *that,
                                                                  struct wire_list_prim_u_8 *b);

void wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync(int64_t port_);

void wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync(int64_t port_,
                                                                           uint32_t key,
                                                                           uint32_t max);

void wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync(int64_t port_,
                                                                         struct wire_concatenate_with_twin_sync *that);

void wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync(int64_t port_,
                                                                    struct wire_concatenate_with_twin_sync *that,
                                                                    uint32_t key,
                                                                    uint32_t max);

WireSyncReturn wire_ConcatenateWithTwinSync_new_twin_sync(struct wire_list_prim_u_8 *a);

WireSyncReturn wire_SumWithTwinSync_sum_twin_sync(struct wire_sum_with_twin_sync *that,
                                                  uint32_t y,
                                                  uint32_t z);

WireSyncReturn wire_get_sum_array_twin_sync(uint32_t a, uint32_t b, uint32_t c);

WireSyncReturn wire_get_sum_struct_twin_sync(void);

void wire_app_settings_stream_twin_rust_async(int64_t port_);

void wire_app_settings_vec_stream_twin_rust_async(int64_t port_);

void wire_first_number_twin_rust_async(int64_t port_, struct wire_numbers *nums);

void wire_first_sequence_twin_rust_async(int64_t port_, struct wire_sequences *seqs);

void wire_get_app_settings_twin_rust_async(int64_t port_);

void wire_get_fallible_app_settings_twin_rust_async(int64_t port_);

void wire_get_message_twin_rust_async(int64_t port_);

void wire_is_app_embedded_twin_rust_async(int64_t port_,
                                          struct wire_application_settings *app_settings);

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

void wire_app_settings_stream_twin_sync(int64_t port_);

void wire_app_settings_vec_stream_twin_sync(int64_t port_);

WireSyncReturn wire_first_number_twin_sync(struct wire_numbers *nums);

WireSyncReturn wire_first_sequence_twin_sync(struct wire_sequences *seqs);

WireSyncReturn wire_get_app_settings_twin_sync(void);

WireSyncReturn wire_get_fallible_app_settings_twin_sync(void);

WireSyncReturn wire_get_message_twin_sync(void);

WireSyncReturn wire_is_app_embedded_twin_sync(struct wire_application_settings *app_settings);

void wire_mirror_struct_stream_twin_sync(int64_t port_);

void wire_mirror_tuple_stream_twin_sync(int64_t port_);

WireSyncReturn wire_repeat_number_twin_sync(int32_t num, uintptr_t times);

WireSyncReturn wire_repeat_sequence_twin_sync(int32_t seq, uintptr_t times);

WireSyncReturn wire_test_contains_mirrored_sub_struct_twin_sync(void);

WireSyncReturn wire_test_fallible_of_raw_string_mirrored_twin_sync(void);

WireSyncReturn wire_test_list_of_nested_enums_mirrored_twin_sync(void);

WireSyncReturn wire_test_list_of_raw_nested_string_mirrored_twin_sync(void);

WireSyncReturn wire_test_nested_raw_string_mirrored_twin_sync(void);

WireSyncReturn wire_test_raw_string_enum_mirrored_twin_sync(bool nested);

WireSyncReturn wire_test_raw_string_mirrored_twin_sync(void);

void wire_handle_big_buffers_twin_rust_async(int64_t port_);

void wire_handle_complex_struct_twin_rust_async(int64_t port_,
                                                struct wire_my_tree_node_twin_rust_async *s);

void wire_handle_nested_struct_twin_rust_async(int64_t port_,
                                               struct wire_my_nested_struct_twin_rust_async *s);

void wire_handle_string_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *s);

void wire_handle_struct_twin_rust_async(int64_t port_,
                                        struct wire_my_size *arg,
                                        struct wire_my_size *boxed);

void wire_handle_vec_u8_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *v);

void wire_list_of_primitive_enums_twin_rust_async(int64_t port_,
                                                  struct wire_list_weekdays_twin_rust_async *weekdays);

void wire_test_abc_enum_twin_rust_async(int64_t port_, struct wire_abc_twin_rust_async *abc);

void wire_test_struct_with_enum_twin_rust_async(int64_t port_,
                                                struct wire_struct_with_enum_twin_rust_async *se);

WireSyncReturn wire_handle_big_buffers_twin_sync(void);

WireSyncReturn wire_handle_complex_struct_twin_sync(struct wire_my_tree_node_twin_sync *s);

WireSyncReturn wire_handle_nested_struct_twin_sync(struct wire_my_nested_struct_twin_sync *s);

WireSyncReturn wire_handle_string_twin_sync(struct wire_list_prim_u_8 *s);

WireSyncReturn wire_handle_struct_twin_sync(struct wire_my_size *arg, struct wire_my_size *boxed);

WireSyncReturn wire_handle_vec_u8_twin_sync(struct wire_list_prim_u_8 *v);

WireSyncReturn wire_list_of_primitive_enums_twin_sync(struct wire_list_weekdays_twin_sync *weekdays);

WireSyncReturn wire_test_abc_enum_twin_sync(struct wire_abc_twin_sync *abc);

WireSyncReturn wire_test_struct_with_enum_twin_sync(struct wire_struct_with_enum_twin_sync *se);

void wire_empty_struct_twin_rust_async(int64_t port_, struct wire_empty_twin_rust_async *empty);

void wire_func_return_unit_twin_rust_async(int64_t port_);

void wire_func_string_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_handle_list_of_struct_twin_rust_async(int64_t port_, struct wire_list_my_size *l);

void wire_handle_string_list_twin_rust_async(int64_t port_, struct wire_StringList *names);

WireSyncReturn wire_empty_struct_twin_sync(struct wire_empty_twin_sync *empty);

WireSyncReturn wire_func_return_unit_twin_sync(void);

WireSyncReturn wire_func_string_twin_sync(struct wire_list_prim_u_8 *arg);

WireSyncReturn wire_handle_list_of_struct_twin_sync(struct wire_list_my_size *l);

WireSyncReturn wire_handle_string_list_twin_sync(struct wire_StringList *names);

void wire_handle_newtype_twin_rust_async(int64_t port_,
                                         struct wire_new_type_int_twin_rust_async *arg);

WireSyncReturn wire_handle_newtype_twin_sync(struct wire_new_type_int_twin_sync *arg);

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

WireSyncReturn wire_primitive_optional_types_twin_sync(int32_t *my_i32,
                                                       int64_t *my_i64,
                                                       double *my_f64,
                                                       bool *my_bool);

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

WireSyncReturn wire_example_optional_primitive_type_bool_twin_sync(bool *arg);

WireSyncReturn wire_example_optional_primitive_type_f32_twin_sync(float *arg);

WireSyncReturn wire_example_optional_primitive_type_f64_twin_sync(double *arg);

WireSyncReturn wire_example_optional_primitive_type_i16_twin_sync(int16_t *arg);

WireSyncReturn wire_example_optional_primitive_type_i32_twin_sync(int32_t *arg);

WireSyncReturn wire_example_optional_primitive_type_i64_twin_sync(int64_t *arg);

WireSyncReturn wire_example_optional_primitive_type_i8_twin_sync(int8_t *arg);

WireSyncReturn wire_example_optional_primitive_type_u16_twin_sync(uint16_t *arg);

WireSyncReturn wire_example_optional_primitive_type_u32_twin_sync(uint32_t *arg);

WireSyncReturn wire_example_optional_primitive_type_u64_twin_sync(uint64_t *arg);

WireSyncReturn wire_example_optional_primitive_type_u8_twin_sync(uint8_t *arg);

void wire_handle_increment_boxed_optional_twin_rust_async(int64_t port_, double *opt);

void wire_handle_option_box_arguments_twin_rust_async(int64_t port_,
                                                      int8_t *i8box,
                                                      uint8_t *u8box,
                                                      int32_t *i32box,
                                                      int64_t *i64box,
                                                      double *f64box,
                                                      bool *boolbox,
                                                      struct wire_exotic_optionals_twin_rust_async *structbox);

void wire_handle_optional_increment_twin_rust_async(int64_t port_,
                                                    struct wire_exotic_optionals_twin_rust_async *opt);

void wire_handle_optional_return_twin_rust_async(int64_t port_, double left, double right);

void wire_handle_optional_struct_twin_rust_async(int64_t port_,
                                                 struct wire_list_prim_u_8 *document);

void wire_handle_vec_of_opts_twin_rust_async(int64_t port_,
                                             struct wire_opt_vecs_twin_rust_async *opt);

WireSyncReturn wire_handle_increment_boxed_optional_twin_sync(double *opt);

WireSyncReturn wire_handle_option_box_arguments_twin_sync(int8_t *i8box,
                                                          uint8_t *u8box,
                                                          int32_t *i32box,
                                                          int64_t *i64box,
                                                          double *f64box,
                                                          bool *boolbox,
                                                          struct wire_exotic_optionals_twin_sync *structbox);

WireSyncReturn wire_handle_optional_increment_twin_sync(struct wire_exotic_optionals_twin_sync *opt);

WireSyncReturn wire_handle_optional_return_twin_sync(double left, double right);

WireSyncReturn wire_handle_optional_struct_twin_sync(struct wire_list_prim_u_8 *document);

WireSyncReturn wire_handle_vec_of_opts_twin_sync(struct wire_opt_vecs_twin_sync *opt);

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

void wire_example_primitive_list_type_bool_twin_normal(int64_t port_, struct wire_list_bool *arg);

void wire_example_primitive_list_type_f32_twin_normal(int64_t port_,
                                                      struct wire_list_prim_f_32 *arg);

void wire_example_primitive_list_type_f64_twin_normal(int64_t port_,
                                                      struct wire_list_prim_f_64 *arg);

void wire_example_primitive_list_type_i16_twin_normal(int64_t port_,
                                                      struct wire_list_prim_i_16 *arg);

void wire_example_primitive_list_type_i32_twin_normal(int64_t port_,
                                                      struct wire_list_prim_i_32 *arg);

void wire_example_primitive_list_type_i64_twin_normal(int64_t port_,
                                                      struct wire_list_prim_i_64 *arg);

void wire_example_primitive_list_type_i8_twin_normal(int64_t port_, struct wire_list_prim_i_8 *arg);

void wire_example_primitive_list_type_u16_twin_normal(int64_t port_,
                                                      struct wire_list_prim_u_16 *arg);

void wire_example_primitive_list_type_u32_twin_normal(int64_t port_,
                                                      struct wire_list_prim_u_32 *arg);

void wire_example_primitive_list_type_u64_twin_normal(int64_t port_,
                                                      struct wire_list_prim_u_64 *arg);

void wire_example_primitive_list_type_u8_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_handle_vec_of_primitive_twin_rust_async(int64_t port_, int32_t n);

void wire_handle_zero_copy_vec_of_primitive_twin_rust_async(int64_t port_, int32_t n);

WireSyncReturn wire_handle_vec_of_primitive_twin_sync(int32_t n);

WireSyncReturn wire_handle_zero_copy_vec_of_primitive_twin_sync(int32_t n);

void wire_example_primitive_list_type_bool_twin_rust_async(int64_t port_,
                                                           struct wire_list_bool *arg);

void wire_example_primitive_list_type_f32_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_f_32 *arg);

void wire_example_primitive_list_type_f64_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_f_64 *arg);

void wire_example_primitive_list_type_i16_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_i_16 *arg);

void wire_example_primitive_list_type_i32_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_i_32 *arg);

void wire_example_primitive_list_type_i64_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_i_64 *arg);

void wire_example_primitive_list_type_i8_twin_rust_async(int64_t port_,
                                                         struct wire_list_prim_i_8 *arg);

void wire_example_primitive_list_type_u16_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_u_16 *arg);

void wire_example_primitive_list_type_u32_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_u_32 *arg);

void wire_example_primitive_list_type_u64_twin_rust_async(int64_t port_,
                                                          struct wire_list_prim_u_64 *arg);

void wire_example_primitive_list_type_u8_twin_rust_async(int64_t port_,
                                                         struct wire_list_prim_u_8 *arg);

WireSyncReturn wire_example_primitive_list_type_bool_twin_sync(struct wire_list_bool *arg);

WireSyncReturn wire_example_primitive_list_type_f32_twin_sync(struct wire_list_prim_f_32 *arg);

WireSyncReturn wire_example_primitive_list_type_f64_twin_sync(struct wire_list_prim_f_64 *arg);

WireSyncReturn wire_example_primitive_list_type_i16_twin_sync(struct wire_list_prim_i_16 *arg);

WireSyncReturn wire_example_primitive_list_type_i32_twin_sync(struct wire_list_prim_i_32 *arg);

WireSyncReturn wire_example_primitive_list_type_i64_twin_sync(struct wire_list_prim_i_64 *arg);

WireSyncReturn wire_example_primitive_list_type_i8_twin_sync(struct wire_list_prim_i_8 *arg);

WireSyncReturn wire_example_primitive_list_type_u16_twin_sync(struct wire_list_prim_u_16 *arg);

WireSyncReturn wire_example_primitive_list_type_u32_twin_sync(struct wire_list_prim_u_32 *arg);

WireSyncReturn wire_example_primitive_list_type_u64_twin_sync(struct wire_list_prim_u_64 *arg);

WireSyncReturn wire_example_primitive_list_type_u8_twin_sync(struct wire_list_prim_u_8 *arg);

void wire_primitive_types_twin_rust_async(int64_t port_,
                                          int32_t my_i32,
                                          int64_t my_i64,
                                          double my_f64,
                                          bool my_bool);

void wire_primitive_u32_twin_rust_async(int64_t port_, uint32_t my_u32);

WireSyncReturn wire_primitive_types_twin_sync(int32_t my_i32,
                                              int64_t my_i64,
                                              double my_f64,
                                              bool my_bool);

WireSyncReturn wire_primitive_u32_twin_sync(uint32_t my_u32);

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

WireSyncReturn wire_example_primitive_type_bool_twin_sync(bool arg);

WireSyncReturn wire_example_primitive_type_f32_twin_sync(float arg);

WireSyncReturn wire_example_primitive_type_f64_twin_sync(double arg);

WireSyncReturn wire_example_primitive_type_i16_twin_sync(int16_t arg);

WireSyncReturn wire_example_primitive_type_i32_twin_sync(int32_t arg);

WireSyncReturn wire_example_primitive_type_i64_twin_sync(int64_t arg);

WireSyncReturn wire_example_primitive_type_i8_twin_sync(int8_t arg);

WireSyncReturn wire_example_primitive_type_u16_twin_sync(uint16_t arg);

WireSyncReturn wire_example_primitive_type_u32_twin_sync(uint32_t arg);

WireSyncReturn wire_example_primitive_type_u64_twin_sync(uint64_t arg);

WireSyncReturn wire_example_primitive_type_u8_twin_sync(uint8_t arg);

void wire_test_more_than_just_one_raw_string_struct_twin_rust_async(int64_t port_);

void wire_test_raw_string_item_struct_twin_rust_async(int64_t port_);

WireSyncReturn wire_test_more_than_just_one_raw_string_struct_twin_sync(void);

WireSyncReturn wire_test_raw_string_item_struct_twin_sync(void);

WireSyncReturn wire_NonCloneSimpleTwinSync_instance_method_arg_borrow_twin_sync(const void *that);

WireSyncReturn wire_NonCloneSimpleTwinSync_instance_method_arg_mut_borrow_twin_sync(const void *that);

WireSyncReturn wire_NonCloneSimpleTwinSync_instance_method_arg_own_twin_sync(const void *that);

WireSyncReturn wire_NonCloneSimpleTwinSync_instance_method_return_own_twin_sync(const void *that);

WireSyncReturn wire_NonCloneSimpleTwinSync_new_custom_name_twin_sync(void);

WireSyncReturn wire_NonCloneSimpleTwinSync_new_twin_sync(void);

WireSyncReturn wire_NonCloneSimpleTwinSync_static_method_arg_borrow_twin_sync(const void *arg);

WireSyncReturn wire_NonCloneSimpleTwinSync_static_method_arg_mut_borrow_twin_sync(const void *arg);

WireSyncReturn wire_NonCloneSimpleTwinSync_static_method_arg_own_twin_sync(const void *arg);

WireSyncReturn wire_NonCloneSimpleTwinSync_static_method_return_own_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_arg_borrow_twin_sync(const void *arg, int32_t expect);

WireSyncReturn wire_rust_auto_opaque_arg_mut_borrow_twin_sync(const void *arg,
                                                              int32_t expect,
                                                              int32_t adder);

WireSyncReturn wire_rust_auto_opaque_arg_own_and_return_own_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_arg_own_twin_sync(const void *arg, int32_t expect);

WireSyncReturn wire_rust_auto_opaque_callable_arg_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_callable_return_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync(const void *a,
                                                                     struct wire_list_prim_u_8 *b);

WireSyncReturn wire_rust_auto_opaque_plus_sign_arg_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_plus_sign_return_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_return_own_twin_sync(int32_t initial);

WireSyncReturn wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync(const void *arg);

WireSyncReturn wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync(const void *arg,
                                                                       struct wire_list_prim_u_8 *expect);

WireSyncReturn wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync(const void *arg,
                                                                           struct wire_list_prim_u_8 *expect);

WireSyncReturn wire_rust_auto_opaque_trait_object_arg_own_twin_sync(const void *arg,
                                                                    struct wire_list_prim_u_8 *expect);

WireSyncReturn wire_rust_auto_opaque_trait_object_return_own_one_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_trait_object_return_own_two_twin_sync(void);

WireSyncReturn wire_rust_auto_opaque_two_args_twin_sync(const void *a, const void *b);

void wire_create_array_opaque_enum_twin_rust_async(int64_t port_);

void wire_create_nested_opaque_twin_rust_async(int64_t port_);

void wire_create_opaque_twin_rust_async(int64_t port_);

void wire_create_option_opaque_twin_rust_async(int64_t port_, const void **opaque);

void wire_create_sync_opaque_twin_rust_async(int64_t port_);

void wire_frb_generator_test_twin_rust_async(int64_t port_);

void wire_opaque_array_run_twin_rust_async(int64_t port_,
                                           struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_array_twin_rust_async(int64_t port_);

void wire_opaque_vec_run_twin_rust_async(int64_t port_,
                                         struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_vec_twin_rust_async(int64_t port_);

void wire_run_enum_opaque_twin_rust_async(int64_t port_,
                                          struct wire_enum_opaque_twin_rust_async *opaque);

void wire_run_nested_opaque_twin_rust_async(int64_t port_,
                                            struct wire_opaque_nested_twin_rust_async *opaque);

void wire_run_non_clone_twin_rust_async(int64_t port_, const void *clone);

void wire_run_opaque_twin_rust_async(int64_t port_, const void *opaque);

void wire_run_opaque_with_delay_twin_rust_async(int64_t port_, const void *opaque);

void wire_unwrap_rust_opaque_twin_rust_async(int64_t port_, const void *opaque);

WireSyncReturn wire_create_array_opaque_enum_twin_sync(void);

WireSyncReturn wire_create_nested_opaque_twin_sync(void);

WireSyncReturn wire_create_opaque_twin_sync(void);

WireSyncReturn wire_create_option_opaque_twin_sync(const void **opaque);

WireSyncReturn wire_create_sync_opaque_twin_sync(void);

WireSyncReturn wire_frb_generator_test_twin_sync(void);

WireSyncReturn wire_opaque_array_run_twin_sync(struct wire_list_RustOpaque_hide_data *data);

WireSyncReturn wire_opaque_array_twin_sync(void);

WireSyncReturn wire_opaque_vec_run_twin_sync(struct wire_list_RustOpaque_hide_data *data);

WireSyncReturn wire_opaque_vec_twin_sync(void);

WireSyncReturn wire_run_enum_opaque_twin_sync(struct wire_enum_opaque_twin_sync *opaque);

WireSyncReturn wire_run_nested_opaque_twin_sync(struct wire_opaque_nested_twin_sync *opaque);

WireSyncReturn wire_run_non_clone_twin_sync(const void *clone);

WireSyncReturn wire_run_opaque_twin_sync(const void *opaque);

WireSyncReturn wire_run_opaque_with_delay_twin_sync(const void *opaque);

WireSyncReturn wire_unwrap_rust_opaque_twin_sync(const void *opaque);

void wire_simple_adder_twin_rust_async(int64_t port_, int32_t a, int32_t b);

WireSyncReturn wire_simple_adder_twin_sync(int32_t a, int32_t b);

void wire_func_stream_return_error_twin_rust_async(int64_t port_);

void wire_func_stream_return_panic_twin_rust_async(int64_t port_);

void wire_func_stream_sink_arg_position_twin_rust_async(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct_twin_rust_async(int64_t port_);

void wire_handle_stream_sink_at_1_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3_twin_rust_async(int64_t port_, uint32_t key, uint32_t max);

void wire_func_struct_with_one_field_twin_rust_async(int64_t port_,
                                                     struct wire_struct_with_one_field_twin_rust_async *arg);

void wire_func_struct_with_two_field_twin_rust_async(int64_t port_,
                                                     struct wire_struct_with_two_field_twin_rust_async *arg);

void wire_func_struct_with_zero_field_twin_rust_async(int64_t port_,
                                                      struct wire_struct_with_zero_field_twin_rust_async *arg);

void wire_func_tuple_struct_with_one_field_twin_rust_async(int64_t port_,
                                                           struct wire_tuple_struct_with_one_field_twin_rust_async *arg);

void wire_func_tuple_struct_with_two_field_twin_rust_async(int64_t port_,
                                                           struct wire_tuple_struct_with_two_field_twin_rust_async *arg);

WireSyncReturn wire_func_struct_with_one_field_twin_sync(struct wire_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_two_field_twin_sync(struct wire_struct_with_two_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_zero_field_twin_sync(struct wire_struct_with_zero_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_one_field_twin_sync(struct wire_tuple_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_two_field_twin_sync(struct wire_tuple_struct_with_two_field_twin_sync *arg);

void wire_test_tuple_2_twin_rust_async(int64_t port_, struct wire_list_record_string_i_32 *value);

void wire_test_tuple_twin_rust_async(int64_t port_, struct wire_record_string_i_32 *value);

WireSyncReturn wire_test_tuple_2_twin_sync(struct wire_list_record_string_i_32 *value);

WireSyncReturn wire_test_tuple_twin_sync(struct wire_record_string_i_32 *value);

void wire_handle_type_alias_id_twin_rust_async(int64_t port_, uint64_t input);

void wire_handle_type_alias_model_twin_rust_async(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id_twin_rust_async(int64_t port_, uint64_t input);

WireSyncReturn wire_handle_type_alias_id_twin_sync(uint64_t input);

WireSyncReturn wire_handle_type_alias_model_twin_sync(uint64_t input);

WireSyncReturn wire_handle_type_nest_alias_id_twin_sync(uint64_t input);

void wire_handle_nested_uuids_twin_rust_async(int64_t port_,
                                              struct wire_feature_uuid_twin_rust_async *ids);

void wire_handle_uuid_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_handle_uuids_twin_rust_async(int64_t port_, struct wire_list_prim_u_8 *ids);

WireSyncReturn wire_handle_nested_uuids_twin_sync(struct wire_feature_uuid_twin_sync *ids);

WireSyncReturn wire_handle_uuid_twin_sync(struct wire_list_prim_u_8 *id);

WireSyncReturn wire_handle_uuids_twin_sync(struct wire_list_prim_u_8 *ids);

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
                                                             struct wire_list_prim_u_8 *b);

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
                                                               struct wire_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_normal(int64_t port_,
                                                                   const void *arg,
                                                                   struct wire_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_arg_own_twin_normal(int64_t port_,
                                                            const void *arg,
                                                            struct wire_list_prim_u_8 *expect);

void wire_rust_auto_opaque_trait_object_return_own_one_twin_normal(int64_t port_);

void wire_rust_auto_opaque_trait_object_return_own_two_twin_normal(int64_t port_);

void wire_rust_auto_opaque_two_args_twin_normal(int64_t port_, const void *a, const void *b);

void wire_create_array_opaque_enum_twin_normal(int64_t port_);

void wire_create_nested_opaque_twin_normal(int64_t port_);

void wire_create_opaque_twin_normal(int64_t port_);

void wire_create_option_opaque_twin_normal(int64_t port_, const void **opaque);

void wire_create_sync_opaque_twin_normal(int64_t port_);

void wire_frb_generator_test_twin_normal(int64_t port_);

void wire_opaque_array_run_twin_normal(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_array_twin_normal(int64_t port_);

void wire_opaque_vec_run_twin_normal(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_vec_twin_normal(int64_t port_);

void wire_run_enum_opaque_twin_normal(int64_t port_, struct wire_enum_opaque_twin_normal *opaque);

void wire_run_nested_opaque_twin_normal(int64_t port_,
                                        struct wire_opaque_nested_twin_normal *opaque);

void wire_run_non_clone_twin_normal(int64_t port_, const void *clone);

void wire_run_opaque_twin_normal(int64_t port_, const void *opaque);

void wire_run_opaque_with_delay_twin_normal(int64_t port_, const void *opaque);

void wire_unwrap_rust_opaque_twin_normal(int64_t port_, const void *opaque);

WireSyncReturn wire_frb_sync_generator_test_twin_normal(void);

WireSyncReturn wire_sync_create_non_clone_twin_normal(void);

WireSyncReturn wire_sync_create_opaque_twin_normal(void);

WireSyncReturn wire_sync_create_sync_opaque_twin_normal(void);

WireSyncReturn wire_sync_option_rust_opaque_twin_normal(void);

WireSyncReturn wire_sync_run_opaque_twin_normal(const void *opaque);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_stream_return_error_twin_normal(int64_t port_);

void wire_func_stream_return_panic_twin_normal(int64_t port_);

void wire_func_stream_sink_arg_position_twin_normal(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct_twin_normal(int64_t port_);

void wire_handle_stream_sink_at_1_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3_twin_normal(int64_t port_, uint32_t key, uint32_t max);

void wire_func_stream_realistic_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

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

struct wire_StringList *new_StringList(int32_t len);

struct wire_application_env *new_box_application_env(void);

int64_t *new_box_autoadd_Chrono_Utc(int64_t value);

struct wire_DartOpaque *new_box_autoadd_DartOpaque(void);

const void **new_box_autoadd_RustOpaque_hide_data(const void *value);

struct wire_a_twin_normal *new_box_autoadd_a_twin_normal(void);

struct wire_a_twin_rust_async *new_box_autoadd_a_twin_rust_async(void);

struct wire_a_twin_sync *new_box_autoadd_a_twin_sync(void);

struct wire_abc_twin_normal *new_box_autoadd_abc_twin_normal(void);

struct wire_abc_twin_rust_async *new_box_autoadd_abc_twin_rust_async(void);

struct wire_abc_twin_sync *new_box_autoadd_abc_twin_sync(void);

struct wire_application_env *new_box_autoadd_application_env(void);

struct wire_application_settings *new_box_autoadd_application_settings(void);

struct wire_attribute_twin_normal *new_box_autoadd_attribute_twin_normal(void);

struct wire_attribute_twin_rust_async *new_box_autoadd_attribute_twin_rust_async(void);

struct wire_attribute_twin_sync *new_box_autoadd_attribute_twin_sync(void);

struct wire_b_twin_normal *new_box_autoadd_b_twin_normal(void);

struct wire_b_twin_rust_async *new_box_autoadd_b_twin_rust_async(void);

struct wire_b_twin_sync *new_box_autoadd_b_twin_sync(void);

bool *new_box_autoadd_bool(bool value);

struct wire_c_twin_normal *new_box_autoadd_c_twin_normal(void);

struct wire_c_twin_rust_async *new_box_autoadd_c_twin_rust_async(void);

struct wire_c_twin_sync *new_box_autoadd_c_twin_sync(void);

struct wire_concatenate_with_twin_normal *new_box_autoadd_concatenate_with_twin_normal(void);

struct wire_concatenate_with_twin_rust_async *new_box_autoadd_concatenate_with_twin_rust_async(void);

struct wire_concatenate_with_twin_sync *new_box_autoadd_concatenate_with_twin_sync(void);

struct wire_custom_nested_error_inner_twin_normal *new_box_autoadd_custom_nested_error_inner_twin_normal(void);

struct wire_custom_nested_error_inner_twin_rust_async *new_box_autoadd_custom_nested_error_inner_twin_rust_async(void);

struct wire_custom_nested_error_inner_twin_sync *new_box_autoadd_custom_nested_error_inner_twin_sync(void);

struct wire_custom_nested_error_outer_twin_normal *new_box_autoadd_custom_nested_error_outer_twin_normal(void);

struct wire_custom_nested_error_outer_twin_rust_async *new_box_autoadd_custom_nested_error_outer_twin_rust_async(void);

struct wire_custom_nested_error_outer_twin_sync *new_box_autoadd_custom_nested_error_outer_twin_sync(void);

struct wire_custom_struct_error_twin_normal *new_box_autoadd_custom_struct_error_twin_normal(void);

struct wire_custom_struct_error_twin_rust_async *new_box_autoadd_custom_struct_error_twin_rust_async(void);

struct wire_custom_struct_error_twin_sync *new_box_autoadd_custom_struct_error_twin_sync(void);

struct wire_custom_struct_twin_normal *new_box_autoadd_custom_struct_twin_normal(void);

struct wire_custom_struct_twin_rust_async *new_box_autoadd_custom_struct_twin_rust_async(void);

struct wire_custom_struct_twin_sync *new_box_autoadd_custom_struct_twin_sync(void);

struct wire_customized_twin_normal *new_box_autoadd_customized_twin_normal(void);

struct wire_customized_twin_rust_async *new_box_autoadd_customized_twin_rust_async(void);

struct wire_customized_twin_sync *new_box_autoadd_customized_twin_sync(void);

struct wire_dart_opaque_nested_twin_normal *new_box_autoadd_dart_opaque_nested_twin_normal(void);

struct wire_dart_opaque_nested_twin_rust_async *new_box_autoadd_dart_opaque_nested_twin_rust_async(void);

struct wire_dart_opaque_nested_twin_sync *new_box_autoadd_dart_opaque_nested_twin_sync(void);

struct wire_empty_twin_normal *new_box_autoadd_empty_twin_normal(void);

struct wire_empty_twin_rust_async *new_box_autoadd_empty_twin_rust_async(void);

struct wire_empty_twin_sync *new_box_autoadd_empty_twin_sync(void);

struct wire_enum_dart_opaque_twin_normal *new_box_autoadd_enum_dart_opaque_twin_normal(void);

struct wire_enum_dart_opaque_twin_rust_async *new_box_autoadd_enum_dart_opaque_twin_rust_async(void);

struct wire_enum_dart_opaque_twin_sync *new_box_autoadd_enum_dart_opaque_twin_sync(void);

struct wire_enum_opaque_twin_normal *new_box_autoadd_enum_opaque_twin_normal(void);

struct wire_enum_opaque_twin_rust_async *new_box_autoadd_enum_opaque_twin_rust_async(void);

struct wire_enum_opaque_twin_sync *new_box_autoadd_enum_opaque_twin_sync(void);

struct wire_enum_with_item_mixed_twin_normal *new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_enum_with_item_mixed_twin_rust_async *new_box_autoadd_enum_with_item_mixed_twin_rust_async(void);

struct wire_enum_with_item_mixed_twin_sync *new_box_autoadd_enum_with_item_mixed_twin_sync(void);

struct wire_enum_with_item_struct_twin_normal *new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_enum_with_item_struct_twin_rust_async *new_box_autoadd_enum_with_item_struct_twin_rust_async(void);

struct wire_enum_with_item_struct_twin_sync *new_box_autoadd_enum_with_item_struct_twin_sync(void);

struct wire_enum_with_item_tuple_twin_normal *new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_enum_with_item_tuple_twin_rust_async *new_box_autoadd_enum_with_item_tuple_twin_rust_async(void);

struct wire_enum_with_item_tuple_twin_sync *new_box_autoadd_enum_with_item_tuple_twin_sync(void);

struct wire_event_twin_normal *new_box_autoadd_event_twin_normal(void);

struct wire_event_twin_rust_async *new_box_autoadd_event_twin_rust_async(void);

struct wire_event_twin_sync *new_box_autoadd_event_twin_sync(void);

struct wire_exotic_optionals_twin_normal *new_box_autoadd_exotic_optionals_twin_normal(void);

struct wire_exotic_optionals_twin_rust_async *new_box_autoadd_exotic_optionals_twin_rust_async(void);

struct wire_exotic_optionals_twin_sync *new_box_autoadd_exotic_optionals_twin_sync(void);

float *new_box_autoadd_f_32(float value);

double *new_box_autoadd_f_64(double value);

struct wire_feature_chrono_twin_normal *new_box_autoadd_feature_chrono_twin_normal(void);

struct wire_feature_chrono_twin_rust_async *new_box_autoadd_feature_chrono_twin_rust_async(void);

struct wire_feature_chrono_twin_sync *new_box_autoadd_feature_chrono_twin_sync(void);

struct wire_feature_uuid_twin_normal *new_box_autoadd_feature_uuid_twin_normal(void);

struct wire_feature_uuid_twin_rust_async *new_box_autoadd_feature_uuid_twin_rust_async(void);

struct wire_feature_uuid_twin_sync *new_box_autoadd_feature_uuid_twin_sync(void);

struct wire_feed_id_twin_normal *new_box_autoadd_feed_id_twin_normal(void);

struct wire_feed_id_twin_rust_async *new_box_autoadd_feed_id_twin_rust_async(void);

struct wire_feed_id_twin_sync *new_box_autoadd_feed_id_twin_sync(void);

int16_t *new_box_autoadd_i_16(int16_t value);

int32_t *new_box_autoadd_i_32(int32_t value);

int64_t *new_box_autoadd_i_64(int64_t value);

int8_t *new_box_autoadd_i_8(int8_t value);

struct wire_kitchen_sink_twin_normal *new_box_autoadd_kitchen_sink_twin_normal(void);

struct wire_kitchen_sink_twin_rust_async *new_box_autoadd_kitchen_sink_twin_rust_async(void);

struct wire_kitchen_sink_twin_sync *new_box_autoadd_kitchen_sink_twin_sync(void);

struct wire_macro_struct *new_box_autoadd_macro_struct(void);

struct wire_measure_twin_normal *new_box_autoadd_measure_twin_normal(void);

struct wire_measure_twin_rust_async *new_box_autoadd_measure_twin_rust_async(void);

struct wire_measure_twin_sync *new_box_autoadd_measure_twin_sync(void);

struct wire_message_id_twin_normal *new_box_autoadd_message_id_twin_normal(void);

struct wire_message_id_twin_rust_async *new_box_autoadd_message_id_twin_rust_async(void);

struct wire_message_id_twin_sync *new_box_autoadd_message_id_twin_sync(void);

struct wire_my_nested_struct_twin_normal *new_box_autoadd_my_nested_struct_twin_normal(void);

struct wire_my_nested_struct_twin_rust_async *new_box_autoadd_my_nested_struct_twin_rust_async(void);

struct wire_my_nested_struct_twin_sync *new_box_autoadd_my_nested_struct_twin_sync(void);

struct wire_my_size *new_box_autoadd_my_size(void);

struct wire_my_struct *new_box_autoadd_my_struct(void);

struct wire_my_tree_node_twin_normal *new_box_autoadd_my_tree_node_twin_normal(void);

struct wire_my_tree_node_twin_rust_async *new_box_autoadd_my_tree_node_twin_rust_async(void);

struct wire_my_tree_node_twin_sync *new_box_autoadd_my_tree_node_twin_sync(void);

struct wire_new_type_int_twin_normal *new_box_autoadd_new_type_int_twin_normal(void);

struct wire_new_type_int_twin_rust_async *new_box_autoadd_new_type_int_twin_rust_async(void);

struct wire_new_type_int_twin_sync *new_box_autoadd_new_type_int_twin_sync(void);

struct wire_note_twin_normal *new_box_autoadd_note_twin_normal(void);

struct wire_note_twin_rust_async *new_box_autoadd_note_twin_rust_async(void);

struct wire_note_twin_sync *new_box_autoadd_note_twin_sync(void);

struct wire_numbers *new_box_autoadd_numbers(void);

struct wire_opaque_nested_twin_normal *new_box_autoadd_opaque_nested_twin_normal(void);

struct wire_opaque_nested_twin_rust_async *new_box_autoadd_opaque_nested_twin_rust_async(void);

struct wire_opaque_nested_twin_sync *new_box_autoadd_opaque_nested_twin_sync(void);

struct wire_opt_vecs_twin_normal *new_box_autoadd_opt_vecs_twin_normal(void);

struct wire_opt_vecs_twin_rust_async *new_box_autoadd_opt_vecs_twin_rust_async(void);

struct wire_opt_vecs_twin_sync *new_box_autoadd_opt_vecs_twin_sync(void);

struct wire_record_string_i_32 *new_box_autoadd_record_string_i_32(void);

struct wire_sequences *new_box_autoadd_sequences(void);

struct wire_some_struct_twin_normal *new_box_autoadd_some_struct_twin_normal(void);

struct wire_some_struct_twin_rust_async *new_box_autoadd_some_struct_twin_rust_async(void);

struct wire_some_struct_twin_sync *new_box_autoadd_some_struct_twin_sync(void);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_struct_with_comments_twin_rust_async *new_box_autoadd_struct_with_comments_twin_rust_async(void);

struct wire_struct_with_comments_twin_sync *new_box_autoadd_struct_with_comments_twin_sync(void);

struct wire_struct_with_enum_twin_normal *new_box_autoadd_struct_with_enum_twin_normal(void);

struct wire_struct_with_enum_twin_rust_async *new_box_autoadd_struct_with_enum_twin_rust_async(void);

struct wire_struct_with_enum_twin_sync *new_box_autoadd_struct_with_enum_twin_sync(void);

struct wire_struct_with_one_field_twin_normal *new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_struct_with_one_field_twin_rust_async *new_box_autoadd_struct_with_one_field_twin_rust_async(void);

struct wire_struct_with_one_field_twin_sync *new_box_autoadd_struct_with_one_field_twin_sync(void);

struct wire_struct_with_two_field_twin_normal *new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_struct_with_two_field_twin_rust_async *new_box_autoadd_struct_with_two_field_twin_rust_async(void);

struct wire_struct_with_two_field_twin_sync *new_box_autoadd_struct_with_two_field_twin_sync(void);

struct wire_struct_with_zero_field_twin_normal *new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_struct_with_zero_field_twin_rust_async *new_box_autoadd_struct_with_zero_field_twin_rust_async(void);

struct wire_struct_with_zero_field_twin_sync *new_box_autoadd_struct_with_zero_field_twin_sync(void);

struct wire_sum_with_twin_normal *new_box_autoadd_sum_with_twin_normal(void);

struct wire_sum_with_twin_rust_async *new_box_autoadd_sum_with_twin_rust_async(void);

struct wire_sum_with_twin_sync *new_box_autoadd_sum_with_twin_sync(void);

struct wire_test_id_twin_normal *new_box_autoadd_test_id_twin_normal(void);

struct wire_test_id_twin_rust_async *new_box_autoadd_test_id_twin_rust_async(void);

struct wire_test_id_twin_sync *new_box_autoadd_test_id_twin_sync(void);

struct wire_tuple_struct_with_one_field_twin_normal *new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_tuple_struct_with_one_field_twin_rust_async *new_box_autoadd_tuple_struct_with_one_field_twin_rust_async(void);

struct wire_tuple_struct_with_one_field_twin_sync *new_box_autoadd_tuple_struct_with_one_field_twin_sync(void);

struct wire_tuple_struct_with_two_field_twin_normal *new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_tuple_struct_with_two_field_twin_rust_async *new_box_autoadd_tuple_struct_with_two_field_twin_rust_async(void);

struct wire_tuple_struct_with_two_field_twin_sync *new_box_autoadd_tuple_struct_with_two_field_twin_sync(void);

uint16_t *new_box_autoadd_u_16(uint16_t value);

uint32_t *new_box_autoadd_u_32(uint32_t value);

uint64_t *new_box_autoadd_u_64(uint64_t value);

uint8_t *new_box_autoadd_u_8(uint8_t value);

struct wire_user_id_twin_normal *new_box_autoadd_user_id_twin_normal(void);

struct wire_user_id_twin_rust_async *new_box_autoadd_user_id_twin_rust_async(void);

struct wire_user_id_twin_sync *new_box_autoadd_user_id_twin_sync(void);

int32_t *new_box_autoadd_weekdays_twin_normal(int32_t value);

int32_t *new_box_autoadd_weekdays_twin_rust_async(int32_t value);

int32_t *new_box_autoadd_weekdays_twin_sync(int32_t value);

struct wire_blob_twin_normal *new_box_blob_twin_normal(void);

struct wire_blob_twin_rust_async *new_box_blob_twin_rust_async(void);

struct wire_blob_twin_sync *new_box_blob_twin_sync(void);

bool *new_box_bool(bool value);

struct wire_distance_twin_normal *new_box_distance_twin_normal(void);

struct wire_distance_twin_rust_async *new_box_distance_twin_rust_async(void);

struct wire_distance_twin_sync *new_box_distance_twin_sync(void);

struct wire_exotic_optionals_twin_normal *new_box_exotic_optionals_twin_normal(void);

struct wire_exotic_optionals_twin_rust_async *new_box_exotic_optionals_twin_rust_async(void);

struct wire_exotic_optionals_twin_sync *new_box_exotic_optionals_twin_sync(void);

double *new_box_f_64(double value);

int32_t *new_box_i_32(int32_t value);

int64_t *new_box_i_64(int64_t value);

int8_t *new_box_i_8(int8_t value);

struct wire_kitchen_sink_twin_normal *new_box_kitchen_sink_twin_normal(void);

struct wire_kitchen_sink_twin_rust_async *new_box_kitchen_sink_twin_rust_async(void);

struct wire_kitchen_sink_twin_sync *new_box_kitchen_sink_twin_sync(void);

struct wire_my_size *new_box_my_size(void);

struct wire_speed_twin_normal *new_box_speed_twin_normal(void);

struct wire_speed_twin_rust_async *new_box_speed_twin_rust_async(void);

struct wire_speed_twin_sync *new_box_speed_twin_sync(void);

uint8_t *new_box_u_8(uint8_t value);

int32_t *new_box_weekdays_twin_normal(int32_t value);

int32_t *new_box_weekdays_twin_rust_async(int32_t value);

int32_t *new_box_weekdays_twin_sync(int32_t value);

struct wire_list_DartOpaque *new_list_DartOpaque(int32_t len);

struct wire_list_RustOpaque_hide_data *new_list_RustOpaque_hide_data(int32_t len);

struct wire_list_application_env_var *new_list_application_env_var(int32_t len);

struct wire_list_attribute_twin_normal *new_list_attribute_twin_normal(int32_t len);

struct wire_list_attribute_twin_rust_async *new_list_attribute_twin_rust_async(int32_t len);

struct wire_list_attribute_twin_sync *new_list_attribute_twin_sync(int32_t len);

struct wire_list_bool *new_list_bool(int32_t len);

struct wire_list_my_size *new_list_my_size(int32_t len);

struct wire_list_my_tree_node_twin_normal *new_list_my_tree_node_twin_normal(int32_t len);

struct wire_list_my_tree_node_twin_rust_async *new_list_my_tree_node_twin_rust_async(int32_t len);

struct wire_list_my_tree_node_twin_sync *new_list_my_tree_node_twin_sync(int32_t len);

struct wire_list_opt_String *new_list_opt_String(int32_t len);

struct wire_list_opt_box_autoadd_attribute_twin_normal *new_list_opt_box_autoadd_attribute_twin_normal(int32_t len);

struct wire_list_opt_box_autoadd_attribute_twin_rust_async *new_list_opt_box_autoadd_attribute_twin_rust_async(int32_t len);

struct wire_list_opt_box_autoadd_attribute_twin_sync *new_list_opt_box_autoadd_attribute_twin_sync(int32_t len);

struct wire_list_opt_box_autoadd_i_32 *new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_list_opt_box_autoadd_weekdays_twin_normal *new_list_opt_box_autoadd_weekdays_twin_normal(int32_t len);

struct wire_list_opt_box_autoadd_weekdays_twin_rust_async *new_list_opt_box_autoadd_weekdays_twin_rust_async(int32_t len);

struct wire_list_opt_box_autoadd_weekdays_twin_sync *new_list_opt_box_autoadd_weekdays_twin_sync(int32_t len);

struct wire_list_opt_list_prim_i_32 *new_list_opt_list_prim_i_32(int32_t len);

struct wire_list_prim_f_32 *new_list_prim_f_32(int32_t len);

struct wire_list_prim_f_64 *new_list_prim_f_64(int32_t len);

struct wire_list_prim_i_16 *new_list_prim_i_16(int32_t len);

struct wire_list_prim_i_32 *new_list_prim_i_32(int32_t len);

struct wire_list_prim_i_64 *new_list_prim_i_64(int32_t len);

struct wire_list_prim_i_8 *new_list_prim_i_8(int32_t len);

struct wire_list_prim_u_16 *new_list_prim_u_16(int32_t len);

struct wire_list_prim_u_32 *new_list_prim_u_32(int32_t len);

struct wire_list_prim_u_64 *new_list_prim_u_64(int32_t len);

struct wire_list_prim_u_8 *new_list_prim_u_8(int32_t len);

struct wire_list_record_string_i_32 *new_list_record_string_i_32(int32_t len);

struct wire_list_test_id_twin_normal *new_list_test_id_twin_normal(int32_t len);

struct wire_list_test_id_twin_rust_async *new_list_test_id_twin_rust_async(int32_t len);

struct wire_list_test_id_twin_sync *new_list_test_id_twin_sync(int32_t len);

struct wire_list_weekdays_twin_normal *new_list_weekdays_twin_normal(int32_t len);

struct wire_list_weekdays_twin_rust_async *new_list_weekdays_twin_rust_async(int32_t len);

struct wire_list_weekdays_twin_sync *new_list_weekdays_twin_sync(int32_t len);

void rust_arc_increment_strong_count_RustOpaque_MutexHideData(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_MutexHideData(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_RwLockHideData(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_RwLockHideData(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSync(const void *ptr);

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

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync(const void *ptr);

union AbcTwinNormalKind *inflate_AbcTwinNormal_A(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_B(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_C(void);

union AbcTwinNormalKind *inflate_AbcTwinNormal_JustInt(void);

union AbcTwinRustAsyncKind *inflate_AbcTwinRustAsync_A(void);

union AbcTwinRustAsyncKind *inflate_AbcTwinRustAsync_B(void);

union AbcTwinRustAsyncKind *inflate_AbcTwinRustAsync_C(void);

union AbcTwinRustAsyncKind *inflate_AbcTwinRustAsync_JustInt(void);

union AbcTwinSyncKind *inflate_AbcTwinSync_A(void);

union AbcTwinSyncKind *inflate_AbcTwinSync_B(void);

union AbcTwinSyncKind *inflate_AbcTwinSync_C(void);

union AbcTwinSyncKind *inflate_AbcTwinSync_JustInt(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Three(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Four(void);

union CustomNestedErrorInnerTwinRustAsyncKind *inflate_CustomNestedErrorInnerTwinRustAsync_Three(void);

union CustomNestedErrorInnerTwinRustAsyncKind *inflate_CustomNestedErrorInnerTwinRustAsync_Four(void);

union CustomNestedErrorInnerTwinSyncKind *inflate_CustomNestedErrorInnerTwinSync_Three(void);

union CustomNestedErrorInnerTwinSyncKind *inflate_CustomNestedErrorInnerTwinSync_Four(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_One(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_Two(void);

union CustomNestedErrorOuterTwinRustAsyncKind *inflate_CustomNestedErrorOuterTwinRustAsync_One(void);

union CustomNestedErrorOuterTwinRustAsyncKind *inflate_CustomNestedErrorOuterTwinRustAsync_Two(void);

union CustomNestedErrorOuterTwinSyncKind *inflate_CustomNestedErrorOuterTwinSync_One(void);

union CustomNestedErrorOuterTwinSyncKind *inflate_CustomNestedErrorOuterTwinSync_Two(void);

union DistanceTwinNormalKind *inflate_DistanceTwinNormal_Map(void);

union DistanceTwinRustAsyncKind *inflate_DistanceTwinRustAsync_Map(void);

union DistanceTwinSyncKind *inflate_DistanceTwinSync_Map(void);

union EnumDartOpaqueTwinNormalKind *inflate_EnumDartOpaqueTwinNormal_Primitive(void);

union EnumDartOpaqueTwinNormalKind *inflate_EnumDartOpaqueTwinNormal_Opaque(void);

union EnumDartOpaqueTwinRustAsyncKind *inflate_EnumDartOpaqueTwinRustAsync_Primitive(void);

union EnumDartOpaqueTwinRustAsyncKind *inflate_EnumDartOpaqueTwinRustAsync_Opaque(void);

union EnumDartOpaqueTwinSyncKind *inflate_EnumDartOpaqueTwinSync_Primitive(void);

union EnumDartOpaqueTwinSyncKind *inflate_EnumDartOpaqueTwinSync_Opaque(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Struct(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Primitive(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_TraitObj(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_Mutex(void);

union EnumOpaqueTwinNormalKind *inflate_EnumOpaqueTwinNormal_RwLock(void);

union EnumOpaqueTwinRustAsyncKind *inflate_EnumOpaqueTwinRustAsync_Struct(void);

union EnumOpaqueTwinRustAsyncKind *inflate_EnumOpaqueTwinRustAsync_Primitive(void);

union EnumOpaqueTwinRustAsyncKind *inflate_EnumOpaqueTwinRustAsync_TraitObj(void);

union EnumOpaqueTwinRustAsyncKind *inflate_EnumOpaqueTwinRustAsync_Mutex(void);

union EnumOpaqueTwinRustAsyncKind *inflate_EnumOpaqueTwinRustAsync_RwLock(void);

union EnumOpaqueTwinSyncKind *inflate_EnumOpaqueTwinSync_Struct(void);

union EnumOpaqueTwinSyncKind *inflate_EnumOpaqueTwinSync_Primitive(void);

union EnumOpaqueTwinSyncKind *inflate_EnumOpaqueTwinSync_TraitObj(void);

union EnumOpaqueTwinSyncKind *inflate_EnumOpaqueTwinSync_Mutex(void);

union EnumOpaqueTwinSyncKind *inflate_EnumOpaqueTwinSync_RwLock(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_B(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_C(void);

union EnumWithItemMixedTwinRustAsyncKind *inflate_EnumWithItemMixedTwinRustAsync_B(void);

union EnumWithItemMixedTwinRustAsyncKind *inflate_EnumWithItemMixedTwinRustAsync_C(void);

union EnumWithItemMixedTwinSyncKind *inflate_EnumWithItemMixedTwinSync_B(void);

union EnumWithItemMixedTwinSyncKind *inflate_EnumWithItemMixedTwinSync_C(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_A(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_B(void);

union EnumWithItemStructTwinRustAsyncKind *inflate_EnumWithItemStructTwinRustAsync_A(void);

union EnumWithItemStructTwinRustAsyncKind *inflate_EnumWithItemStructTwinRustAsync_B(void);

union EnumWithItemStructTwinSyncKind *inflate_EnumWithItemStructTwinSync_A(void);

union EnumWithItemStructTwinSyncKind *inflate_EnumWithItemStructTwinSync_B(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_A(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_B(void);

union EnumWithItemTupleTwinRustAsyncKind *inflate_EnumWithItemTupleTwinRustAsync_A(void);

union EnumWithItemTupleTwinRustAsyncKind *inflate_EnumWithItemTupleTwinRustAsync_B(void);

union EnumWithItemTupleTwinSyncKind *inflate_EnumWithItemTupleTwinSync_A(void);

union EnumWithItemTupleTwinSyncKind *inflate_EnumWithItemTupleTwinSync_B(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Primitives(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Nested(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Optional(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Buffer(void);

union KitchenSinkTwinNormalKind *inflate_KitchenSinkTwinNormal_Enums(void);

union KitchenSinkTwinRustAsyncKind *inflate_KitchenSinkTwinRustAsync_Primitives(void);

union KitchenSinkTwinRustAsyncKind *inflate_KitchenSinkTwinRustAsync_Nested(void);

union KitchenSinkTwinRustAsyncKind *inflate_KitchenSinkTwinRustAsync_Optional(void);

union KitchenSinkTwinRustAsyncKind *inflate_KitchenSinkTwinRustAsync_Buffer(void);

union KitchenSinkTwinRustAsyncKind *inflate_KitchenSinkTwinRustAsync_Enums(void);

union KitchenSinkTwinSyncKind *inflate_KitchenSinkTwinSync_Primitives(void);

union KitchenSinkTwinSyncKind *inflate_KitchenSinkTwinSync_Nested(void);

union KitchenSinkTwinSyncKind *inflate_KitchenSinkTwinSync_Optional(void);

union KitchenSinkTwinSyncKind *inflate_KitchenSinkTwinSync_Buffer(void);

union KitchenSinkTwinSyncKind *inflate_KitchenSinkTwinSync_Enums(void);

union MeasureTwinNormalKind *inflate_MeasureTwinNormal_Speed(void);

union MeasureTwinNormalKind *inflate_MeasureTwinNormal_Distance(void);

union MeasureTwinRustAsyncKind *inflate_MeasureTwinRustAsync_Speed(void);

union MeasureTwinRustAsyncKind *inflate_MeasureTwinRustAsync_Distance(void);

union MeasureTwinSyncKind *inflate_MeasureTwinSync_Speed(void);

union MeasureTwinSyncKind *inflate_MeasureTwinSync_Distance(void);

union SpeedTwinNormalKind *inflate_SpeedTwinNormal_GPS(void);

union SpeedTwinRustAsyncKind *inflate_SpeedTwinRustAsync_GPS(void);

union SpeedTwinSyncKind *inflate_SpeedTwinSync_GPS(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinNormal_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinRustAsync_C);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinRustAsync_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinSync_A);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinSync_C);
    dummy_var ^= ((int64_t) (void*) inflate_AbcTwinSync_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinRustAsync_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinRustAsync_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinSync_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinSync_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_Two);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinRustAsync_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinRustAsync_Two);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinSync_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinSync_Two);
    dummy_var ^= ((int64_t) (void*) inflate_DistanceTwinNormal_Map);
    dummy_var ^= ((int64_t) (void*) inflate_DistanceTwinRustAsync_Map);
    dummy_var ^= ((int64_t) (void*) inflate_DistanceTwinSync_Map);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinNormal_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinRustAsync_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinRustAsync_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinSync_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaqueTwinSync_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinNormal_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinRustAsync_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinRustAsync_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinRustAsync_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinRustAsync_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinRustAsync_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinSync_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinSync_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinSync_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinSync_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaqueTwinSync_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinRustAsync_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinSync_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinSync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinRustAsync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinRustAsync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinSync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinNormal_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinRustAsync_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinRustAsync_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinRustAsync_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinRustAsync_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinRustAsync_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinSync_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinSync_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinSync_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinSync_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSinkTwinSync_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinNormal_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinNormal_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinRustAsync_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinRustAsync_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinSync_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_MeasureTwinSync_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_SpeedTwinNormal_GPS);
    dummy_var ^= ((int64_t) (void*) inflate_SpeedTwinRustAsync_GPS);
    dummy_var ^= ((int64_t) (void*) inflate_SpeedTwinSync_GPS);
    dummy_var ^= ((int64_t) (void*) new_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_StringList);
    dummy_var ^= ((int64_t) (void*) new_box_application_env);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_env);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_settings);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_macro_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_numbers);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sequences);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_bool);
    dummy_var ^= ((int64_t) (void*) new_box_distance_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_distance_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_distance_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_speed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_speed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_speed_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_list_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_list_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_list_application_env_var);
    dummy_var ^= ((int64_t) (void*) new_list_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_list_bool);
    dummy_var ^= ((int64_t) (void*) new_list_my_size);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_list_opt_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_f_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_f_64);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_16);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_64);
    dummy_var ^= ((int64_t) (void*) new_list_prim_i_8);
    dummy_var ^= ((int64_t) (void*) new_list_prim_u_16);
    dummy_var ^= ((int64_t) (void*) new_list_prim_u_32);
    dummy_var ^= ((int64_t) (void*) new_list_prim_u_64);
    dummy_var ^= ((int64_t) (void*) new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) new_list_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays_twin_sync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_box_dynDartDebugTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinRustAsync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_box_dynDartDebugTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_frb_opaque_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_frb_opaque_sync_return);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_i_32);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_non_clone_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_non_send_hide_data);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynFnStringStringSendSyncUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynHelloTraitTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinNormalSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynMyTraitTwinSyncSendSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockNonCloneSimpleTwinSync);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinNormal);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockStructWithGoodAndOpaqueFieldTwinSync);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_concatenate_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinNormal_new_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_concatenate_static_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_concatenate_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_single_arg_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_static_stream_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_at_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_handle_some_stream_sink_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWithTwinRustAsync_new_twin_rust_async);
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
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_new_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_nonstatic_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_static_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinRustAsync_static_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinNormal_as_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinRustAsync_as_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_EventTwinSync_as_string_twin_sync);
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
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_new_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_non_static_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_non_static_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_static_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinRustAsync_static_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_new_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsync_instance_method_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinRustAsync_static_method_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_instance_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_static_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinNormal_sum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinRustAsync_sum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_SumWithTwinSync_sum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_another_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_input_bytes_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_output_bytes_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_benchmark_void_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_event_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_duration_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_first_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_async_simple_add);
    dummy_var ^= ((int64_t) (void*) wire_func_async_void);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_macro_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_realistic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_message_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_last_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_nested_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_print_note_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_return_non_droppable_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_and_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_callable_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_normal_and_opaque_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_arg_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_plus_sign_return_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_struct_with_good_and_opaque_field_return_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_mut_borrow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_arg_own_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_one_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_trait_object_return_own_two_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_rust_auto_opaque_two_args_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_loopback);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_one_arg);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_return);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_simple);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_two_args);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_with_dart_opaque_arg);
    dummy_var ^= ((int64_t) (void*) wire_rust_call_dart_with_dart_opaque_result);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_rust_async);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid_twin_sync);
    return dummy_var;
}
