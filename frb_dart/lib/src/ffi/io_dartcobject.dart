/// These bindings directly map Rust's allo_isolate::ffi
/// Note that allo_isolate itself reverse-engineered Dart's Dart_CObject,
/// but Dart's sdk doesn't expose its contents.
// ignore_for_file: non_constant_identifier_names, unused_field

import 'dart:ffi' as ffi;

class DartNativeSendPort extends ffi.Struct {
  @ffi.Int()
  external int id;

  @ffi.Int()
  external int origin_id;
}

class DartNativeCapability extends ffi.Struct {
  @ffi.Int()
  external int id;
}

class DartNativeArray extends ffi.Struct {
  @ffi.Size()
  external int length;

  external ffi.Pointer<ffi.Pointer<DartCObject>> values;
}

class DartNativeTypedData extends ffi.Struct {
  @ffi.Int()
  external int ty;

  @ffi.Size()
  external int length;

  external ffi.Pointer<ffi.Void> values;
}

class DartNativeExternalTypedData extends ffi.Struct {
  @ffi.Int()
  external int ty;

  @ffi.Size()
  external int length;

  external ffi.Pointer<ffi.Void> data;

  external ffi.Pointer<ffi.Void> peer;

  external ffi.Pointer<ffi.Void> callback;
}

class DartNativePointer extends ffi.Struct {
  @ffi.Size()
  external int ptr;

  @ffi.Size()
  external int size;

  @ffi.Int()
  external int callback;
}

class DartCObjectValue extends ffi.Union {
  @ffi.Bool()
  external bool as_bool;

  @ffi.Int()
  external int as_int32;

  @ffi.Int64()
  external int as_int64;

  @ffi.Double()
  external double as_double;

  external ffi.Pointer<ffi.Char> as_string;

  external DartNativeSendPort as_send_port;

  external DartNativeCapability as_capability;

  external DartNativeArray as_array;

  external DartNativeTypedData as_typed_data;

  external DartNativeExternalTypedData as_external_typed_data;

  external DartNativePointer as_native_pointer;

  @ffi.Array.multi([5])
  external ffi.Array<ffi.Int> _bindgen_union_align;
}

class DartCObject extends ffi.Struct {
  @ffi.Int()
  external int ty;

  external DartCObjectValue value;
}
