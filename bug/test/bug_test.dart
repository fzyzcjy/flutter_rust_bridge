import 'dart:ffi' as ffi;

Future<void> main() async {
  // TODO build release?
  final lib = ffi.DynamicLibrary.open(
      'rust/target/debug/libfrb_example_dart_minimal.dylib');
  final binding = MultiPackageCBinding(lib);
  binding.init_frb_dart_api_dl(ffi.NativeApi.initializeApiDLData);

  String f() => 'Test_String';
  final persistentHandle = binding.naive_NewPersistentHandle(f);
  binding.naive_HandleFromPersistent(persistentHandle);
}

class MultiPackageCBinding {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  MultiPackageCBinding(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  MultiPackageCBinding.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> data,
  ) {
    return _init_frb_dart_api_dl(
      data,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  int naive_NewPersistentHandle(
    Object non_persistent_handle,
  ) {
    return _naive_NewPersistentHandle(
      non_persistent_handle,
    );
  }

  late final _naive_NewPersistentHandlePtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>(
          'naive_NewPersistentHandle');
  late final _naive_NewPersistentHandle =
      _naive_NewPersistentHandlePtr.asFunction<int Function(Object)>();

  int naive_HandleFromPersistent(
    int persistent_handle,
  ) {
    return _naive_HandleFromPersistent(
      persistent_handle,
    );
  }

  late final _naive_HandleFromPersistentPtr =
      _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.UintPtr)>>(
          'naive_HandleFromPersistent');
  late final _naive_HandleFromPersistent =
      _naive_HandleFromPersistentPtr.asFunction<int Function(int)>();
}
