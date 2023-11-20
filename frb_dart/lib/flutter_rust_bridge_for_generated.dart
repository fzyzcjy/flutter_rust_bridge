/// {@template flutter_rust_bridge.only_for_generated_code}
/// This is only intended to be used by automatically generated code,
/// instead of developers.
/// {@endtemplate}
library;

export 'dart:typed_data' hide Int64List, Uint64List;

export 'package:meta/meta.dart' show internal, protected;

export 'flutter_rust_bridge.dart';
export 'src/exceptions.dart';
export 'src/main_components/bulk.dart';
export 'src/main_components/dispatcher.dart';
export 'src/main_components/entrypoint.dart';
export 'src/main_components/handler.dart';
export 'src/main_components/wire/wire.dart';
export 'src/task.dart';
export 'src/utils/io_utils.dart' if (dart.library.html) 'src/utils/web_utils.dart';

// TODO export more
