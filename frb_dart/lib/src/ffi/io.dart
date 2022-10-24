import 'dart:ffi' as ffi;
import 'dart:ffi';
export 'dart:ffi' show NativePort, DynamicLibrary;
import 'dart:typed_data';
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

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external ffi.Pointer<ffi.Uint8> ptr;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Int32()
  external int len;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Uint8()
  external int success;

  Uint8List get buffer => Uint8List.fromList(ptr.asTypedList(len));
  bool get isSuccess => success > 0;
}

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at some point during runtime.
class FrbOpaque implements Finalizable {
  ffi.Pointer? _ptr;
  late ffi.Pointer<ffi.NativeFunction<ffi.Void Function(ffi.Pointer)>> _drop;
  late ffi.Pointer<ffi.NativeFunction<ffi.Pointer Function(ffi.Pointer)>>
      _share;

  /// This constructor should never be called manually.
  FrbOpaque.unsafe(int ptr, int drop, int share) {
    assert(ptr > 0);
    assert(drop > 0);
    assert(share > 0);
    _ptr = ffi.Pointer.fromAddress(ptr);
    _drop = ffi.Pointer.fromAddress(drop);
    _share = ffi.Pointer.fromAddress(share);
    _finalizer = NativeFinalizer(ffi.Pointer.fromAddress(drop));
    _finalizer.attach(this, _ptr!.cast(), detach: this);
  }

  /// The native finalizer runs [_drop] on [_ptr]
  /// if the object is garbage collected.
  late final NativeFinalizer _finalizer;

  /// Call Rust destructors on the backing memory of this pointer.
  /// This function should be run at least once during the lifetime of the program,
  /// and can be run many times.
  ///
  /// When passed into a Rust function,
  /// Rust enacts *shared ownership* and inhibits disposal of this pointer's contents,
  /// even if [dispose] is immediately run.
  /// Furthermore, if that same function reuses the allocation
  /// (usually by returning the same opaque pointer)
  /// ownership of this pointer will be moved into that new opaque pointer.
  void dispose() {
    if (!isStale()) {
      _finalizer.detach(this);
      _drop.asFunction<void Function(ffi.Pointer)>()(_ptr!);
      _ptr = null;
    }
  }

  /// Returns pointer with shares ownership if Dart owner else throws error.
  static ffi.Pointer share(FrbOpaque ptr) {
    if (!ptr.isStale()) {
      return ptr._share
          .asFunction<ffi.Pointer Function(ffi.Pointer)>()(ptr._ptr!);
    } else {
      throw "Use after dispose";
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has actually
  /// been reclaimed.
  // not nullptr, this is an internal bookkeeping method
  bool isStale() => _ptr == null;
}
