import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
export 'dart:ffi' show NativePort, DynamicLibrary;
import 'dart:typed_data';
import 'io_dartcobject.dart';
import 'stub.dart' show FlutterRustBridgeWireBase;
export 'stub.dart'
    show castInt, castNativeBigInt, FlutterRustBridgeWireBase, WasmModule;

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;
typedef ExternalLibrary = ffi.DynamicLibrary;
typedef DartPostCObject = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}

class DartApiDl {
  static int? _initCode;
  final int Function(ffi.Pointer<ffi.Void>) _initFn;
  DartApiDl(this._initFn);

  void initApi() {
    _initCode ??= _initFn(ffi.NativeApi.initializeApiDLData);
    if (_initCode != 0) {
      throw 'Failed to initialize Dart API. Code: $_initCode';
    }
  }
}

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external DartCObject data;

  @ffi.Bool()
  external bool isSuccess;

  external ffi.Pointer<ffi.Void> ptr;

  dynamic intoDart() => data.intoDart();
}

extension DartCObjectWireSyncReturn on DartCObject {
  dynamic intoDart() {
    switch (ty) {
      case 0: // DartNull
        return null;
      case 1: // DartBool
        return value.as_bool;
      case 2: // DartInt32
        return value.as_int32;
      case 3: // DartInt64
        return value.as_int64;
      case 4: // DartDouble
        return value.as_double;

      case 5: // DartString
        int len = 0;
        while (value.as_string.elementAt(len).value != 0) {
          len++;
        }
        return utf8.decode(value.as_string.cast<ffi.Uint8>().asTypedList(len));

      case 6: // DartArray
        return List.generate(value.as_array.length,
            (i) => value.as_array.values.elementAt(i).value.ref.intoDart());

      case 7: // DartTypedData
        return _typedDataIntoDart(
          value.as_typed_data.ty,
          value.as_typed_data.values,
          value.as_typed_data.length,
          copy: true,
        );

      case 8: // DartExternalTypedData
        final externalTypedData = _typedDataIntoDart(
          value.as_external_typed_data.ty,
          value.as_external_typed_data.data,
          value.as_external_typed_data.length,
          copy: false,
        );
        _externalTypedDataFinalizer.attach(
            externalTypedData, value.as_external_typed_data);
        return externalTypedData;

      case 9: // DartSendPort
      case 10: // DartCapability
      case 11: // DartNativePointer
      case 12: // DartUnsupported
      case 13: // DartNumberOfTypes
      default:
        throw "Can't read invalid data type $ty";
    }
  }

  static dynamic _typedDataIntoDart(
    int ty,
    ffi.Pointer<ffi.Void> typedValues,
    int nValues, {
    required bool copy,
  }) {
    switch (ty) {
      case 0: // ByteData
        final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
        final bytes = copy ? view : Uint8List.fromList(view);
        return ByteData.view(bytes.buffer);
      case 1: // Int8
        final view = typedValues.cast<ffi.Int8>().asTypedList(nValues);
        return copy ? Int8List.fromList(view) : view;
      case 2: // Uint8
        final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
        return copy ? Uint8List.fromList(view) : view;
      case 4: // Int16
        final view = typedValues.cast<ffi.Int16>().asTypedList(nValues);
        return copy ? Int16List.fromList(view) : view;
      case 5: // Uint16
        final view = typedValues.cast<ffi.Uint16>().asTypedList(nValues);
        return copy ? Uint16List.fromList(view) : view;
      case 6: // Int32
        final view = typedValues.cast<ffi.Int32>().asTypedList(nValues);
        return copy ? Int32List.fromList(view) : view;
      case 7: // Uint32
        final view = typedValues.cast<ffi.Uint32>().asTypedList(nValues);
        return copy ? Uint32List.fromList(view) : view;
      case 8: // Int64
        final view = typedValues.cast<ffi.Int64>().asTypedList(nValues);
        return copy ? Int64List.fromList(view) : view;
      case 9: // Uint64
        final view = typedValues.cast<ffi.Uint64>().asTypedList(nValues);
        return copy ? Uint64List.fromList(view) : view;
      case 10: // Float32
        final view = typedValues.cast<ffi.Float>().asTypedList(nValues);
        return copy ? Float32List.fromList(view) : view;
      case 11: // Float64
        final view = typedValues.cast<ffi.Double>().asTypedList(nValues);
        return copy ? Float64List.fromList(view) : view;

      case 3: // Uint8Clamped
      case 12: // Float32x4
      case 13: // Invalid
      default:
        throw "Can't read invalid typed data type $ty";
    }
  }

  static final _externalTypedDataFinalizer =
      Finalizer<DartNativeExternalTypedData>((externalTypedData) {
    final handleFinalizer = externalTypedData.callback
        .cast<
            ffi.NativeFunction<
                ffi.Void Function(
                    ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>>()
        .asFunction<
            void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>)>();
    handleFinalizer(externalTypedData.data, externalTypedData.peer);
  });
}

typedef PlatformPointer = ffi.Pointer<ffi.Void>;
typedef OpaqueTypeFinalizer = NativeFinalizer;

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
class FrbOpaqueBase implements Finalizable {
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);
  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;
  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr,
          int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
