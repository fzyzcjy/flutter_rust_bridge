#pragma once


#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_StringList wire_StringList;
typedef struct wire_CrossSharedStructInBlock1And2 wire_CrossSharedStructInBlock1And2;
typedef struct wire_float_32_list wire_float_32_list;
typedef struct wire_int_32_list wire_int_32_list;
typedef struct wire_list_shared_complex_enum_in_all_blocks wire_list_shared_complex_enum_in_all_blocks;
typedef struct wire_list_shared_struct_in_all_blocks wire_list_shared_struct_in_all_blocks;
typedef struct wire_list_shared_weekdays_enum_in_all_blocks wire_list_shared_weekdays_enum_in_all_blocks;
typedef struct wire_SharedComplexEnumInAllBlocks wire_SharedComplexEnumInAllBlocks;
typedef struct wire_SharedStructInAllBlocks wire_SharedStructInAllBlocks;
typedef struct wire_SharedStructInBlock1And2 wire_SharedStructInBlock1And2;
typedef struct wire_SharedStructOnlyForSyncTest wire_SharedStructOnlyForSyncTest;
typedef struct wire_uint_8_list wire_uint_8_list;

typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_CrossSharedStructInBlock1And2 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock1And2;

typedef struct wire_CrossSharedStructInBlock2And3 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock2And3;

typedef struct wire_SharedComplexEnumInAllBlocks_Empty {

} wire_SharedComplexEnumInAllBlocks_Empty;

typedef struct wire_SharedComplexEnumInAllBlocks_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_SharedComplexEnumInAllBlocks_Primitives;

typedef struct wire_SharedComplexEnumInAllBlocks_Nested {
  struct wire_SharedComplexEnumInAllBlocks *field0;
} wire_SharedComplexEnumInAllBlocks_Nested;

typedef struct wire_SharedComplexEnumInAllBlocks_Optional {
  int32_t *field0;
  struct wire_uint_8_list *field1;
} wire_SharedComplexEnumInAllBlocks_Optional;

typedef struct wire_float_32_list {
  float *ptr;
  int32_t len;
} wire_float_32_list;

typedef struct wire_SharedComplexEnumInAllBlocks_Buffer {
  struct wire_float_32_list *field0;
} wire_SharedComplexEnumInAllBlocks_Buffer;

typedef struct wire_SharedComplexEnumInAllBlocks_Enums {
  int32_t field0;
} wire_SharedComplexEnumInAllBlocks_Enums;

typedef struct wire_SharedComplexEnumInAllBlocks_BytesArray {
  struct wire_uint_8_list *field0;
} wire_SharedComplexEnumInAllBlocks_BytesArray;

typedef union SharedComplexEnumInAllBlocksKind {
  struct wire_SharedComplexEnumInAllBlocks_Empty *Empty;
  struct wire_SharedComplexEnumInAllBlocks_Primitives *Primitives;
  struct wire_SharedComplexEnumInAllBlocks_Nested *Nested;
  struct wire_SharedComplexEnumInAllBlocks_Optional *Optional;
  struct wire_SharedComplexEnumInAllBlocks_Buffer *Buffer;
  struct wire_SharedComplexEnumInAllBlocks_Enums *Enums;
  struct wire_SharedComplexEnumInAllBlocks_BytesArray *BytesArray;
} SharedComplexEnumInAllBlocksKind;

typedef struct wire_SharedComplexEnumInAllBlocks {
  int32_t tag;
  union SharedComplexEnumInAllBlocksKind *kind;
} wire_SharedComplexEnumInAllBlocks;

typedef struct wire_list_shared_complex_enum_in_all_blocks {
  struct wire_SharedComplexEnumInAllBlocks *ptr;
  int32_t len;
} wire_list_shared_complex_enum_in_all_blocks;

typedef struct wire_SharedStructInAllBlocks {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
  struct wire_list_shared_complex_enum_in_all_blocks *enum_list;
} wire_SharedStructInAllBlocks;

typedef struct wire_SharedStructInBlock1And2 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock1And2;

typedef struct wire_SharedStructInBlock2And3 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock2And3;

typedef struct wire_SharedStructOnlyForSyncTest {
  struct wire_uint_8_list *name;
  double score;
} wire_SharedStructOnlyForSyncTest;

typedef struct wire_int_32_list {
  int32_t *ptr;
  int32_t len;
} wire_int_32_list;

typedef struct wire_list_shared_struct_in_all_blocks {
  struct wire_SharedStructInAllBlocks *ptr;
  int32_t len;
} wire_list_shared_struct_in_all_blocks;

typedef struct wire_list_shared_weekdays_enum_in_all_blocks {
  int32_t *ptr;
  int32_t len;
} wire_list_shared_weekdays_enum_in_all_blocks;

typedef struct DartCObject *WireSyncReturn;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_EnumDefinedInBlock1_Quit {

} wire_EnumDefinedInBlock1_Quit;

typedef struct wire_EnumDefinedInBlock1_Move {
  int32_t x;
  int32_t y;
} wire_EnumDefinedInBlock1_Move;

typedef struct wire_EnumDefinedInBlock1_Write {
  struct wire_uint_8_list *field0;
} wire_EnumDefinedInBlock1_Write;

typedef struct wire_EnumDefinedInBlock1_ChangeColor {
  int32_t field0;
  int32_t field1;
  int32_t field2;
} wire_EnumDefinedInBlock1_ChangeColor;

typedef union EnumDefinedInBlock1Kind {
  struct wire_EnumDefinedInBlock1_Quit *Quit;
  struct wire_EnumDefinedInBlock1_Move *Move;
  struct wire_EnumDefinedInBlock1_Write *Write;
  struct wire_EnumDefinedInBlock1_ChangeColor *ChangeColor;
} EnumDefinedInBlock1Kind;

typedef struct wire_EnumDefinedInBlock1 {
  int32_t tag;
  union EnumDefinedInBlock1Kind *kind;
} wire_EnumDefinedInBlock1;

typedef struct wire_StructDefinedInBlock1 {
  struct wire_uint_8_list *name;
} wire_StructDefinedInBlock1;

typedef struct wire_list_struct_defined_in_block_1 {
  struct wire_StructDefinedInBlock1 *ptr;
  int32_t len;
} wire_list_struct_defined_in_block_1;

typedef struct wire_list_enum_defined_in_block_1 {
  struct wire_EnumDefinedInBlock1 *ptr;
  int32_t len;
} wire_list_enum_defined_in_block_1;

typedef struct wire_StructOnlyForBlock1 {
  int8_t *id;
  double *num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock1;

struct wire_StringList *new_StringList(int32_t len);

struct wire_CrossSharedStructInBlock1And2 *new_box_autoadd_cross_shared_struct_in_block_1_and_2(void);

struct wire_CrossSharedStructInBlock2And3 *new_box_autoadd_cross_shared_struct_in_block_2_and_3(void);

double *new_box_autoadd_f64(double value);

int32_t *new_box_autoadd_i32(int32_t value);

struct wire_SharedComplexEnumInAllBlocks *new_box_autoadd_shared_complex_enum_in_all_blocks(void);

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_SharedStructOnlyForSyncTest *new_box_autoadd_shared_struct_only_for_sync_test(void);

struct wire_SharedComplexEnumInAllBlocks *new_box_shared_complex_enum_in_all_blocks(void);

struct wire_float_32_list *new_float_32_list(int32_t len);

struct wire_int_32_list *new_int_32_list(int32_t len);

struct wire_list_shared_complex_enum_in_all_blocks *new_list_shared_complex_enum_in_all_blocks(int32_t len);

struct wire_list_shared_struct_in_all_blocks *new_list_shared_struct_in_all_blocks(int32_t len);

struct wire_list_shared_weekdays_enum_in_all_blocks *new_list_shared_weekdays_enum_in_all_blocks(int32_t len);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Primitives(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Nested(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Optional(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Buffer(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_Enums(void);

union SharedComplexEnumInAllBlocksKind *inflate_SharedComplexEnumInAllBlocks_BytesArray(void);

void free_WireSyncReturn(WireSyncReturn ptr);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_test_all_shared_struct_in_block_1(int64_t port_,
                                            struct wire_SharedStructInAllBlocks *custom,
                                            struct wire_uint_8_list *s,
                                            int32_t i);

void wire_test_cross_shared_struct_in_block_1_for_1_and_2(int64_t port_,
                                                          struct wire_CrossSharedStructInBlock1And2 *custom);

void wire_test_enum_defined_in_block_1(int64_t port_, struct wire_EnumDefinedInBlock1 *custom);

void wire_test_inbuilt_type_in_block_1(int64_t port_, int32_t a, float b);

void wire_test_list_in_block_1(int64_t port_,
                               struct wire_list_shared_struct_in_all_blocks *shared_structs,
                               struct wire_StringList *strings,
                               struct wire_int_32_list *nums,
                               struct wire_list_shared_weekdays_enum_in_all_blocks *weekdays,
                               struct wire_list_struct_defined_in_block_1 *struct_list,
                               struct wire_list_enum_defined_in_block_1 *enum_list);

void wire_test_method__method__EnumDefinedInBlock1(int64_t port_,
                                                   struct wire_EnumDefinedInBlock1 *that,
                                                   struct wire_uint_8_list *message);

void wire_test_method__method__StructDefinedInBlock1(int64_t port_,
                                                     struct wire_StructDefinedInBlock1 *that,
                                                     struct wire_uint_8_list *message);

void wire_test_method__method__StructOnlyForBlock1(int64_t port_,
                                                   struct wire_StructOnlyForBlock1 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_optional_string_in_block_1(int64_t port_, struct wire_uint_8_list *s, int32_t i);

WireSyncReturn wire_test_optional_string_in_sync_in_block_1(struct wire_uint_8_list *s, int32_t i);

void wire_test_shared_struct_in_block_1_for_1_and_2(int64_t port_,
                                                    struct wire_SharedStructInBlock1And2 *custom,
                                                    struct wire_uint_8_list *s,
                                                    int32_t i);

WireSyncReturn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(struct wire_uint_8_list *name,
                                                                                 double score);

void wire_test_static_method__static_method__EnumDefinedInBlock1(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void wire_test_static_method__static_method__StructDefinedInBlock1(int64_t port_,
                                                                   struct wire_uint_8_list *message);

void wire_test_static_method__static_method__StructOnlyForBlock1(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void wire_test_string_in_block_1(int64_t port_, struct wire_uint_8_list *s, uint64_t i);

WireSyncReturn wire_test_string_in_sync_in_block_1(struct wire_uint_8_list *s, uint64_t i);

void wire_test_struct_defined_in_block_1(int64_t port_, struct wire_StructDefinedInBlock1 *custom);

void wire_test_unique_struct_1(int64_t port_,
                               struct wire_StructOnlyForBlock1 *custom,
                               struct wire_uint_8_list *s,
                               int8_t i);

struct wire_EnumDefinedInBlock1 *new_box_autoadd_enum_defined_in_block_1(void);

int8_t *new_box_autoadd_i8(int8_t value);

struct wire_StructDefinedInBlock1 *new_box_autoadd_struct_defined_in_block_1(void);

struct wire_StructOnlyForBlock1 *new_box_autoadd_struct_only_for_block_1(void);

struct wire_list_enum_defined_in_block_1 *new_list_enum_defined_in_block_1(int32_t len);

struct wire_list_struct_defined_in_block_1 *new_list_struct_defined_in_block_1(int32_t len);

union EnumDefinedInBlock1Kind *inflate_EnumDefinedInBlock1_Move(void);

union EnumDefinedInBlock1Kind *inflate_EnumDefinedInBlock1_Write(void);

union EnumDefinedInBlock1Kind *inflate_EnumDefinedInBlock1_ChangeColor(void);

static int64_t dummy_method_to_enforce_bundling_ApiBlock1Class(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_enum_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_inbuilt_type_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_list_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__EnumDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructOnlyForBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_optional_string_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_optional_string_in_sync_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__EnumDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructOnlyForBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_sync_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_unique_struct_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_only_for_block_1);
    dummy_var ^= ((int64_t) (void*) new_list_enum_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) new_list_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock1_Move);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock1_Write);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDefinedInBlock1_ChangeColor);
    return dummy_var;
}
