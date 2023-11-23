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

typedef struct wire_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_normal;

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

typedef struct wire_macro_struct {
  int32_t data;
} wire_macro_struct;

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

typedef struct wire_my_size {
  int32_t width;
  int32_t height;
} wire_my_size;

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

typedef struct wire_list_prim_f_32 {
  float *ptr;
  int32_t len;
} wire_list_prim_f_32;

typedef struct wire_list_prim_i_16 {
  int16_t *ptr;
  int32_t len;
} wire_list_prim_i_16;

typedef struct wire_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_list_prim_i_64;

typedef struct wire_list_prim_i_8 {
  int8_t *ptr;
  int32_t len;
} wire_list_prim_i_8;

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

void wire_StructWithCommentsTwinNormal_instance_method_twin_normal(int64_t port_,
                                                                   struct wire_struct_with_comments_twin_normal *that);

void wire_StructWithCommentsTwinNormal_static_method_twin_normal(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_normal(int64_t port_);

void wire_func_enum_simple_twin_normal(int64_t port_, int32_t arg);

void wire_func_enum_with_item_mixed_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_mixed_twin_normal *arg);

void wire_func_enum_with_item_struct_twin_normal(int64_t port_,
                                                 struct wire_enum_with_item_struct_twin_normal *arg);

void wire_func_enum_with_item_tuple_twin_normal(int64_t port_,
                                                struct wire_enum_with_item_tuple_twin_normal *arg);

void wire_handle_enum_parameter(int64_t port_, int32_t weekday);

void wire_handle_return_enum(int64_t port_, struct wire_list_prim_u_8 *input);

void wire_multiply_by_ten(int64_t port_, struct wire_measure *measure);

void wire_print_note(int64_t port_, struct wire_note *note);

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

void wire_func_macro_struct(int64_t port_, struct wire_macro_struct *arg);

void wire_handle_big_buffers(int64_t port_);

void wire_handle_complex_struct(int64_t port_, struct wire_my_tree_node *s);

void wire_handle_nested_struct(int64_t port_, struct wire_my_nested_struct *s);

void wire_list_of_primitive_enums(int64_t port_, struct wire_list_weekdays *weekdays);

void wire_test_abc_enum(int64_t port_, struct wire_abc *abc);

void wire_test_struct_with_enum(int64_t port_, struct wire_struct_with_enum *se);

void wire_func_return_unit_twin_normal(int64_t port_);

void wire_func_string_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_handle_list_of_struct(int64_t port_, struct wire_list_my_size *l);

void wire_handle_string_list(int64_t port_, struct wire_StringList *names);

void wire_handle_newtype(int64_t port_, struct wire_new_type_int *arg);

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

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_func_stream_realistic_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

void wire_func_stream_return_error_twin_normal(int64_t port_);

void wire_func_stream_return_panic_twin_normal(int64_t port_);

void wire_func_stream_sink_arg_position_twin_normal(int64_t port_, uint32_t a, uint32_t b);

void wire_handle_stream_of_struct(int64_t port_);

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

struct wire_StringList *new_StringList(int32_t len);

struct wire_a *new_box_autoadd_a(void);

struct wire_abc *new_box_autoadd_abc(void);

struct wire_b *new_box_autoadd_b(void);

bool *new_box_autoadd_bool(bool value);

struct wire_c *new_box_autoadd_c(void);

struct wire_custom_nested_error_inner_twin_normal *new_box_autoadd_custom_nested_error_inner_twin_normal(void);

struct wire_custom_nested_error_inner_twin_sync *new_box_autoadd_custom_nested_error_inner_twin_sync(void);

struct wire_custom_nested_error_outer_twin_normal *new_box_autoadd_custom_nested_error_outer_twin_normal(void);

struct wire_custom_nested_error_outer_twin_sync *new_box_autoadd_custom_nested_error_outer_twin_sync(void);

struct wire_custom_struct_error_twin_normal *new_box_autoadd_custom_struct_error_twin_normal(void);

struct wire_custom_struct_error_twin_sync *new_box_autoadd_custom_struct_error_twin_sync(void);

struct wire_enum_with_item_mixed_twin_normal *new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_enum_with_item_mixed_twin_sync *new_box_autoadd_enum_with_item_mixed_twin_sync(void);

struct wire_enum_with_item_struct_twin_normal *new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_enum_with_item_struct_twin_sync *new_box_autoadd_enum_with_item_struct_twin_sync(void);

struct wire_enum_with_item_tuple_twin_normal *new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_enum_with_item_tuple_twin_sync *new_box_autoadd_enum_with_item_tuple_twin_sync(void);

float *new_box_autoadd_f_32(float value);

double *new_box_autoadd_f_64(double value);

struct wire_feed_id *new_box_autoadd_feed_id(void);

int16_t *new_box_autoadd_i_16(int16_t value);

int32_t *new_box_autoadd_i_32(int32_t value);

int64_t *new_box_autoadd_i_64(int64_t value);

int8_t *new_box_autoadd_i_8(int8_t value);

struct wire_macro_struct *new_box_autoadd_macro_struct(void);

struct wire_measure *new_box_autoadd_measure(void);

struct wire_message_id *new_box_autoadd_message_id(void);

struct wire_my_nested_struct *new_box_autoadd_my_nested_struct(void);

struct wire_my_tree_node *new_box_autoadd_my_tree_node(void);

struct wire_new_type_int *new_box_autoadd_new_type_int(void);

struct wire_note *new_box_autoadd_note(void);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_struct_with_comments_twin_sync *new_box_autoadd_struct_with_comments_twin_sync(void);

struct wire_struct_with_enum *new_box_autoadd_struct_with_enum(void);

struct wire_struct_with_one_field_twin_normal *new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_struct_with_one_field_twin_sync *new_box_autoadd_struct_with_one_field_twin_sync(void);

struct wire_struct_with_two_field_twin_normal *new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_struct_with_two_field_twin_sync *new_box_autoadd_struct_with_two_field_twin_sync(void);

struct wire_struct_with_zero_field_twin_normal *new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_struct_with_zero_field_twin_sync *new_box_autoadd_struct_with_zero_field_twin_sync(void);

struct wire_test_id *new_box_autoadd_test_id(void);

struct wire_tuple_struct_with_one_field_twin_normal *new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_tuple_struct_with_one_field_twin_sync *new_box_autoadd_tuple_struct_with_one_field_twin_sync(void);

struct wire_tuple_struct_with_two_field_twin_normal *new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_tuple_struct_with_two_field_twin_sync *new_box_autoadd_tuple_struct_with_two_field_twin_sync(void);

uint16_t *new_box_autoadd_u_16(uint16_t value);

uint32_t *new_box_autoadd_u_32(uint32_t value);

uint64_t *new_box_autoadd_u_64(uint64_t value);

uint8_t *new_box_autoadd_u_8(uint8_t value);

struct wire_blob *new_box_blob(void);

struct wire_distance *new_box_distance(void);

struct wire_speed *new_box_speed(void);

int32_t *new_box_weekdays(int32_t value);

struct wire_list_bool *new_list_bool(int32_t len);

struct wire_list_my_size *new_list_my_size(int32_t len);

struct wire_list_my_tree_node *new_list_my_tree_node(int32_t len);

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

struct wire_list_test_id *new_list_test_id(int32_t len);

struct wire_list_weekdays *new_list_weekdays(int32_t len);

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

union MeasureKind *inflate_Measure_Speed(void);

union MeasureKind *inflate_Measure_Distance(void);

union SpeedKind *inflate_Speed_GPS(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
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
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_Speed_GPS);
    dummy_var ^= ((int64_t) (void*) new_StringList);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_inner_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_nested_error_outer_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_macro_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_8);
    dummy_var ^= ((int64_t) (void*) new_box_blob);
    dummy_var ^= ((int64_t) (void*) new_box_distance);
    dummy_var ^= ((int64_t) (void*) new_box_speed);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_list_bool);
    dummy_var ^= ((int64_t) (void*) new_list_my_size);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node);
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
    dummy_var ^= ((int64_t) (void*) new_list_test_id);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_instance_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_static_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob);
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
    dummy_var ^= ((int64_t) (void*) wire_get_array);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list);
    dummy_var ^= ((int64_t) (void*) wire_last_number);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten);
    dummy_var ^= ((int64_t) (void*) wire_nested_id);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid);
    dummy_var ^= ((int64_t) (void*) wire_print_note);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid);
    return dummy_var;
}
