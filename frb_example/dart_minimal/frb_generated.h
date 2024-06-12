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

typedef struct wire_cst_struct_with_field_rename_twin_normal {
  int32_t class_;
} wire_cst_struct_with_field_rename_twin_normal;

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__func_for_struct_with_field_rename_twin_normal(int64_t port_,
                                                                                                              struct wire_cst_struct_with_field_rename_twin_normal *arg);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

struct wire_cst_struct_with_field_rename_twin_normal *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_struct_with_field_rename_twin_normal(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_struct_with_field_rename_twin_normal);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__func_for_struct_with_field_rename_twin_normal);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
