/// Like `dart:typed_data` but generalized

export '_io.dart' if (dart.library.html) '_web.dart';
