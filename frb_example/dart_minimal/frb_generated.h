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

typedef struct wire_cst_list_prim_u_8_loose {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_loose;

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__f(int64_t port_,
                                                                  struct wire_cst_list_prim_u_8_loose *a,
                                                                  int64_t *b,
                                                                  int32_t *c);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

int32_t *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_i_32(int32_t value);

int64_t *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_i_64(int64_t value);

struct wire_cst_list_prim_u_8_loose *frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__f);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
