export 'ffi/ffi_io.dart' if (dart.library.html) 'ffi/ffi_web.dart';

/// A JS function that returns a Promise to a WASM module.
typedef WasmModule = Object Function([String? moduleName]);
