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

typedef struct wire_cst_MyEnum_A {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_MyEnum_A;

typedef union MyEnumKind {
  struct wire_cst_MyEnum_A A;
} MyEnumKind;

typedef struct wire_cst_my_enum {
  int32_t tag;
  union MyEnumKind kind;
} wire_cst_my_enum;

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8_strict **ptr;
  int32_t len;
} wire_cst_list_String;

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__f(int64_t port_,
                                                                  struct wire_cst_my_enum *a,
                                                                  struct wire_cst_list_String *b);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app(int64_t port_);

void frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder(int64_t port_,
                                                                              int32_t a,
                                                                              int32_t b);

struct wire_cst_my_enum *frbgen_frb_example_dart_minimal_cst_new_box_autoadd_my_enum(void);

struct wire_cst_list_String *frbgen_frb_example_dart_minimal_cst_new_list_String(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_box_autoadd_my_enum);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__f);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__init_app);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_dart_minimal_wire__crate__api__minimal__minimal_adder);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
