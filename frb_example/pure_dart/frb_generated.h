#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

void wire_function_with_comments_slash_star_star(int64_t port_);

void wire_function_with_comments_triple_slash_multi_line(int64_t port_);

void wire_function_with_comments_triple_slash_single_line(int64_t port_);

void wire_simple_adder(int64_t port_, int32_t a, int32_t b);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_slash_star_star);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_multi_line);
    dummy_var ^= ((int64_t) (void*) wire_function_with_comments_triple_slash_single_line);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder);
    return dummy_var;
}
