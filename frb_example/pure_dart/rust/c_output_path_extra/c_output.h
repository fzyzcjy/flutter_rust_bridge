#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_MySize {
  int32_t width;
  int32_t height;
} wire_MySize;

typedef struct wire_MySizeFreezed {
  int32_t width;
  int32_t height;
} wire_MySizeFreezed;

typedef struct wire_NewTypeInt {
  int64_t field0;
} wire_NewTypeInt;

typedef struct wire_list_my_size {
  struct wire_MySize *ptr;
  int32_t len;
} wire_list_my_size;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_list_my_tree_node {
  struct wire_MyTreeNode *ptr;
  int32_t len;
} wire_list_my_tree_node;

typedef struct wire_MyTreeNode {
  int32_t value_i32;
  struct wire_uint_8_list *value_vec_u8;
  bool value_boolean;
  struct wire_list_my_tree_node *children;
} wire_MyTreeNode;

typedef struct wire_MyNestedStruct {
  struct wire_MyTreeNode tree_node;
  int32_t weekday;
} wire_MyNestedStruct;

typedef struct wire_int_8_list {
  int8_t *ptr;
  int32_t len;
} wire_int_8_list;

typedef struct wire_int_32_list {
  int32_t *ptr;
  int32_t len;
} wire_int_32_list;

typedef struct wire_float_32_list {
  float *ptr;
  int32_t len;
} wire_float_32_list;

typedef struct wire_float_64_list {
  double *ptr;
  int32_t len;
} wire_float_64_list;

typedef struct wire_Attribute {
  struct wire_uint_8_list *key;
  struct wire_uint_8_list *value;
} wire_Attribute;

typedef struct wire_list_attribute {
  struct wire_Attribute *ptr;
  int32_t len;
} wire_list_attribute;

typedef struct wire_list_opt_box_autoadd_attribute {
  struct wire_Attribute **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_attribute;

typedef struct wire_ExoticOptionals {
  int32_t *int32;
  int64_t *int64;
  double *float64;
  bool *boolean;
  struct wire_uint_8_list *zerocopy;
  struct wire_int_8_list *int8list;
  struct wire_uint_8_list *uint8list;
  struct wire_int_32_list *int32list;
  struct wire_float_32_list *float32list;
  struct wire_float_64_list *float64list;
  struct wire_list_attribute *attributes;
  struct wire_list_opt_box_autoadd_attribute *attributes_nullable;
  struct wire_list_opt_box_autoadd_attribute *nullable_attributes;
  struct wire_NewTypeInt *newtypeint;
} wire_ExoticOptionals;

typedef struct wire_list_opt_box_autoadd_i32 {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_i32;

typedef struct wire_list_opt_box_autoadd_weekdays {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_box_autoadd_weekdays;

typedef struct wire_list_opt_String {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_list_opt_String;

typedef struct wire_list_opt_int_32_list {
  struct wire_int_32_list **ptr;
  int32_t len;
} wire_list_opt_int_32_list;

typedef struct wire_OptVecs {
  struct wire_list_opt_box_autoadd_i32 *i32;
  struct wire_list_opt_box_autoadd_weekdays *enums;
  struct wire_list_opt_String *strings;
  struct wire_list_opt_int_32_list *buffers;
} wire_OptVecs;

typedef struct wire_Note {
  int32_t *day;
  struct wire_uint_8_list *body;
} wire_Note;

typedef struct wire_MyEnumFreezed_A {
  int32_t field0;
} wire_MyEnumFreezed_A;

typedef struct wire_MyEnumFreezed_B {
  struct wire_uint_8_list *field0;
} wire_MyEnumFreezed_B;

typedef union MyEnumFreezedKind {
  struct wire_MyEnumFreezed_A *A;
  struct wire_MyEnumFreezed_B *B;
} MyEnumFreezedKind;

typedef struct wire_MyEnumFreezed {
  int32_t tag;
  union MyEnumFreezedKind *kind;
} wire_MyEnumFreezed;

typedef struct wire_Customized {
  struct wire_uint_8_list *final_field;
  struct wire_uint_8_list *non_final_field;
} wire_Customized;

typedef struct wire_KitchenSink_Empty {

} wire_KitchenSink_Empty;

typedef struct wire_KitchenSink_Primitives {
  int32_t int32;
  double float64;
  bool boolean;
} wire_KitchenSink_Primitives;

typedef struct wire_KitchenSink_Nested {
  int32_t field0;
  struct wire_KitchenSink *field1;
} wire_KitchenSink_Nested;

typedef struct wire_KitchenSink_Optional {
  int32_t *field0;
  int32_t *field1;
} wire_KitchenSink_Optional;

typedef struct wire_KitchenSink_Buffer {
  struct wire_uint_8_list *field0;
} wire_KitchenSink_Buffer;

typedef struct wire_KitchenSink_Enums {
  int32_t field0;
} wire_KitchenSink_Enums;

typedef union KitchenSinkKind {
  struct wire_KitchenSink_Empty *Empty;
  struct wire_KitchenSink_Primitives *Primitives;
  struct wire_KitchenSink_Nested *Nested;
  struct wire_KitchenSink_Optional *Optional;
  struct wire_KitchenSink_Buffer *Buffer;
  struct wire_KitchenSink_Enums *Enums;
} KitchenSinkKind;

typedef struct wire_KitchenSink {
  int32_t tag;
  union KitchenSinkKind *kind;
} wire_KitchenSink;

typedef struct wire_MyStruct {
  bool content;
} wire_MyStruct;

typedef struct wire_ApplicationEnvVar {
  struct wire_uint_8_list *field0;
  bool field1;
} wire_ApplicationEnvVar;

typedef struct wire_list_application_env_var {
  struct wire_ApplicationEnvVar *ptr;
  int32_t len;
} wire_list_application_env_var;

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

typedef struct wire_Numbers {
  struct wire_int_32_list *field0;
} wire_Numbers;

typedef struct wire_Sequences {
  struct wire_int_32_list *field0;
} wire_Sequences;

typedef struct wire_UserId {
  uint32_t value;
} wire_UserId;

typedef struct wire_Speed_Unknown {

} wire_Speed_Unknown;

typedef struct wire_Speed_GPS {
  double field0;
} wire_Speed_GPS;

typedef union SpeedKind {
  struct wire_Speed_Unknown *Unknown;
  struct wire_Speed_GPS *GPS;
} SpeedKind;

typedef struct wire_Speed {
  int32_t tag;
  union SpeedKind *kind;
} wire_Speed;

typedef struct wire_Measure_Speed {
  struct wire_Speed *field0;
} wire_Measure_Speed;

typedef struct wire_Distance_Unknown {

} wire_Distance_Unknown;

typedef struct wire_Distance_Map {
  double field0;
} wire_Distance_Map;

typedef union DistanceKind {
  struct wire_Distance_Unknown *Unknown;
  struct wire_Distance_Map *Map;
} DistanceKind;

typedef struct wire_Distance {
  int32_t tag;
  union DistanceKind *kind;
} wire_Distance;

typedef struct wire_Measure_Distance {
  struct wire_Distance *field0;
} wire_Measure_Distance;

typedef union MeasureKind {
  struct wire_Measure_Speed *Speed;
  struct wire_Measure_Distance *Distance;
} MeasureKind;

typedef struct wire_Measure {
  int32_t tag;
  union MeasureKind *kind;
} wire_Measure;

typedef struct wire_int_64_list {
  int64_t *ptr;
  int32_t len;
} wire_int_64_list;

typedef struct wire_FeatureChrono {
  int64_t utc;
  int64_t local;
  int64_t duration;
  int64_t naive;
} wire_FeatureChrono;

typedef struct wire_FeatureUuid {
  struct wire_uint_8_list *one;
  struct wire_uint_8_list *many;
} wire_FeatureUuid;

typedef struct wire_MessageId {
  struct wire_uint_8_list *field0;
} wire_MessageId;

typedef struct wire_Blob {
  struct wire_uint_8_list *field0;
} wire_Blob;

typedef struct wire_FeedId {
  struct wire_uint_8_list *field0;
} wire_FeedId;

typedef struct wire_TestId {
  struct wire_int_32_list *field0;
} wire_TestId;

typedef struct wire_list_test_id {
  struct wire_TestId *ptr;
  int32_t len;
} wire_list_test_id;

typedef struct wire_DartOpaque {
  int64_t port;
  uintptr_t handle;
} wire_DartOpaque;

typedef struct wire_list_DartOpaque {
  struct wire_DartOpaque *ptr;
  int32_t len;
} wire_list_DartOpaque;

typedef struct wire_HideData {
  const void *ptr;
} wire_HideData;

typedef struct wire_EnumOpaque_Struct {
  struct wire_HideData field0;
} wire_EnumOpaque_Struct;

typedef struct wire_I32 {
  const void *ptr;
} wire_I32;

typedef struct wire_EnumOpaque_Primitive {
  struct wire_I32 field0;
} wire_EnumOpaque_Primitive;

typedef struct wire_BoxDartDebug {
  const void *ptr;
} wire_BoxDartDebug;

typedef struct wire_EnumOpaque_TraitObj {
  struct wire_BoxDartDebug field0;
} wire_EnumOpaque_TraitObj;

typedef struct wire_MutexHideData {
  const void *ptr;
} wire_MutexHideData;

typedef struct wire_EnumOpaque_Mutex {
  struct wire_MutexHideData field0;
} wire_EnumOpaque_Mutex;

typedef struct wire_RwLockHideData {
  const void *ptr;
} wire_RwLockHideData;

typedef struct wire_EnumOpaque_RwLock {
  struct wire_RwLockHideData field0;
} wire_EnumOpaque_RwLock;

typedef union EnumOpaqueKind {
  struct wire_EnumOpaque_Struct *Struct;
  struct wire_EnumOpaque_Primitive *Primitive;
  struct wire_EnumOpaque_TraitObj *TraitObj;
  struct wire_EnumOpaque_Mutex *Mutex;
  struct wire_EnumOpaque_RwLock *RwLock;
} EnumOpaqueKind;

typedef struct wire_EnumOpaque {
  int32_t tag;
  union EnumOpaqueKind *kind;
} wire_EnumOpaque;

typedef struct wire_NonCloneData {
  const void *ptr;
} wire_NonCloneData;

typedef struct wire_NonSendHideData {
  const void *ptr;
} wire_NonSendHideData;

typedef struct wire_list_HideData {
  struct wire_HideData *ptr;
  int32_t len;
} wire_list_HideData;

typedef struct wire_OpaqueNested {
  struct wire_HideData first;
  struct wire_HideData second;
} wire_OpaqueNested;

typedef struct wire_DartOpaqueNested {
  struct wire_DartOpaque first;
  struct wire_DartOpaque second;
} wire_DartOpaqueNested;

typedef struct wire_EnumDartOpaque_Primitive {
  int32_t field0;
} wire_EnumDartOpaque_Primitive;

typedef struct wire_EnumDartOpaque_Opaque {
  struct wire_DartOpaque field0;
} wire_EnumDartOpaque_Opaque;

typedef union EnumDartOpaqueKind {
  struct wire_EnumDartOpaque_Primitive *Primitive;
  struct wire_EnumDartOpaque_Opaque *Opaque;
} EnumDartOpaqueKind;

typedef struct wire_EnumDartOpaque {
  int32_t tag;
  union EnumDartOpaqueKind *kind;
} wire_EnumDartOpaque;

typedef struct wire_Empty {

} wire_Empty;

typedef struct wire_list_weekdays {
  int32_t *ptr;
  int32_t len;
} wire_list_weekdays;

typedef struct wire_A {
  struct wire_uint_8_list *a;
} wire_A;

typedef struct wire_Abc_A {
  struct wire_A *field0;
} wire_Abc_A;

typedef struct wire_B {
  int32_t b;
} wire_B;

typedef struct wire_Abc_B {
  struct wire_B *field0;
} wire_Abc_B;

typedef struct wire_C {
  bool c;
} wire_C;

typedef struct wire_Abc_C {
  struct wire_C *field0;
} wire_Abc_C;

typedef struct wire_Abc_JustInt {
  int32_t field0;
} wire_Abc_JustInt;

typedef union AbcKind {
  struct wire_Abc_A *A;
  struct wire_Abc_B *B;
  struct wire_Abc_C *C;
  struct wire_Abc_JustInt *JustInt;
} AbcKind;

typedef struct wire_Abc {
  int32_t tag;
  union AbcKind *kind;
} wire_Abc;

typedef struct wire_StructWithEnum {
  struct wire_Abc abc1;
  struct wire_Abc abc2;
} wire_StructWithEnum;

typedef struct wire___record__String_i32 {
  struct wire_uint_8_list *field0;
  int32_t field1;
} wire___record__String_i32;

typedef struct wire_list___record__String_i32 {
  struct wire___record__String_i32 *ptr;
  int32_t len;
} wire_list___record__String_i32;

typedef struct wire_Event {
  struct wire_uint_8_list *address;
  struct wire_uint_8_list *payload;
} wire_Event;

typedef struct wire_SumWith {
  uint32_t x;
} wire_SumWith;

typedef struct wire_ConcatenateWith {
  struct wire_uint_8_list *a;
} wire_ConcatenateWith;

typedef struct wire_SomeStruct {
  uint32_t value;
} wire_SomeStruct;

typedef struct wire_CustomStruct {
  struct wire_uint_8_list *message;
} wire_CustomStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_simple_adder(int64_t port_, int32_t a, int32_t b);

WireSyncReturn wire_simple_adder_sync(int32_t a, int32_t b);

void wire_primitive_types(int64_t port_,
                          int32_t my_i32,
                          int64_t my_i64,
                          double my_f64,
                          bool my_bool);

void wire_primitive_optional_types(int64_t port_,
                                   int32_t *my_i32,
                                   int64_t *my_i64,
                                   double *my_f64,
                                   bool *my_bool);

WireSyncReturn wire_primitive_types_sync(int32_t my_i32,
                                         int64_t my_i64,
                                         double my_f64,
                                         bool my_bool);

void wire_primitive_u32(int64_t port_, uint32_t my_u32);

WireSyncReturn wire_primitive_u32_sync(uint32_t my_u32);

void wire_handle_string(int64_t port_, struct wire_uint_8_list *s);

WireSyncReturn wire_handle_string_sync(struct wire_uint_8_list *s);

void wire_handle_return_unit(int64_t port_);

WireSyncReturn wire_handle_return_unit_sync(void);

void wire_handle_vec_u8(int64_t port_, struct wire_uint_8_list *v);

WireSyncReturn wire_handle_vec_u8_sync(struct wire_uint_8_list *v);

void wire_handle_vec_of_primitive(int64_t port_, int32_t n);

WireSyncReturn wire_handle_vec_of_primitive_sync(int32_t n);

void wire_handle_zero_copy_vec_of_primitive(int64_t port_, int32_t n);

WireSyncReturn wire_handle_zero_copy_vec_of_primitive_sync(int32_t n);

void wire_handle_struct(int64_t port_, struct wire_MySize *arg, struct wire_MySize *boxed);

WireSyncReturn wire_handle_struct_sync(struct wire_MySize *arg, struct wire_MySize *boxed);

WireSyncReturn wire_handle_struct_sync_freezed(struct wire_MySizeFreezed *arg,
                                               struct wire_MySizeFreezed *boxed);

void wire_handle_newtype(int64_t port_, struct wire_NewTypeInt *arg);

WireSyncReturn wire_handle_newtype_sync(struct wire_NewTypeInt *arg);

void wire_handle_list_of_struct(int64_t port_, struct wire_list_my_size *l);

WireSyncReturn wire_handle_list_of_struct_sync(struct wire_list_my_size *l);

void wire_handle_string_list(int64_t port_, struct wire_StringList *names);

WireSyncReturn wire_handle_string_list_sync(struct wire_StringList *names);

void wire_handle_complex_struct(int64_t port_, struct wire_MyTreeNode *s);

WireSyncReturn wire_handle_complex_struct_sync(struct wire_MyTreeNode *s);

void wire_handle_nested_struct(int64_t port_, struct wire_MyNestedStruct *s);

WireSyncReturn wire_handle_sync_return(struct wire_uint_8_list *mode);

void wire_handle_stream(int64_t port_, struct wire_uint_8_list *arg);

void wire_handle_stream_of_struct(int64_t port_);

void wire_return_err(int64_t port_);

void wire_return_panic(int64_t port_);

void wire_handle_optional_return(int64_t port_, double left, double right);

void wire_handle_optional_struct(int64_t port_, struct wire_uint_8_list *document);

void wire_handle_optional_increment(int64_t port_, struct wire_ExoticOptionals *opt);

void wire_handle_increment_boxed_optional(int64_t port_, double *opt);

void wire_handle_vec_of_opts(int64_t port_, struct wire_OptVecs *opt);

void wire_handle_option_box_arguments(int64_t port_,
                                      int8_t *i8box,
                                      uint8_t *u8box,
                                      int32_t *i32box,
                                      int64_t *i64box,
                                      double *f64box,
                                      bool *boolbox,
                                      struct wire_ExoticOptionals *structbox);

void wire_print_note(int64_t port_, struct wire_Note *note);

void wire_handle_return_enum(int64_t port_, struct wire_uint_8_list *input);

void wire_handle_enum_parameter(int64_t port_, int32_t weekday);

WireSyncReturn wire_handle_enum_sync_freezed(struct wire_MyEnumFreezed *value);

void wire_handle_customized_struct(int64_t port_, struct wire_Customized *val);

void wire_handle_enum_struct(int64_t port_, struct wire_KitchenSink *val);

void wire_use_imported_struct(int64_t port_, struct wire_MyStruct *my_struct);

void wire_use_imported_enum(int64_t port_, int32_t my_enum);

void wire_get_app_settings(int64_t port_);

void wire_get_fallible_app_settings(int64_t port_);

void wire_is_app_embedded(int64_t port_, struct wire_ApplicationSettings *app_settings);

void wire_app_settings_stream(int64_t port_);

void wire_app_settings_vec_stream(int64_t port_);

void wire_mirror_struct_stream(int64_t port_);

void wire_mirror_tuple_stream(int64_t port_);

void wire_get_message(int64_t port_);

void wire_repeat_number(int64_t port_, int32_t num, uintptr_t times);

void wire_repeat_sequence(int64_t port_, int32_t seq, uintptr_t times);

void wire_first_number(int64_t port_, struct wire_Numbers *nums);

void wire_first_sequence(int64_t port_, struct wire_Sequences *seqs);

void wire_get_array(int64_t port_);

void wire_get_complex_array(int64_t port_);

void wire_get_usize(int64_t port_, uintptr_t u);

void wire_next_user_id(int64_t port_, struct wire_UserId *user_id);

void wire_register_event_listener(int64_t port_);

void wire_close_event_listener(int64_t port_);

void wire_create_event(int64_t port_,
                       struct wire_uint_8_list *address,
                       struct wire_uint_8_list *payload);

void wire_handle_stream_sink_at_1(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_2(int64_t port_, uint32_t key, uint32_t max);

void wire_handle_stream_sink_at_3(int64_t port_, uint32_t key, uint32_t max);

void wire_get_sum_struct(int64_t port_);

void wire_get_sum_array(int64_t port_, uint32_t a, uint32_t b, uint32_t c);

void wire_multiply_by_ten(int64_t port_, struct wire_Measure *measure);

void wire_call_old_module_system(int64_t port_);

void wire_call_new_module_system(int64_t port_);

void wire_handle_big_buffers(int64_t port_);

void wire_datetime_utc(int64_t port_, int64_t d);

void wire_datetime_local(int64_t port_, int64_t d);

void wire_naivedatetime(int64_t port_, int64_t d);

void wire_optional_empty_datetime_utc(int64_t port_, int64_t *d);

void wire_duration(int64_t port_, int64_t d);

void wire_handle_timestamps(int64_t port_, struct wire_int_64_list *timestamps, int64_t epoch);

void wire_handle_durations(int64_t port_, struct wire_int_64_list *durations, int64_t since);

void wire_test_chrono(int64_t port_);

void wire_test_precise_chrono(int64_t port_);

void wire_how_long_does_it_take(int64_t port_, struct wire_FeatureChrono *mine);

void wire_handle_uuid(int64_t port_, struct wire_uint_8_list *id);

void wire_handle_uuids(int64_t port_, struct wire_uint_8_list *ids);

void wire_handle_nested_uuids(int64_t port_, struct wire_FeatureUuid *ids);

void wire_new_msgid(int64_t port_, struct wire_uint_8_list *id);

void wire_use_msgid(int64_t port_, struct wire_MessageId *id);

void wire_boxed_blob(int64_t port_, struct wire_uint_8_list *blob);

void wire_use_boxed_blob(int64_t port_, struct wire_Blob *blob);

void wire_return_boxed_feed_id(int64_t port_, struct wire_uint_8_list *id);

void wire_return_boxed_raw_feed_id(int64_t port_, struct wire_FeedId *id);

void wire_test_id(int64_t port_, struct wire_TestId *id);

void wire_last_number(int64_t port_, struct wire_float_64_list *array);

void wire_nested_id(int64_t port_, struct wire_list_test_id *id);

WireSyncReturn wire_sync_accept_dart_opaque(struct wire_DartOpaque opaque);

void wire_async_accept_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_option(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_array(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_vec(int64_t port_, struct wire_DartOpaque opaque);

void wire_loop_back_option_get(int64_t port_, struct wire_DartOpaque *opaque);

void wire_loop_back_array_get(int64_t port_, struct wire_list_DartOpaque *opaque);

void wire_loop_back_vec_get(int64_t port_, struct wire_list_DartOpaque *opaque);

WireSyncReturn wire_unwrap_dart_opaque(struct wire_DartOpaque opaque);

void wire_panic_unwrap_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_create_opaque(int64_t port_);

void wire_create_option_opaque(int64_t port_, struct wire_HideData *opaque);

WireSyncReturn wire_sync_create_opaque(void);

void wire_create_array_opaque_enum(int64_t port_);

void wire_run_enum_opaque(int64_t port_, struct wire_EnumOpaque *opaque);

void wire_run_opaque(int64_t port_, struct wire_HideData opaque);

void wire_run_opaque_with_delay(int64_t port_, struct wire_HideData opaque);

void wire_opaque_array(int64_t port_);

WireSyncReturn wire_sync_create_non_clone(void);

void wire_run_non_clone(int64_t port_, struct wire_NonCloneData clone);

void wire_create_sync_opaque(int64_t port_);

WireSyncReturn wire_sync_create_sync_opaque(void);

WireSyncReturn wire_sync_run_opaque(struct wire_NonSendHideData opaque);

void wire_opaque_array_run(int64_t port_, struct wire_list_HideData *data);

void wire_opaque_vec(int64_t port_);

void wire_opaque_vec_run(int64_t port_, struct wire_list_HideData *data);

void wire_create_nested_opaque(int64_t port_);

WireSyncReturn wire_sync_loopback(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_option_loopback(struct wire_DartOpaque *opaque);

WireSyncReturn wire_sync_option(void);

WireSyncReturn wire_sync_option_null(void);

WireSyncReturn wire_sync_option_rust_opaque(void);

WireSyncReturn wire_sync_option_dart_opaque(struct wire_DartOpaque opaque);

WireSyncReturn wire_sync_void(void);

void wire_run_nested_opaque(int64_t port_, struct wire_OpaqueNested *opaque);

void wire_create_nested_dart_opaque(int64_t port_,
                                    struct wire_DartOpaque opaque1,
                                    struct wire_DartOpaque opaque2);

void wire_get_nested_dart_opaque(int64_t port_, struct wire_DartOpaqueNested *opaque);

void wire_create_enum_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_get_enum_dart_opaque(int64_t port_, struct wire_EnumDartOpaque *opaque);

void wire_set_static_dart_opaque(int64_t port_, struct wire_DartOpaque opaque);

void wire_drop_static_dart_opaque(int64_t port_);

void wire_unwrap_rust_opaque(int64_t port_, struct wire_HideData opaque);

WireSyncReturn wire_return_non_droppable_dart_opaque(struct wire_DartOpaque opaque);

void wire_frb_generator_test(int64_t port_);

WireSyncReturn wire_frb_sync_generator_test(void);

void wire_handle_type_alias_id(int64_t port_, uint64_t input);

void wire_handle_type_nest_alias_id(int64_t port_, uint64_t input);

void wire_handle_type_alias_model(int64_t port_, uint64_t input);

void wire_empty_struct(int64_t port_, struct wire_Empty *empty);

void wire_return_dart_dynamic(int64_t port_);

void wire_test_raw_string_item_struct(int64_t port_);

void wire_test_more_than_just_one_raw_string_struct(int64_t port_);

void wire_test_raw_string_mirrored(int64_t port_);

void wire_test_nested_raw_string_mirrored(int64_t port_);

void wire_test_raw_string_enum_mirrored(int64_t port_, bool nested);

void wire_test_list_of_raw_nested_string_mirrored(int64_t port_);

void wire_test_fallible_of_raw_string_mirrored(int64_t port_);

void wire_list_of_primitive_enums(int64_t port_, struct wire_list_weekdays *weekdays);

void wire_test_abc_enum(int64_t port_, struct wire_Abc *abc);

void wire_test_contains_mirrored_sub_struct(int64_t port_);

void wire_test_struct_with_enum(int64_t port_, struct wire_StructWithEnum *se);

void wire_test_tuple(int64_t port_, struct wire___record__String_i32 *value);

void wire_test_tuple_2(int64_t port_, struct wire_list___record__String_i32 *value);

WireSyncReturn wire_sync_return_mirror(void);

void wire_macro_struct(int64_t port_);

void wire_return_err_custom_error(int64_t port_);

void wire_return_ok_custom_error(int64_t port_);

void wire_return_error_variant(int64_t port_, uint32_t variant);

void wire_return_custom_nested_error_1(int64_t port_);

void wire_return_custom_nested_error_1_variant1(int64_t port_);

void wire_return_custom_nested_error_2(int64_t port_);

void wire_return_custom_struct_error(int64_t port_);

WireSyncReturn wire_sync_return_custom_struct_error(void);

void wire_return_custom_struct_ok(int64_t port_);

void wire_throw_anyhow(int64_t port_);

void wire_panic_with_custom_result(int64_t port_);

void wire_stream_sink_throw_anyhow(int64_t port_);

void wire_as_string__method__Event(int64_t port_, struct wire_Event *that);

void wire_sum__method__SumWith(int64_t port_, struct wire_SumWith *that, uint32_t y, uint32_t z);

void wire_new__static_method__ConcatenateWith(int64_t port_, struct wire_uint_8_list *a);

void wire_concatenate__method__ConcatenateWith(int64_t port_,
                                               struct wire_ConcatenateWith *that,
                                               struct wire_uint_8_list *b);

void wire_concatenate_static__static_method__ConcatenateWith(int64_t port_,
                                                             struct wire_uint_8_list *a,
                                                             struct wire_uint_8_list *b);

void wire_handle_some_stream_sink__method__ConcatenateWith(int64_t port_,
                                                           struct wire_ConcatenateWith *that,
                                                           uint32_t key,
                                                           uint32_t max);

void wire_handle_some_stream_sink_at_1__method__ConcatenateWith(int64_t port_,
                                                                struct wire_ConcatenateWith *that);

void wire_handle_some_static_stream_sink__static_method__ConcatenateWith(int64_t port_,
                                                                         uint32_t key,
                                                                         uint32_t max);

void wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith(int64_t port_);

void wire_new__static_method__SomeStruct(int64_t port_, uint32_t value);

void wire_static_return_err_custom_error__static_method__SomeStruct(int64_t port_);

void wire_static_return_ok_custom_error__static_method__SomeStruct(int64_t port_);

void wire_non_static_return_err_custom_error__method__SomeStruct(int64_t port_,
                                                                 struct wire_SomeStruct *that);

void wire_non_static_return_ok_custom_error__method__SomeStruct(int64_t port_,
                                                                struct wire_SomeStruct *that);

void wire_new__static_method__CustomStruct(int64_t port_, struct wire_uint_8_list *message);

void wire_static_return_custom_struct_error__static_method__CustomStruct(int64_t port_);

void wire_static_return_custom_struct_ok__static_method__CustomStruct(int64_t port_);

void wire_nonstatic_return_custom_struct_error__method__CustomStruct(int64_t port_,
                                                                     struct wire_CustomStruct *that);

void wire_nonstatic_return_custom_struct_ok__method__CustomStruct(int64_t port_,
                                                                  struct wire_CustomStruct *that);

struct wire_BoxDartDebug new_BoxDartDebug(void);

struct wire_DartOpaque new_DartOpaque(void);

struct wire_HideData new_HideData(void);

struct wire_I32 new_I32(void);

struct wire_MutexHideData new_MutexHideData(void);

struct wire_NonCloneData new_NonCloneData(void);

struct wire_NonSendHideData new_NonSendHideData(void);

struct wire_RwLockHideData new_RwLockHideData(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_ApplicationEnv *new_box_application_env_0(void);

int64_t *new_box_autoadd_Chrono_Utc_0(int64_t value);

struct wire_DartOpaque *new_box_autoadd_DartOpaque_0(void);

struct wire_HideData *new_box_autoadd_HideData_0(void);

struct wire___record__String_i32 *new_box_autoadd___record__String_i32_0(void);

struct wire_A *new_box_autoadd_a_0(void);

struct wire_Abc *new_box_autoadd_abc_0(void);

struct wire_ApplicationEnv *new_box_autoadd_application_env_0(void);

struct wire_ApplicationSettings *new_box_autoadd_application_settings_0(void);

struct wire_Attribute *new_box_autoadd_attribute_0(void);

struct wire_B *new_box_autoadd_b_0(void);

bool *new_box_autoadd_bool_0(bool value);

struct wire_C *new_box_autoadd_c_0(void);

struct wire_ConcatenateWith *new_box_autoadd_concatenate_with_0(void);

struct wire_CustomStruct *new_box_autoadd_custom_struct_0(void);

struct wire_Customized *new_box_autoadd_customized_0(void);

struct wire_DartOpaqueNested *new_box_autoadd_dart_opaque_nested_0(void);

struct wire_Empty *new_box_autoadd_empty_0(void);

struct wire_EnumDartOpaque *new_box_autoadd_enum_dart_opaque_0(void);

struct wire_EnumOpaque *new_box_autoadd_enum_opaque_0(void);

struct wire_Event *new_box_autoadd_event_0(void);

struct wire_ExoticOptionals *new_box_autoadd_exotic_optionals_0(void);

double *new_box_autoadd_f64_0(double value);

struct wire_FeatureChrono *new_box_autoadd_feature_chrono_0(void);

struct wire_FeatureUuid *new_box_autoadd_feature_uuid_0(void);

struct wire_FeedId *new_box_autoadd_feed_id_0(void);

int32_t *new_box_autoadd_i32_0(int32_t value);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_KitchenSink *new_box_autoadd_kitchen_sink_0(void);

struct wire_Measure *new_box_autoadd_measure_0(void);

struct wire_MessageId *new_box_autoadd_message_id_0(void);

struct wire_MyEnumFreezed *new_box_autoadd_my_enum_freezed_0(void);

struct wire_MyNestedStruct *new_box_autoadd_my_nested_struct_0(void);

struct wire_MySize *new_box_autoadd_my_size_0(void);

struct wire_MySizeFreezed *new_box_autoadd_my_size_freezed_0(void);

struct wire_MyStruct *new_box_autoadd_my_struct_0(void);

struct wire_MyTreeNode *new_box_autoadd_my_tree_node_0(void);

struct wire_NewTypeInt *new_box_autoadd_new_type_int_0(void);

struct wire_Note *new_box_autoadd_note_0(void);

struct wire_Numbers *new_box_autoadd_numbers_0(void);

struct wire_OpaqueNested *new_box_autoadd_opaque_nested_0(void);

struct wire_OptVecs *new_box_autoadd_opt_vecs_0(void);

struct wire_Sequences *new_box_autoadd_sequences_0(void);

struct wire_SomeStruct *new_box_autoadd_some_struct_0(void);

struct wire_StructWithEnum *new_box_autoadd_struct_with_enum_0(void);

struct wire_SumWith *new_box_autoadd_sum_with_0(void);

struct wire_TestId *new_box_autoadd_test_id_0(void);

struct wire_UserId *new_box_autoadd_user_id_0(void);

int32_t *new_box_autoadd_weekdays_0(int32_t value);

struct wire_Blob *new_box_blob_0(void);

bool *new_box_bool_0(bool value);

struct wire_Distance *new_box_distance_0(void);

struct wire_ExoticOptionals *new_box_exotic_optionals_0(void);

double *new_box_f64_0(double value);

int32_t *new_box_i32_0(int32_t value);

int64_t *new_box_i64_0(int64_t value);

int8_t *new_box_i8_0(int8_t value);

struct wire_KitchenSink *new_box_kitchen_sink_0(void);

struct wire_MySize *new_box_my_size_0(void);

struct wire_MySizeFreezed *new_box_my_size_freezed_0(void);

struct wire_Speed *new_box_speed_0(void);

uint8_t *new_box_u8_0(uint8_t value);

int32_t *new_box_weekdays_0(int32_t value);

struct wire_float_32_list *new_float_32_list_0(int32_t len);

struct wire_float_64_list *new_float_64_list_0(int32_t len);

struct wire_int_32_list *new_int_32_list_0(int32_t len);

struct wire_int_64_list *new_int_64_list_0(int32_t len);

struct wire_int_8_list *new_int_8_list_0(int32_t len);

struct wire_list_DartOpaque *new_list_DartOpaque_0(int32_t len);

struct wire_list_HideData *new_list_HideData_0(int32_t len);

struct wire_list___record__String_i32 *new_list___record__String_i32_0(int32_t len);

struct wire_list_application_env_var *new_list_application_env_var_0(int32_t len);

struct wire_list_attribute *new_list_attribute_0(int32_t len);

struct wire_list_my_size *new_list_my_size_0(int32_t len);

struct wire_list_my_tree_node *new_list_my_tree_node_0(int32_t len);

struct wire_list_opt_String *new_list_opt_String_0(int32_t len);

struct wire_list_opt_box_autoadd_attribute *new_list_opt_box_autoadd_attribute_0(int32_t len);

struct wire_list_opt_box_autoadd_i32 *new_list_opt_box_autoadd_i32_0(int32_t len);

struct wire_list_opt_box_autoadd_weekdays *new_list_opt_box_autoadd_weekdays_0(int32_t len);

struct wire_list_opt_int_32_list *new_list_opt_int_32_list_0(int32_t len);

struct wire_list_test_id *new_list_test_id_0(int32_t len);

struct wire_list_weekdays *new_list_weekdays_0(int32_t len);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_BoxDartDebug(const void *ptr);

const void *share_opaque_BoxDartDebug(const void *ptr);

void drop_opaque_FrbOpaqueReturn(const void *ptr);

const void *share_opaque_FrbOpaqueReturn(const void *ptr);

void drop_opaque_FrbOpaqueSyncReturn(const void *ptr);

const void *share_opaque_FrbOpaqueSyncReturn(const void *ptr);

void drop_opaque_HideData(const void *ptr);

const void *share_opaque_HideData(const void *ptr);

void drop_opaque_I32(const void *ptr);

const void *share_opaque_I32(const void *ptr);

void drop_opaque_MutexHideData(const void *ptr);

const void *share_opaque_MutexHideData(const void *ptr);

void drop_opaque_NonCloneData(const void *ptr);

const void *share_opaque_NonCloneData(const void *ptr);

void drop_opaque_NonSendHideData(const void *ptr);

const void *share_opaque_NonSendHideData(const void *ptr);

void drop_opaque_RwLockHideData(const void *ptr);

const void *share_opaque_RwLockHideData(const void *ptr);

union AbcKind *inflate_Abc_A(void);

union AbcKind *inflate_Abc_B(void);

union AbcKind *inflate_Abc_C(void);

union AbcKind *inflate_Abc_JustInt(void);

union DistanceKind *inflate_Distance_Map(void);

union EnumDartOpaqueKind *inflate_EnumDartOpaque_Primitive(void);

union EnumDartOpaqueKind *inflate_EnumDartOpaque_Opaque(void);

union EnumOpaqueKind *inflate_EnumOpaque_Struct(void);

union EnumOpaqueKind *inflate_EnumOpaque_Primitive(void);

union EnumOpaqueKind *inflate_EnumOpaque_TraitObj(void);

union EnumOpaqueKind *inflate_EnumOpaque_Mutex(void);

union EnumOpaqueKind *inflate_EnumOpaque_RwLock(void);

union KitchenSinkKind *inflate_KitchenSink_Primitives(void);

union KitchenSinkKind *inflate_KitchenSink_Nested(void);

union KitchenSinkKind *inflate_KitchenSink_Optional(void);

union KitchenSinkKind *inflate_KitchenSink_Buffer(void);

union KitchenSinkKind *inflate_KitchenSink_Enums(void);

union MeasureKind *inflate_Measure_Speed(void);

union MeasureKind *inflate_Measure_Distance(void);

union MyEnumFreezedKind *inflate_MyEnumFreezed_A(void);

union MyEnumFreezedKind *inflate_MyEnumFreezed_B(void);

union SpeedKind *inflate_Speed_GPS(void);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_simple_adder);
    dummy_var ^= ((int64_t) (void*) wire_simple_adder_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types);
    dummy_var ^= ((int64_t) (void*) wire_primitive_optional_types);
    dummy_var ^= ((int64_t) (void*) wire_primitive_types_sync);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32);
    dummy_var ^= ((int64_t) (void*) wire_primitive_u32_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_string);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_unit);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_unit_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_u8_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_primitive_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive);
    dummy_var ^= ((int64_t) (void*) wire_handle_zero_copy_vec_of_primitive_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_struct_sync_freezed);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype);
    dummy_var ^= ((int64_t) (void*) wire_handle_newtype_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_list_of_struct_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list);
    dummy_var ^= ((int64_t) (void*) wire_handle_string_list_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_complex_struct_sync);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_sync_return);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_of_struct);
    dummy_var ^= ((int64_t) (void*) wire_return_err);
    dummy_var ^= ((int64_t) (void*) wire_return_panic);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_return);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_optional_increment);
    dummy_var ^= ((int64_t) (void*) wire_handle_increment_boxed_optional);
    dummy_var ^= ((int64_t) (void*) wire_handle_vec_of_opts);
    dummy_var ^= ((int64_t) (void*) wire_handle_option_box_arguments);
    dummy_var ^= ((int64_t) (void*) wire_print_note);
    dummy_var ^= ((int64_t) (void*) wire_handle_return_enum);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_parameter);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_sync_freezed);
    dummy_var ^= ((int64_t) (void*) wire_handle_customized_struct);
    dummy_var ^= ((int64_t) (void*) wire_handle_enum_struct);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_struct);
    dummy_var ^= ((int64_t) (void*) wire_use_imported_enum);
    dummy_var ^= ((int64_t) (void*) wire_get_app_settings);
    dummy_var ^= ((int64_t) (void*) wire_get_fallible_app_settings);
    dummy_var ^= ((int64_t) (void*) wire_is_app_embedded);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_stream);
    dummy_var ^= ((int64_t) (void*) wire_app_settings_vec_stream);
    dummy_var ^= ((int64_t) (void*) wire_mirror_struct_stream);
    dummy_var ^= ((int64_t) (void*) wire_mirror_tuple_stream);
    dummy_var ^= ((int64_t) (void*) wire_get_message);
    dummy_var ^= ((int64_t) (void*) wire_repeat_number);
    dummy_var ^= ((int64_t) (void*) wire_repeat_sequence);
    dummy_var ^= ((int64_t) (void*) wire_first_number);
    dummy_var ^= ((int64_t) (void*) wire_first_sequence);
    dummy_var ^= ((int64_t) (void*) wire_get_array);
    dummy_var ^= ((int64_t) (void*) wire_get_complex_array);
    dummy_var ^= ((int64_t) (void*) wire_get_usize);
    dummy_var ^= ((int64_t) (void*) wire_next_user_id);
    dummy_var ^= ((int64_t) (void*) wire_register_event_listener);
    dummy_var ^= ((int64_t) (void*) wire_close_event_listener);
    dummy_var ^= ((int64_t) (void*) wire_create_event);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_1);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_2);
    dummy_var ^= ((int64_t) (void*) wire_handle_stream_sink_at_3);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_struct);
    dummy_var ^= ((int64_t) (void*) wire_get_sum_array);
    dummy_var ^= ((int64_t) (void*) wire_multiply_by_ten);
    dummy_var ^= ((int64_t) (void*) wire_call_old_module_system);
    dummy_var ^= ((int64_t) (void*) wire_call_new_module_system);
    dummy_var ^= ((int64_t) (void*) wire_handle_big_buffers);
    dummy_var ^= ((int64_t) (void*) wire_datetime_utc);
    dummy_var ^= ((int64_t) (void*) wire_datetime_local);
    dummy_var ^= ((int64_t) (void*) wire_naivedatetime);
    dummy_var ^= ((int64_t) (void*) wire_optional_empty_datetime_utc);
    dummy_var ^= ((int64_t) (void*) wire_duration);
    dummy_var ^= ((int64_t) (void*) wire_handle_timestamps);
    dummy_var ^= ((int64_t) (void*) wire_handle_durations);
    dummy_var ^= ((int64_t) (void*) wire_test_chrono);
    dummy_var ^= ((int64_t) (void*) wire_test_precise_chrono);
    dummy_var ^= ((int64_t) (void*) wire_how_long_does_it_take);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuid);
    dummy_var ^= ((int64_t) (void*) wire_handle_uuids);
    dummy_var ^= ((int64_t) (void*) wire_handle_nested_uuids);
    dummy_var ^= ((int64_t) (void*) wire_new_msgid);
    dummy_var ^= ((int64_t) (void*) wire_use_msgid);
    dummy_var ^= ((int64_t) (void*) wire_boxed_blob);
    dummy_var ^= ((int64_t) (void*) wire_use_boxed_blob);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_return_boxed_raw_feed_id);
    dummy_var ^= ((int64_t) (void*) wire_test_id);
    dummy_var ^= ((int64_t) (void*) wire_last_number);
    dummy_var ^= ((int64_t) (void*) wire_nested_id);
    dummy_var ^= ((int64_t) (void*) wire_sync_accept_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_async_accept_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_loop_back);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_option_get);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_array_get);
    dummy_var ^= ((int64_t) (void*) wire_loop_back_vec_get);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_panic_unwrap_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_option_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_array_opaque_enum);
    dummy_var ^= ((int64_t) (void*) wire_run_enum_opaque);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque);
    dummy_var ^= ((int64_t) (void*) wire_run_opaque_with_delay);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_non_clone);
    dummy_var ^= ((int64_t) (void*) wire_run_non_clone);
    dummy_var ^= ((int64_t) (void*) wire_create_sync_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_create_sync_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_run_opaque);
    dummy_var ^= ((int64_t) (void*) wire_opaque_array_run);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec);
    dummy_var ^= ((int64_t) (void*) wire_opaque_vec_run);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_loopback);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_loopback);
    dummy_var ^= ((int64_t) (void*) wire_sync_option);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_null);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_rust_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_option_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_sync_void);
    dummy_var ^= ((int64_t) (void*) wire_run_nested_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_nested_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_get_nested_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_create_enum_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_get_enum_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_set_static_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_drop_static_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_unwrap_rust_opaque);
    dummy_var ^= ((int64_t) (void*) wire_return_non_droppable_dart_opaque);
    dummy_var ^= ((int64_t) (void*) wire_frb_generator_test);
    dummy_var ^= ((int64_t) (void*) wire_frb_sync_generator_test);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_id);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_nest_alias_id);
    dummy_var ^= ((int64_t) (void*) wire_handle_type_alias_model);
    dummy_var ^= ((int64_t) (void*) wire_empty_struct);
    dummy_var ^= ((int64_t) (void*) wire_return_dart_dynamic);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_item_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_more_than_just_one_raw_string_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_nested_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_raw_string_enum_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_list_of_raw_nested_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_test_fallible_of_raw_string_mirrored);
    dummy_var ^= ((int64_t) (void*) wire_list_of_primitive_enums);
    dummy_var ^= ((int64_t) (void*) wire_test_abc_enum);
    dummy_var ^= ((int64_t) (void*) wire_test_contains_mirrored_sub_struct);
    dummy_var ^= ((int64_t) (void*) wire_test_struct_with_enum);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple);
    dummy_var ^= ((int64_t) (void*) wire_test_tuple_2);
    dummy_var ^= ((int64_t) (void*) wire_sync_return_mirror);
    dummy_var ^= ((int64_t) (void*) wire_macro_struct);
    dummy_var ^= ((int64_t) (void*) wire_return_err_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_return_ok_custom_error);
    dummy_var ^= ((int64_t) (void*) wire_return_error_variant);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_1_variant1);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_nested_error_2);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_sync_return_custom_struct_error);
    dummy_var ^= ((int64_t) (void*) wire_return_custom_struct_ok);
    dummy_var ^= ((int64_t) (void*) wire_throw_anyhow);
    dummy_var ^= ((int64_t) (void*) wire_panic_with_custom_result);
    dummy_var ^= ((int64_t) (void*) wire_stream_sink_throw_anyhow);
    dummy_var ^= ((int64_t) (void*) wire_as_string__method__Event);
    dummy_var ^= ((int64_t) (void*) wire_sum__method__SumWith);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_concatenate__method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_concatenate_static__static_method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_handle_some_stream_sink__method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_handle_some_stream_sink_at_1__method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_handle_some_static_stream_sink__static_method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_handle_some_static_stream_sink_single_arg__static_method__ConcatenateWith);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__SomeStruct);
    dummy_var ^= ((int64_t) (void*) wire_static_return_err_custom_error__static_method__SomeStruct);
    dummy_var ^= ((int64_t) (void*) wire_static_return_ok_custom_error__static_method__SomeStruct);
    dummy_var ^= ((int64_t) (void*) wire_non_static_return_err_custom_error__method__SomeStruct);
    dummy_var ^= ((int64_t) (void*) wire_non_static_return_ok_custom_error__method__SomeStruct);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__CustomStruct);
    dummy_var ^= ((int64_t) (void*) wire_static_return_custom_struct_error__static_method__CustomStruct);
    dummy_var ^= ((int64_t) (void*) wire_static_return_custom_struct_ok__static_method__CustomStruct);
    dummy_var ^= ((int64_t) (void*) wire_nonstatic_return_custom_struct_error__method__CustomStruct);
    dummy_var ^= ((int64_t) (void*) wire_nonstatic_return_custom_struct_ok__method__CustomStruct);
    dummy_var ^= ((int64_t) (void*) new_BoxDartDebug);
    dummy_var ^= ((int64_t) (void*) new_DartOpaque);
    dummy_var ^= ((int64_t) (void*) new_HideData);
    dummy_var ^= ((int64_t) (void*) new_I32);
    dummy_var ^= ((int64_t) (void*) new_MutexHideData);
    dummy_var ^= ((int64_t) (void*) new_NonCloneData);
    dummy_var ^= ((int64_t) (void*) new_NonSendHideData);
    dummy_var ^= ((int64_t) (void*) new_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_box_application_env_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_Chrono_Utc_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_DartOpaque_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_HideData_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd___record__String_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_a_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_abc_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_env_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_application_settings_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_attribute_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_b_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_c_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_concatenate_with_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_custom_struct_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_customized_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_dart_opaque_nested_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_empty_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_dart_opaque_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_enum_opaque_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_exotic_optionals_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_chrono_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feature_uuid_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_feed_id_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_kitchen_sink_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_measure_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_message_id_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_enum_freezed_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_nested_struct_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_size_freezed_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_struct_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_my_tree_node_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_new_type_int_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_note_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_numbers_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opaque_nested_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_opt_vecs_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sequences_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_some_struct_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_struct_with_enum_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sum_with_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_test_id_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_user_id_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_weekdays_0);
    dummy_var ^= ((int64_t) (void*) new_box_blob_0);
    dummy_var ^= ((int64_t) (void*) new_box_bool_0);
    dummy_var ^= ((int64_t) (void*) new_box_distance_0);
    dummy_var ^= ((int64_t) (void*) new_box_exotic_optionals_0);
    dummy_var ^= ((int64_t) (void*) new_box_f64_0);
    dummy_var ^= ((int64_t) (void*) new_box_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_i8_0);
    dummy_var ^= ((int64_t) (void*) new_box_kitchen_sink_0);
    dummy_var ^= ((int64_t) (void*) new_box_my_size_0);
    dummy_var ^= ((int64_t) (void*) new_box_my_size_freezed_0);
    dummy_var ^= ((int64_t) (void*) new_box_speed_0);
    dummy_var ^= ((int64_t) (void*) new_box_u8_0);
    dummy_var ^= ((int64_t) (void*) new_box_weekdays_0);
    dummy_var ^= ((int64_t) (void*) new_float_32_list_0);
    dummy_var ^= ((int64_t) (void*) new_float_64_list_0);
    dummy_var ^= ((int64_t) (void*) new_int_32_list_0);
    dummy_var ^= ((int64_t) (void*) new_int_64_list_0);
    dummy_var ^= ((int64_t) (void*) new_int_8_list_0);
    dummy_var ^= ((int64_t) (void*) new_list_DartOpaque_0);
    dummy_var ^= ((int64_t) (void*) new_list_HideData_0);
    dummy_var ^= ((int64_t) (void*) new_list___record__String_i32_0);
    dummy_var ^= ((int64_t) (void*) new_list_application_env_var_0);
    dummy_var ^= ((int64_t) (void*) new_list_attribute_0);
    dummy_var ^= ((int64_t) (void*) new_list_my_size_0);
    dummy_var ^= ((int64_t) (void*) new_list_my_tree_node_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_String_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_attribute_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_box_autoadd_weekdays_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_int_32_list_0);
    dummy_var ^= ((int64_t) (void*) new_list_test_id_0);
    dummy_var ^= ((int64_t) (void*) new_list_weekdays_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_BoxDartDebug);
    dummy_var ^= ((int64_t) (void*) share_opaque_BoxDartDebug);
    dummy_var ^= ((int64_t) (void*) drop_opaque_FrbOpaqueReturn);
    dummy_var ^= ((int64_t) (void*) share_opaque_FrbOpaqueReturn);
    dummy_var ^= ((int64_t) (void*) drop_opaque_FrbOpaqueSyncReturn);
    dummy_var ^= ((int64_t) (void*) share_opaque_FrbOpaqueSyncReturn);
    dummy_var ^= ((int64_t) (void*) drop_opaque_HideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_HideData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_I32);
    dummy_var ^= ((int64_t) (void*) share_opaque_I32);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexHideData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_NonCloneData);
    dummy_var ^= ((int64_t) (void*) share_opaque_NonCloneData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_NonSendHideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_NonSendHideData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockHideData);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_A);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_B);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_C);
    dummy_var ^= ((int64_t) (void*) inflate_Abc_JustInt);
    dummy_var ^= ((int64_t) (void*) inflate_Distance_Map);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaque_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumDartOpaque_Opaque);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Primitive);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_TraitObj);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_Mutex);
    dummy_var ^= ((int64_t) (void*) inflate_EnumOpaque_RwLock);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Primitives);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Nested);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Optional);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Buffer);
    dummy_var ^= ((int64_t) (void*) inflate_KitchenSink_Enums);
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Speed);
    dummy_var ^= ((int64_t) (void*) inflate_Measure_Distance);
    dummy_var ^= ((int64_t) (void*) inflate_MyEnumFreezed_A);
    dummy_var ^= ((int64_t) (void*) inflate_MyEnumFreezed_B);
    dummy_var ^= ((int64_t) (void*) inflate_Speed_GPS);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
