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

void frbgen_frb_example_deliberate_bad_dart_fn_deliver_output(int32_t call_id,
                                                              uint8_t *ptr_,
                                                              int32_t rust_vec_len_,
                                                              int32_t data_len_);

void frbgen_frb_example_deliberate_bad_wire_make_data_race(int64_t port_);

void frbgen_frb_example_deliberate_bad_wire_make_heap_use_after_free(int64_t port_);

void frbgen_frb_example_deliberate_bad_wire_make_memory_leak(int64_t port_);

void frbgen_frb_example_deliberate_bad_wire_make_stack_buffer_overflow(int64_t port_);

void frbgen_frb_example_deliberate_bad_wire_make_use_of_uninitialized_value(int64_t port_);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_wire_make_data_race);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_wire_make_heap_use_after_free);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_wire_make_memory_leak);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_wire_make_stack_buffer_overflow);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_deliberate_bad_wire_make_use_of_uninitialized_value);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
