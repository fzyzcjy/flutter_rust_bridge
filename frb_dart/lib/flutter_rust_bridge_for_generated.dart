/// {@macro flutter_rust_bridge.only_for_generated_code}
library;

export 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart'
    if (dart.library.js_interop) 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
export 'package:flutter_rust_bridge/src/logging/frb_logging.dart'
    show FrbDartLogging, FrbLogRecordData, kFrbDartLogging;
