import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart';
import 'package:meta/meta.dart';

export 'dart:ffi' show NativePort, DynamicLibrary;

/// {@template flutter_rust_bridge.internal}
/// The code is used only internally and is not a public API. The comment exists mainly to satisfy the linter.
/// {@endtemplate}
@internal
dynamic dartCObjectIntoDart(Dart_CObject object) {
  switch (object.type) {
    case Dart_CObject_Type.Dart_CObject_kNull:
      return null;
    case Dart_CObject_Type.Dart_CObject_kBool:
      return object.value.as_bool;
    case Dart_CObject_Type.Dart_CObject_kInt32:
      return object.value.as_int32;
    case Dart_CObject_Type.Dart_CObject_kInt64:
      return object.value.as_int64;
    case Dart_CObject_Type.Dart_CObject_kDouble:
      return object.value.as_double;

    case Dart_CObject_Type.Dart_CObject_kString:
      // `DartCObject` strings being encoded with std::ffi::CString assert us nul-termination.
      // See [allo-isolate's String::into_dart](https://github.com/sunshine-protocol/allo-isolate/blob/71b9760993d64ef46794176ca276d1cc637b2599/src/into_dart.rs#L106)
      // and [std::ffi::CString](https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html)
      int len = 0;
      // ignore: deprecated_member_use
      while (object.value.as_string.elementAt(len).value != 0) {
        len++;
      }
      return utf8
          .decode(object.value.as_string.cast<ffi.Uint8>().asTypedList(len));

    case Dart_CObject_Type.Dart_CObject_kArray:
      return List.generate(
          object.value.as_array.length, (i) => dartCObjectIntoDart(
              // ignore: deprecated_member_use
              object.value.as_array.values.elementAt(i).value.ref));

    case Dart_CObject_Type.Dart_CObject_kTypedData:
      return _typedDataIntoDart(
        ty: object.value.as_typed_data.type,
        typedValues: object.value.as_typed_data.values,
        nValues: object.value.as_typed_data.length,
      ).clone();

    case Dart_CObject_Type.Dart_CObject_kExternalTypedData:
      final converted = _typedDataIntoDart(
        ty: object.value.as_external_typed_data.type,
        typedValues: object.value.as_external_typed_data.data,
        nValues: object.value.as_external_typed_data.length,
      );

      // Copy the data
      final ans = converted.clone();

      // And immediately free the original data
      final callback = object.value.as_external_typed_data.callback
          .cast<ffi.NativeFunction<_NativeExternalTypedDataFinalizer>>()
          .asFunction<_DartExternalTypedDataFinalizer>();
      callback(
        object.value.as_external_typed_data.length,
        object.value.as_external_typed_data.peer,
      );

      return ans;

    // The commented approach enables zero-copy, but it does not tell
    // Dart VM the external object size, thus Dart VM may choose to GC too
    // sparsely.
    //
    // Later when the current non-zerocopy approach is too slow and we want to
    // further optimize, we can use the `NativeFinalizer` and tweak a few pointers
    // and maybe add an adapter function.
    //
    // _externalTypedDataFinalizer.attach(
    //   externalTypedData,
    //   // Copy the cleanup info into a finalization token:
    //   // `value`'s underlying memory will probably be freed
    //   // before the zero-copy finalizer is called.
    //   _ExternalTypedDataFinalizerArgs(
    //     length: object.value.as_external_typed_data.length,
    //     peer: object.value.as_external_typed_data.peer,
    //     callback: object.value.as_external_typed_data.callback
    //         .cast<ffi.NativeFunction<_NativeExternalTypedDataFinalizer>>(),
    //   ),
    // );
    // return externalTypedData;

    case Dart_CObject_Type.Dart_CObject_kSendPort:
    case Dart_CObject_Type.Dart_CObject_kCapability:
    case Dart_CObject_Type.Dart_CObject_kNativePointer:
    case Dart_CObject_Type.Dart_CObject_kUnsupported:
    case Dart_CObject_Type.Dart_CObject_kNumberOfTypes:
    // coverage:ignore-start
    default:
      throw Exception("Can't read invalid data type ${object.type}");
    // coverage:ignore-end
  }
}

_TypedData _typedDataIntoDart({
  required int ty,
  required ffi.Pointer<ffi.Uint8> typedValues,
  required int nValues,
}) {
  switch (ty) {
    case Dart_TypedData_Type.Dart_TypedData_kByteData:
      return _TypedData<ByteData>(
        ByteData.view(
          typedValues.cast<ffi.Uint8>().asTypedList(nValues).buffer,
        ),
        (view) => ByteData.view(
          Uint8List.fromList(view.buffer.asUint8List()).buffer,
        ),
      );
    case Dart_TypedData_Type.Dart_TypedData_kInt8:
      final view = typedValues.cast<ffi.Int8>().asTypedList(nValues);
      return _TypedData<Int8List>(view, Int8List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kUint8:
      final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
      return _TypedData<Uint8List>(view, Uint8List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kInt16:
      final view = typedValues.cast<ffi.Int16>().asTypedList(nValues);
      return _TypedData<Int16List>(view, Int16List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kUint16:
      final view = typedValues.cast<ffi.Uint16>().asTypedList(nValues);
      return _TypedData<Uint16List>(view, Uint16List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kInt32:
      final view = typedValues.cast<ffi.Int32>().asTypedList(nValues);
      return _TypedData<Int32List>(view, Int32List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kUint32:
      final view = typedValues.cast<ffi.Uint32>().asTypedList(nValues);
      return _TypedData<Uint32List>(view, Uint32List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kInt64:
      final view = typedValues.cast<ffi.Int64>().asTypedList(nValues);
      return _TypedData<Int64List>(view, Int64List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kUint64:
      final view = typedValues.cast<ffi.Uint64>().asTypedList(nValues);
      return _TypedData<Uint64List>(view, Uint64List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kFloat32:
      final view = typedValues.cast<ffi.Float>().asTypedList(nValues);
      return _TypedData<Float32List>(view, Float32List.fromList);
    case Dart_TypedData_Type.Dart_TypedData_kFloat64:
      final view = typedValues.cast<ffi.Double>().asTypedList(nValues);
      return _TypedData<Float64List>(view, Float64List.fromList);

    // coverage:ignore-start
    case Dart_TypedData_Type.Dart_TypedData_kUint8Clamped:
    case Dart_TypedData_Type.Dart_TypedData_kFloat32x4:
    case Dart_TypedData_Type.Dart_TypedData_kInvalid:
    default:
      throw Exception("Can't read invalid typed data type $ty");
    // coverage:ignore-end
  }
}

class _TypedData<T> {
  final T view;
  final T Function(T) _cloneView;

  _TypedData(this.view, this._cloneView);

  T clone() => _cloneView(view);
}

// final _externalTypedDataFinalizer =
//     Finalizer<_ExternalTypedDataFinalizerArgs>((externalTypedData) {
//   final handleFinalizer =
//       externalTypedData.callback.asFunction<_DartExternalTypedDataFinalizer>();
//   handleFinalizer(externalTypedData.length, externalTypedData.peer);
//
//   debugOnExternalTypedDataFinalizer?.call(externalTypedData.length);
// });
//
// /// {@macro flutter_rust_bridge.internal}
// void Function(int dataLength)? debugOnExternalTypedDataFinalizer;
//
// class _ExternalTypedDataFinalizerArgs {
//   final int length;
//   final ffi.Pointer<ffi.Void> peer;
//   final ffi.Pointer<ffi.NativeFunction<_NativeExternalTypedDataFinalizer>>
//       callback;
//
//   _ExternalTypedDataFinalizerArgs({
//     required this.length,
//     required this.peer,
//     required this.callback,
//   });
// }

typedef _NativeExternalTypedDataFinalizer = ffi.Void Function(
    ffi.IntPtr, ffi.Pointer<ffi.Void>);
typedef _DartExternalTypedDataFinalizer = void Function(
    int, ffi.Pointer<ffi.Void>);
