// Trampolines to Dynamically Linked Dart API.
//
// Trampolines allow to call Dynamically Linked Dart API from Rust.
//
// This must be compiled and linked into final library, so Rust can call these
// methods.
//
// All declared methods are simply calling Dart DL API methods with same name
// (without *_Trampolined prefix).

#include "./include/dart_api_dl.c"

Dart_PersistentHandle Dart_NewPersistentHandle_DL_Trampolined(
    Dart_Handle handle) {
  return Dart_NewPersistentHandle_DL(handle);
}

Dart_Handle Dart_HandleFromPersistent_DL_Trampolined(
    Dart_PersistentHandle handle) {
  return Dart_HandleFromPersistent_DL(handle);
}

void Dart_DeletePersistentHandle_DL_Trampolined(Dart_PersistentHandle handle) {
  Dart_DeletePersistentHandle_DL(handle);
}

bool Dart_PostCObject_DL_Trampolined(Dart_Port port_id, Dart_CObject* message) {
  return Dart_PostCObject_DL(port_id, message);
}

Dart_FinalizableHandle Dart_NewFinalizableHandle_DL_Trampolined(
    Dart_Handle object,
    void* peer,
    intptr_t external_allocation_size,
    Dart_HandleFinalizer callback) {
  return Dart_NewFinalizableHandle_DL(object,
                                      peer,
                                      external_allocation_size,
                                      callback);
}

bool Dart_IsError_DL_Trampolined(Dart_Handle handle) {
  return Dart_IsError_DL(handle);
}

const char* Dart_GetError_DL_Trampolined(Dart_Handle handle) {
  return Dart_GetError_DL(handle);
}

void Dart_PropagateError_DL_Trampolined(Dart_Handle handle) {
  Dart_PropagateError_DL(handle);
}
