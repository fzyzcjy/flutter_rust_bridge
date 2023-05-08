#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_ApplicationEnvVar {
  struct wire_uint_8_list *field0;
  bool field1;
} wire_ApplicationEnvVar;

typedef struct wire_ApplicationEnvVar {
  struct wire_uint_8_list *field0;
  bool field1;
} wire_ApplicationEnvVar;

typedef struct wire_list_application_env_var {
  struct wire_ApplicationEnvVar *ptr;
  int32_t len;
} wire_list_application_env_var;

typedef struct wire_list_application_env_var {
  struct wire_ApplicationEnvVar *ptr;
  int32_t len;
} wire_list_application_env_var;

typedef struct wire_ApplicationEnv {
  struct wire_list_application_env_var *vars;
} wire_ApplicationEnv;

typedef struct wire_ApplicationEnv {
  struct wire_list_application_env_var *vars;
} wire_ApplicationEnv;

typedef struct wire_ApplicationSettings {
  struct wire_uint_8_list *name;
  struct wire_uint_8_list *version;
  int32_t mode;
  struct wire_ApplicationEnv *env;
  struct wire_ApplicationEnv *env_optional;
} wire_ApplicationSettings;

typedef struct wire_ApplicationSettings {
  struct wire_uint_8_list *name;
  struct wire_uint_8_list *version;
  int32_t mode;
  struct wire_ApplicationEnv *env;
  struct wire_ApplicationEnv *env_optional;
} wire_ApplicationSettings;

typedef struct wire_Point {
  double x;
  double y;
} wire_Point;

typedef struct wire_BoxedPoint {
  struct wire_Point *point;
} wire_BoxedPoint;

typedef struct wire_Size {
  int32_t width;
  int32_t height;
} wire_Size;

typedef struct wire_SumWith {
  uint32_t x;
} wire_SumWith;

typedef struct wire_list_tree_node {
  struct wire_TreeNode *ptr;
  int32_t len;
} wire_list_tree_node;

typedef struct wire_TreeNode {
  struct wire_uint_8_list *name;
  struct wire_list_tree_node *children;
} wire_TreeNode;

typedef struct wire_UserId {
  uint32_t value;
} wire_UserId;

typedef struct DartCObject *WireSyncReturn;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

struct wire_ApplicationSettings *new_box_autoadd_application_settings_0(void);

struct wire_BoxedPoint *new_box_autoadd_boxed_point_0(void);

struct wire_Point *new_box_autoadd_point_0(void);

struct wire_Size *new_box_autoadd_size_0(void);

struct wire_SumWith *new_box_autoadd_sum_with_0(void);

struct wire_TreeNode *new_box_autoadd_tree_node_0(void);

struct wire_UserId *new_box_autoadd_user_id_0(void);

void free_WireSyncReturn(WireSyncReturn ptr);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_get_app_settings_to_api2(int64_t port_);

void wire_get_fallible_app_settings_to_api2(int64_t port_);

void wire_is_app_embedded_in_api2(int64_t port_, struct wire_ApplicationSettings *app_settings);

struct wire_ApplicationEnv *new_box_application_env_1(void);

struct wire_ApplicationEnv *new_box_autoadd_application_env_1(void);

struct wire_ApplicationSettings *new_box_autoadd_application_settings_1(void);

struct wire_list_application_env_var *new_list_application_env_var_1(int32_t len);

struct wire_uint_8_list *new_uint_8_list_1(int32_t len);

static int64_t dummy_method_to_enforce_bundling_ApiClass2(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings_to_api2);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings_to_api2);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded_in_api2);
    dummy_var ^= ((int64_t) (void*) new_box_application_env_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_env_1);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_settings_1);
    dummy_var ^= ((int64_t) (void*) new_list_application_env_var_1);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_1);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
