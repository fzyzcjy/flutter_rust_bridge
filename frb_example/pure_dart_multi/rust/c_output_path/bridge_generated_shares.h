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

struct wire_SharedStructInAllBlocks *new_box_autoadd_shared_struct_in_all_blocks(void);

struct wire_SharedStructInBlock1And2 *new_box_autoadd_shared_struct_in_block_1_and_2(void);

struct wire_SharedStructInBlock2And3 *new_box_autoadd_shared_struct_in_block_2_and_3(void);

struct wire_SharedStructOnlyForSyncTest *new_box_autoadd_shared_struct_only_for_sync_test(void);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling_BridgeGeneratedShares(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_cross_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_all_blocks);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_1_and_2);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_in_block_2_and_3);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_shared_struct_only_for_sync_test);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
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
