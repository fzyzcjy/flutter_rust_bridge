#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_SharedStructInBlock1And2 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock1And2;

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

typedef struct wire_EnumType_BytesArray {
  struct wire_uint_8_list *field0;
} wire_EnumType_BytesArray;

typedef union EnumTypeKind {
  struct wire_EnumType_Empty *Empty;
  struct wire_EnumType_Primitives *Primitives;
  struct wire_EnumType_Nested *Nested;
  struct wire_EnumType_Optional *Optional;
  struct wire_EnumType_Buffer *Buffer;
  struct wire_EnumType_Enums *Enums;
  struct wire_EnumType_BytesArray *BytesArray;
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

typedef struct wire_CrossSharedStructInBlock1And2 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock1And2;

typedef struct wire_CrossSharedStructInBlock2And3 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock2And3;

typedef struct wire_SharedStructOnlyForSyncTest {
  struct wire_uint_8_list *name;
  double score;
} wire_SharedStructOnlyForSyncTest;

typedef struct wire_SharedStructInBlock2And3 {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_SharedStructInBlock2And3;

typedef struct DartCObject *WireSyncReturn;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_StructOnlyForBlock1 {
  int8_t *id;
  double *num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock1;

typedef struct wire_StructDefinedInBlock1 {
  struct wire_uint_8_list *name;
} wire_StructDefinedInBlock1;

typedef struct wire_StructOnlyForBlock2 {
  int16_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock2;

typedef struct wire_StructOnlyForBlock3 {
  int64_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock3;

void wire_test_enum_method__method__Weekdays(int64_t port_,
                                             int32_t that,
                                             struct wire_uint_8_list *message);

void wire_test_static_enum_method__static_method__Weekdays(int64_t port_,
                                                           struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructInBlock1And2(int64_t port_,
                                                        struct wire_SharedStructInBlock1And2 *that,
                                                        struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructInBlock1And2(int64_t port_,
                                                                      struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructInAllBlocks(int64_t port_,
                                                       struct wire_SharedStructInAllBlocks *that,
                                                       struct wire_uint_8_list *message,
                                                       uint32_t num);

void wire_test_static_method__static_method__SharedStructInAllBlocks(int64_t port_,
                                                                     struct wire_uint_8_list *message);

void wire_test_method__method__CrossSharedStructInBlock1And2(int64_t port_,
                                                             struct wire_CrossSharedStructInBlock1And2 *that,
                                                             struct wire_uint_8_list *message);

void wire_test_static_method__static_method__CrossSharedStructInBlock1And2(int64_t port_,
                                                                           struct wire_uint_8_list *message);

void wire_test_method__method__CrossSharedStructInBlock2And3(int64_t port_,
                                                             struct wire_CrossSharedStructInBlock2And3 *that,
                                                             struct wire_uint_8_list *message);

void wire_test_static_method__static_method__CrossSharedStructInBlock2And3(int64_t port_,
                                                                           struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructOnlyForSyncTest(int64_t port_,
                                                           struct wire_SharedStructOnlyForSyncTest *that,
                                                           struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructOnlyForSyncTest(int64_t port_,
                                                                         struct wire_uint_8_list *message);

void wire_test_method__method__SharedStructInBlock2And3(int64_t port_,
                                                        struct wire_SharedStructInBlock2And3 *that,
                                                        struct wire_uint_8_list *message);

void wire_test_static_method__static_method__SharedStructInBlock2And3(int64_t port_,
                                                                      struct wire_uint_8_list *message);

int32_t *new_box_autoadd_i32(int32_t value);

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_EnumType *new_box_enum_type(void);

struct wire_float_32_list *new_float_32_list(int32_t len);

struct wire_list_enum_type *new_list_enum_type(int32_t len);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

union EnumTypeKind *inflate_EnumType_Primitives(void);

union EnumTypeKind *inflate_EnumType_Nested(void);

union EnumTypeKind *inflate_EnumType_Optional(void);

union EnumTypeKind *inflate_EnumType_Buffer(void);

union EnumTypeKind *inflate_EnumType_Enums(void);

union EnumTypeKind *inflate_EnumType_BytesArray(void);

void free_WireSyncReturn(WireSyncReturn ptr);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_test_inbuilt_type_in_block_1(int64_t port_, int32_t a, float b);

void wire_test_string_in_block_1(int64_t port_, struct wire_uint_8_list *s, uint64_t i);

WireSyncReturn wire_test_string_in_sync_in_block_1(struct wire_uint_8_list *s, uint64_t i);

void wire_test_optional_string_in_block_1(int64_t port_, struct wire_uint_8_list *s, int32_t i);

WireSyncReturn wire_test_optional_string_in_sync_in_block_1(struct wire_uint_8_list *s, int32_t i);

WireSyncReturn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(struct wire_uint_8_list *name,
                                                                                 double score);

void wire_test_all_shared_struct_in_block_1(int64_t port_,
                                            struct wire_SharedStructInAllBlocks *custom,
                                            struct wire_uint_8_list *s,
                                            int32_t i);

void wire_test_shared_struct_in_block_1_for_1_and_2(int64_t port_,
                                                    struct wire_SharedStructInBlock1And2 *custom,
                                                    struct wire_uint_8_list *s,
                                                    int32_t i);

void wire_test_cross_shared_struct_in_block_1_for_1_and_2(int64_t port_,
                                                          struct wire_CrossSharedStructInBlock1And2 *custom);

void wire_test_unique_struct_1(int64_t port_,
                               struct wire_StructOnlyForBlock1 *custom,
                               struct wire_uint_8_list *s,
                               int8_t i);

void wire_test_struct_defined_in_block_1(int64_t port_, struct wire_StructDefinedInBlock1 *custom);

void wire_test_method__method__StructDefinedInBlock1(int64_t port_,
                                                     struct wire_StructDefinedInBlock1 *that,
                                                     struct wire_uint_8_list *message);

void wire_test_static_method__static_method__StructDefinedInBlock1(int64_t port_,
                                                                   struct wire_uint_8_list *message);

void wire_test_method__method__StructOnlyForBlock1(int64_t port_,
                                                   struct wire_StructOnlyForBlock1 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock1(int64_t port_,
                                                                 struct wire_uint_8_list *message);

int8_t *new_box_autoadd_i8(int8_t value);

struct wire_StructDefinedInBlock1 *new_box_autoadd_struct_defined_in_block_1(void);

struct wire_StructOnlyForBlock1 *new_box_autoadd_struct_only_for_block_1(void);

void wire_test_method__method__StructOnlyForBlock2(int64_t port_,
                                                   struct wire_StructOnlyForBlock2 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock2(int64_t port_,
                                                                 struct wire_uint_8_list *message);

void wire_test_method__method__StructOnlyForBlock3(int64_t port_,
                                                   struct wire_StructOnlyForBlock3 *that,
                                                   struct wire_uint_8_list *message,
                                                   uint16_t num);

void wire_test_static_method__static_method__StructOnlyForBlock3(int64_t port_,
                                                                 struct wire_uint_8_list *message);

static int64_t dummy_method_to_enforce_bundling_ApiBlock1Class(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_test_inbuilt_type_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_sync_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_optional_string_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_optional_string_in_sync_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_unique_struct_1);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructOnlyForBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructOnlyForBlock1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_only_for_block_1);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
