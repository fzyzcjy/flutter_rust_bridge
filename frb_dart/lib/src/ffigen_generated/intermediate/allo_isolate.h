#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;
struct DartCObject;
typedef struct DartCObject DartCObject;
#include "dart_api.h"

/**
 * A port is used to send or receive inter-isolate messages
 */
typedef int64_t DartPort;

/**
 *  Posts a message on some port. The message will contain the
 *  Dart_CObject object graph rooted in 'message'.
 *
 *  While the message is being sent the state of the graph of
 *  Dart_CObject structures rooted in 'message' should not be accessed,
 *  as the message generation will make temporary modifications to the
 *  data. When the message has been sent the graph will be fully
 *  restored.
 *
 *  `port_id` The destination port.
 *  `message` The message to send.
 *
 *  return true if the message was posted.
 */
typedef bool (*DartPostCObjectFnType)(DartPort port_id, DartCObject *message);

/**
 * Stores the function pointer of `Dart_PostCObject`, this only should be
 * called once at the start up of the Dart/Flutter Application. it is exported
 * and marked as `#[no_mangle]`.
 *
 * you could use it from Dart as following:
 *
 * #### Safety
 * This function should only be called once at the start up of the Dart
 * application.
 *
 * ### Example
 * ```dart,ignore
 * import 'dart:ffi';
 *
 * typedef dartPostCObject = Pointer Function(
 *         Pointer<NativeFunction<Int8 Function(Int64,
 * Pointer<Dart_CObject>)>>);
 *
 * // assumes that _dl is the `DynamicLibrary`
 * final storeDartPostCObject =
 *     _dl.lookupFunction<dartPostCObject, dartPostCObject>(
 * 'store_dart_post_cobject',
 * );
 *
 * // then later call
 *
 * storeDartPostCObject(NativeApi.postCObject);
 * ```
 */
void store_dart_post_cobject(DartPostCObjectFnType ptr);
