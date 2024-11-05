#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

typedef int64_t DartPort;
typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);
void store_dart_post_cobject(DartPostCObjectFnType ptr);
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__for(int64_t port_,
                                                                    uint8_t *ptr_,
                                                                    int32_t rust_vec_len_,
                                                                    int32_t data_len_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__struct_with_raw_name_field_dummy_function(int64_t port_,
                                                                                                          uint8_t *ptr_,
                                                                                                          int32_t rust_vec_len_,
                                                                                                          int32_t data_len_);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__for);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__struct_with_raw_name_field_dummy_function);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
