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

typedef struct wire_test_id {
  struct wire_list_prim_i_32 *field0;
} wire_test_id;

typedef struct wire_list_prim_f_64 {
  double *ptr;
  int32_t len;
} wire_list_prim_f_64;

typedef struct wire_list_test_id {
  struct wire_test_id *ptr;
  int32_t len;
} wire_list_test_id;

typedef struct wire_feed_id {
  struct wire_list_prim_u_8 *field0;
} wire_feed_id;

typedef struct wire_blob {
  struct wire_list_prim_u_8 *field0;
} wire_blob;

typedef struct wire_message_id {
  struct wire_list_prim_u_8 *field0;
} wire_message_id;

typedef struct wire_customized {
  struct wire_list_prim_u_8 *final_field;
  struct wire_list_prim_u_8 *non_final_field;
} wire_customized;

typedef struct wire_user_id {
  uint32_t value;
} wire_user_id;

typedef struct wire_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_list_prim_i_64;

typedef struct wire_feature_chrono {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_feature_chrono;

typedef struct wire_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_normal;

typedef struct wire_DartOpaque {
  int64_t port;
  uintptr_t handle;
} wire_DartOpaque;

typedef struct wire_EnumDartOpaque_Primitive {
  int32_t field0;
} wire_EnumDartOpaque_Primitive;

typedef struct wire_EnumDartOpaque_Opaque {
  struct wire_DartOpaque field0;
} wire_EnumDartOpaque_Opaque;

typedef union EnumDartOpaqueKind {
  struct wire_EnumDartOpaque_Primitive *Primitive;
  struct wire_EnumDartOpaque_Opaque *Opaque;
} EnumDartOpaqueKind;

typedef struct wire_enum_dart_opaque {
  int32_t tag;
  union EnumDartOpaqueKind *kind;
} wire_enum_dart_opaque;

typedef struct wire_dart_opaque_nested {
  struct wire_DartOpaque first;
  struct wire_DartOpaque second;
} wire_dart_opaque_nested;

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

typedef struct wire_KitchenSink_Empty {

} wire_KitchenSink_Empty;

typedef struct wire_KitchenSink_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_KitchenSink_Primitives;

typedef struct wire_KitchenSink_Nested {
  int32_t field0;
  struct wire_kitchen_sink *field1;
} wire_KitchenSink_Nested;

typedef struct wire_KitchenSink_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_KitchenSink_Optional;

typedef struct wire_KitchenSink_Buffer {
  struct wire_list_prim_u_8 *field0;
} wire_KitchenSink_Buffer;

typedef struct wire_KitchenSink_Enums {
  int32_t field0;
} wire_KitchenSink_Enums;

typedef union KitchenSinkKind {
  struct wire_KitchenSink_Empty *Empty;
  struct wire_KitchenSink_Primitives *Primitives;
  struct wire_KitchenSink_Nested *Nested;
  struct wire_KitchenSink_Optional *Optional;
  struct wire_KitchenSink_Buffer *Buffer;
  struct wire_KitchenSink_Enums *Enums;
} KitchenSinkKind;

typedef struct wire_kitchen_sink {
  int32_t tag;
  union KitchenSinkKind *kind;
} wire_kitchen_sink;

typedef struct wire_Speed_Unknown {

} wire_Speed_Unknown;

typedef struct wire_Speed_GPS {
  double field0;
} wire_Speed_GPS;

typedef union SpeedKind {
  struct wire_Speed_Unknown *Unknown;
  struct wire_Speed_GPS *GPS;
} SpeedKind;

typedef struct wire_speed {
  int32_t tag;
  union SpeedKind *kind;
} wire_speed;

typedef struct wire_Measure_Speed {
  struct wire_speed *field0;
} wire_Measure_Speed;

typedef struct wire_Distance_Unknown {

} wire_Distance_Unknown;

typedef struct wire_Distance_Map {
  double field0;
} wire_Distance_Map;

typedef union DistanceKind {
  struct wire_Distance_Unknown *Unknown;
  struct wire_Distance_Map *Map;
} DistanceKind;

typedef struct wire_distance {
  int32_t tag;
  union DistanceKind *kind;
} wire_distance;

typedef struct wire_Measure_Distance {
  struct wire_distance *field0;
} wire_Measure_Distance;

typedef union MeasureKind {
  struct wire_Measure_Speed *Speed;
  struct wire_Measure_Distance *Distance;
} MeasureKind;

typedef struct wire_measure {
  int32_t tag;
  union MeasureKind *kind;
} wire_measure;

typedef struct wire_note {
  int32_t *day;
  struct wire_list_prim_u_8 *body;
} wire_note;

typedef struct wire_event {
  struct wire_list_prim_u_8 *address;
  struct wire_list_prim_u_8 *payload;
} wire_event;

typedef struct wire_custom_struct {
  struct wire_list_prim_u_8 *message;
} wire_custom_struct;

typedef struct wire_some_struct {
  uint32_t value;
} wire_some_struct;

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

typedef struct wire_concatenate_with {
  struct wire_list_prim_u_8 *a;
} wire_concatenate_with;

typedef struct wire_sum_with {
  uint32_t x;
} wire_sum_with;

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

typedef struct wire_list_my_tree_node {
  struct wire_my_tree_node *ptr;
  int32_t len;
} wire_list_my_tree_node;

typedef struct wire_my_tree_node {
  int32_t value_i32;
  struct wire_list_prim_u_8 *value_vec_u8;
  bool value_boolean;
  struct wire_list_my_tree_node *children;
} wire_my_tree_node;

typedef struct wire_my_nested_struct {
  struct wire_my_tree_node tree_node;
  int32_t weekday;
} wire_my_nested_struct;

typedef struct wire_my_size {
  int32_t width;
  int32_t height;
} wire_my_size;

typedef struct wire_my_size_freezed {
  int32_t width;
  int32_t height;
} wire_my_size_freezed;

typedef struct wire_list_weekdays {
  int32_t *ptr;
  int32_t len;
} wire_list_weekdays;

typedef struct wire_a {
  struct wire_list_prim_u_8 *a;
} wire_a;

typedef struct wire_Abc_A {
  struct wire_a *field0;
} wire_Abc_A;

typedef struct wire_b {
  int32_t b;
} wire_b;

typedef struct wire_Abc_B {
  struct wire_b *field0;
} wire_Abc_B;

typedef struct wire_c {
  bool c;
} wire_c;

typedef struct wire_Abc_C {
  struct wire_c *field0;
} wire_Abc_C;

typedef struct wire_Abc_JustInt {
  int32_t field0;
} wire_Abc_JustInt;

typedef union AbcKind {
  struct wire_Abc_A *A;
  struct wire_Abc_B *B;
  struct wire_Abc_C *C;
  struct wire_Abc_JustInt *JustInt;
} AbcKind;

typedef struct wire_abc {
  int32_t tag;
  union AbcKind *kind;
} wire_abc;

typedef struct wire_struct_with_enum {
  struct wire_abc abc1;
  struct wire_abc abc2;
} wire_struct_with_enum;

typedef struct wire_empty {

} wire_empty;

typedef struct wire_list_my_size {
  struct wire_my_size *ptr;
  int32_t len;
} wire_list_my_size;

typedef struct wire_StringList {
  struct wire_list_prim_u_8 **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_new_type_int {
  int64_t field0;
} wire_new_type_int;

typedef struct wire_list_prim_i_8 {
  int8_t *ptr;
  int32_t len;
} wire_list_prim_i_8;

typedef struct wire_list_prim_f_32 {
  float *ptr;
  int32_t len;
} wire_list_prim_f_32;

typedef struct wire_attribute {
  struct wire_list_prim_u_8 *key;
  struct wire_list_prim_u_8 *value;
} wire_attribute;

typedef struct wire_list_attribute {
  struct wire_attribute *ptr;
  int32_t len;
} wire_list_attribute;

typedef struct wire_list_opt_box_autoadd_attribute {
  struct wire_attribute **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_attribute;

typedef struct wire_exotic_optionals {
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
  struct wire_list_attribute *attributes;
  struct wire_list_opt_box_autoadd_attribute *attributes_nullable;
  struct wire_list_opt_box_autoadd_attribute *nullable_attributes;
  struct wire_new_type_int *newtypeint;
} wire_exotic_optionals;

typedef struct wire_list_opt_box_autoadd_i_32 {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_i_32;

typedef struct wire_list_opt_box_autoadd_weekdays {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_weekdays;

typedef struct wire_list_opt_String {
  struct wire_list_prim_u_8 **ptr;
  int32_t len;
} wire_list_opt_String;

typedef struct wire_list_opt_list_prim_i_32 {
  struct wire_list_prim_i_32 **ptr;
  int32_t len;
} wire_list_opt_list_prim_i_32;

typedef struct wire_opt_vecs {
  struct wire_list_opt_box_autoadd_i_32 *i32;
  struct wire_list_opt_box_autoadd_weekdays *enums;
  struct wire_list_opt_String *strings;
  struct wire_list_opt_list_prim_i_32 *buffers;
} wire_opt_vecs;

typedef struct wire_struct_with_comments_twin_sync {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_sync;

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

typedef struct wire_RustOpaque_hide_data {
  const void *ptr;
} wire_RustOpaque_hide_data;

typedef struct wire_list_RustOpaque_hide_data {
  struct wire_RustOpaque_hide_data *ptr;
  int32_t len;
} wire_list_RustOpaque_hide_data;

typedef struct wire_EnumOpaque_Struct {
  struct wire_RustOpaque_hide_data field0;
} wire_EnumOpaque_Struct;

typedef struct wire_RustOpaque_i_32 {
  const void *ptr;
} wire_RustOpaque_i_32;

typedef struct wire_EnumOpaque_Primitive {
  struct wire_RustOpaque_i_32 field0;
} wire_EnumOpaque_Primitive;

typedef struct wire_RustOpaque_box_dynDartDebug {
  const void *ptr;
} wire_RustOpaque_box_dynDartDebug;

typedef struct wire_EnumOpaque_TraitObj {
  struct wire_RustOpaque_box_dynDartDebug field0;
} wire_EnumOpaque_TraitObj;

typedef struct wire_RustOpaque_MutexHideData {
  const void *ptr;
} wire_RustOpaque_MutexHideData;

typedef struct wire_EnumOpaque_Mutex {
  struct wire_RustOpaque_MutexHideData field0;
} wire_EnumOpaque_Mutex;

typedef struct wire_RustOpaque_RwLockHideData {
  const void *ptr;
} wire_RustOpaque_RwLockHideData;

typedef struct wire_EnumOpaque_RwLock {
  struct wire_RustOpaque_RwLockHideData field0;
} wire_EnumOpaque_RwLock;

typedef union EnumOpaqueKind {
  struct wire_EnumOpaque_Struct *Struct;
  struct wire_EnumOpaque_Primitive *Primitive;
  struct wire_EnumOpaque_TraitObj *TraitObj;
  struct wire_EnumOpaque_Mutex *Mutex;
  struct wire_EnumOpaque_RwLock *RwLock;
} EnumOpaqueKind;

typedef struct wire_enum_opaque {
  int32_t tag;
  union EnumOpaqueKind *kind;
} wire_enum_opaque;

typedef struct wire_opaque_nested {
  struct wire_RustOpaque_hide_data first;
  struct wire_RustOpaque_hide_data second;
} wire_opaque_nested;

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

typedef struct wire_feature_uuid {
  struct wire_list_prim_u_8 *one;
  struct wire_list_prim_u_8 *many;
} wire_feature_uuid;

void wire_boxed_blob(int64_t port_, struct wire_list_prim_u_8 *blob);

void wire_func_test_id(int64_t port_, struct wire_test_id *id);

void wire_get_array(int64_t port_);

void wire_get_complex_array(int64_t port_);

void wire_last_number(int64_t port_, struct wire_list_prim_f_64 *array);

void wire_nested_id(int64_t port_, struct wire_list_test_id *id);

void wire_new_msgid(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_feed_id(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_return_boxed_raw_feed_id(int64_t port_, struct wire_feed_id *id);

void wire_use_boxed_blob(int64_t port_, struct wire_blob *blob);

void wire_use_msgid(int64_t port_, struct wire_message_id *id);

void wire_handle_customized_struct(int64_t port_, struct wire_customized *val);

void wire_next_user_id(int64_t port_, struct wire_user_id *user_id);

void wire_datetime_local(int64_t port_, int64_t d);

void wire_datetime_utc(int64_t port_, int64_t d);

void wire_duration(int64_t port_, int64_t d);

void wire_handle_durations(int64_t port_, struct wire_list_prim_i_64 *durations, int64_t since);

void wire_handle_timestamps(int64_t port_, struct wire_list_prim_i_64 *timestamps, int64_t epoch);

void wire_how_long_does_it_take(int64_t port_, struct wire_feature_chrono *mine);

void wire_naivedatetime(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc(int64_t port_, int64_t *d);

void wire_test_chrono(int64_t port_);

void wire_test_precise_chrono(int64_t port_);

void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(int64_t port_,
                                                                   struct wire_struct_with_comments_twin_normal *that);

void wire_StructWithCommentsTwinNormal_static_method_twin_normal(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_normal(int64_t port_);

void wire_return_dart_dynamic(int64_t port_);

void wire_async_accept_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_enum_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_nested_dart_opaque(int64_t port_,
                                    struct wire_DartOpaque opaque1,
                                    struct wire_DartOpaque opaque2);

void wire_drop_static_dart_opaque(int64_t port_);

void wire_get_enum_dart_opaque(int64_t port_, struct wire_enum_dart_opaque *opaque);

void wire_get_nested_dart_opaque(int64_t port_, struct wire_dart_opaque_nested *opaque);

void wire_loop_back(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_array(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_array_get(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_option(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_option_get(int64_t port_, struct wire_DartOpaque *opaque);

void wire_loop_back_vec(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_vec_get(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_panic_unwrap_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_set_static_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

WireSyncReturn wire_return_non_droppable_dart_opaque(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_accept_dart_opaque(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_loopback(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_option_dart_opaque(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_option_loopback(struct wire_DartOpaque *opaque);

WireSyncReturn wire_unwrap_dart_opaque(struct wire_DartOpaque opaque);

void wire_func_enum_simple_twin_normal(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_mixed_twin_normal *arg);

void wire_func_enum_with_item_struct_twin_normal(int64_t port_,
                                                 struct wire_enum_with_item_struct_twin_normal *arg);

void wire_func_enum_with_item_tuple_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_tuple_twin_normal *arg);

void wire_handle_enum_parameter(int64_t port_, int32_t weekday);

void wire_handle_enum_struct(int64_t port_, struct wire_kitchen_sink *val);

void wire_handle_return_enum(int64_t port_, struct wire_list_prim_u_8 *input);

void wire_multiply_by_ten(int64_t port_, struct wire_measure *measure);

void wire_print_note(int64_t port_, struct wire_note *note);

void wire_Event_as_string(int64_t port_, struct wire_event *that);

void wire_close_event_listener(int64_t port_);

void wire_create_event(int64_t port_,
                       struct wire_list_prim_u_8 *address,
                       struct wire_list_prim_u_8 *payload);

void wire_register_event_listener(int64_t port_);

void wire_CustomStruct_new(int64_t port_, struct wire_list_prim_u_8 *message);

void wire_CustomStruct_nonstatic_return_custom_struct_error(int64_t port_,
                                                            struct wire_custom_struct *that);

void wire_CustomStruct_nonstatic_return_custom_struct_ok(int64_t port_,
                                                         struct wire_custom_struct *that);

void wire_CustomStruct_static_return_custom_struct_error(int64_t port_);

void wire_CustomStruct_static_return_custom_struct_ok(int64_t port_);

void wire_SomeStruct_new(int64_t port_, uint32_t value);

void wire_SomeStruct_non_static_return_err_custom_error(int64_t port_,
                                                        struct wire_some_struct *that);

void wire_SomeStruct_non_static_return_ok_custom_error(int64_t port_,
                                                       struct wire_some_struct *that);

void wire_SomeStruct_static_return_err_custom_error(int64_t port_);

void wire_SomeStruct_static_return_ok_custom_error(int64_t port_);

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

void wire_panic_with_custom_result(int64_t port_);

void wire_return_custom_nested_error_1(int64_t port_);

void wire_return_custom_nested_error_1_variant1(int64_t port_);

void wire_return_custom_nested_error_2(int64_t port_);

void wire_return_custom_struct_error(int64_t port_);

void wire_return_custom_struct_ok(int64_t port_);

void wire_return_err_custom_error(int64_t port_);

void wire_return_error_variant(int64_t port_, uint32_t variant);

void wire_return_ok_custom_error(int64_t port_);

void wire_stream_sink_throw_anyhow(int64_t port_);

void wire_sync_return_custom_struct_error(int64_t port_);

void wire_throw_anyhow(int64_t port_);

void wire_call_new_module_system(int64_t port_);

void wire_call_old_module_system(int64_t port_);

void wire_use_imported_enum(int64_t port_, int32_t my_enum);

void wire_use_imported_struct(int64_t port_, struct wire_my_struct *my_struct);

void wire_another_macro_struct(int64_t port_);

void wire_func_macro_struct(int64_t port_, struct wire_macro_struct *arg);

void wire_ConcatenateWith_concatenate(int64_t port_,
                                      struct wire_concatenate_with *that,
                                      struct wire_list_prim_u_8 *b);

void wire_ConcatenateWith_concatenate_static(int64_t port_,
                                             struct wire_list_prim_u_8 *a,
                                             struct wire_list_prim_u_8 *b);

void wire_ConcatenateWith_handle_some_static_stream_sink(int64_t port_, uint32_t key, uint32_t max);

void wire_ConcatenateWith_handle_some_static_stream_sink_single_arg(int64_t port_);

void wire_ConcatenateWith_handle_some_stream_sink(int64_t port_,
                                                  struct wire_concatenate_with *that,
                                                  uint32_t key,
                                                  uint32_t max);

void wire_ConcatenateWith_handle_some_stream_sink_at_1(int64_t port_,
                                                       struct wire_concatenate_with *that);

void wire_ConcatenateWith_new(int64_t port_, struct wire_list_prim_u_8 *a);

void wire_SumWith_sum(int64_t port_, struct wire_sum_with *that, uint32_t y, uint32_t z);

void wire_get_sum_array(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_get_sum_struct(int64_t port_);

void wire_app_settings_stream(int64_t port_);

void wire_app_settings_vec_stream(int64_t port_);

void wire_first_number(int64_t port_, struct wire_numbers *nums);

void wire_first_sequence(int64_t port_, struct wire_sequences *seqs);

void wire_get_app_settings(int64_t port_);

void wire_get_fallible_app_settings(int64_t port_);

void wire_get_message(int64_t port_);

void wire_is_app_embedded(int64_t port_, struct wire_application_settings *app_settings);

void wire_mirror_struct_stream(int64_t port_);

void wire_mirror_tuple_stream(int64_t port_);

void wire_repeat_number(int64_t port_, int32_t num, uintptr_t times);

void wire_repeat_sequence(int64_t port_, int32_t seq, uintptr_t times);

void wire_test_contains_mirrored_sub_struct(int64_t port_);

void wire_test_fallible_of_raw_string_mirrored(int64_t port_);

void wire_test_list_of_nested_enums_mirrored(int64_t port_);

void wire_test_list_of_raw_nested_string_mirrored(int64_t port_);

void wire_test_nested_raw_string_mirrored(int64_t port_);

void wire_test_raw_string_enum_mirrored(int64_t port_, bool nested);

void wire_test_raw_string_mirrored(int64_t port_);

void wire_handle_big_buffers(int64_t port_);

void wire_handle_complex_struct(int64_t port_, struct wire_my_tree_node *s);

void wire_handle_nested_struct(int64_t port_, struct wire_my_nested_struct *s);

void wire_handle_string(int64_t port_, struct wire_list_prim_u_8 *s);

void wire_handle_struct(int64_t port_, struct wire_my_size *arg, struct wire_my_size *boxed);

WireSyncReturn wire_handle_struct_sync_freezed(struct wire_my_size_freezed *arg,
                                               struct wire_my_size_freezed *boxed);

void wire_handle_vec_u8(int64_t port_, struct wire_list_prim_u_8 *v);

void wire_list_of_primitive_enums(int64_t port_, struct wire_list_weekdays *weekdays);

void wire_test_abc_enum(int64_t port_, struct wire_abc *abc);

void wire_test_struct_with_enum(int64_t port_, struct wire_struct_with_enum *se);

void wire_empty_struct(int64_t port_, struct wire_empty *empty);

void wire_func_return_unit_twin_normal(int64_t port_);

void wire_func_string_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_handle_list_of_struct(int64_t port_, struct wire_list_my_size *l);

void wire_handle_string_list(int64_t port_, struct wire_StringList *names);

void wire_handle_newtype(int64_t port_, struct wire_new_type_int *arg);

void wire_handle_increment_boxed_optional(int64_t port_, double *opt);

void wire_handle_option_box_arguments(int64_t port_,
                                      int8_t *i8box,
                                      uint8_t *u8box,
                                      int32_t *i32box,
                                      int64_t *i64box,
                                      double *f64box,
                                      bool *boolbox,
                                      struct wire_exotic_optionals *structbox);

void wire_handle_optional_increment(int64_t port_, struct wire_exotic_optionals *opt);

void wire_handle_optional_return(int64_t port_, double left, double right);

void wire_handle_optional_struct(int64_t port_, struct wire_list_prim_u_8 *document);

void wire_handle_vec_of_opts(int64_t port_, struct wire_opt_vecs *opt);

WireSyncReturn wire_sync_option(void);

WireSyncReturn wire_sync_option_null(void);

void wire_primitive_optional_types(int64_t port_,
                                   int32_t *my_i32,
                                   int64_t *my_i64,
                                   double *my_f64,
                                   bool *my_bool);

void wire_handle_vec_of_primitive(int64_t port_, int32_t n);

void wire_handle_zero_copy_vec_of_primitive(int64_t port_, int32_t n);

void wire_primitive_types(int64_t port_,
                          int32_t my_i32,
                          int64_t my_i64,
                          double my_f64,
                          bool my_bool);

void wire_primitive_u32(int64_t port_, uint32_t my_u32);

WireSyncReturn wire_StructWithCommentsTwinSync_instance_method_twin_sync(struct wire_struct_with_comments_twin_sync *that);

WireSyncReturn wire_StructWithCommentsTwinSync_static_method_twin_sync(void);

WireSyncReturn wire_function_with_comments_slash_star_star_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_multi_line_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_single_line_twin_sync(void);

WireSyncReturn wire_func_enum_simple_twin_sync(int32_t arg);

WireSyncReturn wire_func_enum_with_item_mixed_twin_sync(struct wire_enum_with_item_mixed_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_struct_twin_sync(struct wire_enum_with_item_struct_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_tuple_twin_sync(struct wire_enum_with_item_tuple_twin_sync *arg);

WireSyncReturn wire_custom_enum_error_panic_twin_sync(void);

WireSyncReturn wire_custom_enum_error_return_error_twin_sync(void);

WireSyncReturn wire_custom_enum_error_return_ok_twin_sync(uint32_t arg);

WireSyncReturn wire_custom_nested_error_return_error_twin_sync(struct wire_custom_nested_error_outer_twin_sync *arg);

WireSyncReturn wire_custom_struct_error_return_error_twin_sync(struct wire_custom_struct_error_twin_sync *arg);

WireSyncReturn wire_func_return_error_twin_sync(void);

WireSyncReturn wire_func_type_fallible_panic_twin_sync(void);

WireSyncReturn wire_func_type_infallible_panic_twin_sync(void);

WireSyncReturn wire_func_return_unit_twin_sync(void);

WireSyncReturn wire_func_string_twin_sync(struct wire_list_prim_u_8 *arg);

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

WireSyncReturn wire_simple_adder_twin_sync(int32_t a, int32_t b);

WireSyncReturn wire_func_struct_with_one_field_twin_sync(struct wire_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_two_field_twin_sync(struct wire_struct_with_two_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_zero_field_twin_sync(struct wire_struct_with_zero_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_one_field_twin_sync(struct wire_tuple_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_two_field_twin_sync(struct wire_tuple_struct_with_two_field_twin_sync *arg);

void wire_test_more_than_just_one_raw_string_struct(int64_t port_);

void wire_test_raw_string_item_struct(int64_t port_);

void wire_create_array_opaque_enum(int64_t port_);

void wire_create_nested_opaque(int64_t port_);

void wire_create_opaque(int64_t port_);

void wire_create_option_opaque(int64_t port_, struct wire_RustOpaque_hide_data *opaque);

void wire_create_sync_opaque(int64_t port_);

void wire_frb_generator_test(int64_t port_);

void wire_opaque_array(int64_t port_);

void wire_opaque_array_run(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_opaque_vec(int64_t port_);

void wire_opaque_vec_run(int64_t port_, struct wire_list_RustOpaque_hide_data *data);

void wire_run_enum_opaque(int64_t port_, struct wire_enum_opaque *opaque);

void wire_run_nested_opaque(int64_t port_, struct wire_opaque_nested *opaque);

void wire_run_non_clone(int64_t port_, struct wire_RustOpaque_non_clone_data clone);

void wire_run_opaque(int64_t port_, struct wire_RustOpaque_hide_data opaque);

void wire_run_opaque_with_delay(int64_t port_, struct wire_RustOpaque_hide_data opaque);

void wire_unwrap_rust_opaque(int64_t port_, struct wire_RustOpaque_hide_data opaque);

WireSyncReturn wire_frb_sync_generator_test(void);

WireSyncReturn wire_sync_create_non_clone(void);

WireSyncReturn wire_sync_create_opaque(void);

WireSyncReturn wire_sync_create_sync_opaque(void);

WireSyncReturn wire_sync_option_rust_opaque(void);

WireSyncReturn wire_sync_run_opaque(struct wire_RustOpaque_non_send_hide_data opaque);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_stream_realistic_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_func_stream_return_error_twin_normal(int64_t port_);

void wire_func_stream_return_panic_twin_normal(int64_t port_);

void wire_func_stream_sink_arg_position_twin_normal(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct(int64_t port_);

void wire_handle_stream_sink_at_1(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3(int64_t port_, uint32_t key, uint32_t max);

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

void wire_test_tuple(int64_t port_, struct wire_record_string_i_32 *value);

void wire_test_tuple_2(int64_t port_, struct wire_list_record_string_i_32 *value);

void wire_handle_type_alias_id(int64_t port_, uint64_t input);

void wire_handle_type_alias_model(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id(int64_t port_, uint64_t input);

void wire_handle_nested_uuids(int64_t port_, struct wire_feature_uuid *ids);

void wire_handle_uuid(int64_t port_, struct wire_list_prim_u_8 *id);

void wire_handle_uuids(int64_t port_, struct wire_list_prim_u_8 *ids);

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

struct wire_a *new_box_autoadd_a(void);

struct wire_abc *new_box_autoadd_abc(void);

struct wire_application_env *new_box_autoadd_application_env(void);

struct wire_application_settings *new_box_autoadd_application_settings(void);

struct wire_attribute *new_box_autoadd_attribute(void);

struct wire_b *new_box_autoadd_b(void);

bool *new_box_autoadd_bool(bool value);

struct wire_c *new_box_autoadd_c(void);

struct wire_concatenate_with *new_box_autoadd_concatenate_with(void);

struct wire_custom_nested_error_inner_twin_normal *new_box_autoadd_custom_nested_error_inner_twin_normal(void);

struct wire_custom_nested_error_inner_twin_sync *new_box_autoadd_custom_nested_error_inner_twin_sync(void);

struct wire_custom_nested_error_outer_twin_normal *new_box_autoadd_custom_nested_error_outer_twin_normal(void);

struct wire_custom_nested_error_outer_twin_sync *new_box_autoadd_custom_nested_error_outer_twin_sync(void);

struct wire_custom_struct *new_box_autoadd_custom_struct(void);

struct wire_custom_struct_error_twin_normal *new_box_autoadd_custom_struct_error_twin_normal(void);

struct wire_custom_struct_error_twin_sync *new_box_autoadd_custom_struct_error_twin_sync(void);

struct wire_customized *new_box_autoadd_customized(void);

struct wire_dart_opaque_nested *new_box_autoadd_dart_opaque_nested(void);

struct wire_empty *new_box_autoadd_empty(void);

struct wire_enum_dart_opaque *new_box_autoadd_enum_dart_opaque(void);

struct wire_enum_opaque *new_box_autoadd_enum_opaque(void);

struct wire_enum_with_item_mixed_twin_normal *new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_enum_with_item_mixed_twin_sync *new_box_autoadd_enum_with_item_mixed_twin_sync(void);

struct wire_enum_with_item_struct_twin_normal *new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_enum_with_item_struct_twin_sync *new_box_autoadd_enum_with_item_struct_twin_sync(void);

struct wire_enum_with_item_tuple_twin_normal *new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_enum_with_item_tuple_twin_sync *new_box_autoadd_enum_with_item_tuple_twin_sync(void);

struct wire_event *new_box_autoadd_event(void);

struct wire_exotic_optionals *new_box_autoadd_exotic_optionals(void);

float *new_box_autoadd_f_32(float value);

double *new_box_autoadd_f_64(double value);

struct wire_feature_chrono *new_box_autoadd_feature_chrono(void);

struct wire_feature_uuid *new_box_autoadd_feature_uuid(void);

struct wire_feed_id *new_box_autoadd_feed_id(void);

int16_t *new_box_autoadd_i_16(int16_t value);

int32_t *new_box_autoadd_i_32(int32_t value);

int64_t *new_box_autoadd_i_64(int64_t value);

int8_t *new_box_autoadd_i_8(int8_t value);

struct wire_kitchen_sink *new_box_autoadd_kitchen_sink(void);

struct wire_macro_struct *new_box_autoadd_macro_struct(void);

struct wire_measure *new_box_autoadd_measure(void);

struct wire_message_id *new_box_autoadd_message_id(void);

struct wire_my_nested_struct *new_box_autoadd_my_nested_struct(void);

struct wire_my_size *new_box_autoadd_my_size(void);

struct wire_my_size_freezed *new_box_autoadd_my_size_freezed(void);

struct wire_my_struct *new_box_autoadd_my_struct(void);

struct wire_my_tree_node *new_box_autoadd_my_tree_node(void);

struct wire_new_type_int *new_box_autoadd_new_type_int(void);

struct wire_note *new_box_autoadd_note(void);

struct wire_numbers *new_box_autoadd_numbers(void);

struct wire_opaque_nested *new_box_autoadd_opaque_nested(void);

struct wire_opt_vecs *new_box_autoadd_opt_vecs(void);

struct wire_record_string_i_32 *new_box_autoadd_record_string_i_32(void);

struct wire_sequences *new_box_autoadd_sequences(void);

struct wire_some_struct *new_box_autoadd_some_struct(void);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_struct_with_comments_twin_sync *new_box_autoadd_struct_with_comments_twin_sync(void);

struct wire_struct_with_enum *new_box_autoadd_struct_with_enum(void);

struct wire_struct_with_one_field_twin_normal *new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_struct_with_one_field_twin_sync *new_box_autoadd_struct_with_one_field_twin_sync(void);

struct wire_struct_with_two_field_twin_normal *new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_struct_with_two_field_twin_sync *new_box_autoadd_struct_with_two_field_twin_sync(void);

struct wire_struct_with_zero_field_twin_normal *new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_struct_with_zero_field_twin_sync *new_box_autoadd_struct_with_zero_field_twin_sync(void);

struct wire_sum_with *new_box_autoadd_sum_with(void);

struct wire_test_id *new_box_autoadd_test_id(void);

struct wire_tuple_struct_with_one_field_twin_normal *new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_tuple_struct_with_one_field_twin_sync *new_box_autoadd_tuple_struct_with_one_field_twin_sync(void);

struct wire_tuple_struct_with_two_field_twin_normal *new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_tuple_struct_with_two_field_twin_sync *new_box_autoadd_tuple_struct_with_two_field_twin_sync(void);

uint16_t *new_box_autoadd_u_16(uint16_t value);

uint32_t *new_box_autoadd_u_32(uint32_t value);

uint64_t *new_box_autoadd_u_64(uint64_t value);

uint8_t *new_box_autoadd_u_8(uint8_t value);

struct wire_user_id *new_box_autoadd_user_id(void);

int32_t *new_box_autoadd_weekdays(int32_t value);

struct wire_blob *new_box_blob(void);

bool *new_box_bool(bool value);

struct wire_distance *new_box_distance(void);

struct wire_exotic_optionals *new_box_exotic_optionals(void);

double *new_box_f_64(double value);

int32_t *new_box_i_32(int32_t value);

int64_t *new_box_i_64(int64_t value);

int8_t *new_box_i_8(int8_t value);

struct wire_kitchen_sink *new_box_kitchen_sink(void);

struct wire_my_size *new_box_my_size(void);

struct wire_my_size_freezed *new_box_my_size_freezed(void);

struct wire_speed *new_box_speed(void);

uint8_t *new_box_u_8(uint8_t value);

int32_t *new_box_weekdays(int32_t value);

struct wire_list_DartOpaque *new_list_DartOpaque(int32_t len);

struct wire_list_RustOpaque_hide_data *new_list_RustOpaque_hide_data(int32_t len);

struct wire_list_application_env_var *new_list_application_env_var(int32_t len);

struct wire_list_attribute *new_list_attribute(int32_t len);

struct wire_list_bool *new_list_bool(int32_t len);

struct wire_list_my_size *new_list_my_size(int32_t len);

struct wire_list_my_tree_node *new_list_my_tree_node(int32_t len);

struct wire_list_opt_String *new_list_opt_String(int32_t len);

struct wire_list_opt_box_autoadd_attribute *new_list_opt_box_autoadd_attribute(int32_t len);

struct wire_list_opt_box_autoadd_i_32 *new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_list_opt_box_autoadd_weekdays *new_list_opt_box_autoadd_weekdays(int32_t len);

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

struct wire_list_test_id *new_list_test_id(int32_t len);

struct wire_list_weekdays *new_list_weekdays(int32_t len);

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

union AbcKind *inflate_Abc_A(void);

union AbcKind *inflate_Abc_B(void);

union AbcKind *inflate_Abc_C(void);

union AbcKind *inflate_Abc_JustInt(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Three(void);

union CustomNestedErrorInnerTwinNormalKind *inflate_CustomNestedErrorInnerTwinNormal_Four(void);

union CustomNestedErrorInnerTwinSyncKind *inflate_CustomNestedErrorInnerTwinSync_Three(void);

union CustomNestedErrorInnerTwinSyncKind *inflate_CustomNestedErrorInnerTwinSync_Four(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_One(void);

union CustomNestedErrorOuterTwinNormalKind *inflate_CustomNestedErrorOuterTwinNormal_Two(void);

union CustomNestedErrorOuterTwinSyncKind *inflate_CustomNestedErrorOuterTwinSync_One(void);

union CustomNestedErrorOuterTwinSyncKind *inflate_CustomNestedErrorOuterTwinSync_Two(void);

union DistanceKind *inflate_Distance_Map(void);

union EnumDartOpaqueKind *inflate_EnumDartOpaque_Primitive(void);

union EnumDartOpaqueKind *inflate_EnumDartOpaque_Opaque(void);

union EnumOpaqueKind *inflate_EnumOpaque_Struct(void);

union EnumOpaqueKind *inflate_EnumOpaque_Primitive(void);

union EnumOpaqueKind *inflate_EnumOpaque_TraitObj(void);

union EnumOpaqueKind *inflate_EnumOpaque_Mutex(void);

union EnumOpaqueKind *inflate_EnumOpaque_RwLock(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_B(void);

union EnumWithItemMixedTwinNormalKind *inflate_EnumWithItemMixedTwinNormal_C(void);

union EnumWithItemMixedTwinSyncKind *inflate_EnumWithItemMixedTwinSync_B(void);

union EnumWithItemMixedTwinSyncKind *inflate_EnumWithItemMixedTwinSync_C(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_A(void);

union EnumWithItemStructTwinNormalKind *inflate_EnumWithItemStructTwinNormal_B(void);

union EnumWithItemStructTwinSyncKind *inflate_EnumWithItemStructTwinSync_A(void);

union EnumWithItemStructTwinSyncKind *inflate_EnumWithItemStructTwinSync_B(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_A(void);

union EnumWithItemTupleTwinNormalKind *inflate_EnumWithItemTupleTwinNormal_B(void);

union EnumWithItemTupleTwinSyncKind *inflate_EnumWithItemTupleTwinSync_A(void);

union EnumWithItemTupleTwinSyncKind *inflate_EnumWithItemTupleTwinSync_B(void);

union KitchenSinkKind *inflate_KitchenSink_Primitives(void);

union KitchenSinkKind *inflate_KitchenSink_Nested(void);

union KitchenSinkKind *inflate_KitchenSink_Optional(void);

union KitchenSinkKind *inflate_KitchenSink_Buffer(void);

union KitchenSinkKind *inflate_KitchenSink_Enums(void);

union MeasureKind *inflate_Measure_Speed(void);

union MeasureKind *inflate_Measure_Distance(void);

union SpeedKind *inflate_Speed_GPS(void);
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
    dummy_var ^= ((int64_t) (void*) inflate_Abc_A);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_B);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_C);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinNormal_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinSync_Four);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorInnerTwinSync_Three);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinNormal_Two);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinSync_One);
    dummy_var ^= ((int64_t) (void*) inflate_CustomNestedErrorOuterTwinSync_Two);
    dummy_var ^= ((int64_t) (void*) inflate_Distance_Map);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaque_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaque_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinNormal_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemMixedTwinSync_C);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinSync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemStructTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinNormal_B);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinSync_A);
    dummy_var ^= ((int64_t) (void*) inflate_EnumWithItemTupleTwinSync_B);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_Speed_GPS);
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
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_env);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_settings);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_macro_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size_freezed);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_numbers);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_record_string_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sequences);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays);
    dummy_var ^= ((int64_t) (void*) new_box_blob);
    dummy_var ^= ((int64_t) (void*) new_box_bool);
    dummy_var ^= ((int64_t) (void*) new_box_distance);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals);
    dummy_var ^= ((int64_t) (void*) new_box_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink);
    dummy_var ^= ((int64_t) (void*) new_box_my_size);
    dummy_var ^= ((int64_t) (void*) new_box_my_size_freezed);
    dummy_var ^= ((int64_t) (void*) new_box_speed);
    dummy_var ^= ((int64_t) (void*) new_box_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_list_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_list_RustOpaque_hide_data);
    dummy_var ^= ((int64_t) (void*) new_list_application_env_var);
    dummy_var ^= ((int64_t) (void*) new_list_attribute);
    dummy_var ^= ((int64_t) (void*) new_list_bool);
    dummy_var ^= ((int64_t) (void*) new_list_my_size);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node);
    dummy_var ^= ((int64_t) (void*) new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays);
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
    dummy_var ^= ((int64_t) (void*) new_list_test_id);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays);
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
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_concatenate);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_concatenate_static);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_handle_some_static_stream_sink);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_handle_some_static_stream_sink_single_arg);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_handle_some_stream_sink);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_handle_some_stream_sink_at_1);
    dummy_var ^= ((int64_t) (void*) wire_ConcatenateWith_new);
    dummy_var ^= ((int64_t) (void*) wire_CustomStruct_new);
    dummy_var ^= ((int64_t) (void*) wire_CustomStruct_nonstatic_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_CustomStruct_nonstatic_return_custom_struct_ok);
    dummy_var ^= ((int64_t) (void*) wire_CustomStruct_static_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_CustomStruct_static_return_custom_struct_ok);
    dummy_var ^= ((int64_t) (void*) wire_Event_as_string);
    dummy_var ^= ((int64_t) (void*) wire_SomeStruct_new);
    dummy_var ^= ((int64_t) (void*) wire_SomeStruct_non_static_return_err_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_SomeStruct_non_static_return_ok_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_SomeStruct_static_return_err_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_SomeStruct_static_return_ok_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_instance_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_static_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_SumWith_sum);
    dummy_var ^= ((int64_t) (void*) wire_another_macro_struct);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_event);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_enum_error_return_ok_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_nested_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_custom_struct_error_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_duration);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_optional_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_bool_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_f64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_i8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u16_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u32_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u64_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_type_u8_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_first_number);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_simple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_macro_struct);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_realistic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_return_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_stream_sink_arg_position_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_test_id);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_fallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_type_infallible_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings);
    dummy_var ^= ((int64_t) (void*) wire_get_array);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings);
    dummy_var ^= ((int64_t) (void*) wire_get_message);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3);
    dummy_var ^= ((int64_t) (void*) wire_handle_string);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_sync_freezed);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded);
    dummy_var ^= ((int64_t) (void*) wire_last_number);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums);
    dummy_var ^= ((int64_t) (void*) wire_loop_back);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime);
    dummy_var ^= ((int64_t) (void*) wire_nested_id);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32);
    dummy_var ^= ((int64_t) (void*) wire_print_note);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant);
    dummy_var ^= ((int64_t) (void*) wire_return_non_droppable_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback);
    dummy_var ^= ((int64_t) (void*) wire_sync_option);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_null);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_nested_enums_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid);
    return dummy_var;
}
