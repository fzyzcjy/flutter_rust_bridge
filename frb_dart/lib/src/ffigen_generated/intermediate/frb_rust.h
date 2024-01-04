#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;
typedef struct Dart_CObject DartCObject;
#include "dart_api.h"
#include "dart_native_api.h"

typedef Dart_Handle GeneralizedDartHandle;

typedef int64_t MessagePort;

typedef Dart_CObject *WireSyncRust2DartDco;

typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

/**
 * # Safety
 *
 * This should never be called manually.
 */
const void *dart_opaque_dart2rust_encode(GeneralizedDartHandle handle,
                                         MessagePort dart_handler_port);

void dart_opaque_drop_thread_box_persistent_handle(uintptr_t ptr);

GeneralizedDartHandle dart_opaque_rust2dart_decode(uintptr_t ptr);

uint8_t *rust_vec_u8_new(int32_t len);

uint8_t *rust_vec_u8_resize(uint8_t *ptr, int32_t old_len, int32_t new_len);

void rust_vec_u8_free(uint8_t *ptr, int32_t len);

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
void free_wire_sync_rust2dart_dco(WireSyncRust2DartDco value);

/**
 * # Safety
 *
 * This function should never be called manually.
 */
void free_wire_sync_rust2dart_sse(struct WireSyncRust2DartSse value);
