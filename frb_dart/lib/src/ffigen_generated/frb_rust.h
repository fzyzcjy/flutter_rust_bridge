#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Result_JsValue Result_JsValue;

typedef DartCObject *WireSyncReturn;

typedef JsValue WireSyncReturn;

extern void error(const str *msg);

extern struct Result_JsValue post_message(const PortLike *this_, const JsValue *value);

extern struct Result_JsValue close(const PortLike *this_);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
uintptr_t new_dart_opaque(Dart_Handle handle);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
Dart_Handle get_dart_object(uintptr_t ptr);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
void drop_dart_object(uintptr_t ptr);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
intptr_t init_frb_dart_api_dl(void *data);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
void free_wire_sync_return(WireSyncReturn ptr);
