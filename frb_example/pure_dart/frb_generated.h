#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct DartCObject *WireSyncReturn;
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_struct_with_comments_twin_normal {
  int32_t field_with_comments;
} wire_struct_with_comments_twin_normal;

void wire_StructWithCommentsTwinNormal_instance_method(int64_t port_,
                                                       struct wire_struct_with_comments_twin_normal *that);

void wire_StructWithCommentsTwinNormal_static_method(int64_t port_);

void wire_function_with_comments_slash_star_star_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line_twin_normal(int64_t port_);

void wire_function_with_comments_triple_slash_single_line_twin_normal(int64_t port_);

void wire_simple_adder_twin_normal(int64_t port_, int32_t a, int32_t b);

struct wire_struct_with_comments_twin_normal *new_box_autoadd_struct_with_comments_twin_normal(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_comments_twin_normal);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_instance_method);
    dummy_var ^= ((int64_t) (void*) wire_StructWithCommentsTwinNormal_static_method);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line_twin_normal);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_twin_normal);
    return dummy_var;
}
