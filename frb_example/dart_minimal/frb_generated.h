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

void frbgen_frb_example_dart_minimal_dart_fn_deliver_output(int32_t call_id,
                                                            uint8_t *ptr_,
                                                            int32_t rust_vec_len_,
                                                            int32_t data_len_);

void frbgen_frb_example_dart_minimal_wire_bar(int64_t port_);

void frbgen_frb_example_dart_minimal_wire_foo(int64_t port_);

void frbgen_frb_example_dart_minimal_wire_init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

void frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType(const void *ptr);

void frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType(const void *ptr);

void frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(const void *ptr);

void frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType(const void *ptr);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockAnotherOpaqueType);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockMyOpaqueTypeAnotherOpaqueType);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_bar);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_foo);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_minimal_adder);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
