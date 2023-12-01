#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturn;
typedef struct _Dart_Handle* Dart_Handle;

void wire_make_heap_use_after_free(int64_t port_);

void wire_make_stack_buffer_overflow(int64_t port_);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_make_heap_use_after_free);
    dummy_var ^= ((int64_t) (void*) wire_make_stack_buffer_overflow);
    return dummy_var;
}
