/// Like Dart's `isolate`, but works for both native and Web.

export '_io.dart' if (dart.library.html) '_web.dart';
