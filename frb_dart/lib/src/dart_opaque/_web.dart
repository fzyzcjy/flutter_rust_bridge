import 'dart:js_util';

/// {@macro flutter_rust_bridge.internal}
Object prepareDartOpaqueForEncoding(Object raw) {
  // #2183
  if (raw is Function) {
    return allowInterop(raw);
  }
  return raw;
}
