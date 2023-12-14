import 'dart:ffi' as ffi;

import 'package:bug/src/use_rust_bindings_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  final lib = ffi.DynamicLibrary.open(
    // 'rust/target/debug/libfrb_example_dart_minimal.dylib',
    '/Volumes/MyExternal/ExternalRefCode/flutter_rust_bridge/bug_dcharkes/rust/target/debug/libfrb_example_dart_minimal.dylib',
    // '/Users/dacoharkes/src/fzyzcjy/flutter_rust_bridge/bug/rust/target/debug/libfrb_example_dart_minimal.dylib',
  );
  final binding = UseRustBindings(lib);
  binding.InitializeApiDL(ffi.NativeApi.initializeApiDLData);

  // NOTE: you can remove this `test(...)` closure and directly run the code,
  // and it is the same
  test('hello test', () {
    int x = 42;
    final persistentHandle = binding.NewPersistentHandle(x);
    binding.HandleFromPersistent(persistentHandle);
  });
}
