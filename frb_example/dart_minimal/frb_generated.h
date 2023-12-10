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

typedef struct wire_cst_TheEnum_TheVariant {
  int32_t field0;
} wire_cst_TheEnum_TheVariant;

typedef union TheEnumKind {
  struct wire_cst_TheEnum_TheVariant *TheVariant;
} TheEnumKind;

typedef struct wire_cst_the_enum {
  int32_t tag;
  union TheEnumKind *kind;
} wire_cst_the_enum;

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

void dart_fn_deliver_output(int32_t call_id,
                            uint8_t *ptr_,
                            int32_t rust_vec_len_,
                            int32_t data_len_);

void wire_hi(int64_t port_, struct wire_cst_the_enum *a);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

struct wire_cst_the_enum *cst_new_box_autoadd_the_enum(void);

union TheEnumKind *cst_inflate_TheEnum_TheVariant(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_inflate_TheEnum_TheVariant);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_the_enum);
    dummy_var ^= ((int64_t) (void*) dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_hi);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
