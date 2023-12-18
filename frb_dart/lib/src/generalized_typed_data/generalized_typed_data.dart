/// Like `dart:typed_data` but generalized
library;

export '_io.dart' if (dart.library.html) '_web.dart';
