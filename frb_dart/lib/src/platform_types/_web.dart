import 'dart:async';

import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

typedef NativePortType = dynamic;

typedef WireSyncReturn = List<dynamic>;

typedef PlatformPointer = int;

typedef ExternalLibrary = FutureOr<WasmModule>;

typedef DartPostCObject = void;
