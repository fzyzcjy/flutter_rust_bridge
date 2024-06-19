/// Like `dart:typed_data` but generalized
library;

export '_io.dart' if (dart.library.js_interop) '_web.dart';
