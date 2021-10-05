#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_MySize {
  int32_t width;
  int32_t height;
} wire_MySize;

typedef struct wire_NewTypeInt {
  int64_t field0;
} wire_NewTypeInt;

typedef struct wire_list_my_size {
  struct wire_MySize *ptr;
  int32_t len;
} wire_list_my_size;

typedef struct wire_list_my_tree_node {
  struct wire_MyTreeNode *ptr;
  int32_t len;
} wire_list_my_tree_node;

typedef struct wire_MyTreeNode {
  int32_t value_i32;
  struct wire_uint_8_list *value_vec_u8;
  struct wire_list_my_tree_node *children;
} wire_MyTreeNode;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_simple_adder(int64_t port, int32_t a, int32_t b);

void wire_primitive_types(int64_t port,
                          int32_t my_i32,
                          int64_t my_i64,
                          double my_f64,
                          bool my_bool);

void wire_handle_string(int64_t port, struct wire_uint_8_list *s);

void wire_handle_vec_u8(int64_t port, struct wire_uint_8_list *v);

void wire_handle_zero_copy_result(int64_t port, int32_t n);

void wire_handle_struct(int64_t port, struct wire_MySize *arg, struct wire_MySize *boxed);

void wire_handle_newtype(int64_t port, struct wire_NewTypeInt *arg);

void wire_handle_list_of_struct(int64_t port, struct wire_list_my_size *l);

void wire_handle_complex_struct(int64_t port, struct wire_MyTreeNode *s);

void wire_return_err(int64_t port);

void wire_return_panic(int64_t port);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

struct wire_MySize *new_box_autoadd_my_size(void);

struct wire_MySize *new_box_my_size(void);

struct wire_NewTypeInt *new_box_autoadd_new_type_int(void);

struct wire_list_my_size *new_list_my_size(int32_t len);

struct wire_MyTreeNode *new_box_autoadd_my_tree_node(void);

struct wire_list_my_tree_node *new_list_my_tree_node(int32_t len);

void store_dart_post_cobject(DartPostCObjectFnType ptr);
