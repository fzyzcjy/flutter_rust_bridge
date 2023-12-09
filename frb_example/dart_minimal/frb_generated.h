#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturnDco;
typedef struct _Dart_Handle* Dart_Handle;

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

void wire_hello(int64_t port_, uint8_t *ptr_, int32_t rust_vec_len_, int32_t data_len_);

WireSyncReturnSse wire_hello_sync(uint8_t *ptr_, int32_t rust_vec_len_, int32_t data_len_);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_hello);
    dummy_var ^= ((int64_t) (void*) wire_hello_sync);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
