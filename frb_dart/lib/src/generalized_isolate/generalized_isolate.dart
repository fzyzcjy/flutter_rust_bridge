/// Like Dart's `isolate`, but works for both native and Web.
library;

export '_io.dart' if (dart.library.js_interop) '_web.dart';
