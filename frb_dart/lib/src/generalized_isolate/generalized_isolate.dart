/// Like Dart's `isolate`, but works for both native and Web.
library;

export '_io.dart' if (dart.library.html) '_web.dart';
