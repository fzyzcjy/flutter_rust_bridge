#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncReturnDco;
typedef struct WireSyncReturnSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncReturnSse;
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_Hello_Apple {

} wire_cst_Hello_Apple;

typedef struct wire_cst_Hello_Orange {
  int32_t field0;
} wire_cst_Hello_Orange;

typedef struct wire_cst_Hello_Raspi {
  int32_t x;
  int32_t y;
} wire_cst_Hello_Raspi;

typedef union HelloKind {
  struct wire_cst_Hello_Apple *Apple;
  struct wire_cst_Hello_Orange *Orange;
  struct wire_cst_Hello_Raspi *Raspi;
} HelloKind;

typedef struct wire_cst_hello {
  int32_t tag;
  union HelloKind *kind;
} wire_cst_hello;

void frb_initialize_rust(MessagePort dart_opaque_drop_port, MessagePort dart_fn_invoke_port);

void wire_hello(int64_t port_, struct wire_cst_hello *a);

void wire_minimal_adder(int64_t port_, int32_t a, int32_t b);

struct wire_cst_hello *cst_new_box_autoadd_hello(void);

union HelloKind *cst_inflate_Hello_Orange(void);

union HelloKind *cst_inflate_Hello_Raspi(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_inflate_Hello_Orange);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Hello_Raspi);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_hello);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frb_initialize_rust);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_hello);
    dummy_var ^= ((int64_t) (void*) wire_minimal_adder);
    return dummy_var;
}
