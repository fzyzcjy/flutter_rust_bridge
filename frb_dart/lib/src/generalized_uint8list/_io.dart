import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/generalized_uint8list/rust_vec_u8.dart';

/// {@macro flutter_rust_bridge.internal}
typedef PlatformGeneralizedUint8List = RustVecU8;

/// {@macro flutter_rust_bridge.internal}
typedef PlatformGeneralizedUint8ListPtr = ffi.Pointer<ffi.Uint8>;
