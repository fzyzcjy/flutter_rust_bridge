#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_CrossSharedStructInBlock1And2 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock1And2;

typedef struct wire_CrossSharedStructInBlock2And3 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock2And3;

typedef struct wire_EnumType_Empty {

} wire_EnumType_Empty;

typedef struct wire_EnumType_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_EnumType_Primitives;

typedef struct wire_EnumType_Nested {
  struct wire_EnumType *field0;
} wire_EnumType_Nested;

typedef struct wire_EnumType_Optional {
  int32_t *field0;
  struct wire_uint_8_list *field1;
} wire_EnumType_Optional;

typedef struct wire_float_32_list {
  float *ptr;
  int32_t len;
} wire_float_32_list;

typedef struct wire_EnumType_Buffer {
  struct wire_float_32_list *field0;
} wire_EnumType_Buffer;

typedef struct wire_EnumType_Enums {
  int32_t field0;
} wire_EnumType_Enums;

typedef union EnumTypeKind {
  struct wire_EnumType_Empty *Empty;
  struct wire_EnumType_Primitives *Primitives;
  struct wire_EnumType_Nested *Nested;
  struct wire_EnumType_Optional *Optional;
  struct wire_EnumType_Buffer *Buffer;
  struct wire_EnumType_Enums *Enums;
} EnumTypeKind;

typedef struct wire_EnumType {
  int32_t tag;
  union EnumTypeKind *kind;
} wire_EnumType;

typedef struct wire_list_enum_type {
  struct wire_EnumType *ptr;
  int32_t len;
} wire_list_enum_type;

typedef struct wire_SharedStructInAllBlocks {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
  struct wire_list_enum_type *enum_list;
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

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

struct wire_CrossSharedStructInBlock1And2 *new_box_autoadd_cross_shared_struct_in_block_1_and_2(void);

struct wire_CrossSharedStructInBlock2And3 *new_box_autoadd_cross_shared_struct_in_block_2_and_3(void);

double *new_box_autoadd_f64(double value);

int32_t *new_box_autoadd_i32(int32_t value);

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_SharedStructOnlyForSyncTest *new_box_autoadd_shared_struct_only_for_sync_test(void);

struct wire_EnumType *new_box_enum_type(void);

struct wire_float_32_list *new_float_32_list(int32_t len);

struct wire_list_enum_type *new_list_enum_type(int32_t len);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

union EnumTypeKind *inflate_EnumType_Primitives(void);

union EnumTypeKind *inflate_EnumType_Nested(void);

union EnumTypeKind *inflate_EnumType_Optional(void);

union EnumTypeKind *inflate_EnumType_Buffer(void);

union EnumTypeKind *inflate_EnumType_Enums(void);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling_BridgeGeneratedShares(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_only_for_sync_test);
    dummy_var ^= ((int64_t) (void*) new_box_enum_type);
    dummy_var ^= ((int64_t) (void*) new_float_32_list);
    dummy_var ^= ((int64_t) (void*) new_list_enum_type);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) inflate_EnumType_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_EnumType_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_EnumType_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_EnumType_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_EnumType_Enums);
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
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock1Class);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock2Class);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_ApiBlock3Class);
    dummy_var ^= ((int64_t) (void*) dummy_method_to_enforce_bundling_BridgeGeneratedShares);
    return dummy_var;
}
