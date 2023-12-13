#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8;

void frb_initialize_rust(int64_t dart_opaque_drop_port, int64_t dart_fn_invoke_port);

void dart_fn_deliver_output(int32_t call_id,
                            uint8_t *ptr_,
                            int32_t rust_vec_len_,
                            int32_t data_len_);

void wire_hi(int64_t port_, uint8_t *ptr_, int32_t rust_vec_len_, int32_t data_len_);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

const void *dart_opaque_dart2rust_encode(Dart_Handle handle);

struct wire_cst_list_prim_u_8 *cst_new_list_prim_u_8(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) dart_opaque_dart2rust_encode);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_hi);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
