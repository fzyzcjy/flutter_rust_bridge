/// Like Dart's `isolate`, but works for both native and Web.
library;

import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.internal}
String serializeNativePort(NativePortType port) => port.toString();
