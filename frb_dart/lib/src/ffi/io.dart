import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:io';
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
      throw Exception('Failed to initialize Dart API. Code: $_initCode');
    }
  }
}

typedef WireSyncReturn = ffi.Pointer<DartCObject>;

dynamic wireSyncReturnIntoDart(WireSyncReturn syncReturn) =>
    syncReturn.ref.intoDart();

extension DartCObjectExt on DartCObject {
  dynamic intoDart() {
    switch (ty) {
      case DartCObjectType.DartNull:
        return null;
      case DartCObjectType.DartBool:
        return value.as_bool;
      case DartCObjectType.DartInt32:
        return value.as_int32;
      case DartCObjectType.DartInt64:
        return value.as_int64;
      case DartCObjectType.DartDouble:
        return value.as_double;

      case DartCObjectType.DartString:
        // `DartCObject` strings being encoded with std::ffi::CString assert us nul-termination.
        // See [allo-isolate's String::into_dart](https://github.com/sunshine-protocol/allo-isolate/blob/71b9760993d64ef46794176ca276d1cc637b2599/src/into_dart.rs#L106)
        // and [std::ffi::CString](https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html)
        int len = 0;
        while (value.as_string.elementAt(len).value != 0) {
          len++;
        }
        return utf8.decode(value.as_string.cast<ffi.Uint8>().asTypedList(len));

      case DartCObjectType.DartArray:
        return List.generate(value.as_array.length,
            (i) => value.as_array.values.elementAt(i).value.ref.intoDart());

      case DartCObjectType.DartTypedData:
        return _typedDataIntoDart(
          value.as_typed_data.ty,
          value.as_typed_data.values,
          value.as_typed_data.length,
        ).clone();

      case DartCObjectType.DartExternalTypedData:
        final externalTypedData = _typedDataIntoDart(
          value.as_external_typed_data.ty,
          value.as_external_typed_data.data,
          value.as_external_typed_data.length,
        ).view;
        _externalTypedDataFinalizer.attach(
          externalTypedData,
          // Copy the cleanup info into a finalization token:
          // `value`'s underlying memory will probably be freed
          // before the zero-copy finalizer is called.
          _ExternalTypedDataFinalizerArgs(value.as_external_typed_data),
        );
        return externalTypedData;

      case DartCObjectType.DartSendPort:
      case DartCObjectType.DartCapability:
      case DartCObjectType.DartNativePointer:
      case DartCObjectType.DartUnsupported:
      case DartCObjectType.DartNumberOfTypes:
      default:
        throw Exception("Can't read invalid data type $ty");
    }
  }

  static _TypedData _typedDataIntoDart(
    int ty,
    ffi.Pointer<ffi.Uint8> typedValues,
    int nValues,
  ) {
    switch (ty) {
      case DartTypedDataType.ByteData:
        return _TypedData<ByteData>(
          ByteData.view(
            typedValues.cast<ffi.Uint8>().asTypedList(nValues).buffer,
          ),
          (view) => ByteData.view(
            Uint8List.fromList(view.buffer.asUint8List()).buffer,
          ),
        );
      case DartTypedDataType.Int8:
        final view = typedValues.cast<ffi.Int8>().asTypedList(nValues);
        return _TypedData<Int8List>(view, Int8List.fromList);
      case DartTypedDataType.Uint8:
        final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
        return _TypedData<Uint8List>(view, Uint8List.fromList);
      case DartTypedDataType.Int16:
        final view = typedValues.cast<ffi.Int16>().asTypedList(nValues);
        return _TypedData<Int16List>(view, Int16List.fromList);
      case DartTypedDataType.Uint16:
        final view = typedValues.cast<ffi.Uint16>().asTypedList(nValues);
        return _TypedData<Uint16List>(view, Uint16List.fromList);
      case DartTypedDataType.Int32:
        final view = typedValues.cast<ffi.Int32>().asTypedList(nValues);
        return _TypedData<Int32List>(view, Int32List.fromList);
      case DartTypedDataType.Uint32:
        final view = typedValues.cast<ffi.Uint32>().asTypedList(nValues);
        return _TypedData<Uint32List>(view, Uint32List.fromList);
      case DartTypedDataType.Int64:
        final view = typedValues.cast<ffi.Int64>().asTypedList(nValues);
        return _TypedData<Int64List>(view, Int64List.fromList);
      case DartTypedDataType.Uint64:
        final view = typedValues.cast<ffi.Uint64>().asTypedList(nValues);
        return _TypedData<Uint64List>(view, Uint64List.fromList);
      case DartTypedDataType.Float32:
        final view = typedValues.cast<ffi.Float>().asTypedList(nValues);
        return _TypedData<Float32List>(view, Float32List.fromList);
      case DartTypedDataType.Float64:
        final view = typedValues.cast<ffi.Double>().asTypedList(nValues);
        return _TypedData<Float64List>(view, Float64List.fromList);

      case DartTypedDataType.Uint8Clamped:
      case DartTypedDataType.Float32x4:
      case DartTypedDataType.Invalid:
      default:
        throw Exception("Can't read invalid typed data type $ty");
    }
  }

  static final _externalTypedDataFinalizer =
      Finalizer<_ExternalTypedDataFinalizerArgs>((externalTypedData) {
    final handleFinalizer =
        externalTypedData.callback.asFunction<DartExternalTypedDataFinalizer>();
    handleFinalizer(externalTypedData.length, externalTypedData.peer);

    if (Platform.environment.containsKey('FRB_TEST')) {
      for (var handler in ioTestTool!.onExternalTypedDataFinalizer) {
        handler(externalTypedData.length);
      }
    }
  });
}

class _TypedData<T> {
  final T view;
  final T Function(T) _cloneView;
  _TypedData(this.view, this._cloneView);

  T clone() => _cloneView(view);
}

class _ExternalTypedDataFinalizerArgs {
  final int length;
  final ffi.Pointer<ffi.Void> peer;
  final ffi.Pointer<ffi.NativeFunction<NativeExternalTypedDataFinalizer>>
      callback;

  _ExternalTypedDataFinalizerArgs(DartNativeExternalTypedData typedData)
      : length = typedData.length,
        peer = typedData.peer,
        callback = typedData.callback
            .cast<ffi.NativeFunction<NativeExternalTypedDataFinalizer>>();
}

typedef NativeExternalTypedDataFinalizer = ffi.Void Function(
    ffi.IntPtr, ffi.Pointer<ffi.Void>);
typedef DartExternalTypedDataFinalizer = void Function(
    int, ffi.Pointer<ffi.Void>);

class _TestTool {
  final Set<void Function(int)> onExternalTypedDataFinalizer = {};
}

final ioTestTool =
    Platform.environment.containsKey('FRB_TEST') ? _TestTool() : null;

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
