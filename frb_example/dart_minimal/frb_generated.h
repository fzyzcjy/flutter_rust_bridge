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

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

void wire_hi_rust_opaque(int64_t port_, const void *a);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynFnUnwindSafeRefUnwindSafe(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynFnUnwindSafeRefUnwindSafe(const void *ptr);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockBoxdynFnUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockBoxdynFnUnwindSafeRefUnwindSafe);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_hi_rust_opaque);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
