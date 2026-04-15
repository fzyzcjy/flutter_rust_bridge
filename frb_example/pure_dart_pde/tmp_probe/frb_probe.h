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

#define CONST_INT_TWIN_NORMAL 42

#define CONST_WITH_EXPLICIT_IGNORE_SHOULD_IGNORE 42

typedef struct benchmark_raw_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} benchmark_raw_list_prim_u_8;

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_more_than_just_one_raw_string_struct_twin_normal {
  struct wire_cst_list_prim_u_8_strict *regular;
  struct wire_cst_list_prim_u_8_strict *type;
  bool async;
  struct wire_cst_list_prim_u_8_strict *another;
} wire_cst_more_than_just_one_raw_string_struct_twin_normal;

typedef struct wire_cst_RawStringItemEnumTwinNormal_Regular {
  struct wire_cst_list_prim_u_8_strict *regular;
} wire_cst_RawStringItemEnumTwinNormal_Regular;

typedef struct wire_cst_RawStringItemEnumTwinNormal_Raw {
  struct wire_cst_list_prim_u_8_strict *type;
} wire_cst_RawStringItemEnumTwinNormal_Raw;

typedef union RawStringItemEnumTwinNormalKind {
  struct wire_cst_RawStringItemEnumTwinNormal_Regular Regular;
  struct wire_cst_RawStringItemEnumTwinNormal_Raw Raw;
} RawStringItemEnumTwinNormalKind;

typedef struct wire_cst_raw_string_item_enum_twin_normal {
  int32_t tag;
  union RawStringItemEnumTwinNormalKind kind;
} wire_cst_raw_string_item_enum_twin_normal;

typedef struct wire_cst_raw_string_item_struct_twin_normal {
  struct wire_cst_list_prim_u_8_strict *type;
} wire_cst_raw_string_item_struct_twin_normal;

void benchmark_raw_void_sync(void);

struct benchmark_raw_list_prim_u_8 benchmark_raw_new_list_prim_u_8(int32_t len);

int32_t benchmark_raw_input_bytes(struct benchmark_raw_list_prim_u_8 bytes);

void benchmark_raw_output_bytes(int64_t port, int32_t message_id, int32_t size);

void frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_more_than_just_one_raw_string_struct_twin_normal(int64_t port_);

void frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_raw_string_item_enum_twin_normal(int64_t port_);

void frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_raw_string_item_struct_twin_normal(int64_t port_);

struct wire_cst_list_prim_u_8_strict *frbgen_frb_example_pure_dart_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_pure_dart_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_more_than_just_one_raw_string_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_raw_string_item_enum_twin_normal);
    dummy_var ^= ((int64_t) (void*) frbgen_frb_example_pure_dart_wire__crate__api__raw_string__test_raw_string_item_struct_twin_normal);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
