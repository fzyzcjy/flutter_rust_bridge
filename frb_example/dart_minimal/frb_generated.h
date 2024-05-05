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

typedef struct wire_cst_my_struct {
  int32_t template_;
} wire_cst_my_struct;

void frbgen_frb_example_dart_minimal_wire_f(int64_t port_, struct wire_cst_my_struct *a);

void frbgen_frb_example_dart_minimal_wire_init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

struct wire_cst_my_struct *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_my_struct(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_my_struct);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_f);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire_minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
