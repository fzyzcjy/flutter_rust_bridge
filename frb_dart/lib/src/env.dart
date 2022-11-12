// see https://github.com/flutter/flutter/issues/55870

export 'env/io.dart' if (dart.library.html) 'env/web.dart';
