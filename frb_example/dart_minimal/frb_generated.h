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

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem {
  uintptr_t *ptr;
  int32_t len;
} wire_cst_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem;

typedef struct wire_cst_item_container_solution_two {
  struct wire_cst_list_prim_u_8_strict *name;
  struct wire_cst_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem *items;
} wire_cst_item_container_solution_two;

typedef struct wire_cst_list_prim_i_32_strict {
  int32_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_32_strict;

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__item_container_solution_two_create(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__item_container_solution_two_get_item_contents(int64_t port_,
                                                                                                              struct wire_cst_item_container_solution_two *that);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

void frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem(const void *ptr);

void frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem(const void *ptr);

struct wire_cst_item_container_solution_two *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_item_container_solution_two(void);

struct wire_cst_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem *frbgen_frb_example_dart_minimal_cst_new_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem(int32_t len);

struct wire_cst_list_prim_i_32_strict *frbgen_frb_example_dart_minimal_cst_new_list_prim_i_32_strict(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_item_container_solution_two);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_AutoExplicit_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_prim_i_32_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerOpaqueItem);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__item_container_solution_two_create);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__item_container_solution_two_get_item_contents);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
