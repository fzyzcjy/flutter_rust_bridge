/// Please refer to https://github.com/fzyzcjy/flutter_rust_bridge for full documentation.
///
/// The API exposed here is thin, because usually you only need to call the (rich-featured)
/// automatically generated code.
///
/// You often do not need to use the APIs here, except (for example) when you want to customize some behaviors.
library;

export 'src/exceptions.dart';
export 'src/generalized_typed_data/generalized_typed_data.dart'
    show Int64List, Uint64List;
export 'src/loader/loader.dart' show loadExternalLibrary;
export 'src/main_components/handler.dart' show BaseHandler;
export 'src/task.dart' show NormalTask, SyncTask;
export 'src/stream/stream_sink.dart' show RustStreamSink;
