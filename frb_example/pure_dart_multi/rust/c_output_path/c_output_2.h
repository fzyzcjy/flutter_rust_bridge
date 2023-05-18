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
  struct wire_uint_8_list *u8_list;
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

typedef struct wire_CrossSharedStructInBlock2And3 {
  struct wire_uint_8_list *name;
} wire_CrossSharedStructInBlock2And3;

typedef struct wire_StructOnlyForBlock2 {
  int16_t id;
  double num;
  struct wire_uint_8_list *name;
} wire_StructOnlyForBlock2;

typedef struct wire_StructDefinedInBlock2 {
  struct wire_uint_8_list *name;
} wire_StructDefinedInBlock2;

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

void wire_test_inbuilt_type_in_block_2(int64_t port_, int32_t a, float b);

void wire_test_string_in_block_2(int64_t port_, struct wire_uint_8_list *s, uint64_t i);

void wire_test_all_shared_struct_in_block_2(int64_t port_,
                                            struct wire_SharedStructInAllBlocks *custom,
                                            struct wire_uint_8_list *s,
                                            int32_t i);

WireSyncReturn wire_test_all_shared_struct_in_sync_in_block_2(struct wire_SharedStructInAllBlocks *custom,
                                                              struct wire_uint_8_list *s,
                                                              int32_t i);

void wire_test_shared_struct_in_block_2_for_1_and_2(int64_t port_,
                                                    struct wire_SharedStructInBlock1And2 *custom,
                                                    struct wire_uint_8_list *s,
                                                    int32_t i);

void wire_test_cross_shared_struct_in_block_2_for_1_and_2(int64_t port_,
                                                          struct wire_uint_8_list *name);

void wire_test_shared_struct_in_block_2_for_2_and_3(int64_t port_,
                                                    struct wire_SharedStructInBlock2And3 *custom,
                                                    struct wire_uint_8_list *s,
                                                    int32_t i);

void wire_test_cross_shared_struct_in_block_2_for_2_and_3(int64_t port_,
                                                          struct wire_CrossSharedStructInBlock2And3 *custom);

void wire_test_unique_struct_2(int64_t port_,
                               struct wire_StructOnlyForBlock2 *custom,
                               struct wire_uint_8_list *s,
                               int16_t i);

void wire_test_struct_defined_in_block_2(int64_t port_, struct wire_StructDefinedInBlock2 *custom);

void wire_test_method__method__StructDefinedInBlock2(int64_t port_,
                                                     struct wire_StructDefinedInBlock2 *that,
                                                     struct wire_uint_8_list *message);

void wire_test_static_method__static_method__StructDefinedInBlock2(int64_t port_,
                                                                   struct wire_uint_8_list *message);

struct wire_StructDefinedInBlock2 *new_box_autoadd_struct_defined_in_block_2(void);

struct wire_StructOnlyForBlock2 *new_box_autoadd_struct_only_for_block_2(void);

static int64_t dummy_method_to_enforce_bundling_ApiBlock2Class(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_test_inbuilt_type_in_block_2);
    dummy_var ^= ((int64_t) (void*) wire_test_string_in_block_2);
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_block_2);
    dummy_var ^= ((int64_t) (void*) wire_test_all_shared_struct_in_sync_in_block_2);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_2_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_2_for_1_and_2);
    dummy_var ^= ((int64_t) (void*) wire_test_shared_struct_in_block_2_for_2_and_3);
    dummy_var ^= ((int64_t) (void*) wire_test_cross_shared_struct_in_block_2_for_2_and_3);
    dummy_var ^= ((int64_t) (void*) wire_test_unique_struct_2);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_defined_in_block_2);
    dummy_var ^= ((int64_t) (void*) wire_test_method__method__StructDefinedInBlock2);
    dummy_var ^= ((int64_t) (void*) wire_test_static_method__static_method__StructDefinedInBlock2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_defined_in_block_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_only_for_block_2);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
