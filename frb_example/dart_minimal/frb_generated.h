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

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__S_f(int64_t port_,
                                                                    uintptr_t that,
                                                                    int32_t a);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

void frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerS(const void *ptr);

void frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerS(const void *ptr);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerS);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerS);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__S_f);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
