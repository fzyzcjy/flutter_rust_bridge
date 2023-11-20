/// Like Dart's `isolate`, but works for both native and Web.

export '_generalized_isolate_io.dart' if (dart.library.html) '_generalized_isolate_web.dart';
