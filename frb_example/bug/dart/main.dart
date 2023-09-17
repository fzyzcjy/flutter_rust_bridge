import 'dart:ffi' as ffi;
import 'dart:ffi';

typedef NativeRustHelloFunction = ffi.Void Function();
typedef NativeHelloFunction = void Function();

void main() {
  final dl = ffi.DynamicLibrary.open("../../../target/debug/libhello_rust.so");
  final f = dl.lookupFunction<NativeRustHelloFunction, NativeHelloFunction>("hello_rust_function");
  f();
}
