/// Platform-dependent helpers.
library platform;

export 'platform.io.dart' if (dart.library.js) 'platform.js.dart';
