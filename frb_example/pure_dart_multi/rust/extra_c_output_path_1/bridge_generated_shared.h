#pragma once


#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

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

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

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

typedef struct wire_CrossSharedStructInBlock1And2 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock1And2;

typedef struct wire_CrossSharedStructInBlock2And3 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock2And3;

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

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

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

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_print_weekday__method__SharedWeekdaysEnumInAllBlocks(int64_t port_, int32_t that);

void wire_test_enum_method__method__SharedComplexEnumInAllBlocks(int64_t port_,
                                                                 struct wire_SharedComplexEnumInAllBlocks *that,
                                                                 struct wire_uint_8_list *message);

void wire_test_enum_method__method__SharedWeekdaysEnumInAllBlocks(int64_t port_,
                                                                  int32_t that,
                                                                  struct wire_uint_8_list *message);

void wire_test_method__method__CrossSharedStructInBlock1And2(int64_t port_,
                                                             struct wire_CrossSharedStructInBlock1And2 *that,
                                                             struct wire_uint_8_list *message);

void wire_test_method__method__CrossSharedStructInBlock2And3(int64_t port_,
                                                             struct wire_CrossSharedStructInBlock2And3 *that,
                                                             struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructInAllBlocks(int64_t port_,
                                                       struct wire_SharedStructInAllBlocks *that,
                                                       struct wire_uint_8_list *message,
                                                       uint32_t num);

void wire_test_method__method__SharedStructInBlock1And2(int64_t port_,
                                                        struct wire_SharedStructInBlock1And2 *that,
                                                        struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructInBlock2And3(int64_t port_,
                                                        struct wire_SharedStructInBlock2And3 *that,
                                                        struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructOnlyForSyncTest(int64_t port_,
                                                           struct wire_SharedStructOnlyForSyncTest *that,
                                                           struct wire_uint_8_list *message);

void wire_test_static_enum_method__static_method__SharedComplexEnumInAllBlocks(int64_t port_,
                                                                               struct wire_uint_8_list *message);

void wire_test_static_enum_method__static_method__SharedWeekdaysEnumInAllBlocks(int64_t port_,
                                                                                struct wire_uint_8_list *message);

void wire_test_static_method__static_method__CrossSharedStructInBlock1And2(int64_t port_,
                                                                           struct wire_uint_8_list *message);

void wire_test_static_method__static_method__CrossSharedStructInBlock2And3(int64_t port_,
                                                                           struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructInAllBlocks(int64_t port_,
                                                                     struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructInBlock1And2(int64_t port_,
                                                                      struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructInBlock2And3(int64_t port_,
                                                                      struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructOnlyForSyncTest(int64_t port_,
                                                                         struct wire_uint_8_list *message);

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

static int64_t dummy_method_to_enforce_bundling_BridgeGeneratedShared(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_print_weekday__method__SharedWeekdaysEnumInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_enum_method__method__SharedComplexEnumInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_enum_method__method__SharedWeekdaysEnumInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__CrossSharedStructInBlock1And2);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__CrossSharedStructInBlock2And3);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__SharedStructInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__SharedStructInBlock1And2);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__SharedStructInBlock2And3);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__SharedStructOnlyForSyncTest);
    dummy_var ^= ((int64_t) (void*) wire_test_static_enum_method__static_method__SharedComplexEnumInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_static_enum_method__static_method__SharedWeekdaysEnumInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__CrossSharedStructInBlock1And2);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__CrossSharedStructInBlock2And3);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__SharedStructInAllBlocks);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__SharedStructInBlock1And2);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__SharedStructInBlock2And3);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__SharedStructOnlyForSyncTest);
    dummy_var ^= ((int64_t) (void*) new_StringList);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_complex_enum_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_only_for_sync_test);
    dummy_var ^= ((int64_t) (void*) new_box_shared_complex_enum_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_float_32_list);
    dummy_var ^= ((int64_t) (void*) new_int_32_list);
    dummy_var ^= ((int64_t) (void*) new_list_shared_complex_enum_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_list_shared_struct_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_list_shared_weekdays_enum_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_SharedComplexEnumInAllBlocks_BytesArray);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}

#include "c_output_1.h"
#include "c_output_2.h"
#include "c_output_3.h"
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_BridgeGeneratedShared);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock1Class);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock2Class);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock3Class);
    return dummy_var;
}
