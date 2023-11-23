#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturn;
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_normal;

typedef struct wire_EnumWithItemMixedTwinNormal_A {

} wire_EnumWithItemMixedTwinNormal_A;

typedef struct wire_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_list_prim_u_8;

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

typedef struct wire_list_prim_i_32 {
  int32_t *ptr;
  int32_t len;
} wire_list_prim_i_32;

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

typedef struct wire_list_bool {
  bool *ptr;
  int32_t len;
} wire_list_bool;

typedef struct wire_list_prim_f_32 {
  float *ptr;
  int32_t len;
} wire_list_prim_f_32;

typedef struct wire_list_prim_f_64 {
  double *ptr;
  int32_t len;
} wire_list_prim_f_64;

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

void wire_func_return_error_twin_normal(int64_t port_);

void wire_func_return_panic_twin_normal(int64_t port_);

void wire_func_return_unit_twin_normal(int64_t port_);

void wire_func_string_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

WireSyncReturn wire_StructWithCommentsTwinSync_instance_method_twin_sync(struct wire_struct_with_comments_twin_sync *that);

WireSyncReturn wire_StructWithCommentsTwinSync_static_method_twin_sync(void);

WireSyncReturn wire_function_with_comments_slash_star_star_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_multi_line_twin_sync(void);

WireSyncReturn wire_function_with_comments_triple_slash_single_line_twin_sync(void);

WireSyncReturn wire_func_enum_simple_twin_sync(int32_t arg);

WireSyncReturn wire_func_enum_with_item_mixed_twin_sync(struct wire_enum_with_item_mixed_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_struct_twin_sync(struct wire_enum_with_item_struct_twin_sync *arg);

WireSyncReturn wire_func_enum_with_item_tuple_twin_sync(struct wire_enum_with_item_tuple_twin_sync *arg);

WireSyncReturn wire_func_return_error_twin_sync(void);

WireSyncReturn wire_func_return_panic_twin_sync(void);

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

void wire_handle_stream_realistic_twin_sync(int64_t port_, struct wire_list_prim_u_8 *arg);

WireSyncReturn wire_func_struct_with_one_field_twin_sync(struct wire_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_two_field_twin_sync(struct wire_struct_with_two_field_twin_sync *arg);

WireSyncReturn wire_func_struct_with_zero_field_twin_sync(struct wire_struct_with_zero_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_one_field_twin_sync(struct wire_tuple_struct_with_one_field_twin_sync *arg);

WireSyncReturn wire_func_tuple_struct_with_two_field_twin_sync(struct wire_tuple_struct_with_two_field_twin_sync *arg);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

void wire_handle_stream_realistic_twin_normal(int64_t port_, struct wire_list_prim_u_8 *arg);

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

bool *new_box_autoadd_bool(bool value);

struct wire_enum_with_item_mixed_twin_normal *new_box_autoadd_enum_with_item_mixed_twin_normal(void);

struct wire_enum_with_item_mixed_twin_sync *new_box_autoadd_enum_with_item_mixed_twin_sync(void);

struct wire_enum_with_item_struct_twin_normal *new_box_autoadd_enum_with_item_struct_twin_normal(void);

struct wire_enum_with_item_struct_twin_sync *new_box_autoadd_enum_with_item_struct_twin_sync(void);

struct wire_enum_with_item_tuple_twin_normal *new_box_autoadd_enum_with_item_tuple_twin_normal(void);

struct wire_enum_with_item_tuple_twin_sync *new_box_autoadd_enum_with_item_tuple_twin_sync(void);

float *new_box_autoadd_f_32(float value);

double *new_box_autoadd_f_64(double value);

int16_t *new_box_autoadd_i_16(int16_t value);

int32_t *new_box_autoadd_i_32(int32_t value);

int64_t *new_box_autoadd_i_64(int64_t value);

int8_t *new_box_autoadd_i_8(int8_t value);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);

struct wire_struct_with_comments_twin_sync *new_box_autoadd_struct_with_comments_twin_sync(void);

struct wire_struct_with_one_field_twin_normal *new_box_autoadd_struct_with_one_field_twin_normal(void);

struct wire_struct_with_one_field_twin_sync *new_box_autoadd_struct_with_one_field_twin_sync(void);

struct wire_struct_with_two_field_twin_normal *new_box_autoadd_struct_with_two_field_twin_normal(void);

struct wire_struct_with_two_field_twin_sync *new_box_autoadd_struct_with_two_field_twin_sync(void);

struct wire_struct_with_zero_field_twin_normal *new_box_autoadd_struct_with_zero_field_twin_normal(void);

struct wire_struct_with_zero_field_twin_sync *new_box_autoadd_struct_with_zero_field_twin_sync(void);

struct wire_tuple_struct_with_one_field_twin_normal *new_box_autoadd_tuple_struct_with_one_field_twin_normal(void);

struct wire_tuple_struct_with_one_field_twin_sync *new_box_autoadd_tuple_struct_with_one_field_twin_sync(void);

struct wire_tuple_struct_with_two_field_twin_normal *new_box_autoadd_tuple_struct_with_two_field_twin_normal(void);

struct wire_tuple_struct_with_two_field_twin_sync *new_box_autoadd_tuple_struct_with_two_field_twin_sync(void);

uint16_t *new_box_autoadd_u_16(uint16_t value);

uint32_t *new_box_autoadd_u_32(uint32_t value);

uint64_t *new_box_autoadd_u_64(uint64_t value);

uint8_t *new_box_autoadd_u_8(uint8_t value);

struct wire_list_bool *new_list_bool(int32_t len);

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
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
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
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_mixed_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_struct_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_with_item_tuple_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i_8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_16);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u_8);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) new_list_bool);
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
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_instance_method_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinSync_static_method_twin_sync);
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
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_error_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_panic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_panic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_return_unit_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_string_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_struct_with_zero_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_one_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_func_tuple_struct_with_two_field_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_realistic_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_realistic_twin_sync);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_sync);
    return dummy_var;
}
