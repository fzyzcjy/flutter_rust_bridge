#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;
#include "dart_api.h"
#include "dart_native_api.h"

typedef struct Result_JsValue Result_JsValue;

typedef JsValue GeneralizedDartHandle;

typedef Dart_Handle GeneralizedDartHandle;

typedef Dart_CObject *WireSyncReturn;

extern struct Result_JsValue post_message(const PortLike *this_, const JsValue *value);

extern struct Result_JsValue close(const PortLike *this_);

extern void js_console_error(const str *msg);

void dart_opaque_drop_thread_box_persistent_handle(uintptr_t ptr);

GeneralizedDartHandle dart_opaque_rust2dart_decode(uintptr_t ptr);

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
