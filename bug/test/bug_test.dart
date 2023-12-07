import 'dart:ffi';

Future<void> main() async {
  // TODO build release?
  final binding = DynamicLibrary.open(
      'rust/target/debug/libfrb_example_dart_minimal.dylib');
  String f() => 'Test_String';
  final persistentHandle = binding.naive_NewPersistentHandle(f);
  binding.naive_HandleFromPersistent(persistentHandle);
}
