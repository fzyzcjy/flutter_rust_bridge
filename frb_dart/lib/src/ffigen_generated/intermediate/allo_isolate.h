#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A Dart_CObject is used for representing Dart objects as native C
 * data outside the Dart heap. These objects are totally detached from
 * the Dart heap. Only a subset of the Dart objects have a
 * representation as a Dart_CObject.
 *
 * The string encoding in the 'value.as_string' is UTF-8.
 *
 * All the different types from dart:typed_data are exposed as type
 * kTypedData. The specific type from dart:typed_data is in the type
 * field of the as_typed_data structure. The length in the
 * as_typed_data structure is always in bytes.
 *
 * The data for kTypedData is copied on message send and ownership remains with
 * the caller. The ownership of data for kExternalTyped is passed to the VM on
 * message send and returned when the VM invokes the
 * Dart_WeakPersistentHandleFinalizer callback; a non-NULL callback must be
 * provided.
 *
 * https://github.com/dart-lang/sdk/blob/main/runtime/include/dart_native_api.h
 */
enum DartCObjectType {
  DartNull = 0,
  DartBool = 1,
  DartInt32 = 2,
  DartInt64 = 3,
  DartDouble = 4,
  DartString = 5,
  DartArray = 6,
  DartTypedData = 7,
  DartExternalTypedData = 8,
  DartSendPort = 9,
  DartCapability = 10,
  DartNativePointer = 11,
  DartUnsupported = 12,
  DartNumberOfTypes = 13,
};
typedef int32_t DartCObjectType;

enum DartTypedDataType {
  ByteData = 0,
  Int8 = 1,
  Uint8 = 2,
  Uint8Clamped = 3,
  Int16 = 4,
  Uint16 = 5,
  Int32 = 6,
  Uint32 = 7,
  Int64 = 8,
  Uint64 = 9,
  Float32 = 10,
  Float64 = 11,
  Float32x4 = 12,
  Invalid = 13,
};
typedef int32_t DartTypedDataType;

/**
 * A port is used to send or receive inter-isolate messages
 */
typedef int64_t DartPort;

typedef struct DartNativeSendPort {
  DartPort id;
  DartPort origin_id;
} DartNativeSendPort;

typedef struct DartNativeCapability {
  int64_t id;
} DartNativeCapability;

typedef struct DartNativeArray {
  intptr_t length;
  struct DartCObject **values;
} DartNativeArray;

typedef struct DartNativeTypedData {
  DartTypedDataType ty;
  intptr_t length;
  uint8_t *values;
} DartNativeTypedData;

/**
 * https://github.com/dart-lang/sdk/blob/main/runtime/include/dart_api.h
 */
typedef void (*DartHandleFinalizer)(void *isolate_callback_data, void *peer);

typedef struct DartNativeExternalTypedData {
  DartTypedDataType ty;
  intptr_t length;
  uint8_t *data;
  void *peer;
  DartHandleFinalizer callback;
} DartNativeExternalTypedData;

typedef struct DartNativePointer {
  intptr_t ptr;
  intptr_t size;
  DartHandleFinalizer callback;
} DartNativePointer;

typedef union DartCObjectValue {
  bool as_bool;
  int32_t as_int32;
  int64_t as_int64;
  double as_double;
  char *as_string;
  struct DartNativeSendPort as_send_port;
  struct DartNativeCapability as_capability;
  struct DartNativeArray as_array;
  struct DartNativeTypedData as_typed_data;
  struct DartNativeExternalTypedData as_external_typed_data;
  struct DartNativePointer as_native_pointer;
  uint64_t _bindgen_union_align[5];
} DartCObjectValue;

typedef struct DartCObject {
  DartCObjectType ty;
  union DartCObjectValue value;
} DartCObject;

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
typedef bool (*DartPostCObjectFnType)(DartPort port_id, struct DartCObject *message);

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
