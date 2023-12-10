#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncReturnDco;
typedef struct WireSyncReturnSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncReturnSse;
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8;

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

WireSyncReturnSse wire_example_primitive_list_type_u8_twin_sync_sse(uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

struct wire_cst_list_prim_u_8 *cst_new_list_prim_u_8(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_example_primitive_list_type_u8_twin_sync_sse);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
