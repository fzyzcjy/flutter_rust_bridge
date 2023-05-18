#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_SharedStructInAllBlocks {
  int32_t id;
  double num;
  struct wire_uint_8_list *name;
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

typedef struct DartCObject *WireSyncReturn;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_CrossSharedStructInBlock1And2 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock1And2;

typedef struct wire_StructOnlyForBlock1 {
  int8_t *id;
  double *num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock1;

typedef struct wire_StructDefinedInBlock1 {
  struct wire_uint_8_list *name;
} wire_StructDefinedInBlock1;

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_test_inbuilt_type_in_block_1(int64_t port_, int32_t a, float b);

void wire_test_string_in_block_1(int64_t port_, struct wire_uint_8_list *s, uint64_t i);

WireSyncReturn wire_test_string_in_sync_in_block_1(struct wire_uint_8_list *s, uint64_t i);

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

int8_t *new_box_autoadd_i8(int8_t value);

struct wire_StructDefinedInBlock1 *new_box_autoadd_struct_defined_in_block_1(void);

struct wire_StructOnlyForBlock1 *new_box_autoadd_struct_only_for_block_1(void);

static int64_t dummy_method_to_enforce_bundling_ApiBlock1Class(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_test_inbuilt_type_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_sync_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_1_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_unique_struct_1);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructDefinedInBlock1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i8);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_defined_in_block_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_only_for_block_1);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
