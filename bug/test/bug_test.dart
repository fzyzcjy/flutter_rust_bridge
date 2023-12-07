import 'dart:ffi' as ffi;

Future<void> main() async {
  // TODO build release?
  final lib = ffi.DynamicLibrary.open(
      'rust/target/debug/libfrb_example_dart_minimal.dylib');
  final binding = MultiPackageCBinding(lib);
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
